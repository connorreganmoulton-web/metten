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
.scratch/issue3-finished-runner target/release/bundle/macos/Metten.app > .scratch/issue3-final-verification.log 2>&1
python3 tests/credential-runner.test.py
```

The complete packaged sequence returned **exit 0**. The process-only regression check also passed on the final runner source. Packaged-sequence runner source SHA-256 (before the later broken-connection fix) `0b5278ec71601d78f4109324d6000b270ff48dd09d2200da5d97c639f3a24c6d`; final sanitized log SHA-256 `197d29a8c6670c262ab8c1d21d018d062146b3320e58d5392a01c65b633bc624`.

| Issue #3 requirement | Evidence and result |
| --- | --- |
| Native nonsynchronizing Keychain add/read-for-trusted-use/replace/delete in packaged app | Passed Save A, duplicate refusal, B mismatch/A match, reopened A match, Replace B, A mismatch/B match, Remove and reopened Missing. Explicit file-based/nonsynchronizing options are source-inspected. The actual item location and creating-app/sensitive-operation ACL checks passed; the second matching-identity fixture remained unchanged. Native local comparison is the trusted-use stand-in; no provider connection is claimed. |
| Secret-free returned status and inspected persistence surfaces; no generic secret-read command | Native command returns only fixed statuses/errors. Native request-decoder and resolved-permission tests cover malformed requests and permitted origins. Packaged input clearing, screen checks, captured app output, disposable files including SQLite auxiliaries, and app-specific WebKit/cache scans passed for both generated markers. HTTPStorages was absent. No export/backup feature exists; its future behavior and exhaustive in-memory browser-storage/IPC enumeration are outside this proof. |
| Unavailable/denied access refuses safely, lock/unlock behavior and signing requirements recorded | Packaged Save without bootstrap returned Unavailable; locked Save/local use returned Locked; explicit runner unlock/retry passed. The real native replacement write refusal preserved B. A runner-created protected dummy item caused an actual Metten access prompt; pressing Deny led to the app's fixed **Canceled** status. This is observed native prompt refusal on trusted local use, not a save-specific denial or merely injected status mapping. Other refusal categories retain controlled mapping coverage. Signing limitations are recorded above. |
| Dummy values only; real accounts and paid signing need separate authority | Only in-memory generated dummy values/passwords and disposable Keychains/workspaces were used. No real credentials, network/provider/account calls, paid signing or trading. |

Additional observed lifecycle evidence: on a second native read prompt, the runner requested guarded window-close. Metten remained alive during the pending prompt, then exited with status 0 after Deny. The log records both “guarded Quit retained the app during an actual pending Security prompt” and “guarded Quit finished after actual native access denial”. This is genuine pending native-call Quit evidence. Deterministic queued-command refusal remains the separate controlled test of the production mutex/quitting guard; it is not claimed as packaged queued IPC injection during that prompt. No arbitrary Security-call cancellation mechanism was added.

Final scans passed, default/search-list path metadata matched both before the sequence and after cleanup, and the runner deleted its exact owned main/fixture Keychains and recorded failed-write sidecar/directory. The final and preceding failed-attempt directories were independently confirmed absent, along with the three owner-approved historical targets. No owned app or verifier remained running.

### Final scope/review and applicable checks

The lead checked the connected credential input, native operations, status handling, permissions, close/recovery paths, tests and dependencies using Ponytail review guidance. No additional reviewers were used. The concrete adverse transition checked was Quit while a real Keychain read awaited a macOS prompt: the app stayed alive until denial and then saved/stopped/exited; the separate shared-guard test refused queued actions. No outstanding in-scope finding remained. The failed prompt-discovery path was repaired in the runner, without changing the app or engine. No speculative framework, provider implementation or unnecessary dependency was added.

Previously passing TypeScript/three interface tests, fourteen reported Rust tests, Clippy with warnings denied and the production app build still cover the unchanged app/engine source and the exact finished bundle. The final runner was recompiled and its process regression/native sequence repeated because prompt handling changed. Final Rust formatting and tracked/untracked whitespace checks passed. No remote CI was triggered. The two intended local commits separate the issue implementation/tests/evidence from `AGENTS.md`'s no-bloat rule. Global communication preferences remain outside this repository.

Distribution and upgrade-signing continuity, other Macs, power-loss durability, runtime compromise protection, future exports/backups and trading readiness remain outside scope. Deprecated file-Keychain APIs remain a maintenance limit. Full independent enumeration of browser memory/IPC and a native dictionary-interception trace were not performed; source, native resolver checks and the exact packaged behavior provide the bounded evidence stated here.

## Historical failures and cleanup

Early verification attempts left disposable dummy state and a retained verifier. The available evidence initially established later filesystem identity, rather than creation-time ownership, so the state was preserved pending exact owner authorization. Private session records and machine-specific process/file identifiers are intentionally omitted from this public report.

The owner subsequently authorized disposal of the identified test state. Process and filesystem identities were rechecked before narrowly scoped termination and deletion; only known disposable sidecar restrictions were cleared. All approved targets were confirmed absent. This was authorized disposal of failed test state, not proof that those failed sequences passed. The later complete packaged sequence above supplies acceptance evidence.

Runner repairs made progress visible while waiting, retained ownership until actual child exit, and preserved a failed result after late cleanup. Process-only checks verified delayed exit recovery and unsuccessful-exit reporting. The later broken-connection fix applies checked `SO_NOSIGPIPE` before private-channel writes so a closed connection reaches existing failure/recovery handling. Its regression checks normal message framing and handled closed-peer failure; the test reproduced signal-13 termination before the fix and passes afterward. App and Keychain operations were not repeated for that runner-only change.

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

## Earlier package checkpoint (historical)

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
| `cargo test --workspace --locked` | Eleven reported tests passed: four desktop/native/permission tests, one engine unit test and six integration/child entries. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed. |
| `cargo fmt --all --check` | Passed. |
| `git diff --check` | Passed. |
| `npm run tauri build -- --bundles app` | Passed; includes production TypeScript/Vite build. |
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

## Earlier incomplete evidence and incidents (historical; superseded above)

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
