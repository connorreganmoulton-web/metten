# Issue #30: CodeQL coverage and merge checks

## Current evidence index

Issue #30 remains open. Default setup has been started, but Rust coverage and
representative pull-request acceptance are not yet established. Analysis is
nonblocking; the existing `mac-foundation` merge requirement remains in force.
This record must not be read as security or release acceptance.

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
with HTTP 422: the endpoint's accepted language enum omitted Rust. Retrying
with automatic language detection (omitting `languages`) was accepted and
started separate jobs for Rust, JavaScript/TypeScript, Actions, Python and Swift.
Do not work around the API mismatch by silently omitting Rust.

Initial default-branch run:
[43563413565](https://github.com/moultonlavah/metten/actions/runs/43563413565).
CodeQL version observed in completed analyses: **2.27.4**.

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

Rust run outcome, extraction diagnostics and file coverage remain pending.
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
not a Swift app project. No successful Swift analysis is claimed. If Swift is
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

Before promotion, inspect all initial alerts, fix confirmed in-scope findings
through linked issues, and record any justified dismissals individually in
GitHub. Demonstrate both languages on the default branch and a representative
pull request. Then verify the actual required checks and that missing or failed
analysis blocks merging. A successful JavaScript scan cannot substitute for
missing Rust evidence. No merge gate is claimed until that sequence passes.

The existing Foundation workflow supplies TypeScript checking, Rust formatting
and clippy, interface/native tests, release-mode hour tests and the packaged
Mac build. There is no separate JavaScript lint gate. Final candidate CI and
independent review outcomes remain pending.

## Primary references

- [Default setup](https://docs.github.com/en/code-security/how-tos/find-and-fix-code-vulnerabilities/configure-code-scanning/configure-code-scanning)
- [Rust build mode and requirements](https://docs.github.com/en/code-security/reference/code-scanning/codeql/build-options-for-compiled-languages)
- [Merge-protection behavior and exceptions](https://docs.github.com/en/enterprise-cloud%40latest/code-security/concepts/code-scanning/merge-protection)
