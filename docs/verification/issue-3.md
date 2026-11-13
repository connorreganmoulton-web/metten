# Issue #3 — local dummy credential implementation

Local work on 12 November 2026 (America/Chicago; execution continued on 13 November UTC). Base `56c8a3cee9b920d4fa2685a15cca02e5cb50ba3a`, initially clean. Issue #3 was open with no comments; its only dependency, #2, was closed. No commits, pushes, PRs, issue changes, paid signing, real credentials, provider calls or trading occurred.

**Current disposition: issue #3’s local dummy-only acceptance checks passed, and all known disposable state was cleaned up.** Metten and the verifier are stopped. No real credentials, accounts, provider calls, money or trades were used. This is local integration evidence within the stated limits below; no release or trading readiness is claimed. Earlier checkpoints are historical evidence, not current blockers or operating instructions.

## Finished local verification (13 November UTC)

Connor requested execution of the three discussed tasks: finish verification, check it against issue #3, and save the credential work and project instructions in two local commits once the evidence passes. No push, PR, GitHub write, additional reviewer, paid signing or real credential/account operation was authorized or performed. Issue #3 was rechecked open with no comments; its sole dependency #2 remained closed.

### Cause, repair and complete packaged evidence

The repaired runner first exposed the precise prompt-discovery failure. All preceding checks passed, but the long-running process failed to find the actual SecurityAgent prompt. An independently started native helper found exactly one Metten/dummy Deny control and pressed it. The repaired runner then observed the late app exit, reported failure honestly and cleaned its own Keychains/sidecar/directory. This failed sequence is preserved in `.scratch/issue3-prompt-discovery-failure.log`; it is not counted as passing acceptance.

The installed public AppKit `NSRunningApplication.h` states that `NSWorkspace.runningApplications` changes only when the main run loop runs in a common mode (lines 180–184 in this installed SDK). The runner had polled that property while sleeping. Its narrow prompt loop now services the main run loop before each lookup. The same real-prompt check then passed without an external helper. The guarded close request was factored for reuse by ordinary Quit and the pending-prompt check; there is one implementation of the close request, with its existing once-per-child guard.

The runner also prints its newly created private directory's path/device/inode, without passwords or dummy values, so later provenance does not depend only on matching a temporary name. It keeps dummy values/passwords in memory and the inherited private socket.

**One unchanged finished bundle** was used for the complete passing sequence: executable SHA-256 `d0bb70982f753311c7be29a3b0f0f31062639398dc5f95a58d9d10356fda15ec`; bundle-tree SHA-256 `1dbb5e4d1800af790295abe1e150a886a4cd6a7a42fc3a21e3579753f3d62f93`. Strict signature verification passed before and after the sequence. Existing ad-hoc signing, identifier `com.metten.foundation`, arm64, absent team identity/entitlements remain unchanged. No rebuild was needed for runner-only changes.

Commands executed:

```sh
swiftc -suppress-warnings scripts/verify-credential.swift -o .scratch/issue3-finished-runner
.scratch/issue3-finished-runner /Users/connormoulton/Desktop/metten/target/release/bundle/macos/Metten.app > .scratch/issue3-final-verification.log 2>&1
python3 tests/credential-runner.test.py
```

The complete packaged sequence returned **exit 0**. The process-only regression check also passed on the final runner source. Source SHA-256 `0b5278ec71601d78f4109324d6000b270ff48dd09d2200da5d97c639f3a24c6d`; final sanitized log SHA-256 `197d29a8c6670c262ab8c1d21d018d062146b3320e58d5392a01c65b633bc624`.

