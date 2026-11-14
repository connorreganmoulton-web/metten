//! A controlled non-trading session. No desktop, network, or exchange dependency.
mod storage;
pub mod strategy;
pub use strategy::{
    AdvanceBatch, Decision, Evaluator, RuleError, RunContext, SliceControl, SliceResult,
    ValidatedStrategy,
};

#[derive(Clone, Debug, Serialize)]
pub struct StrategyVersion {
    pub id: i64,
    pub json: String,
    pub explanation: String,
    pub recorded_at: String,
}

use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{Arc, Mutex, mpsc},
    thread,
};
use storage::Recorder;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Start,
    Pause,
    Advance,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Stopped,
    Running,
    Paused,
}
impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Self::Stopped => "stopped",
            Self::Running => "running",
            Self::Paused => "paused",
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Record {
    pub id: i64,
    pub status: Status,
    pub tick: i64,
    pub reason: String,
    pub recorded_at: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct WriterSettings {
    pub journal_mode: String,
    pub synchronous: i64,
    pub fullfsync: i64,
    pub checkpoint_fullfsync: i64,
    pub sqlite_version: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Snapshot {
    /// Effective control state; pause takes effect before its recording completes.
    pub running: bool,
    pub pending: usize,
    pub recording_error: Option<String>,
    /// Only committed rows read back from SQLite, newest first.
    pub history: Vec<Record>,
    pub writer_settings: WriterSettings,
}

struct Shared {
    snapshot: Snapshot,
    generation: u64,
    stop_requested: bool,
    #[cfg(test)]
    pause_recording_barrier: Option<(mpsc::Sender<()>, mpsc::Receiver<()>)>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    Control(Action),
    Stop,
}
struct Request {
    operation: Operation,
    generation: u64,
    completion: Option<mpsc::Sender<Result<(), String>>>,
}

pub struct Engine {
    recorder: Arc<Mutex<Recorder>>,
    sender: Option<mpsc::SyncSender<Request>>,
    shared: Arc<Mutex<Shared>>,
    worker: Option<thread::JoinHandle<()>>,
}

fn recording_error(detail: &str) -> String {
    format!(
        "Session stopped: recording failed. The requested change is not confirmed saved. Check free disk space and workspace permissions, close other workspace tools, then quit and reopen Metten. Saved history is retained. Details: {detail}"
    )
}

impl Engine {
    /// The caller selects a local, nonsecret workspace; the renderer cannot select paths.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
        let (mut recorder, settings) =
            Recorder::open(path.as_ref()).map_err(|e| recording_error(&e))?;
        let previous = recorder.latest().map_err(|e| recording_error(&e))?;
        let tick = previous.first().map_or(0, |r| r.tick);
        // Every open records a stopped boundary, including after an unexpected exit.
        let history = recorder
            .append(Status::Stopped, tick, "Workspace opened — stopped")
            .map_err(|e| recording_error(&e))?;
        let recorder = Arc::new(Mutex::new(recorder));
        let shared = Arc::new(Mutex::new(Shared {
            snapshot: Snapshot {
                running: false,
                pending: 0,
                recording_error: None,
                history,
                writer_settings: settings,
            },
            generation: 0,
            stop_requested: false,
            #[cfg(test)]
            pause_recording_barrier: None,
        }));
        let (sender, receiver) = mpsc::sync_channel::<Request>(8);
        let state = shared.clone();
        let writer = recorder.clone();
        let worker = thread::Builder::new()
            .name("metten-recorder".into())
            .spawn(move || {
                for request in receiver {
                    let (tick, failed) = {
                        let state = state.lock().expect("engine state poisoned");
                        (
                            state.snapshot.history[0].tick,
                            state.snapshot.recording_error.is_some(),
                        )
                    };
                    let stopping = request.operation == Operation::Stop;
                    if failed && !stopping {
                        state
                            .lock()
                            .expect("engine state poisoned")
                            .snapshot
                            .pending -= 1;
                        continue;
                    }
                    #[cfg(test)]
                    if request.operation == Operation::Control(Action::Pause) {
                        let barrier = state.lock().unwrap().pause_recording_barrier.take();
                        if let Some((entered, resume)) = barrier {
                            entered.send(()).unwrap();
                            resume.recv().unwrap();
                        }
                    }
                    let (status, tick, reason) = match request.operation {
                        Operation::Control(Action::Start) => {
                            (Status::Running, tick, "Controlled session started")
                        }
                        Operation::Control(Action::Pause) => {
                            (Status::Paused, tick, "Controlled session paused")
                        }
                        Operation::Control(Action::Advance) => (
                            Status::Running,
                            tick.checked_add(1).unwrap_or(-1),
                            "Test clock advanced one second",
                        ),
                        Operation::Stop => (Status::Stopped, tick, "Workspace closed — stopped"),
                    };
                    let mut recorder = writer.lock().expect("recorder poisoned");
                    let result = if stopping {
                        recorder
                            .latest()
                            .and_then(|rows| recorder.append(status, rows[0].tick, reason))
                    } else if tick < 0 {
                        Err("The test clock reached its supported limit.".into())
                    } else {
                        recorder.append(status, tick, reason)
                    };
                    let completion = result.as_ref().map(|_| ()).map_err(|error| {
                        let error = recording_error(error);
                        if stopping {
                            format!(
                                "Quit canceled. {error} After fixing the problem, try Quit again."
                            )
                        } else {
                            error
                        }
                    });
                    let mut state = state.lock().expect("engine state poisoned");
                    state.snapshot.pending -= 1;
                    match result {
                        Ok(history) => {
                            state.snapshot.history = history;
                            if stopping {
                                state.snapshot.recording_error = None;
                            }
                            if state.generation == request.generation {
                                state.snapshot.running = status == Status::Running;
                            }
                        }
                        Err(_) => {
                            if stopping {
                                state.stop_requested = false;
                            }
                            state.snapshot.running = false;
                            state.snapshot.recording_error = completion.as_ref().err().cloned();
                        }
                    }
                    drop(state);
                    let stopped = stopping && completion.is_ok();
                    if let Some(reply) = request.completion {
                        let _ = reply.send(completion);
                    }
                    if stopped {
                        break;
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        Ok(Self {
            recorder,
            sender: Some(sender),
            shared,
            worker: Some(worker),
        })
    }

    /// Acceptance means queued, never saved. Observe pending/error and committed history.
    pub fn request(&self, action: Action) -> Result<(), String> {
        self.enqueue(Operation::Control(action), None)
    }

    /// Stop immediately, then wait for the saved stop. Call off the desktop event thread.
    /// On failure the engine stays stopped and this operation may be retried.
    pub fn stop_and_save(&self) -> Result<(), String> {
        let (reply, result) = mpsc::channel();
        self.enqueue(Operation::Stop, Some(reply))?;
        result.recv().map_err(|_| {
            "Quit canceled: recorder unavailable. Reopen Metten after checking the workspace."
                .to_string()
        })?
    }

    fn enqueue(
        &self,
        operation: Operation,
        completion: Option<mpsc::Sender<Result<(), String>>>,
    ) -> Result<(), String> {
        let mut state = self
            .shared
            .lock()
            .map_err(|_| "Engine unavailable. Quit and reopen Metten.".to_string())?;
        if state.stop_requested {
            return Err("The session is stopping or closed.".into());
        }
        if matches!(
            operation,
            Operation::Control(Action::Pause) | Operation::Stop
        ) {
            state.snapshot.running = false;
            state.generation += 1;
        }
        if operation != Operation::Stop
            && let Some(error) = &state.snapshot.recording_error
        {
            return Err(error.clone());
        }
        if state.snapshot.pending > 0
            && !matches!(
                operation,
                Operation::Control(Action::Pause) | Operation::Stop
            )
        {
            return Err("Wait for recording to finish.".into());
        }
        match operation {
            Operation::Control(Action::Start) if state.snapshot.running => {
                return Err("The session is already running.".into());
            }
            Operation::Control(Action::Advance) if !state.snapshot.running => {
                return Err("Start the controlled session before advancing the clock.".into());
            }
            _ => (),
        }
        let request = Request {
            operation,
            generation: state.generation,
            completion,
        };
        self.sender
            .as_ref()
            .ok_or("Engine closed")?
            .try_send(request)
            .map_err(|_| {
                "Controls are busy. The session remains paused; wait for recording to finish."
                    .to_string()
            })?;
        if operation == Operation::Stop {
            state.stop_requested = true;
        }
        state.snapshot.pending += 1;
        Ok(())
    }

    /// Append-only import; existing versions and session records are never rewritten.
    pub fn import_strategy(&self, json: &str) -> Result<StrategyVersion, String> {
        let strategy = strategy::Strategy::parse(json)?;
        self.recorder
            .lock()
            .map_err(|_| "Strategy storage unavailable")?
            .import_strategy(&strategy)
    }

    pub fn strategy_versions(&self) -> Result<Vec<StrategyVersion>, String> {
        self.recorder
            .lock()
            .map_err(|_| "Strategy storage unavailable")?
            .strategy_versions()
    }

    pub fn snapshot(&self) -> Snapshot {
        self.shared
            .lock()
            .expect("engine state poisoned")
            .snapshot
            .clone()
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.sender.take();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn pause_stays_effective_after_start_saves_before_pause_saves() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("session.sqlite");
        let engine = Engine::open(&path).unwrap();
        let lock = rusqlite::Connection::open(&path).unwrap();
        lock.execute_batch("BEGIN IMMEDIATE").unwrap();
        let (entered, at_pause) = mpsc::channel();
        let (resume, proceed) = mpsc::channel();
        engine.shared.lock().unwrap().pause_recording_barrier = Some((entered, proceed));
        engine.request(Action::Start).unwrap();
        engine.request(Action::Pause).unwrap();
        lock.execute_batch("ROLLBACK").unwrap();
        at_pause.recv_timeout(Duration::from_secs(5)).unwrap();
        // Start has published; Pause has not begun recording. No timing-based sleep.
        let between_records = engine.snapshot();
        resume.send(()).unwrap();
        engine.stop_and_save().unwrap();
        assert_eq!(between_records.pending, 1);
        assert_eq!(between_records.history[0].status, Status::Running);
        assert!(
            !between_records.running,
            "a saved Start must not undo Pause"
        );
        assert!(
            engine
                .snapshot()
                .history
                .iter()
                .any(|r| r.status == Status::Paused)
        );
    }
}
