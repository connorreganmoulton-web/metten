# Native desktop bridge and dummy credentials

## Overview

[main.rs](./main.rs) owns Tauri startup, the native-selected workspace, screen
commands, and guarded exit. [credential.rs](./credential.rs) is the Mac-only,
dummy-only Security adapter. The shared engine remains the source of session and
recording truth; this bridge is not a trading engine or provider integration.
See the [desktop package contract](../README.md) for build, permissions, icons,
configuration, and evidence limits. Issue #4 additions are excluded from this
contract, including additions inside `main.rs`.

## Interface

| Symbol | In-scope contract |
| --- | --- |
| `main.rs:Workspace` | Holds `Result<Engine, String>`, Mac `Credentials`, `quitting`, `exit_ready`, and a mutex-protected `quit_error`. A workspace-open failure is retained for command errors rather than inventing engine state. |
| `main.rs:snapshot` | Returns engine `Snapshot` with effective `running`, `pending`, `recording_error`, committed `history`, and `writer_settings`; overlays a saved quit error when present. Propagates workspace-open errors. |
| `main.rs:control` | Accepts the engine `Action` values `start`, `pause`, or `advance`; refuses while quitting and otherwise calls `Engine::request`. Acceptance means queued, not saved; callers must observe the returned snapshot. |
| `main.rs:decode_credential` | Requires a JSON object with only `operation` and optional `value`. `save`/`replace` require a string value; `remove`/`check` forbid the value key. Other shapes, keys, operations, and raw bodies return `Invalid credential request.` |
| `main.rs:credential` | Uses raw request decoding and a blocking worker. `check` consumes a runner expectation, then all operations acquire `Credentials::begin_operation` before `execute`. Returns fixed status/error strings, never the saved bytes. Non-Mac builds refuse with the isolated-Mac-runner requirement. |
| `main.rs:begin_quit` | Marks quitting once, waits for `Credentials::operation`, calls `Engine::stop_and_save`, then sets `exit_ready` and exits on success. Failure retains its error, clears quitting for retry, and shows/focuses `main`. If no engine opened, there is no engine state to save. |
| `main.rs:main`, `context` | Build the Tauri context, replace the Mac predefined Quit item with guarded `quit-session`, open native-selected `session.sqlite`, register the in-scope commands, and intercept close/exit requests until shutdown is ready. |
| `credential.rs:Credentials::setup` | Accepts `METTEN_VERIFY_FD=3` and a valid inherited Unix socket bootstrap; otherwise stores no usable target. Opens/unlocks the validated disposable Keychain, clears its retained password string, and starts the expectation observer. |
| `credential.rs:Credentials::begin_operation`, `operation` | Serialize credential operations and shutdown. A command checks the quitting flag after obtaining the lock and returns `Quitting — credential action refused.` when set. |
| `credential.rs:Credentials::expectation` | Takes exactly one pending expected dummy value. A failed channel or missing expectation returns the fixed verification-unavailable error without waiting for input. |
| `credential.rs:Credentials::execute` | `save` uses `SecItemAdd`; `replace` uses `SecItemUpdate`; `remove` uses `SecItemDelete`; `check` uses `SecItemCopyMatching` and compares the actual native-read bytes with the private expectation. |

Credential success categories are **Saved**, **Replaced**, **Removed**, and
**Readable**. `credential.rs:map_status` maps duplicate, missing, canceled,
authentication-failed, and denied OS statuses to fixed text; other failures use
**Unavailable**. `unlocked` supplies **Locked**; `execute` also reports invalid
dummy input, unsupported action, or **Mismatch**. Saved and readable are separate
states, and every success explicitly says no exchange is connected. A blocking
worker failure in `main.rs:credential` has a fixed reopen-app error.

## Invariants

- `main.rs:main` chooses the directory from native `METTEN_TEST_WORKSPACE` or
  `app.path().app_data_dir()`. `Engine::open` records a stopped boundary on open;
  neither a controlled session nor a reopened session establishes a financial run.
  History and writer settings are supplied by the engine, not reconstructed by
  the bridge.