| Issue #3 requirement | Evidence and result |
| --- | --- |
| Native nonsynchronizing Keychain add/read-for-trusted-use/replace/delete in packaged app | Passed Save A, duplicate refusal, B mismatch/A match, reopened A match, Replace B, A mismatch/B match, Remove and reopened Missing. Explicit file-based/nonsynchronizing options are source-inspected. The actual item location and creating-app/sensitive-operation ACL checks passed; the second matching-identity fixture remained unchanged. Native local comparison is the trusted-use stand-in; no provider connection is claimed. |
| Secret-free returned status and inspected persistence surfaces; no generic secret-read command | Native command returns only fixed statuses/errors. Native request-decoder and resolved-permission tests cover malformed requests and permitted origins. Packaged input clearing, screen checks, captured app output, disposable files including SQLite auxiliaries, and app-specific WebKit/cache scans passed for both generated markers. HTTPStorages was absent. No export/backup feature exists; its future behavior and exhaustive in-memory browser-storage/IPC enumeration are outside this proof. |
| Unavailable/denied access refuses safely, lock/unlock behavior and signing requirements recorded | Packaged Save without bootstrap returned Unavailable; locked Save/local use returned Locked; explicit runner unlock/retry passed. The real native replacement write refusal preserved B. A runner-created protected dummy item caused an actual Metten access prompt; pressing Deny led to the app's fixed **Canceled** status. This is observed native prompt refusal on trusted local use, not a save-specific denial or merely injected status mapping. Other refusal categories retain controlled mapping coverage. Signing limitations are recorded above. |
| Dummy values only; real accounts and paid signing need separate authority | Only in-memory generated dummy values/passwords and disposable Keychains/workspaces were used. No real credentials, network/provider/account calls, paid signing or trading. |

Additional observed lifecycle evidence: on a second native read prompt, the runner requested guarded window-close. Metten remained alive during the pending prompt, then exited with status 0 after Deny. The log records both “guarded Quit retained the app during an actual pending Security prompt” and “guarded Quit finished after actual native access denial”. This is genuine pending native-call Quit evidence. Deterministic queued-command refusal remains the separate controlled test of the production mutex/quitting guard; it is not claimed as packaged queued IPC injection during that prompt. No arbitrary Security-call cancellation mechanism was added.

Final scans passed, default/search-list path metadata matched both before the sequence and after cleanup, and the runner deleted its exact owned main/fixture Keychains and recorded failed-write sidecar/directory. The final directory `metten-credential-A15B4B68-CBFA-4179-9F9F-4A41A5C74AF0` and the preceding cleaned failed-attempt directory `metten-credential-660AB6F3-9DFF-47BE-AC53-65B5DC70505F` were independently confirmed absent, along with the three owner-approved historical targets. No owned app or verifier remained running.

### Final scope/review and applicable checks

The lead checked the connected credential input, native operations, status handling, permissions, close/recovery paths, tests and dependencies using Ponytail review guidance. No additional reviewers were used. The concrete adverse transition checked was Quit while a real Keychain read awaited a macOS prompt: the app stayed alive until denial and then saved/stopped/exited; the separate shared-guard test refused queued actions. No outstanding in-scope finding remained. The failed prompt-discovery path was repaired in the runner, without changing the app or engine. No speculative framework, provider implementation or unnecessary dependency was added.

Previously passing TypeScript/three interface tests, fourteen reported Rust tests, Clippy with warnings denied and the production app build still cover the unchanged app/engine source and the exact finished bundle. The final runner was recompiled and its process regression/native sequence repeated because prompt handling changed. Final Rust formatting and tracked/untracked whitespace checks passed. No remote CI was triggered. The two intended local commits separate the issue implementation/tests/evidence from `AGENTS.md`'s no-bloat rule. Global communication preferences remain outside this repository.

Distribution and upgrade-signing continuity, other Macs, power-loss durability, runtime compromise protection, future exports/backups and trading readiness remain outside scope. Deprecated file-Keychain APIs remain a maintenance limit. Full independent enumeration of browser memory/IPC and a native dictionary-interception trace were not performed; source, native resolver checks and the exact packaged behavior provide the bounded evidence stated here.

## Owner-authorized cleanup and lean-code check (13 November UTC)

Connor explicitly approved ending the old verifier and deleting only the three previously identified disposable directories after checking they had not changed. This supersedes the earlier preserve/wait instructions **only for those targets**.

