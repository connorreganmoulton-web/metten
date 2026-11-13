# Issue #2 — local foundation verification

Checked 12 November 2026 (America/Chicago; packaged checks at 03:19–03:21 UTC on 13 November). Owner explicitly authorized implementation and local verification in this session. Base: `main`, `970c12172c179eb77b3b21f2126533ed765881ea`, initially clean.

**Result:** the local packaged Mac path passes. Remote CI is configured but has not run. Issue #2 remains open/incomplete pending actual remote CI evidence. Nothing was pushed, published, merged or closed.

## Scope and boundaries

- `crates/engine/src/lib.rs`: desktop-independent public `Engine::open`, `request(Action)` and `snapshot()` interface. Only Start, Pause and Advance are accepted. Invalid state transitions are rejected. The controlled clock moves one second per accepted Advance; it is not a market clock, paper simulation or financial run.
- `crates/engine/src/storage.rs`: one SQLite connection owns writing and reads committed records back. No engine dependency on Tauri, browser rendering, credentials or network transport. An operating-system file lock prevents two Metten engines opening the same workspace.
- `src-tauri/src/main.rs`: native workspace selection and two screen commands, `snapshot` and `control`. No SQL, paths, shell, exchanges or arbitrary actions can be supplied through those commands. Tauri has no plugins; the main-window capability grants no plugin permissions. The production content policy restricts loaded content and connections.
- `src/`: screen formatting and action requests only. Now uses the engine's effective control state and committed clock; History contains rows read from SQLite. Pending changes explicitly remain unconfirmed. No optimistic successful-save display.
- The bounded control queue and recording thread keep database work out of control callbacks after workspace opening. Pause immediately disables further test events, including when Start is being recorded. A generation check prevents that pending Start from re-enabling the session. No numerical responsiveness or capacity guarantee is claimed.
- Start is enabled only after its record commits and is read back. A recording failure stops the session, blocks Start/Advance, and leaves last confirmed history visible. Reopen is required after fixing the storage problem. The guarded Quit menu and window-close path wait for an explicitly saved stopped record before exit; a failed save keeps the window open. Dropping an engine only joins its worker and does not claim a saved stop. Unexpected exit bypasses cleanup; reopening still records a stopped boundary before any start is allowed.
- No earlier application data or callers existed. Changes reach the engine, recorder, desktop bridge, screen, focused tests, build/lock files, CI and README. Existing product decisions and private ignored files are preserved. Only the existing placeholder icon was converted for the Mac bundle.

## Tooling and commands

Actual local versions:

| Tool | Version |
| --- | --- |
| macOS / architecture | 26.3 (25D125), Apple Silicon arm64 |
| Apple clang / desktop SDK | 17.0.0 / MacOSX.sdk from Command Line Tools |
| Rust | 1.100.0 (`0ef1bc386`, 2026-11-09) |
| Cargo | 1.100.0 (`3acc79c0c`, 2026-11-05) |
| Node / npm | 25.9.0 / 11.12.1 |
| Tauri CLI / JavaScript API | 2.12.4 / 2.12.4 |
| Tauri Rust / build crate | 2.12.5 / 2.7.4 |
| TypeScript / Vite | 7.0.2 / 8.3.8 |
| rusqlite / bundled SQLite | 0.42.0 / 3.54.1 |

Rust was absent from the command path. Installed from official `https://sh.rustup.rs` with `-y --profile minimal --no-modify-path`; commands used `/Users/connormoulton/.cargo/bin` explicitly. No shell startup files were changed. The pinned project toolchain adds rustfmt and clippy. `xcodebuild -version` reports no full Xcode installation; `clang --version` and `xcrun --show-sdk-path` confirm the desktop Command Line Tools. The actual bundle build passed with those tools.

| Exact check | Observed result |
| --- | --- |
| `npm install` | Installed locked interface dependencies; npm audit reported zero vulnerabilities at installation. This is not a security audit. |
| `cargo test -p metten-engine -- --nocapture` | Initial focused engine checks passed; writer settings printed below. |
| `npm run check` | TypeScript check and one focused interface test passed. |
| `npm run build` | Production interface build passed. |
| `cargo test --workspace --locked` | Six reported tests passed: five behavior checks plus the child-process test entry point. No failures. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed, including desktop bridge and tests. |
| `cargo fmt --all --check` | Passed. |
| `git diff --check` | Passed for tracked changes; final new-file whitespace inspection also performed. |
| `npm run tauri build -- --bundles app` | Passed; generated `target/release/bundle/macos/Metten.app`, arm64, approximately 12.25 MiB. |
| `/usr/bin/codesign -dv target/release/bundle/macos/Metten.app` | Linker ad-hoc signature, no developer team identity or sealed resources. This is not distribution signing/notarization. |

