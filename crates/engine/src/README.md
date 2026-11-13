# Engine session and recording

## Overview

This directory implements the controlled, non-trading foundation session. It has a manually advanced test clock and durable session history; it does not implement the product's market clock, purchases, accounting or recovery of exchange orders. The architectural boundary follows [ADR 0001](../../../docs/adr/0001-rust-engine-and-tauri-boundaries.md).

Scope is the foundation at Git `b1eb58c` plus its unchanged working-copy behavior. `lib.rs` and `storage.rs` contain ongoing issue #4 work: the strategy module, version/import APIs, recorder-sharing changes and strategy persistence/schema changes are excluded from this contract. `strategy.rs` was not inspected. This is deliberately not a complete description of the mixed working copy.

## Interface

| Boundary | Contract |
| --- | --- |
| [lib.rs](./lib.rs): `Engine::open(path)` | Opens the caller-selected local nonsecret workspace, reads prior history, preserves its latest tick, and commits a stopped opening record before returning an engine. Returns `Result<Engine, String>`. |
| `Engine::request(Action)` | Queues `Start`, `Pause` or `Advance`. `Ok(())` confirms acceptance into the queue, not saving. Read `Snapshot.pending`, `recording_error` and `history` for the result. |
| `Engine::snapshot()` | Clones the current `Snapshot`: effective `running`, pending request count, recording error, committed history and effective writer settings. |
| `Engine::stop_and_save()` | Stops effective control immediately, then blocks for the saved stopped boundary. Returns a visible error on failure and permits retry. Call away from the desktop event thread. |
| `Action`, `Status` | `Action` deserializes/serializes only `start`, `pause`, `advance`; `Status` serializes `stopped`, `running`, `paused`. No arbitrary SQL, paths, shell or order action is accepted by `Action`. |
| `Record`, `WriterSettings` | `Record` exposes `id`, `status`, `tick`, `reason`, `recorded_at`; `WriterSettings` exposes `journal_mode`, `synchronous`, `fullfsync`, `checkpoint_fullfsync`, `sqlite_version`. These and `Snapshot` serialize for callers. |
| [storage.rs](./storage.rs): `Recorder::{open, latest, append}` | Crate-private SQLite adapter. `open` returns a recorder and effective settings; `latest` returns newest committed session rows; `append` commits a session record and reads history back. It is not a public desktop API. |

## Invariants

- `Engine::open` always starts stopped, including after a previously recorded running state. Its controlled clock resumes the latest saved tick, not automatic running activity.
- `Engine::enqueue` rejects duplicate Start, Advance while not running, and Start/Advance while a write is pending. `mpsc::sync_channel(8)` bounds requests; `try_send` refuses an unavailable/full queue.
- `Engine::enqueue` makes Pause and Stop effective before recording. `Shared.generation` prevents an older Start completion from undoing a later pause. Effective control can therefore be stopped while the newest saved `Record.status` is still running.
- The recorder worker publishes refreshed history only after `Recorder::append` commits and reads it back. A recording error sets `running = false`, retains the last confirmed snapshot history and blocks further control requests; Stop remains available to retry saving. A failed operation is not proof that nothing reached disk.
- `Recorder::open` holds `File::try_lock` on `path.with_extension("lock")` for its lifetime, preventing another cooperating engine from opening the same workspace. It uses a 250 ms SQLite busy timeout.
- `Recorder::open` sets and reads back WAL, `synchronous = 2` (FULL), `fullfsync = 1` and `checkpoint_fullfsync = 1`; mismatches refuse opening. The schema-version guard exists, but its changed issue #4 version is outside scope.
- Foundation table `session_events` is `STRICT`: `id INTEGER PRIMARY KEY`, `status TEXT` restricted to `stopped/running/paused`, nonnegative integer `tick`, required text `reason`, and a UTC `recorded_at` default from SQLite `strftime`. `Recorder::latest` orders by descending `id` and limits the returned view to 100 rows; it does not delete older rows.
- `Action::Advance` adds exactly one controlled second with checked arithmetic; overflow becomes a recording error rather than a wrapped tick. This is not the product's next-full-market start behavior.

## Side Effects

- `Recorder::open` creates/opens a local lock file and SQLite database, configures SQLite, and initializes the foundation `session_events` table inside `BEGIN IMMEDIATE`. SQLite manages WAL auxiliary files. Strategy schema additions in the mixed file are excluded.
- `Engine::open`, accepted controls and successful `stop_and_save` append session events. `Recorder::append` uses a transaction with bound SQL parameters, then reads committed history. Foundation code has no history deletion path.
- `Engine::open` starts the `metten-recorder` thread; control requests cross a bounded channel and share snapshot state through a mutex. `Engine::drop` closes the sender and joins that worker; Drop does not promise a saved stopped record.
- `stop_and_save` can wait on the recorder; failure reports that Quit was canceled. On a successful stop the worker exits and later controls are rejected.
- The inspected foundation performs no network, credential, exchange or real-money operations. Filesystem settings and process-restart tests are not proof of physical power-loss durability.

## Verification

From the repository root, the existing focused checks are:

```sh
cargo test -p metten-engine --test session --locked
cargo test -p metten-engine --lib pause_stays_effective_after_start_saves_before_pause_saves --locked
```

[Session tests](../tests/session.rs) cover public controls, real SQLite lock failure, single workspace ownership, stopped reopen, child-process exit recovery, action deserialization, and failed quit/retry. `lib.rs::tests::pause_stays_effective_after_start_saves_before_pause_saves` controls the precise Start/Pause recording race with a test-only barrier.

These commands target foundation checks, but compilation still includes the mixed working copy; they are not a validation of excluded issue #4 behavior. No tests or builds were run for this documentation-only change. [Issue #2 verification](../../../docs/verification/issue-2.md) records earlier local results and their limits; it is historical evidence, not a fresh result for all current files.