The exact verifier identity was rechecked: PID `8235`, parent `45982`, executable `/tmp/metten-credential-runner`, start `Thu Nov 12 23:41:36 2026`. No packaged Metten process was running. Each approved directory's recorded device/inode, owner, mode and exact file set matched; files were regular, singly linked, or known directories, with no symlinks. The newest workspace contained only `session.sqlite` (inode `170420797`) and `session.lock` (inode `170420796`), beneath recorded workspace inode `170420795`.

Only PID `8235` received SIGTERM; its process entry disappeared. All target metadata was rechecked before deletion. Cleanup used directory-relative operations with no-follow directory/file opens, checking identities again before removal. Only the two recorded sidecars (inodes `170399360` and `170420984`) had their immutable flags cleared, through exact file descriptors. No Keychain API, password recovery or file-content read was used. The default foundation workspace, private conversation records, saved logs, source work and unrelated processes/files were preserved.

Verified absent after cleanup:

- `/private/var/folders/8b/3ct60xws49sc9pdgdx551vmc0000gn/T/metten-credential-606A4BFD-D3EC-435B-A3BE-1DF6835731CA`
- `/private/var/folders/8b/3ct60xws49sc9pdgdx551vmc0000gn/T/metten-credential-B3088A3D-3C69-4251-8117-ADF0B7D5C5A8`
- `/private/var/folders/8b/3ct60xws49sc9pdgdx551vmc0000gn/T/metten-credential-E896B059-FA89-411F-9CFC-82D0AE5C9F68`

This is authorized disposal of failed dummy test state, **not** successful native Keychain deletion verification or proof that the stalled sequence passed. It intentionally discards the old verifier's dummy in-memory state. No real credentials, account access, provider calls, money or trades were used.

The repository's current engine, desktop/credential flow, interface, runner, tests, dependency/build configuration and documentation roles were checked for unnecessary additions. No speculative feature/framework or demonstrably unused dependency was identified. The required isolation, secret-free errors, identity checks, retained evidence and regression tests have current purposes and were preserved. One redundant failure assignment in the runner's late-exit branch was removed: failure is already recorded before waiting. Its process-only regression check passed again after this edit. No broad redesign or generated-output purge was performed; this lean-code check is not an exhaustive security audit.

The no-bloat requirement is now explicit in the project's `AGENTS.md`: current approved requirements only, reuse existing tools, remove confirmed unused/duplicate work within scope, and preserve necessary checks/evidence and unrelated work. Saved changes remain uncommitted; no GitHub changes occurred. Next credential work remains a separate focused completion of the unresolved native prompt test; cleanup itself is finished.

## Runner recovery repair (13 November UTC, after the blocked prompt test)

Owner authorized repairing the runner's progress reporting and recovery path. This does **not** authorize ending the retained old verifier or disposing of the three retained directories. No new packaged-app/Keychain sequence was started.

A read-only `/usr/bin/sample 8235 1 10 -file /dev/stdout` check showed the old verifier's main thread in `main -> NSRunLoop.run -> CFRunLoopRun`, across all 87 samples of that thread. Together with the source's only explicit `RunLoop.current.run()` call, this establishes its current preservation wait. It does not establish the initial failing test step. Metten was absent from the targeted process listing.

The repaired `scripts/verify-credential.swift`:

- Disables stdout buffering before progress is printed, so fixed, sanitized messages reach redirected output while the process is still pending. Adds fixed stage messages around the prompt request and guarded close.
- Uses one shared owned-child wait path for ordinary Quit and late-exit recovery. It clears the child identity only after `waitpid` confirms that exact child exited; only then does it close/capture the existing channels and permit the existing owned-state cleanup. A nonzero exit remains a failure.
- Replaces the endless preservation RunLoop with repeated read-only exit checks. Pending/unknown exit state still retains ownership and blocks cleanup; it sends no termination signal or arbitrary Security cancellation.
- Records that a close action was attempted before sending it, so an ambiguous Accessibility reply does not cause a second close request. The record resets only for a newly spawned owned app.
- Keeps the failed verification result after a late app exit; cleanup does not convert failure into success.

