# Contributing to Metten

Metten is in early development. The current foundation is non-trading; paper and live trading are not available. Read the [README](README.md) for setup and the [product specification](METTEN_DESIGN_DRAFT_2026-11-12.md) for intended behavior.

## Find work

Check [open issues](https://github.com/connorreganmoulton-web/metten/issues) and their blockers before starting. Discuss substantial changes in an issue first. The [implementation plan](docs/implementation-plan.md) describes the planned sequence, and [ADR 0001](docs/adr/0001-rust-engine-and-tauri-boundaries.md) explains the engine and desktop boundary.

## Set up and check changes

Use macOS, Node.js 24 or later, Apple's Xcode Command Line Tools, and the Rust toolchain pinned in `rust-toolchain.toml`. Run `npm ci` and `npm run tauri dev` to start the local foundation. The README explains its test session and isolated workspace.

Run checks relevant to your change. The current automated workflow runs these on macOS:

```sh
python3 tests/credential-runner.test.py
npm run check
npm run build
cargo fmt --all --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run tauri build -- --bundles app
```

Describe what you changed, link the issue, and report checks and any limitations in your pull request. Do not present paper or simulated results as proof of live behavior.

## Protect users and their data

Use controlled inputs and dummy credentials for tests. Never commit secrets, private runtime records, or account data. An issue or contribution does not authorize access to real credentials or accounts, paid services, deployment, or real-money validation. Do not operate another trading application or import its code or strategy thresholds. See the [product specification](METTEN_DESIGN_DRAFT_2026-11-12.md) for the intended safety boundaries.

Report vulnerabilities privately using the instructions in [SECURITY.md](SECURITY.md). Do not post secrets or exploitable vulnerability details in a public issue.

## Maintain build-action pins

External actions in `.github/workflows/` use full commit SHAs, with
upstream versions or branch names in comments. `.github/dependabot.yml` requests
monthly update proposals for the repository’s GitHub Actions. This
configuration takes effect on the default branch;
no proposal from it has yet been observed. Proposals require source review and
passing Foundation checks before merge; they are never automatic approval.

Connor Moulton (@moultonlavah) owns a manual review on the first of each month
for `dtolnay/rust-toolchain`'s `master`-derived pin, and checks that the other two
pins have received proposals or manually reviews them if automation is silent.
Dependabot branch-derived SHA updates are not relied on: its current updater
returns without updating when no version tag is available. See the
[issue #28 evidence](docs/verification/issue-28.md) for the source and limitation.

For each update, resolve the version/ref in the upstream repository, review the
old-to-new source diff and action entry points (including nested action refs),
and record the upstream ref, full SHA and review findings in the update issue or
PR. Change the SHA and readable comment together. Preserve Node 24, restricted
`contents: read` permissions, and every existing check, including the release
full-hour tests and packaged Mac build. Rust **1.100.0** stays fixed: a compiler
upgrade needs a separate issue and reassessment of the engine's
compiler-dependent resource proof. Run the complete Foundation checks workflow
on the exact proposed revision before accepting any pin update.

## Maintain security scanning

`.github/workflows/codeql.yml` runs the standard CodeQL queries. Rust runs on
macOS so the Mac-only credential module participates in analysis;
JavaScript/TypeScript, Actions and Python run on Linux. Swift's standalone
verification harness is not scanned. The scanner does not execute that harness
or access credentials. See [issue #30 evidence](docs/verification/issue-30.md)
for coverage limitations and the acceptance still needed before making scanning
a merge requirement.

The CodeQL `init` and `analyze` actions share one reviewed upstream release SHA.
Update both together through the existing monthly Dependabot route, with the
same manual fallback owner above. Review action entry points and release changes,
then inspect each language's extraction diagnostics and run all Foundation checks
on the final revision. A successful job or file count alone does not prove native
coverage. Keep the pinned Rust compiler and ordinary workflow permissions intact.
