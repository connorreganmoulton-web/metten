#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use metten_engine::{Action, Engine, Snapshot};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;

struct Workspace {
    engine: Result<Engine, String>,
    quitting: AtomicBool,
    exit_ready: AtomicBool,
    quit_error: std::sync::Mutex<Option<String>>,
}

#[tauri::command]
fn snapshot(workspace: tauri::State<'_, Workspace>) -> Result<Snapshot, String> {
    let mut snapshot = workspace.engine.as_ref().map_err(Clone::clone)?.snapshot();
    if let Some(error) = workspace
        .quit_error
        .lock()
        .map_err(|e| e.to_string())?
        .as_ref()
    {
        snapshot.recording_error = Some(error.clone());
    }
    Ok(snapshot)
}

#[tauri::command]
fn control(action: Action, workspace: tauri::State<'_, Workspace>) -> Result<(), String> {
    if workspace.quitting.load(Ordering::SeqCst) {
        return Err("Quitting — wait for the stopped state to be saved.".into());
    }
    workspace
        .engine
        .as_ref()
        .map_err(Clone::clone)?
        .request(action)
}

fn begin_quit(app: &tauri::AppHandle) {
    let workspace = app.state::<Workspace>();
    if workspace.quitting.swap(true, Ordering::SeqCst) {
        return;
    }
    let app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let workspace = app.state::<Workspace>();
        let result = workspace
            .engine
            .as_ref()
            .map_or(Ok(()), Engine::stop_and_save);
        match result {
            Ok(()) => {
                workspace.exit_ready.store(true, Ordering::SeqCst);
                app.exit(0);
            }
            Err(error) => {
                *workspace
                    .quit_error
                    .lock()
                    .expect("quit error state poisoned") = Some(error);
                workspace.quitting.store(false, Ordering::SeqCst);
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
    });
}

fn main() {
    tauri::Builder::default()
        .menu(|app| {
            let menu = tauri::menu::Menu::default(app)?;
            let quit_text = tauri::menu::PredefinedMenuItem::quit(app, None)?.text()?;
            // macOS's predefined Quit terminates before Tauri can prevent exit.
            for item in menu.items()? {
                if let Some(submenu) = item.as_submenu() {
                    for (index, item) in submenu.items()?.iter().enumerate() {
                        if let Some(predefined) = item.as_predefined_menuitem() && predefined.text()? == quit_text {
                            submenu.remove_at(index)?;
                            submenu.insert(&tauri::menu::MenuItem::with_id(app, "quit-session", "Quit Metten", true, Some("CmdOrCtrl+Q"))?, index)?;
                        }
                    }
                }
            }
            Ok(menu)
        })
        .on_menu_event(|app, event| {
            if event.id().as_ref() == "quit-session" { begin_quit(app); }
        })
        .setup(|app| {
            // Only the native process may select storage. Smoke tests use a fresh directory.
            let engine = (|| -> Result<Engine, String> {
                let directory = match std::env::var_os("METTEN_TEST_WORKSPACE") {
                    Some(path) => std::path::PathBuf::from(path),
                    None => app.path().app_data_dir().map_err(|e| e.to_string())?,
                };
                std::fs::create_dir_all(&directory).map_err(|e| format!("Cannot open workspace. Check disk space and folder permissions, then reopen. {e}"))?;
                Engine::open(directory.join("session.sqlite"))
            })();
            app.manage(Workspace { engine, quitting: AtomicBool::new(false), exit_ready: AtomicBool::new(false), quit_error: std::sync::Mutex::new(None) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![snapshot, control])
        .build(tauri::generate_context!())
        .expect("Could not open the Metten application")
        .run(|app, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                if !app.state::<Workspace>().exit_ready.load(Ordering::SeqCst) {
                    api.prevent_exit();
                    begin_quit(app);
                }
            }
            tauri::RunEvent::WindowEvent { event: tauri::WindowEvent::CloseRequested { api, .. }, .. } => {
                api.prevent_close();
                begin_quit(app);
            }
            _ => (),
        });
}