**Focused verification performed:** `python3 tests/credential-runner.test.py` compiled the complete Swift runner in a newly owned temporary directory and ran its explicit `--self-test` mode. This mode bypasses app, Keychain, workspace and Accessibility operations. A controlled `/bin/cat` child stayed alive past the initial wait. The Python check observed the runner's progress in a redirected file before releasing its gate. After release, the actual recovery path detected exit and closed its channels; repeated waiting with no child was safe. A separate `/usr/bin/false` child verified that unsuccessful exit still reports failure. The final check passed in approximately 2.4 seconds, including compilation. Its temporary files were removed by the test's own temporary-directory context.

A temporary-copy negative control removed only the stdout-buffering change. Before gate release the process remained alive and its output file stayed empty; after release the same process-only check finished and its messages appeared. This independently reproduces the previous missing-progress symptom without changing the saved source or accessing Keychain. This control passed. `git diff --check` and whitespace checks on the newly added runner/test/document passed.

These are controlled process-recovery and output checks. They do **not** prove the complete native access-denial sequence, behavior of a pending Security call, or real Keychain cleanup. The app bundle and Rust/interface source were unchanged by this repair, so their existing applicable checks were reused without rebuilding. No signing, publication, account access, provider calls or trading occurred.

**Retained limitation:** a source repair cannot update the already-running old executable. PID `8235` and all three retained directories remain untouched; the old runner still has no recovery channel. The exact original prompt-test failure and full issue acceptance remain unresolved. The existing separate recovery/disposal decision below still applies.

## Completion-session checkpoint (13 November UTC)

**Acceptance remains incomplete; do not rerun the prompt sequence or terminate the retained verifier as routine cleanup.** Metten itself is stopped. The failed verification runner PID `8235` remains alive, deliberately retaining its in-memory dummy password and owned references. No real credential, provider, account, money or trade was used. No publication or GitHub write occurred. Existing source changes, ignored records and foundation workspace history were preserved.

### Changes and focused proof

- The native request decoder now rejects wrong field types, unknown fields, unsupported operations and any value accompanying Check/Remove. All such errors are the same fixed, secret-free message. Save/Replace still validate the bounded dummy value before a native write.
- The existing mutex-then-quitting check now lives in `Credentials::begin_operation`, called by every accepted credential operation. A controlled concurrency test holds current work, starts four queued workers, marks quitting and verifies each refuses through this actual guard. A simulated quit waiter cannot acquire the operation mutex until current work releases it. This is deterministic controlled locking evidence, **not** a packaged pending-Security-call/engine-exit test. The actual quit handler still marks quitting before waiting on the same mutex and then invokes the existing engine stop/save/exit path; that linkage was source-inspected.
- A filesystem-only test (empty synthetic files, no Keychain APIs) verifies original identity acceptance and refusal of unexpected replacement, hard links, symlinks and a nonprivate parent. The successful-native-write identity refresh remains unchanged. It cannot protect against a compromised same-user process or OS.
- Installed public `SecItem.h` documents the explicit add target and restricted search-list behavior, and warns that no-authentication-UI does not suppress legacy file-Keychain prompts. Installed `SecKeychain.h`, Rust bindings, Tauri authority source, generated app permissions and the actual resolver test were inspected. Submitted file-based/nonsynchronizing options and fixed item identity remain unchanged. This is source/resolver evidence, not an API-interception trace.
- `npm run check` passed (three interface tests); `cargo test --workspace --locked` passed (seven desktop tests, one engine unit test, six integration/child entries); Clippy with warnings denied, formatting and `git diff --check` passed. Engine source and SQLite schema remain unchanged.

### Rebuilt bundle and failed prompt experiment

