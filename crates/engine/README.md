# Metten engine crate

## Overview

`metten-engine` is the Rust library for a controlled, persisted non-trading session. It provides desktop-independent controls and saved history as the first implementation slice of the [Gen 1 specification](../../METTEN_DESIGN_DRAFT_2026-11-12.md). Future product behavior belongs to the [implementation sequence](../../docs/implementation-plan.md); it must not be inferred from this foundation.

This contract covers Git `b1eb58c` foundation behavior that remains unchanged in the working copy. Ongoing issue #4 strategy work, including its source, tests, import/version APIs and schema changes, is excluded. Mixed-file exclusions are detailed in the source contract.

## Interface

- [Source contract](./src/README.md): `Engine::{open, request, snapshot, stop_and_save}`, `Action::{Start, Pause, Advance}`, `Status::{Stopped, Running, Paused}`, `Snapshot`, `Record`, and `WriterSettings`.
- [Cargo manifest](./Cargo.toml): library package `metten-engine`, version `0.1.0`, Rust edition 2024, minimum Rust `1.100`. The foundation uses `rusqlite` with bundled SQLite and `serde`; `tempfile` and `serde_json` support foundation tests. The working-copy promotion of `serde_json` to a runtime dependency belongs to excluded issue #4 work.
- [Session tests](./tests/session.rs) exercise the public library against temporary real SQLite files. The tests directory receives no separate contract because it contains verification code only.

## Invariants

- `Engine::open` returns stopped with a committed opening record; reopening preserves the controlled clock and saved history without automatically resuming.
- `Engine::request` acceptance is not save confirmation. `Snapshot.running` is effective control state; `Snapshot.history` contains only the last successfully read committed rows, newest first.
- `Engine::enqueue` stops effective control immediately on Pause/Stop, rejects invalid transitions, and blocks Start/Advance during pending or failed recording. A generation guard preserves a later pause through an earlier Start completion.
- `Recorder::open` requires one cooperating workspace owner and verified SQLite writer settings. `Recorder::append` commits session records transactionally; recording errors stop control and retain the last confirmed history.
- The dependency boundary in [ADR 0001](../../docs/adr/0001-rust-engine-and-tauri-boundaries.md) remains intact for this foundation: the engine has no Tauri or provider dependency. Session events are not paper fills, live fills or financial truth.

## Side Effects

- Opening creates/opens a local SQLite workspace and lock file; SQLite may create auxiliary files. `Recorder` reads/writes `session_events`, whose contract is specified under [source](./src/README.md).
- The engine owns a recorder thread and bounded control queue. Controls append records; `stop_and_save` waits for durable recording confirmation, while Drop only closes/joins the worker.
- The foundation makes no network calls and handles no credentials or money. Excluded strategy work is not covered by these side-effect claims.

## Verification

Run from the repository root:

```sh
cargo test -p metten-engine --test session --locked
cargo test -p metten-engine --lib pause_stays_effective_after_start_saves_before_pause_saves --locked
```

The [source contract](./src/README.md) maps checks to behavior. [Issue #2 evidence](../../docs/verification/issue-2.md) records prior local tests and packaged-app checks, with their distribution and physical power-loss limits. Its pending-CI and review dispositions describe that historical checkpoint, not current PR status. No tests/builds were rerun for this documentation change; these commands still compile the current mixed source and do not establish issue #4 acceptance.
