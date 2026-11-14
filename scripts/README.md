# Verification tools

## Staged checkpoint checks

Use [check-staged.py](check-staged.py) to check a mixed-work checkpoint independently of unstaged files. Usage, failure handling and archival-format exceptions are in [the agent verification guide](../docs/agents/verification.md#verify-a-mixed-work-checkpoint). Its disposable-Git regressions run with `python3 tests/check-staged.test.py` and in CI. The helper runs only the supplied command; it does not start native or credential checks itself.

## Overview

[verify-credential.swift](verify-credential.swift) is the dummy-only packaged-app runner for issue #3. It is a verification tool, not part of the trading engine. Its completed evidence and limitations are in [issue-3.md](../docs/verification/issue-3.md); process-only regression checks are described in [tests](../tests/README.md).

## Interface

- Normal invocation accepts exactly one finished `.app` path. `run()` requires an existing Accessibility grant, obtains `CFBundleExecutable` from `Contents/Info.plist`, and launches that executable directly through `launch()`.
- `launch()` selects a fresh `METTEN_TEST_WORKSPACE`. With isolated setup enabled it passes descriptor 3 as `METTEN_VERIFY_FD`, sending owned Keychain identity/password/session over a private socket, followed by sequenced `expectation()` frames. Dummy values are generated in memory, not passed in command arguments or environment variables.
- `--self-test` runs `recoverySelfTest()` using child processes and sockets without opening Metten or calling Keychain/Accessibility APIs. `--output-child` is its internal large-output fixture.
- `press()`, `fill()`, `waitText()` and `clearCheck()` operate the owned app through Accessibility. `denyOwnedPrompt()` touches only one uniquely identified Metten/dummy SecurityAgent prompt's Deny control.

## Invariants

- `waitForChildExit()` must confirm the exact spawned child's exit before resources are released. `requestQuit()` requests guarded window-close once; `awaitRetainedChildExit()` retains ownership if exit is pending. No force-kill or success inference from a timeout.
- `send()` catches a closed socket without terminating through SIGPIPE. `startChildOutputCapture()` drains output concurrently; `collectChildOutput()` joins the reader before scanning or closing it.
- `inspectAccess()` checks the actual item location, creator identity and protected sensitive authorizations. `scan()` rejects either dummy marker in captured output or regular files. This is bounded marker evidence, not an exhaustive secret-leak audit.
- `metadata()` snapshots default/search-list paths and requires equality after creation, execution and cleanup. The runner does not change or restore global Keychain lists.
- Cleanup targets only recorded owned Keychains, directory and failed-write sidecars. Sidecar removal checks inode identity; unconfirmed child exit keeps resources owned, and failures remain failures after late cleanup.

## Side Effects

Normal execution launches Metten repeatedly, creates two disposable Keychains and SQLite workspaces, enters dummy values, inspects access metadata, locks/unlocks its own Keychain and temporarily sets `UF_IMMUTABLE` on its own file to provoke a failed replacement. It denies its own access prompt, scans app-specific WebKit/cache/HTTPStorages files if present, and deletes owned test resources only after child exit. A stalled native prompt may keep the runner waiting for actual app exit. It does not grant Accessibility permission or access real accounts.

## Verification

`python3 tests/credential-runner.test.py` compiles this runner and exercises the process-only self-test. The separate packaged invocation is documented in [issue-3.md](../docs/verification/issue-3.md). Do not rerun packaged credential verification for documentation-only changes; existing bounded evidence is reused, not newly executed.