The production build passed; the new bundle was signed once ad-hoc and strict signature verification passed. Identifier `com.metten.foundation`, arm64, no team identity or entitlements. Executable SHA-256 `d0bb70982f753311c7be29a3b0f0f31062639398dc5f95a58d9d10356fda15ec`; bundle-tree SHA-256 `1dbb5e4d1800af790295abe1e150a886a4cd6a7a42fc3a21e3579753f3d62f93`; CDHash `85d4c5ec2b64899eaf98f9f0531fb8b3d807f715` below belongs only to the earlier bundle.

The Swift runner compiled and its existing Accessibility grant was confirmed before starting. Only its explicitly spawned app and disposable workspace were used. The sequence was extended with a runner-created dummy item after ordinary removal and fixture checks: its normal creating-app access protection should make Metten request access, without modifying ACLs, broadening permissions, changing global interaction settings or touching login items.

**Observed failure:** the packaged run stalled around this real prompt. Read-only native Accessibility inspection of the owned child PID `8436` showed “Checking local credential action” and exactly one SecurityAgent window matching both Metten and dummy, with exactly one Deny button. A narrowly scoped native helper rechecked that unique match and pressed Deny. That action returned success. The owned Metten process subsequently exited; the verifier remained parked. No force exit, debugger, password recovery, permission grant or inspection connector was used. The exact point where the runner stalled is unknown; a blocking Accessibility action is a hypothesis, not established diagnosis.

The verifier redirects stdout to `.scratch/issue3-completion-verification.log`, which remained zero bytes because output was buffered while the process remained alive. Therefore intermediate sequence assertions, the application's resulting access-refusal category, final marker scans, final isolation metadata and cleanup are **not captured proof for the rebuilt bundle**. Do not call this an observed native cancellation status or a successful full packaged sequence. The earlier successful sequence below remains historical evidence for its exact earlier bundle; it does not complete this rebuilt bundle's acceptance. The new prompt sequence is not validated for unattended reuse.

### Cleanup provenance and retained state

The original local execution record was examined at `~/.codex/sessions/2026/11/12/rollout-2026-11-12T23-00-49-01a1d347-9fd8-7611-8cfa-1abc02c5b548.jsonl`. Its operation outputs show the early failures, but the first path-specific record found is a later filesystem inventory at `2026-11-13T05:15:19.956Z`, ordinal 440, command `exec-4ca4ac3b-9d52-484d-b830-f2261bc8a75d`. That later inventory alone is not creation-time ownership proof. No passwords or secret file contents were sought.

Current metadata still matches that later inventory's old file inodes. On device `16777231`, owner uid `501`:

- `.../T/metten-credential-606A4BFD-D3EC-435B-A3BE-1DF6835731CA`: directory inode `170395655`, mode 0700. Keychain inode `170395710`; `.fl7C5EA002` inode `170395656`; workspace SQLite/lock inodes `170395675`, `170395676`, `170395678`, `170395679`. No file was opened or unlocked.
- `.../T/metten-credential-B3088A3D-3C69-4251-8117-ADF0B7D5C5A8`: directory inode `170399210`, mode 0700. Sole sidecar `dummy.keychain-db.sb-1fec5535-vt0rlw`, inode `170399360`, immutable flag `2`. No flag was cleared.
- Newly retained candidate `.../T/metten-credential-E896B059-FA89-411F-9CFC-82D0AE5C9F68`: directory inode `170420767`, mode 0700, birth time `1794548496.460233`, coinciding with this run. It contains a workspace, main/fixture Keychains and immutable failed-write sidecar `dummy.keychain-db.sb-1fec5535-s92PMb` (inode `170420984`). The main Keychain's last inspected inode is `170421547`, fixture inode `170420881`. The runner retains the original in-memory ownership records, but its current code provides no recovery channel from the parked RunLoop. The directory's name/time alone does not independently prove ownership; it has not been reopened or deleted.

All three full paths share prefix `/private/var/folders/8b/3ct60xws49sc9pdgdx551vmc0000gn/T/`. All remain preserved. No directory cleanup, broad deletion, flag clearance or normal workspace alteration was attempted.