Early compile attempts identified an unsupported unsigned SQLite integer type, a missing CSS declaration, the required icon, and a deprecated Tauri state-removal call. These were corrected before the passing checks and packaged smoke. No unresolved compile errors remain.

## Engine and real-storage evidence

`crates/engine/tests/session.rs` exercises the public engine interface with real temporary SQLite files:

1. Start, advance, pause; invalid Advance while stopped/paused and duplicate Start are rejected. History and clock survive reopening, which is stopped. A second engine cannot open the same workspace.
2. A second SQLite connection holds `BEGIN IMMEDIATE`. Advance fails through the real writer after its bounded lock wait. Pause and snapshots remain available while the writer waits. Failure stops events, blocks further Start/Advance, preserves the exact earlier history, and provides actionable text. Reopening after the lock is released preserves those records and does not invent the failed clock event.
3. Pause while Start is pending cannot re-enable the session after recording finishes.
4. A child process saves Start and Advance then calls `process::exit`, bypassing engine cleanup. The parent verifies the last saved status was running, then opens the same database and verifies stopped state and preserved history/clock.
5. Deserialization rejects unrecognized SQL, shell, exchange and object-shaped operations.

The interface test in `tests/view.test.mjs` verifies that pending/failed recording cannot claim successful saving or permit another event, and that Pause remains available during recording.

## SQLite investigation and effective settings

The writer connection sets and reads back:

```json
{"journal_mode":"wal","synchronous":2,"fullfsync":1,"checkpoint_fullfsync":1,"sqlite_version":"3.54.1"}
```

Startup refuses to proceed if these effective settings do not match. They were verified both through the real-storage engine test and the packaged app's Recording details.

SQLite documents a WAL sync per commit with `synchronous=FULL`. Its Mac `fullfsync` setting requests `F_FULLFSYNC`; with that setting enabled, `checkpoint_fullfsync` is redundant, but is explicitly enabled and verified. WAL auxiliary files are SQLite-managed; automatic checkpointing remains at SQLite's default and runs on the recorder thread. History is append-only; the screen shows the newest 100 records without deleting earlier rows.

This proves configured writer settings and process-restart behavior, **not** physical power-loss durability, disk-full handling under every filesystem failure, corruption repair, or a latency guarantee. The controlled failure was lock contention, not simulated power loss or a full disk. No backup/export or trading recovery claim is made.

Primary references consulted for implementation:

