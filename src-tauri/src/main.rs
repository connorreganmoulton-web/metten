#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
mod credential;

use metten_engine::{Action, Engine, Snapshot};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;

struct Workspace {
    engine: Result<Engine, String>,
    #[cfg(target_os = "macos")]
    credentials: credential::Credentials,
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

// Decode manually: framework type errors must never quote submitted values.
fn decode_credential(body: &tauri::ipc::InvokeBody) -> Result<(&str, Option<&str>), &'static str> {
    const INVALID: &str = "Invalid credential request.";
    let tauri::ipc::InvokeBody::Json(body) = body else {
        return Err(INVALID);
    };
    let object = body.as_object().ok_or(INVALID)?;
    if object.keys().any(|k| k != "operation" && k != "value") {
        return Err(INVALID);
    }
    let operation = body
        .get("operation")
        .and_then(|v| v.as_str())
        .ok_or(INVALID)?;
    let value = match operation {
        "save" | "replace" => Some(body.get("value").and_then(|v| v.as_str()).ok_or(INVALID)?),
        "remove" | "check" if !object.contains_key("value") => None,
        _ => return Err(INVALID),
    };
    Ok((operation, value))
}

#[tauri::command]
async fn credential(
    request: tauri::ipc::Request<'_>,
    app: tauri::AppHandle,
) -> Result<&'static str, &'static str> {
    let (operation, value) = decode_credential(request.body())?;
    let operation = operation.to_owned();
    let value = value.map(str::to_owned);
    #[cfg(target_os = "macos")]
    return tauri::async_runtime::spawn_blocking(move || {
        let workspace = app.state::<Workspace>();
        let expected = if operation == "check" {
            workspace.credentials.expectation().map(Some)
        } else {
            Ok(None)
        };
        let _operation = workspace.credentials.begin_operation(&workspace.quitting)?;
        let expected = expected?;
        workspace
            .credentials
            .execute(&operation, value.as_deref(), expected.as_deref())
    })
    .await
    .map_err(|_| "Credential access unavailable — reopen the app.")?;
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (app, operation, value);
        Err("Credential access requires the isolated Mac verification runner.")
    }
}

fn begin_quit(app: &tauri::AppHandle) {
    let workspace = app.state::<Workspace>();
    if workspace.quitting.swap(true, Ordering::SeqCst) {
        return;
    }
    let app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let workspace = app.state::<Workspace>();
        #[cfg(target_os = "macos")]
        let _credential_operation = workspace
            .credentials
            .operation
            .lock()
            .expect("credential operation mutex poisoned");
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

fn context() -> tauri::Context<tauri::Wry> {
    tauri::generate_context!()
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
            app.manage(Workspace { engine, #[cfg(target_os = "macos")] credentials: credential::Credentials::setup(), quitting: AtomicBool::new(false), exit_ready: AtomicBool::new(false), quit_error: std::sync::Mutex::new(None) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![snapshot, control, credential])
        .build(context())
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

#[cfg(test)]
mod permissions_tests {
    #[cfg(target_os = "macos")]
    #[test]
    fn credential_decode_never_echoes_untrusted_input() {
        use serde_json::json;
        use tauri::ipc::InvokeBody;
        let marker = "metten-dummy-parser-only-marker";
        for body in [
            json!(marker),
            json!({"operation": marker}),
            json!({"operation":"save", "value": [marker]}),
            json!({"operation":"replace", "value": null}),
            json!({"operation":"remove", "value":marker}),
            json!({"operation":"check", "path":marker}),
            json!({"operation":"save", "value":marker, "extra":marker}),
        ] {
            assert_eq!(
                super::decode_credential(&InvokeBody::Json(body)),
                Err("Invalid credential request.")
            );
        }
        assert_eq!(
            super::decode_credential(&InvokeBody::Raw(marker.as_bytes().to_vec())),
            Err("Invalid credential request.")
        );
        for operation in ["save", "replace"] {
            let body = InvokeBody::Json(json!({"operation":operation,"value":marker}));
            assert_eq!(
                super::decode_credential(&body),
                Ok((operation, Some(marker)))
            );
        }
        for operation in ["remove", "check"] {
            let body = InvokeBody::Json(json!({"operation":operation}));
            assert_eq!(super::decode_credential(&body), Ok((operation, None)));
        }
    }

    #[test]
    fn app_commands_are_limited_to_the_local_main_window() {
        use tauri::ipc::Origin;
        let mut context = super::context();
        let authority = context.runtime_authority_mut();
        let remote = Origin::Remote {
            url: "https://example.invalid".parse().unwrap(),
        };
        for command in ["snapshot", "control", "credential"] {
            assert!(
                authority
                    .resolve_access(command, "main", "main", &Origin::Local)
                    .is_some()
            );
            assert!(
                authority
                    .resolve_access(command, "other", "other", &Origin::Local)
                    .is_none()
            );
            assert!(
                authority
                    .resolve_access(command, "main", "main", &remote)
                    .is_none()
            );
        }
        assert!(
            authority
                .resolve_access("read_secret", "main", "main", &Origin::Local)
                .is_none()
        );
    }
}