**Smallest next owner decision:** authorize failure recovery to end only verifier PID `8235` (after rechecking its identity and that Metten remains stopped), and explicitly designate the above three exact directories for disposal despite the missing creation-time records. Such authority must name the targets; recheck metadata before acting, stop on changes, and limit flag removal to the two identified disposable sidecars. This would intentionally discard dummy-only retained state; it must not be reported as successful verification. Without that decision, preserve the runner and all targets. A later focused verification repair must resolve the native interaction stall and produce one complete sequence on the unchanged finished bundle. Genuine pending-work Quit and the app's response to actual denial remain missing proof.

## Bounded implementation

- `src-tauri/src/credential.rs` contains the Mac-only native adapter. The engine, its SQLite schema and trading behavior are unchanged. No provider framework or generic secret-read command was added.
- Sources/settings accepts bounded `metten-dummy-` values only. It clears the password field before invoking Save/Replace, including errors. Saved, locally readable and exchange-connected are separate messages; no exchange is connected.
- Native code fixes the item class, service and account. `SecItemAdd` targets the selected Keychain with `kSecUseKeychain`. Reads, updates and removal restrict `kSecMatchSearchList` to that one reference. Every query explicitly submits `kSecUseDataProtectionKeychain=false` and `kSecAttrSynchronizable=false`; no SyncAny or returned-sync-field requirement is used. These submitted options were inspected in the source; packaged API operations succeeded with them. There is no separate interception trace of the submitted dictionaries.
- Save rejects duplicates; Replace uses `SecItemUpdate`, without delete-first/upsert; Remove reports missing honestly. Read bytes go only to the native local-use comparison. No value, fragment or fingerprint returns to the screen.
- Without the private inherited verification socket and a valid bootstrap, commands are unavailable. No login/default-Keychain fallback or screen-supplied path exists. Canonical paths, private-directory mode/owner, file owner, device/inode, regular-file status and link count are checked. System/login paths and aliases are rejected. Reopening uses `SecKeychainOpen`, existence/identity checks, actual-path comparison and status, not Open success alone.
- Installed macOS atomically replaces the file on successful writes: the runner observed unchanged path, changed inode and continued existence. Native code records the resulting identity only immediately after its own successful write; unexplained changes before an operation refuse access. This remains within the ordinary trusted-process/OS boundary, not protection from a compromised same-user process or OS.
- Work runs on blocking workers under an operation mutex. Quit marks quitting first, waits for that mutex, then takes the existing engine stop/save/exit path. Accepted commands check quitting after obtaining the mutex. Channel framing finishes on a separate observer before the command; an acknowledgment confirms receipt. The observer permits one outstanding expectation, checks session/sequence/phase, consumes once and rejects protocol failures. Frame reception has a two-second total deadline; idle polling does not hold the operation mutex. No cancellation of arbitrary Security calls is claimed.
- Raw credential request decoding returns fixed errors instead of framework decoding errors that might quote input. No Debug representation of secrets is added. Actual missing/locked/unavailable errors and mapped denied/canceled/authentication statuses remain distinct.
- The app-command manifest includes all three commands: snapshot, control and credential. Only the local main window receives their permissions. No filesystem, shell, SQL, network or secret-reading permission is granted.

## Native runner and finished package

`scripts/verify-credential.swift` is the dummy-only runner. It generates the password and A/B markers in memory; creates a unique private directory and Keychain with `SecKeychainCreate`; records owned references immediately; explicitly inherits its socket using `posix_spawn` descriptor actions and close-on-exec defaults; and resolves the packaged executable from Info.plist. No LaunchServices, clipboard, secret arguments, secret environment variables, password files, security CLI or plaintext dump is used. An existing Accessibility grant was available; none was granted or broadened. Full Xcode/XCUITest was not assumed or installed.