- [Tauri desktop prerequisites](https://v2.tauri.app/start/prerequisites/) — desktop Command Line Tools support.
- [Tauri command boundary](https://v2.tauri.app/develop/calling-rust/) and [capabilities](https://v2.tauri.app/security/capabilities/) — typed command arguments, native/frontend boundary and permission scope.
- [SQLite synchronous](https://www.sqlite.org/pragma.html#pragma_synchronous), [fullfsync](https://www.sqlite.org/pragma.html#pragma_fullfsync), [checkpoint_fullfsync](https://www.sqlite.org/pragma.html#pragma_checkpoint_fullfsync), and [WAL](https://www.sqlite.org/wal.html) — setting semantics and checkpoint behavior.

## Packaged Mac smoke

Executed the actual release bundle using native Mac accessibility controls, not a browser mock or development server. An isolated temporary workspace was created with:

```sh
mkdir -p .scratch
mktemp -d /tmp/metten-issue2-XXXXXX > .scratch/smoke-workspace
METTEN_TEST_WORKSPACE="$(cat .scratch/smoke-workspace)" \
  target/release/bundle/macos/Metten.app/Contents/MacOS/metten
```

Observed on screen:

1. Opens stopped, clock 0, saved workspace-open record.
2. Start shows running only after saving; Advance shows clock 1 and its saved event.
3. Pause shows paused, disables Advance, and displays the saved pause event.
4. Recording details display WAL / FULL / both fullfsync flags from the writer.
5. Start again, then Command-Q. The launched process exits successfully and direct SQLite inspection shows a saved `Workspace closed — stopped` record. Re-launching the same bundle/workspace shows stopped, clock 1, and all earlier records.
6. Start; in a separate owned Python process, open that isolated SQLite file and hold `BEGIN IMMEDIATE` until Return. Click Advance. The screen first reports saving/unconfirmed, then `Stopped — recording needs attention`, disables controls, and explains checking space/permissions and closing competing tools before reopening. Details report `database is locked`.
7. Release that transaction using `ROLLBACK`. Direct inspection finds eight committed rows, maximum clock 1, no failed Advance record; `PRAGMA integrity_check` returns `ok`.
8. Quit and reopen. The screen is stopped at clock 1 with prior Start, Pause and Advance records intact. The app is left open stopped in this isolated test workspace; the competing test connection is closed.

No desktop Chrome, automated Chromium, credentials, exchange accounts, trading workspace, other trading application or network data source was used. All runtime test files remain outside tracked source. Existing user browser processes and private ignored research/conversation files were untouched.

## Acceptance review and remaining evidence

| Issue acceptance criterion | Evidence / status |
| --- | --- |
| Independent Rust engine; small validated screen interface; no raw privileged requests | Engine dependency boundary, enum validation and transition tests; only two native commands, no plugins; packaged path passes. |
| Controlled clock/events; Now/History use saved truth; stopped restart | Manual engine-owned clock; commit/readback snapshots; engine tests and visible packaged reopen pass. |
| Proposed SQLite durability settings verified on writer; actionable recording failure; no credentials/network needed | Exact settings above; real locked-writer failure demonstrated in test and packaged app; saved history preserved. |
| Meaningful engine/storage test; packaged Mac check; Rust/interface checks in CI | Local checks and native bundle smoke pass. `.github/workflows/checks.yml` configures those checks and a Mac bundle build on `macos-14`, Node 24, pinned Rust. **Remote CI execution is missing: no push or PR was authorized/performed.** |

Do not close issue #2 until actual remote CI evidence is available. The GitHub issue and its labels/comments were left unchanged. This local developer bundle does not demonstrate signed/notarized installation on another Mac, Intel support, financial correctness, paper/live fills, trading readiness, full Gen 1 completion, or power-loss durability. Later-issue features remain excluded.


## Post-review fixes and current checkpoint

The owner authorized fixing all four adversarial-review findings, then narrowed the finish to a local commit and a RISEN handoff for a fresh aggressive review. No Sol review has run on these fixes yet. Do not treat the implementation author's checks as independent post-fix approval.

1. **Quit recording failure:** `Engine::stop_and_save` immediately stops control, waits for a recorded stopped boundary, and returns any failure. Failure preserves confirmed history and permits retrying the save. The desktop performs this work away from the event thread; failed quit stays open and reports the error. The Mac predefined Quit item bypassed cancelable exit handling during the first packaged check; the final build replaces that item with the guarded Quit command. Window-close requests use the same guarded path. Engine Drop no longer silently attempts a final write.
2. **Failed status reads:** the screen reports `Session status unavailable`, disables Start/Advance and retains an attempt to Pause. It does not infer that the engine stopped. `tests/refresh.test.mjs` runs the actual screen source with a controlled failed native response and verifies those behaviors.
3. **Occupied development port:** the development command explicitly selects port 5173 and enables `--strictPort`. An isolated occupied-port probe returned exit code 1 with `already in use`; no alternate screen server was started.
4. **Pause test blind spot:** a test-only barrier holds Pause before its write, after Start has saved and published. The test checks that effective control remains stopped during that exact interval. Removing the generation guard in a temporary external copy makes the new test fail at `a saved Start must not undo Pause`. No timing-based sleep or production test hook is used; the barrier compiles only in Rust's unit-test build.

Passing checks on the final behavior code:

- `cargo test --workspace --locked`: seven reported tests pass (one deterministic race unit test, five integration behavior tests and the child-process entry point).
- `npm run check`: TypeScript and two interface tests pass. The native Node type-stripping API used only in the test harness emits an experimental-feature notice under the local Node version.
- `cargo clippy --workspace --all-targets --locked -- -D warnings` and `cargo fmt --all --check`: pass.
- `npm run tauri build -- --bundles app`: passes; final local arm64 bundle approximately 12.33 MiB.
- New targeted checks: quit failure/retry with real temporary SQLite, failed screen refresh, occupied-port refusal, and the temporary-copy mutation described above all pass.

Final packaged check (03:39–03:40 UTC, 13 November) used a fresh isolated directory selected by `.scratch/fixes-smoke-workspace`: start, preserve the earlier controlled clock/history, block the writer with a second connection, then Command-Q. The window stayed open, the session stopped, controls were disabled and `Quit canceled` explained the failed save and retry. Releasing the lock and retrying Command-Q exited successfully. Direct SQLite inspection confirmed `Workspace closed — stopped`, clock 1, and `integrity_check = ok`. Reopening displayed stopped with history intact. The window-close path also exited. The native UI connector subsequently relaunched the app once into a fresh default foundation workspace; that empty session was closed. Last process inspection found no running Metten process. No credentials, accounts or trading records were involved; existing files were not deleted.

Packaged coverage is specifically the application Quit command/Command-Q and window-close path. Other operating-system termination routes, physical power loss, distribution signing/notarization, other Macs and remote CI remain unverified. Earlier process-restart and unchanged storage-setting evidence remains applicable. No broader desktop automation or testing is needed absent a named unresolved finding.
