# Issue #30: CodeQL coverage and merge checks

## Current evidence index

Issue #30 remains open. The initial default scan's Mac credential coverage gap
is recorded below. A dedicated workflow now proposes Rust analysis on macOS;
its actual extraction evidence and independent review are still pending.
Scanning remains nonblocking; the existing `mac-foundation` merge requirement
remains in force. This record is not security or release acceptance.

## Mac-runner correction — 14 November 2026 UTC

Scope: `.github/workflows/codeql.yml`, the contributor maintenance notes, and
this evidence record. The branch incorporates main at
`4957ea16af9740da64544a1e9911d34f96a77373`; no application source, compiler,
Foundation workflow or required merge check is changed.

The new workflow uses `macos-15` for Rust and `ubuntu-latest` for
JavaScript/TypeScript, Actions and Python. All use build mode `none` and the
standard default queries, without custom packs or path exclusions. Actions and
Python preserve the other working default-setup analyses. Swift remains excluded
because its standalone harness failed default autobuild; no Swift coverage is
claimed. Pushes to main, pull requests to main and a weekly schedule trigger
analysis. No `pull_request_target` event, application launch or credential test
is introduced. The scanner alone receives `security-events: write` to upload
results; ordinary Foundation permissions remain `contents: read`.

The checkout and Rust installer pins reuse #28's reviewed revisions. The Rust
installer explicitly preserves **1.100.0**. CodeQL `init` and `analyze` use
[`33d7b85da1a04171e211befe2c739303b9a4c066`](https://github.com/github/codeql-action/commit/33d7b85da1a04171e211befe2c739303b9a4c066),
resolved from **v4.39.3**, the same action version used by the initial default
scan. The annotated tag is unsigned; GitHub reports the target commit signature
valid. Reviewed both action manifests and their initialization/finalization,
query-running and upload entry paths. Both run bundled Node 24 code and contain
no nested action references. Default tool selection and standard queries remain
upstream-managed, rather than introducing a separate toolchain pin. This is a
bounded provenance/configuration review, not an audit of all bundled code.
The existing monthly Actions Dependabot configuration covers this new workflow;
the contributor guide records paired updates and the existing manual fallback.

Default setup must be disabled before this workflow can upload results; GitHub
blocks advanced uploads while default setup is enabled. That transition and the
first advanced run will be recorded in the issue/PR. Until the workflow is merged,
main has no advanced workflow, so default-branch scanning cannot be claimed.
No required scanning check will be promoted before actual default-branch and
representative-PR evidence, alert triage and missing/failed-analysis enforcement.

Local preparation checks passed: Ruby YAML parsing plus assertions for the
language/runner matrix, event scope, least-privilege permissions, fail-fast
behavior, immutable action references, Rust 1.100.0 and build mode `none`.
Foundation workflow, compiler and application lockfiles match current main
byte-for-byte; whitespace check passed. No repository actionlint gate is wired.
Independent review and hosted execution remain outstanding.

Required next evidence: full final-head Foundation checks; successful per-language
CodeQL jobs; Rust extraction with the native module's semantic analyzer available,
including macro/type diagnostics, not merely disappearance of one warning or a
9/9 file count; and default-branch validation after manual merge.

## Baseline and setup — 14 November 2026 UTC

Repository: `moultonlavah/metten`. Source revision:
`6861d8e264b4a78cacbf91e89bb1145a8775b27d`. Prerequisite #28 is closed and its
pinned workflow is present. Before this work, the default-setup API reported
`not-configured`, no CodeQL workflow existed, and the repository had no rulesets.

Enabled GitHub-managed default setup with the `default` query suite and default
`remote` threat model. No custom workflow, query pack, path exclusion, compiler
change or application change was introduced. GitHub owns the generated workflow;
repository-maintained workflow actions retain their reviewed #28 pins.

An explicit REST request selecting `javascript-typescript` and `rust` failed
with HTTP 422. The exact request was:

```sh
gh api --method PATCH repos/moultonlavah/metten/code-scanning/default-setup \
  -f state=configured -f query_suite=default \
  -f 'languages[]=javascript-typescript' -f 'languages[]=rust'
```

The response stated: `Invalid property /languages/1: rust is not a possible
value. Must be one of the following: actions, c-cpp, csharp, go, java-kotlin,
javascript-typescript, python, ruby, swift.` This was the observed PATCH
validation response, not a claim that GitHub cannot analyze Rust. Retrying
with automatic language detection (omitting `languages`) was accepted and
started separate jobs for Rust, JavaScript/TypeScript, Actions, Python and Swift.
Do not work around the API mismatch by silently omitting Rust.

Initial default-branch run:
[43563413565](https://github.com/moultonlavah/metten/actions/runs/43563413565).
CodeQL version observed in completed analyses: **2.27.4**. Final default setup
was configured at `2026-11-14T05:56:57Z`, weekly schedule, standard runners,
default suite and remote threat model. The GET response lists `actions`,
`javascript`, `javascript-typescript`, `python`, `rust`, `typescript`; GitHub
automatically omitted Swift after its failed setup job. No manual language
removal was performed. The setup run overall concluded success despite its failed Swift job; that
overall status does not clear the individual failure or Rust limitation.

## Per-language evidence

### JavaScript/TypeScript

[Job 132351899537](https://github.com/moultonlavah/metten/actions/runs/43563413565/job/132351899537)
succeeded. Analysis `2166460035`, category `/language:javascript-typescript`,
reports zero results and no error on the source revision above.
The extraction log reports **5/5 JavaScript and 4/4 TypeScript files**, plus the
HTML entry point and GitHub Actions workflow. Its named extraction entries
match all tracked JavaScript/TypeScript sources:

- `src/contract.ts`, `src/main.ts`, `src/view.d.mts`, `src/vite-env.d.ts`;
- `src/view.mjs` and the four `tests/*.test.mjs` files.

No repository-configured exclusions were added. These are static source
coverage counts, not execution coverage or proof that every defect is detected.

### Rust

Expected tracked source inventory is nine files: the engine's `lib.rs`,
`storage.rs`, `strategy.rs`; its three integration-test files; and
`src-tauri/build.rs`, `src-tauri/src/main.rs`, `src-tauri/src/credential.rs`.
The native credential module is gated by `target_os = "macos"`; coverage must
explicitly account for that boundary, not infer it from engine analysis.

[Rust job 132351899545](https://github.com/moultonlavah/metten/actions/runs/43563413565/job/132351899545)
succeeded. Analysis `2166473517` reports zero results, 26 rules, no API-level
error or warning, category `/language:rust`, build mode `none`, and runner
`ubuntu-latest`. The query pack was `codeql/rust-queries` **0.1.46**. Its log
reports **9/9 Rust files extracted without error**, zero with errors.

However, at `05:51:44Z` the extractor explicitly reported:

```text
src-tauri/src/credential.rs:1:1: semantic analyzer unavailable (not included as a module): macro expansion will be skipped.
```

The run used `1.100.0-x86_64-unknown-linux-gnu` with no Cargo target or cfg
overrides. The nine-file count therefore does not establish adequate native
credential analysis. This is the blocking coverage limitation, not a discovered
application vulnerability. The smallest proposed supported setup change is
advanced setup running Rust in build mode `none` on a macOS runner, retaining
standard queries and JavaScript/TypeScript analysis, with #28-reviewed action
pins. A full Rust build is not required merely because Rust is compiled. This
proposal has not been implemented or proven; repeat extraction diagnostics and
default-branch/PR acceptance before promoting any gate.

GitHub documents build mode `none` for Rust: rust-analyzer may run build scripts
and compile macros without building the complete app. Generated code, dependency
resolution and platform configuration therefore need inspection in the actual
run; file enumeration alone does not establish complete semantic coverage.

### Other automatically detected languages

Actions and Python jobs succeeded with zero results (analyses `2166459504` and
`2166459994`). The Swift job
[132351899574](https://github.com/moultonlavah/metten/actions/runs/43563413565/job/132351899574)
failed: autobuild found neither an Xcode project/workspace nor a Swift package.
The repository contains a standalone `scripts/verify-credential.swift` harness,
not a Swift app project. Its uploaded analysis `2166462000` records
`unsuccessful execution`; zero
results on that record are not a passing scan. No successful Swift analysis is
claimed. Omitting this ancillary language is smaller than adding a Swift build,
and GitHub already omitted it automatically. That does not fix the required Rust
coverage gap or make this issue complete. If Swift is
retained in scanning scope, the supported correction is advanced setup with an
explicit compile command for that script; do not execute the credential harness
to obtain extraction. Any new workflow actions must satisfy #28.

## Protections and remaining acceptance

Baseline branch protection on `main`: strict required `mac-foundation` from
GitHub Actions (app ID 15368); enforced for administrators; stale reviews
dismissed; conversation resolution required; force pushes and deletion disabled.
Auto-merge is disabled. Ordinary workflow defaults are read-only and cannot
approve pull requests; `.github/workflows/checks.yml` keeps `contents: read`.
These settings have not been weakened.

After the initial run, the paginated repository alerts API returned `[]`: there
were no initial alerts to triage, fix or dismiss. This does not clear the Rust
coverage limitation. No local alert ledger or unsupported dismissals were added.

Before promotion, inspect any subsequent alerts, fix confirmed in-scope findings
through linked issues, and record any justified dismissals individually in
GitHub. Demonstrate both languages on the default branch and a representative
pull request. Then verify the actual required checks and that missing or failed
analysis blocks merging. A successful JavaScript scan cannot substitute for
missing Rust evidence. No merge gate is claimed until that sequence passes.

The existing Foundation workflow supplies TypeScript checking, Rust formatting
and clippy, interface/native tests, release-mode hour tests and the packaged
Mac build. There is no separate JavaScript lint gate. Final-candidate CI
remains pending; its exact commit and run will be recorded on issue #30. Independent checkpoint reviews: Codex / GPT-6.1 Sol
found no standards violations; OpenCode Free Trial / DeepSeek V4.1 Flash Free
identified missing detail for the PATCH rejection and failed Swift analysis.
Both details are now recorded above. Its remaining observations were correctly
unmet acceptance criteria. The lead checked the complete language/setup flow
and added the final Rust result and concrete native-coverage blocker. No broader
code changes or additional review round were needed for these evidence updates.

## Primary references

- [Default setup](https://docs.github.com/en/code-security/how-tos/find-and-fix-code-vulnerabilities/configure-code-scanning/configure-code-scanning)
- [Rust build mode and requirements](https://docs.github.com/en/code-security/reference/code-scanning/codeql/build-options-for-compiled-languages)
- [Merge-protection behavior and exceptions](https://docs.github.com/en/enterprise-cloud%40latest/code-security/concepts/code-scanning/merge-protection)