- `main.rs:decode_credential` prevents framework decoding errors from quoting
  submitted values. `credential.rs:dummy` requires the `metten-dummy-` prefix,
  20–128 bytes, and only ASCII letters, digits, or hyphens. No generic secret-read
  operation, caller-selected service/account, or screen-supplied Keychain path
  exists in this boundary.
- `credential.rs:Bootstrap::validate` requires a canonical `dummy.keychain-db`
  inside an owner-owned mode-0700 `metten-credential-*` directory, with a regular,
  owner-matching, single-link file and the recorded device/inode. It rejects
  System/login paths and aliases. `Bootstrap::open` checks actual Keychain path
  and revalidates identity. `record_native_write` updates the accepted inode only
  immediately after this adapter's successful write, then validates again.
- `credential.rs:Credentials::execute` fixes generic-password class, service
  `com.metten.dummy-connection`, account `dummy`,
  `kSecAttrSynchronizable=false`, and `kSecUseDataProtectionKeychain=false`.
  Add targets `kSecUseKeychain`; other operations restrict `kSecMatchSearchList`
  to that one reference. There is no default/login fallback or search-list change.
  Replace is neither delete-first nor upsert; missing and duplicate items remain
  distinct failures. `unlocked` refuses a locked target without automatic unlock
  during an operation.
- `credential.rs:Bootstrap` and `Expectation` deny unknown JSON fields. Setup
  bounds password length to 32–128 bytes and session length to 32 bytes.
  `frame`/`receive` require a 1–4096-byte frame within one two-second deadline.
  `observe` permits one pending expectation, requires matching session, next
  sequence, `local-use` phase, and bounded dummy data, and acknowledges receipt.
  Protocol failure clears pending data and makes the channel unusable;
  `expectation` consumes a valid value once.
- `main.rs:begin_quit` waits for an ongoing native credential operation. Queued
  operations refuse after acquiring the lock when quitting is set. There is no
  arbitrary Security-call cancellation: an actual OS prompt must finish before
  the saved-stop/exit path can complete. A quit-save failure remains visible
  through `snapshot` and permits a later Quit retry.

## Side Effects

- `main.rs:main` creates the selected directory and calls `Engine::open`;
  `control` queues engine actions and `begin_quit` requests a durable stopped
  record. SQLite, locking, and committed history belong to the engine.
- `credential.rs:Credentials::setup` consumes the inherited socket, opens and
  explicitly unlocks the runner's disposable Keychain, and spawns `observe`.
  `observe` reads framed private expectations and writes one-byte acknowledgments.
  No screen command creates a Keychain or changes global Keychain configuration.
- `credential.rs:Credentials::execute` can mutate the fixed dummy item or read it
  for native comparison; native APIs may prompt. Reads return only fixed results
  to the screen. Temporary secret bytes and bootstrap data exist in process
  memory; this is not a claim of memory zeroization or compromise protection.
- `main.rs:begin_quit` mutates shutdown flags/error state, may focus the window,
  and exits after successful saving. `CloseRequested` and `ExitRequested` are
  prevented while the guard owns shutdown.

## Verification

From the repository root, `cargo test -p metten --locked` exercises native unit
and command-authority checks. `main.rs:permissions_tests` covers fixed decoding
errors and local-main-only authority. `credential.rs:tests` covers queued refusal
and quit waiting, file identity/alias rejection, bounded dummy input and status
mapping, bounded/closed frames, and one-use/duplicate expectations. These
controlled checks do not themselves prove real prompt refusal or packaged UI
behavior.

The latest finished section of [issue #3's verification record](../../docs/verification/issue-3.md)
records the separate dummy-only packaged result, including actual access denial
and pending-prompt Quit. It distinguishes those observations from mapped error
tests and from the controlled queued-command test. Reuse its recorded evidence
only for unchanged behavior; no test, build, launch, or credential operation was
performed for these README additions. See [issue #2](../../docs/verification/issue-2.md)
for the retained non-trading session and failed-save/retry evidence.
