use metten_engine::{Action, Engine, Snapshot, Status};
use rusqlite::Connection;
use std::{
    thread,
    time::{Duration, Instant},
};

fn settled(engine: &Engine) -> Snapshot {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let snapshot = engine.snapshot();
        if snapshot.pending == 0 {
            return snapshot;
        }
        assert!(Instant::now() < deadline, "recording did not finish");
        thread::sleep(Duration::from_millis(5));
    }
}
fn apply(engine: &Engine, action: Action) -> Snapshot {
    engine.request(action).unwrap();
    let snapshot = settled(engine);
    assert!(
        snapshot.recording_error.is_none(),
        "{:?}",
        snapshot.recording_error
    );
    snapshot
}

#[test]
fn controls_real_storage_and_stopped_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.sqlite");
    let engine = Engine::open(&path).unwrap();
    assert!(!engine.snapshot().running);
    assert!(engine.request(Action::Advance).is_err());
    let started = apply(&engine, Action::Start);
    assert!(started.running);
    assert_eq!(started.history[0].status, Status::Running);
    assert!(engine.request(Action::Start).is_err());
    assert_eq!(apply(&engine, Action::Advance).history[0].tick, 1);
    let paused = apply(&engine, Action::Pause);
    assert!(!paused.running);
    assert_eq!(paused.history[0].status, Status::Paused);
    assert!(engine.request(Action::Advance).is_err());
    let settings = paused.writer_settings;
    assert_eq!(settings.journal_mode, "wal");
    assert_eq!(settings.synchronous, 2);
    assert_eq!(settings.fullfsync, 1);
    assert_eq!(settings.checkpoint_fullfsync, 1);
    eprintln!(
        "writer settings: {}",
        serde_json::to_string(&settings).unwrap()
    );
    apply(&engine, Action::Start);
    let history = engine.snapshot().history;
    assert!(Engine::open(&path).is_err(), "one owner per workspace");
    drop(engine);
    let reopened = Engine::open(&path).unwrap();
    let snapshot = reopened.snapshot();
    assert!(!snapshot.running);
    assert_eq!(snapshot.history[0].status, Status::Stopped);
    assert_eq!(snapshot.history[0].tick, 1);
    for record in history {
        assert!(snapshot.history.contains(&record));
    }
}

#[test]
fn real_write_failure_stops_events_and_preserves_confirmed_history() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.sqlite");
    let engine = Engine::open(&path).unwrap();
    let before = apply(&engine, Action::Start).history;
    let lock = Connection::open(&path).unwrap();
    lock.execute_batch("BEGIN IMMEDIATE").unwrap();
    engine.request(Action::Advance).unwrap();
    // This must remain available while the writer waits on the real SQLite lock.
    engine.request(Action::Pause).unwrap();
    let pending = engine.snapshot();
    assert!(!pending.running);
    assert!(pending.pending > 0);
    let failed = settled(&engine);
    assert!(!failed.running);
    assert_eq!(failed.history, before);
    let error = failed.recording_error.unwrap();
    assert!(error.contains("not confirmed saved"));
    assert!(error.contains("Check free disk space"));
    assert!(error.contains("database is locked"));
    assert!(engine.request(Action::Start).is_err());
    assert!(engine.request(Action::Advance).is_err());
    lock.execute_batch("ROLLBACK").unwrap();
    drop(engine);
    let reopened = Engine::open(&path).unwrap();
    assert!(!reopened.snapshot().running);
    assert_eq!(reopened.snapshot().history[0].tick, 0);
    for record in before {
        assert!(reopened.snapshot().history.contains(&record));
    }
}

#[test]
fn unexpected_exit_child() {
    let Some(path) = std::env::var_os("METTEN_CRASH_TEST_DB") else {
        return;
    };
    let engine = Engine::open(path).unwrap();
    apply(&engine, Action::Start);
    apply(&engine, Action::Advance);
    // Bypass Engine::drop: the next process must recover from a recorded running state.
    std::process::exit(0);
}

#[test]
fn process_exit_keeps_history_but_restart_is_stopped() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.sqlite");
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "unexpected_exit_child", "--nocapture"])
        .env("METTEN_CRASH_TEST_DB", &path)
        .output()
        .unwrap();
    assert!(child.status.success(), "{:?}", child);
    let connection = Connection::open(&path).unwrap();
    let before: String = connection
        .query_row(
            "SELECT status FROM session_events ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(before, "running");
    drop(connection);
    let engine = Engine::open(&path).unwrap();
    let snapshot = engine.snapshot();
    assert!(!snapshot.running);
    assert_eq!(snapshot.history[0].status, Status::Stopped);
    assert_eq!(snapshot.history[0].tick, 1);
    assert!(
        snapshot
            .history
            .iter()
            .any(|record| record.reason == "Test clock advanced one second")
    );
}

#[test]
fn interface_rejects_unrecognized_or_arbitrary_operations() {
    assert_eq!(
        serde_json::from_str::<Action>("\"pause\"").unwrap(),
        Action::Pause
    );
    for input in [
        "\"execute_sql\"",
        "\"shell\"",
        "\"place_order\"",
        "{\"start\":\"/tmp/file\"}",
    ] {
        assert!(serde_json::from_str::<Action>(input).is_err());
    }
}

#[test]
fn quit_recording_failure_is_visible_and_can_be_retried() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.sqlite");
    let engine = Engine::open(&path).unwrap();
    let before = apply(&engine, Action::Start).history;
    let lock = Connection::open(&path).unwrap();
    lock.execute_batch("BEGIN IMMEDIATE").unwrap();
    let error = engine.stop_and_save().unwrap_err();
    assert!(error.contains("Quit canceled"));
    assert!(error.contains("database is locked"));
    assert!(error.contains("try Quit again"));
    let snapshot = engine.snapshot();
    assert!(!snapshot.running);
    assert_eq!(snapshot.history, before);
    assert_eq!(snapshot.recording_error, Some(error));
    assert!(engine.request(Action::Start).is_err());
    lock.execute_batch("ROLLBACK").unwrap();
    engine.stop_and_save().unwrap();
    let snapshot = engine.snapshot();
    assert_eq!(snapshot.history[0].status, Status::Stopped);
    assert_eq!(snapshot.history[0].reason, "Workspace closed — stopped");
    assert!(snapshot.recording_error.is_none());
    assert!(engine.request(Action::Start).is_err());
    let saved: String = lock
        .query_row(
            "SELECT status FROM session_events ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(saved, "stopped");
    for record in before {
        assert!(snapshot.history.contains(&record));
    }
    drop(engine);
    assert!(!Engine::open(&path).unwrap().snapshot().running);
}
