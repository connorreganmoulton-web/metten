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
    strategy_operation: std::sync::Arc<StrategyAdmission>,
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

#[derive(Default)]
struct StrategyAdmission {
    busy: AtomicBool,
    completion: std::sync::Mutex<()>,
    idle: std::sync::Condvar,
}
struct StrategyGuard(std::sync::Arc<StrategyAdmission>);
impl Drop for StrategyGuard {
    fn drop(&mut self) {
        let _completion = self.0.completion.lock().unwrap_or_else(|e| e.into_inner());
        self.0.busy.store(false, Ordering::SeqCst);
        self.0.idle.notify_all();
    }
}
fn admit_strategy(
    operation: &std::sync::Arc<StrategyAdmission>,
    quitting: &AtomicBool,
) -> Result<StrategyGuard, metten_engine::StrategyFailure> {
    if quitting.load(Ordering::SeqCst) {
        return Err(metten_engine::StrategyFailure::new("quitting"));
    }
    operation
        .busy
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .map_err(|_| metten_engine::StrategyFailure::new("busy"))?;
    let guard = StrategyGuard(operation.clone());
    if quitting.load(Ordering::SeqCst) {
        return Err(metten_engine::StrategyFailure::new("quitting"));
    }
    Ok(guard)
}
fn decode_strategies(
    body: &tauri::ipc::InvokeBody,
) -> Result<metten_engine::StrategyRequest, metten_engine::StrategyFailure> {
    let tauri::ipc::InvokeBody::Json(body) = body else {
        return Err(metten_engine::StrategyFailure::new("invalid_request"));
    };
    metten_engine::decode_strategy_request(body)
}
fn wait_for_strategy(operation: &StrategyAdmission) {
    let mut completion = operation
        .completion
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    while operation.busy.load(Ordering::SeqCst) {
        completion = operation
            .idle
            .wait(completion)
            .unwrap_or_else(|e| e.into_inner());
    }
}
#[tauri::command]
async fn strategies(
    request: tauri::ipc::Request<'_>,
    app: tauri::AppHandle,
) -> Result<metten_engine::StrategyResponse, metten_engine::StrategyFailure> {
    let request = decode_strategies(request.body())?;
    let guard = {
        let workspace = app.state::<Workspace>();
        admit_strategy(&workspace.strategy_operation, &workspace.quitting)?
    };
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = guard;
        let workspace = app.state::<Workspace>();
        if workspace.quitting.load(Ordering::SeqCst) {
            return Err(metten_engine::StrategyFailure::new("quitting"));
        }
        workspace
            .engine
            .as_ref()
            .map_err(|_| metten_engine::StrategyFailure::new("storage"))?
            .strategies_unless_quitting(request, &workspace.quitting)
    })
    .await
    .map_err(|_| metten_engine::StrategyFailure::new("storage"))?
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
        wait_for_strategy(&workspace.strategy_operation);
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
            app.manage(Workspace { engine, #[cfg(target_os = "macos")] credentials: credential::Credentials::setup(), strategy_operation: std::sync::Arc::new(StrategyAdmission::default()), quitting: AtomicBool::new(false), exit_ready: AtomicBool::new(false), quit_error: std::sync::Mutex::new(None) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![snapshot, control, credential, strategies])
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

    #[cfg(target_os = "macos")]
    #[test]
    fn direct_strategy_requests_are_bounded_sanitized_and_independent_of_credentials() {
        use serde_json::json;
        use tauri::ipc::InvokeBody;
        let marker = "private-marker";
        for body in [
            InvokeBody::Raw(marker.as_bytes().to_vec()),
            InvokeBody::Json(json!({"operation":marker})),
            InvokeBody::Json(json!({"operation":"preview","json":"x".repeat(262145)})),
            InvokeBody::Json(
                json!({"operation":"save","json":"{}","reviewed_json":"{}","extra":marker}),
            ),
            InvokeBody::Json(json!({"operation":"detail","version_id":9007199254740993u64})),
        ] {
            let error = super::decode_strategies(&body).unwrap_err();
            assert!(!serde_json::to_string(&error).unwrap().contains(marker));
        }
        assert!(
            super::decode_strategies(&InvokeBody::Json(
                json!({"operation":"detail","version_id":"9007199254740993"})
            ))
            .is_ok()
        );
    }

    #[test]
    fn strategy_admission_releases_on_success_error_cancel_and_panic() {
        use std::sync::{Arc, atomic::AtomicBool};
        let operation = Arc::new(super::StrategyAdmission::default());
        let quitting = AtomicBool::new(false);
        let held = super::admit_strategy(&operation, &quitting).unwrap();
        assert_eq!(
            super::admit_strategy(&operation, &quitting)
                .err()
                .unwrap()
                .code,
            "busy"
        );
        drop(held);
        let error: Result<(), ()> = {
            let _guard = super::admit_strategy(&operation, &quitting).unwrap();
            Err(())
        };
        assert!(error.is_err());
        assert!(
            std::panic::catch_unwind(|| {
                let _guard = super::admit_strategy(&operation, &quitting).unwrap();
                panic!("controlled failure");
            })
            .is_err()
        );
        // A rejected/canceled queued closure drops its owned guard without executing.
        let guard = super::admit_strategy(&operation, &quitting).unwrap();
        let task = move || {
            drop(guard);
        };
        drop(task);
        drop(super::admit_strategy(&operation, &quitting).unwrap());
        quitting.store(true, std::sync::atomic::Ordering::SeqCst);
        assert_eq!(
            super::admit_strategy(&operation, &quitting)
                .err()
                .unwrap()
                .code,
            "quitting"
        );
    }

    #[test]
    fn quit_waits_for_admitted_strategy_before_stopping_the_session() {
        use std::sync::{Arc, atomic::AtomicBool};
        let operation = Arc::new(super::StrategyAdmission::default());
        let quitting = AtomicBool::new(false);
        let guard = super::admit_strategy(&operation, &quitting).unwrap();
        quitting.store(true, std::sync::atomic::Ordering::SeqCst);
        let (sent, received) = std::sync::mpsc::channel();
        let waiting = operation.clone();
        let task = std::thread::spawn(move || {
            super::wait_for_strategy(&waiting);
            sent.send(()).unwrap();
        });
        assert!(received.try_recv().is_err());
        drop(guard);
        received
            .recv_timeout(std::time::Duration::from_secs(2))
            .unwrap();
        task.join().unwrap();
        assert_eq!(
            super::admit_strategy(&operation, &quitting)
                .err()
                .unwrap()
                .code,
            "quitting"
        );
    }

    #[test]
    fn app_commands_are_limited_to_the_local_main_window() {
        use tauri::ipc::Origin;
        let mut context = super::context();
        let authority = context.runtime_authority_mut();
        let remote = Origin::Remote {
            url: "https://example.invalid".parse().unwrap(),
        };
        for command in ["snapshot", "control", "credential", "strategies"] {
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