The runner creates A/B in the main Keychain through native Accessibility interaction with the packaged app. Expectations travel only over the private socket and are acknowledged before the corresponding button press. The observer compares the module's actual trusted-use read, not UI input, cached credentials or a second main-item read. Separate item inspection requests a reference and access metadata, not its secret data. The optional second fixture's own read is explicitly separate from the observer.

Final bundle: `target/release/bundle/macos/Metten.app`, arm64. It was rebuilt, then signed once with `/usr/bin/codesign --force --sign -`, and the complete final sequence used that unchanged bundle.

- Bundle/signature identifier: `com.metten.foundation`.
- Signing: ad-hoc; no team identity; sealed resources version 2. `codesign --verify --strict` passed. Entitlement inspection printed no entitlements. No keychain access group, sandbox entitlement, paid developer identity or notarization was used.
- Executable SHA-256: `11a0ad42a85ed30afe6d7c9cec937919105aba76601d8da233b77bd6abb5f735`.
- Bundle-tree SHA-256 (sorted relative file paths plus NUL and raw SHA-256 of each file): `39c0f102990d795b3782ffac05d8c7f0905f32fc04bbe4ee38c8f65b68876fcd`.
- CDHash: `85d4c5ec2b64899eaf98f9f0531fb8b3d807f715`.
- Installed Command Line Tools/SDK supplied the public Security bindings. Deprecated lifecycle functions missing from the Rust bindings are narrowly declared against the installed public header signatures.

## Exact checks and observations

