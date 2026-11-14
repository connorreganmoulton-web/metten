# Issue #31 — repeatable non-trading packaged checks

## Current evidence index

- [Checklist](#checklist): one bounded sequence for the current Mac.
- [Operator commands](#operator-commands): existing native app, Python standard library and dummy runner.
- [Execution record](#execution-record): actual results and missing evidence; historical #2/#3/#4 results are not fresh runs.

This checks implemented local session, strategy-history and dummy-credential behavior only. It does not establish provider access, trading, physical power-loss durability, another Mac, signed-update credential continuity or full #17/#18 security/release acceptance. No real credentials, accounts, signing identities, paid services, new permission grants or publication are authorized.

## Prerequisites and ownership

Use an awake, interactive Mac with the pinned Rust toolchain, Node/npm, Swift Command Line Tools, Python 3, and the existing native Accessibility control surface. The native control surface and compiled credential runner need existing Accessibility authorization. If either is unavailable, record BLOCKED for its rows; do not grant permission, switch to real credentials or count `--self-test` as Keychain evidence.

Work from the selected checkout, initially record `git rev-parse HEAD` and `git status --short`. Record any source changes separately. Build with `npm ci` then `npm run tauri -- build --bundles app --no-sign --config '{"productName":"Metten Verification Test","identifier":"com.metten.verificationtest"}'`. Put the existing Cargo bin directory on PATH if necessary; do not install another toolchain or change shell settings. This creates a uniquely selectable owned bundle under `target/release/bundle/macos/`. No app feature/configuration file changes are needed. Never select a different Metten app through a shared name or identifier.

Keep private absolute paths, ownership identities, native output and runtime records in ignored `.scratch/issue31/` or the disposable directory. Publish only checkout-relative artifact locations, source IDs, hashes, environment, fixed result categories and sanitized observations. Do not publish raw logs/screenshots without checking for private paths and runtime data.

The Python process below owns each launched child through `Popen`; retain that process until `wait()` confirms actual exit. A timeout is failure/pending ownership, never permission to delete state or force-kill. Do not delete workspaces by wildcard, touch user workspaces, alter global Keychain lists, dismiss unrelated prompts or operate another trading application. The credential runner separately owns its children, dummy Keychains and cleanup.

## Checklist

Every row requires its prerequisites and records PASS, FAIL or BLOCKED plus a local evidence pointer in the execution record. Stop dependent steps after failure; retain state until cleanup is verified. No blanket pass based on a screenshot or earlier artifact.

| Step / named harness | Action and prerequisite | Observable pass / failure | Evidence to retain |
| --- | --- | --- | --- |
| A. Artifact identity / build CLI + Python manifest | Build the unique bundle; run manifest recipe before launch. Record macOS/build, architecture, source revision, bundle location, runner source hash, codesign details/verification and notarization status. | Build exits 0; manifest identifies all bundle files. Record unsigned/ad-hoc honestly; no distribution identity or notarization inferred. | Build log, manifest and signature output; sanitized identity in execution record. |
| B. First launch / Python-owned child + native Accessibility | Launch only with fresh owned workspace, no verification descriptor. Confirm PID/executable and open workspace with `ps`/`lsof` before binding the exact bundle path. | Stopped, clock 0, no saved versions; no trading/account activity. Wrong PID/path is FAIL; inaccessible native controls are BLOCKED. | Owned PID/location and initial native observation. |
| C. Ordinary credential refusal / same native app | With no bootstrap, enter a disposable dummy value, press Save, then Check local credential use. Never use real input. | Save reports Unavailable; local use reports Verification unavailable; input clears. No Keychain prompt or credential read is expected. | Fixed statuses and cleared-input observation. |
| D. Session truth / same native app + read-only SQLite | Start, Advance once, Pause. Capture session rows read-only. | Running follows saved Start, clock 1 follows saved Advance, Pause disables Advance; saved history agrees. | Native observations and local `baseline_session` rows. |
| E. Import and preview / native file chooser | Choose this checkout's `examples/learning.json`; Preview current text. | Import populates text; Save requires preview; generated explanation describes remembered unique side below 40 during [0,60), opposite side at most 30 then remembered side at most 30 during [60,120). Preview adds zero versions. Unverified-source/no-trading notice remains. | Native preview and read-only version count. |
| F. Immutable versions / same app + SQLite | Save reviewed version 1; capture entire row. Inspect version 1, Edit a copy, change only the first purchase threshold 30 to 29, Preview, Save version 2. | Two distinct versions; version 1 entire row remains equal; version 2 JSON contains only intended edit and explanation reflects 29. Session baseline unchanged by strategy actions. | `baseline_version`, exact JSON comparison and stored explanations; visible save/detail results. |
| G. Safe quit and reopen / owned child | Request guarded window close; wait for actual exit, then reopen the same bundle/workspace and reconfirm PID/storage before selecting it. Inspect both versions. | Exit 0; saved Workspace closed — stopped; reopen stopped, clock 1, original session rows and both exact version rows retained. | `wait()` result, read-only rows, reopened native observations. |
| H. Controlled storage failure / Python SQLite + native app | Start; capture all confirmed session/version rows. Hold `BEGIN IMMEDIATE` only on this disposable database; press Advance. | Stopped — recording needs attention, requested change unconfirmed, actionable space/permissions/reopen advice; Start/Advance blocked. Confirmed rows unchanged; no failed Advance saved. | Failure UI and full row equality while lock held. |
| I. Failed Quit and recovery / same owned processes | While lock is held, request guarded close; wait for Quit canceled. Confirm child still alive. Release lock with ROLLBACK and close that connection; retry close. Reopen once more. | Failed Quit retains window/child; after release, retry exits 0. Reopen stopped at clock 1, all previously confirmed history/versions preserved, no invented Advance; integrity_check is ok. | Native error/recovery, child wait result and full preserved-row comparison. |
| J. App-only cleanup / Python owner | Close recovered app, wait for exit, close read-only connections, verify bundle manifest unchanged, then remove only owned disposable directory. | Actual child exit confirmed, directory absent, no owned lock holder remains. Pending exit retains ownership and FAIL/BLOCKED status. | Exit results, manifest equality and directory absence. |
| K. Dummy Keychain / existing `verify-credential.swift` | Compile and invoke with the same unchanged bundle, existing Accessibility grant only. See commands below. | Exit 0 and complete PASS sequence: Save/read/reopen/duplicate, Replace/remove/reopened Missing, unavailable write preserving B, locked/unlock, actual owned prompt Deny/refusal, guarded quit while pending, ordinary-launch refusal, marker scans, unchanged default/search-list metadata. A missing prompt/prerequisite is BLOCKED, never inferred success. | Runner log, source hash and exit status; record actual refusal category. |
| L. Native cleanup / same runner | Allow runner's final cleanup and actual child-exit confirmation to finish. | Exact owned dummy Keychains/sidecar/directory deleted, metadata unchanged. Late cleanup does not erase a failed run. If pending, retain runner/process ownership and report exact pending stage. | Cleanup PASS lines and independent absence check of recorded owned directory. |

## Operator commands

Run Python interactively in a persistent terminal so the same owner retains `process` across native UI actions. Use the native connector with the full bundle path only after launch and identity checks; it must bind the existing process, not launch another workspace. For a successful close, send the native close action alone: never combine it with `getAXState()` or another inspection. Call `finish()` in the owner terminal next; the connector can relaunch a closed app during inspection. Only inspect a failed Quit after the owner confirms the child is still alive. Repeat `launch()` only after `finish()` returns. The operator supplies native actions from the checklist; these snippets are not an unattended UI framework.

```python
import hashlib, json, os, pathlib, plistlib, sqlite3, subprocess, tempfile
root = pathlib.Path.cwd()
bundle = root / 'target/release/bundle/macos/Metten Verification Test.app'
info = plistlib.loads((bundle / 'Contents/Info.plist').read_bytes())
exe = bundle / 'Contents/MacOS' / info['CFBundleExecutable']
assert exe.is_file()

def manifest():
    entries = []
    for p in sorted(bundle.rglob('*')):
        if p.is_symlink():
            entries.append([p.relative_to(bundle).as_posix(), 'link', os.readlink(p)])
        elif p.is_file():
            entries.append([p.relative_to(bundle).as_posix(), 'file',
                            hashlib.sha256(p.read_bytes()).hexdigest()])
    encoded = json.dumps(entries, ensure_ascii=True, separators=(',', ':')).encode()
    return hashlib.sha256(encoded).hexdigest(), encoded

identity, manifest_bytes = manifest()
# Save manifest_bytes in the ignored local evidence folder, not public runtime data.
print('bundle-tree-sha256', identity)
owned = pathlib.Path(tempfile.mkdtemp(prefix='metten-issue31-')).resolve()
workspace = owned / 'workspace'
process = None
log = (owned / 'app.log').open('ab')
def launch():
    global process
    assert process is None
    env = dict(os.environ, METTEN_TEST_WORKSPACE=str(workspace))
    env.pop('METTEN_VERIFY_FD', None)
    process = subprocess.Popen([str(exe)], env=env, close_fds=True,
                               stdin=subprocess.DEVNULL, stdout=log, stderr=log)
    print('owned PID', process.pid)  # private evidence only

def finish():
    global process
    assert process is not None
    result = process.wait(timeout=15)  # timeout retains process and workspace
    process = None
    assert result == 0, result

# Call only after the app has opened its database.
def rows(table):
    assert table in ('session_events', 'strategy_versions')
    with sqlite3.connect((workspace / 'session.sqlite').as_uri() + '?mode=ro', uri=True) as db:
        return db.execute('SELECT * FROM ' + table + ' ORDER BY id').fetchall()

launch()
# In another terminal: ps -p <owned PID> -o pid=,comm=
# lsof -a -p <owned PID> -Fn; match exact exe and workspace/session.sqlite.
# Follow native steps B–G; use rows(...) to capture and compare full rows.
# For H–I after Start has saved:
# confirmed = rows('session_events'); versions = rows('strategy_versions')
# lock = sqlite3.connect(workspace / 'session.sqlite')
# lock.execute('BEGIN IMMEDIATE')
# Native Advance, observe failure, then close, observe Quit canceled.
# assert process.poll() is None
# assert rows('session_events') == confirmed
# assert rows('strategy_versions') == versions
# lock.rollback(); lock.close()  # always release this owned lock before retry
# Native close again; finish(); launch(); verify recovery, close; finish().
# Assert every previously confirmed row remains and no tick exceeds 1.
# With read-only SQLite run PRAGMA integrity_check and require [('ok',)].
# assert manifest()[0] == identity
# After final finish(), preserve sanitized evidence outside owned, then:
# log.close(); import shutil; shutil.rmtree(owned); assert not owned.exists()
```

Check the actual table names against `crates/engine/src/storage.rs` before execution. Record full-row equality privately rather than publishing runtime dumps. Never use a user database to provoke the failure. The lock mechanism is the existing issue #2 failure seam, not a claim about disk-full or power-loss behavior.

Compile/run the credential harness only after app-only exit/cleanup:

```sh
mkdir -p .scratch/issue31
swiftc -suppress-warnings scripts/verify-credential.swift -o .scratch/issue31/credential-runner
.scratch/issue31/credential-runner 'target/release/bundle/macos/Metten Verification Test.app' > .scratch/issue31/credential.log 2>&1
```

Record its exit status immediately. Do not impose an external timeout/kill on a runner that owns a pending native prompt. The runner may wait for actual child exit; retain it and record pending ownership. Only its uniquely identified Metten/dummy Deny action is authorized. `python3 tests/credential-runner.test.py` checks process recovery only and cannot replace K/L.

For A/J use `sw_vers`, `uname -m`, `git rev-parse HEAD`, `shasum -a 256 scripts/verify-credential.swift`, `/usr/bin/codesign -dv --verbose=4 '<bundle>'` and `/usr/bin/codesign --verify --deep --strict '<bundle>'`. Record exact statuses, including a nonzero unsigned result; this task does not sign or notarize. A local `--no-sign` build has no notarization submission; report notarization as absent, not verified release approval. Hash the final bundle again after all tests; a changed bundle invalidates same-artifact claims.

## Execution record

Executed 14 November 2026 UTC on macOS 26.3 (25D125), arm64. Starting source revision `96ad37ab547751f1f3b4b55d392bc42be3563e9c`; application source unchanged. Only this record, scripts README and the standalone native runner change. Dependencies #2/#3/#4 were closed; #31 had no comments. The default foundation workspace and other trading apps were not selected. No real credentials, accounts, provider calls, money or trades were used.

### Tested artifact and local evidence

- Artifact: `target/release/bundle/macos/Metten Verification Test.app`, bundle identifier `com.metten.verificationtest`, executable `Contents/MacOS/metten`.
- Source: starting revision above, with the CLI product-name/identifier override shown in prerequisites. The first ordinary build also passed but was not used for acceptance; the unique `--no-sign` build was used for every checklist row.
- Bundle-tree SHA-256 using the exact manifest algorithm above: `d18e1879f91566d08c374d733d5ccbe6d636abf6e15fc18408eb5421092424a2`, equal before launch, after app-only cleanup and after credential cleanup. Executable SHA-256: `293e78ecc2d89c22caadac03618d5db2cb272ee862d89c684f4bcab763795278`.
- Signing: linker ad-hoc signature, signing identifier `metten`, no team, Info.plist not bound, no sealed resources. Strict bundle signature verification returned 1: “code has no resources but signature indicates they must be present.” This is a recorded packaging limitation, not a passing signature check; no distribution signing or notarization exists. No signing identity was used or acquired.
- Credential runner source SHA-256: `e98d408323d4a24dcbd0fced8e56792751087d1598c37973f635a2318c61ffcb`. Complete credential log SHA-256: `33d39faa9605ef69a95d5ca484b4e9b6e36fbc8fe5808fccc6fe518c39cb9658`.
- Private supporting records: `.scratch/issue31-build.log`, `.scratch/issue31/manifest.json`, `.scratch/issue31/observations.json` (saved-row comparisons), `.scratch/issue31/credential.log`. Native UI observations are in this session's tool record. Private paths and raw runtime rows are intentionally not copied here. The process-owning Python terminal executed the documented helpers and checks interactively.

### Results by checklist step

| Step | Result | Actual observation / evidence |
| --- | --- | --- |
| A | PASS identity; signature limitation | Unique build exit 0; manifest stable across both sequences. Signing limitations above remain; no signature/notarization acceptance claim. |
| B | PASS | Exact child executable and disposable SQLite file confirmed with ps/lsof. Native screen stopped, clock 0, no versions. |
| C | PASS | Dummy Save reported Unavailable, input empty; local use reported Verification unavailable. No native credential prompt appeared. |
| D | PASS | Start/Advance/Pause saved four rows; native clock 1 and paused controls agreed with SQLite. |
| E | PASS | Native chooser imported this checkout's learning.json. Preview enabled Save and displayed the generated meaning/support notice. Read-only SQLite still held zero versions. Stored full explanation after Save confirmed the complete intended rule meaning. |
| F | PASS | Saved versions 1 and 2. Full original row remained equal; version 2 JSON equaled original with only first `"value": "30"` replaced by `"value": "29"`; explanation equaled original with only first `at most 30 ContractCents` replaced by `at most 29 ContractCents`. All four earlier session rows unchanged. |
| G | PASS with inspection incident below | Child exit 0; stopped close record saved. Reopened exact owned workspace stopped at clock 1; both native Inspect buttons showed their saved versions. All baseline session rows and full version rows retained. |
| H | PASS | Owned SQLite BEGIN IMMEDIATE caused unconfirmed-saving followed by stopped recording failure and actionable locked-database advice. Start/Advance disabled; all seven confirmed rows and both version rows compared equal. |
| I | PASS | Close while locked reported Quit canceled and child stayed alive. After ROLLBACK/connection close, retry exited 0. Reopen stopped at clock 1; every confirmed row retained, no tick above 1, both full versions unchanged, integrity_check ok. |
| J | PASS disposable cleanup | Final child exit 0 confirmed by wait; final close record saved. Owned app-only directory removed and independently absent. Bundle manifest unchanged. |
| K | PASS | Existing Accessibility authorization worked; complete dummy runner exit 0. Save/read/reopen, duplicate/mismatch, Replace/remove/reopened Missing, locked/unlock and unavailable replacement preserving B passed. Actual owned prompt Deny produced Canceled. Pending-prompt guarded Quit retained app, then exited after denial. Ordinary launch refused Save. Marker scans covered disposable files and the actual verification bundle's WebKit/cache directories; HTTPStorages was absent. |
| L | PASS disposable cleanup | Runner confirmed exit before deleting both owned Keychains, failed-write sidecar and directory; default/search-list metadata matched. Independently checked recorded directory absent and no verification app process remained. |

### In-scope corrections and cleanup incident

Both independent review axes found one shared defect in the proposed checklist: the runner scanned hard-coded foundation-app browser directories while this checklist uses a distinct bundle identifier. The runner now derives the three storage paths from the tested bundle's Info.plist, validates identifier components to exclude path traversal, and scans only those locations. Its process-only self-test asserts the new paths and rejects empty/traversal/slash identifiers. The lead checked the complete affected scan path, existing dummy marker comparisons, process exit and cleanup handling; no production app/engine behavior changed. Standards: one finding resolved. Spec: one finding resolved. Reviewers used independent Codex sessions for native state/authority review.

During G, combining close with native inspection caused the connector to relaunch the uniquely named test app in its default workspace. The original owned child had already exited successfully. The extra instance was identified, closed immediately without entering data, and its process exit independently confirmed. Only ordinary open/close records were added in that test identifier's default workspace; it was retained rather than deleted without a pre-test ownership baseline. The checklist now prohibits inspection after a successful close. Later closes used action-only calls followed by the owning process's wait; no further relaunch occurred. App-specific OS browser caches were scanned and retained, not claimed deleted. Both disposable workspaces/Keychains and the actual lock connection were cleaned up as recorded above.

A preliminary invocation mistakenly supplied a nonexistent bundle path. The runner created its own dummy Keychain before rejecting the missing artifact; it exited 1 and confirmed exact Keychain/directory cleanup with unchanged isolation metadata. It is not acceptance evidence. Initial build failed because Cargo was absent from PATH; selecting the existing Cargo bin directory resolved that prerequisite. Neither failure is counted as a passing run.

### Verification and remaining limits

After review resolution, the final candidate passed:

- `python3 tests/credential-runner.test.py`: process-only recovery plus bundle-specific scan-path and invalid-identifier assertions; one Python test passed.
- `python3 tests/check-staged.test.py`: five tests passed.
- `npm run check`: TypeScript and all 12 interface tests passed. Production TypeScript/Vite build passed in both package builds. There is no separately configured JavaScript lint gate; type checking is wired into check/build.
- `cargo fmt --all --check`, `cargo test --workspace --locked` (100 tests passed, two slow tests explicitly deferred to the next command), and `cargo clippy --workspace --all-targets --locked -- -D warnings`: passed.
- `cargo test -p metten-engine --release --locked --test strategy -- --ignored --nocapture`: both controlled full-hour tests passed.
- `git diff --check`: passed before staging; staged whitespace and final commit checks recorded by the lead.

Logs are local under `.scratch/issue31/` with names `runner-check.log`, `staged-check.log`, `frontend-check.log`, `cargo-test.log`, `clippy.log` and `full-hour.log`. The source changes were reviewed against starting revision above before final regression. The scan correction was resolved by the lead across all three actual bundle-storage locations and exercised by the complete packaged credential sequence.

Process-only checks never substitute for K/L. Base revision remote Foundation checks passed, verified through GitHub. New-head remote CI is missing: this task commits locally and does not push or create a PR. Local checks are not represented as remote CI confirmation. Same-build dummy evidence cannot establish credential continuity across a signed update, release security, other Macs, power-loss durability or live fills.
