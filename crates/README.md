# Rust library crates

## Overview

This subtree currently contains the `metten-engine` Rust library. The [workspace manifest](../Cargo.toml) also includes `src-tauri` outside this directory. [ADR 0001](../docs/adr/0001-rust-engine-and-tauri-boundaries.md) assigns authoritative behavior to the independent engine and leaves the desktop interface to request actions and display state.

This map covers the non-trading foundation at Git `b1eb58c` and unchanged current behavior. Ongoing issue #4 strategy additions are excluded; the engine contract identifies the affected mixed files. The [product specification](../METTEN_DESIGN_DRAFT_2026-11-12.md) describes the intended finished application, not capabilities already supplied by this subtree.

## Interface

| Subdirectory | Boundary |
| --- | --- |
| [engine](./engine/README.md) | `metten-engine` exposes `Engine::{open, request, snapshot, stop_and_save}` and typed session controls/state. [Source details](./engine/src/README.md) cover the internal `Recorder` and `session_events` persistence. |

## Invariants

- `Engine::open` starts stopped and preserves saved session history and the controlled clock across reopen.
- `Engine::request` queues work; `Snapshot` separates effective control from confirmed history. Pause remains effective while earlier recording completes.
- `Recorder::open` enforces one cooperating workspace owner and verifies SQLite writer settings. Failed recording blocks further Start/Advance rather than reporting unconfirmed changes as saved.
- The foundation library is independent of Tauri, credentials and exchange transport. The [implementation plan](../docs/implementation-plan.md) tracks later trading capabilities separately.

## Side Effects

- `Engine` starts a recorder thread and uses a bounded queue and shared snapshot state.
- `Recorder` creates/opens a local SQLite database and lock file and appends `session_events`; SQLite manages auxiliary write files. The inspected foundation performs no network or financial operations.

## Verification

From the repository root, use the existing foundation checks:

```sh
cargo test -p metten-engine --test session --locked
cargo test -p metten-engine --lib pause_stays_effective_after_start_saves_before_pause_saves --locked
```

See the [engine verification map](./engine/README.md) and historical [issue #2 evidence](../docs/verification/issue-2.md). No tests or builds were run for this documentation-only update. These targets compile mixed working-copy files and do not validate excluded issue #4 work.