| Check | Result on final behavior |
| --- | --- |
| `npm run check` | TypeScript and three interface tests passed, including immediate clearing and suppression of unexpected error contents. Node reported its existing experimental type-stripping notice. |
| `/Users/connormoulton/.cargo/bin/cargo test --workspace --locked` | Eleven reported tests passed: four desktop/native/permission tests, one engine unit test and six integration/child entries. |
| `/Users/connormoulton/.cargo/bin/cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed. |
| `/Users/connormoulton/.cargo/bin/cargo fmt --all --check` | Passed. |
| `git diff --check` | Passed. |
| `PATH=/Users/connormoulton/.cargo/bin:$PATH npm run tauri build -- --bundles app` | Passed; includes production TypeScript/Vite build. |
| `swiftc -suppress-warnings scripts/verify-credential.swift -o /tmp/metten-credential-runner` | Passed using installed native APIs. Suppresses expected legacy-API warnings; does not make them supported indefinitely. |
| `/tmp/metten-credential-runner <absolute finished bundle path>` | Final sequence returned exit code 0. See steps below. |
| Native error-mapping checks | Controlled OSStatus inputs exercised duplicate, missing, cancellation, authentication, denial and unknown/unavailable categories. These are mapping tests, not observed access prompts. |
| Tauri resolved-authority check | Actual generated context permits all three commands in local main; rejects other-window and remote origins and an invented secret-read command. This is a native resolver check, not a second packaged hostile-webview test. |

Final packaged sequence:

1. Start, Advance and Pause work through the retained controls; saved clock reaches 1. Window-close exits through guarded Quit on every relaunch.
2. Save A; input clears. Empty dummy input refuses, duplicate Save refuses. Expect B fails; expect A passes. Failure leaves the item available for the following correct comparison.
3. Public item/access inspection confirms the actual item Keychain, the creating-app identity and protected sensitive authorizations. The public `SecAccessCopyACLList`, `SecACLCopyContents`, `SecACLCopyAuthorizations` and `SecTrustedApplicationCopyData` APIs are used. All-app entries are accepted only for encryption; ACLs are never modified. Trusted-app identity is compared with a reference derived from this exact executable.
4. Reopen the same bundle and exact Keychain; A passes. Replace with B; A fails and B passes.
5. A controlled immutable flag on the exact owned disposable file causes a real native replacement refusal. The app reports Unavailable, clears input, and B still passes after removal of the test restriction. This is a real API/storage failure, not an injected return code or an access-denial prompt. macOS creates a failed-write sidecar carrying the flag; the final runner records its exact identity and removes its own restriction/artifact during cleanup. Keychain ACLs/global settings are untouched.
6. Lock the exact owned Keychain with `SecKeychainLock`; Save and local use report Locked without auto-unlocking. Explicit `SecKeychainUnlock` and retry allow B to pass.
7. Remove; fresh local reads report Missing. Reopen; Missing remains. Repeated removal reports Missing.
8. A silent expectation channel fails promptly. Stale and duplicate expectations refuse; a closed channel refuses. An incomplete frame does not block guarded window-close exit. These are controlled protocol failures, not evidence that a native Security prompt can be interrupted.
9. A second disposable Keychain has the same class/service/account identity. The main target still reports Missing; the fixture's independently checked B survives the main replacement/removal sequence. No search-list insertion is used.
10. Launch without isolated bootstrap; Save reports Unavailable and clears input.
11. A/B scans pass for cleared native screen state, captured application output, all disposable workspace files including SQLite/WAL/SHM and Keychain files, and the app-specific WebKit/cache directories. The app-specific HTTPStorages directory was absent. Only pass/fail and locations are emitted. In-memory browser storage and every individual response were not independently enumerated; source adds no browser-storage writes. No future backup/export claim is made.
12. Default/search-list path metadata is identical before creation, after the app sequence and after final cleanup. No credential items in those Keychains were read; no old list was restored. Final run deletes both exact owned Keychains, the recorded failed-write artifact and its private directory. No Metten process remains after final verification.

Issue #2's unchanged engine/storage evidence and PR #20's completed checks remain applicable. Old pending-CI wording in issue #2's local record is not current PR status. This task did not run new remote CI or repeat unchanged power-loss/storage work.

## Incomplete evidence and incident record

- Two early attempts left disposable state: one directory containing an encrypted dummy item plus workspace files, and one directory containing an immutable failed-write sidecar after Keychain deletion. The earlier runner lost its in-memory ownership/password state on exit. These remnants were identified by filesystem metadata only and were not reopened, unlocked, inspected for secret data or deleted without the original identity record. Later/final runs cleaned up successfully. Full cleanup across all attempts is therefore incomplete.
- The native inspection connector unexpectedly reopened the default foundation workspace twice when observing after close. It was closed immediately; ordinary open/close history records were appended there. No credential input, Keychain setup, provider access or trading occurred in those instances. The final runner uses only its explicitly spawned process and disposable workspace.
- Actual denied/canceled/authentication prompts were not induced. Mapping coverage must not be called an observed denial/cancellation. Quit during a genuine pending Security prompt and deterministic queued-command rejection while such OS work is pending remain unverified. Such a prompt must complete/cancel before guarded exit; no force-success/detach mechanism is added.
- The supplied options, UI clearing and resolver have focused checks, but exhaustive malformed JSON/identity-substitution/race tests and an independent native API-interception trace are missing. Do not claim a security audit or complete adversarial coverage.
- Exports/backups do not yet exist. This change introduces no secret persistence source into them or the engine, but future export/backup behavior is unproven.
- Ordinary provider setup is deliberately unavailable. Distribution installation, upgrade signing continuity, other Macs/architectures, physical power-loss durability, runtime compromise protection and trading readiness remain unproven. File-Keychain management APIs are deprecated.

## Public grounding

The complete product specification and ADR 0001 govern behavior and boundaries. Installed SDK headers and current public documentation grounded the native options and lifecycle; installed Tauri source grounded the app manifest/resolver.

- [Apple: file-based versus data-protection Keychain](https://developer.apple.com/documentation/technotes/tn3137-on-mac-keychains)
- [Apple: SecKeychainCreate](https://developer.apple.com/documentation/security/seckeychaincreate(_:_:_:_:_:_:)) and [SecKeychainOpen](https://developer.apple.com/documentation/security/seckeychainopen(_:_:))
- [Apple: explicit add target](https://developer.apple.com/documentation/security/ksecusekeychain)
- [Tauri capabilities and app-command manifest](https://v2.tauri.app/security/capabilities/)
