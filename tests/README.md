# Interface and credential-runner regression checks

## Overview

These four files check presentation, refresh ordering, credential-input handling, and the verification runner's process recovery. They use controlled inputs rather than exchanges or trading accounts. They do not prove the complete acceptance requirements in the [product specification](../METTEN_DESIGN_DRAFT_2026-11-12.md). Issue #4 strategy behavior is excluded from this documentation scope.

## Interface

| Test file and entry point | Boundary exercised |
| --- | --- |
| [view.test.mjs](./view.test.mjs): `pending and failed recording never claim saved success or permit another event` | Calls exported [presentation](../src/view.mjs) with saved, pending, and failed snapshots; checks recording wording and relevant controls. |
| [refresh.test.mjs](./refresh.test.mjs): failed-refresh test and four older/newer-result cases | Reads [main.ts](../src/main.ts), strips TypeScript/imports, and runs it in a VM with fake elements, timers, and `invoke`. Checks unavailable-state messaging, continued Pause access, no automatic stop request, and protection against stale successes/failures overwriting a newer Pause result or failure. |
| [credential.test.mjs](./credential.test.mjs): credential-submission test | Runs the same entry point with fake elements and native calls. Submits one dummy save, checks immediate input clearing and button disabling, rejects with a marker-bearing unexpected error, and checks that the marker is hidden and buttons recover. |
| [credential-runner.test.py](./credential-runner.test.py): `CredentialRunnerRecovery.test_progress_is_visible_before_exit_and_late_exit_is_recovered` | Compiles [verify-credential.swift](../scripts/verify-credential.swift) using `swiftc`, runs `--self-test`, observes progress before exit, releases the controlled gate, and checks the runner's recovery/output markers and successful exit. |

## Invariants

- VM tests replace the DOM, timers, and native calls. They exercise real entry-point logic but neither render a browser nor launch Tauri. The credential test covers one unexpected save failure; it does not test every credential operation or native error prefix.
- Refresh coverage includes all four combinations of an older success/failure and newer success/failure. A failed read must not be represented as proof that the engine stopped, and a stale result must not replace newer display state.
- The Python regression is process-only: its `--self-test` does not launch Metten or access Keychain. It checks visible progress, retained pending children, late exit/channel cleanup, unsuccessful child status preservation, and noisy-child output handling through explicit runner markers. It does not independently establish packaged-app credential protection.
- The [package check script](../package.json) selects `tests/*.test.mjs`; the Python file is a separate check, not part of `npm run check`. These checks do not cover stylesheet layout, actual assistive technology, native persistence, live exchange acceptance, or fills.

## Side Effects

- Refresh and credential checks read the entry-point source and mutate only fake element objects in isolated VM contexts; native commands and timer scheduling are mocked. The presentation check calls the pure function directly.
- The Python test writes a temporary Swift binary and progress log, starts disposable child processes, sends a gate byte over stdin, and removes its temporary directory. On a blocked cleanup path, it kills only the process group it created and waits for exit.

## Verification

Run from the repository root:

```sh
npm run check
python3 tests/credential-runner.test.py
```

The JavaScript entry-point checks require Node's `stripTypeScriptTypes`. The Python check requires Python 3 and `swiftc`; compilation and child waits have explicit timeouts. No checks were executed for this documentation-only bootstrap, and no browser, native application, or credential access was attempted.
