# Issue #28: build-action pins

## Scope and upstream review — 13 November 2026

Baseline: `96ad37ab547751f1f3b4b55d392bc42be3563e9c`.
Affected files: `.github/workflows/checks.yml`, `.github/dependabot.yml`,
`CONTRIBUTING.md`, and this record. No application code, dependencies, compiler,
permissions, workflow events or check commands change.

Resolved through GitHub's upstream ref API; each resolves directly to a commit
and GitHub reports a valid commit signature:

| Action | Existing ref / resolved version | Reviewed commit |
| --- | --- | --- |
| actions/checkout | v4 / v4.4.0 | [11d5960a326750d5838078e36cf38b85af677262](https://github.com/actions/checkout/commit/11d5960a326750d5838078e36cf38b85af677262) |
| actions/setup-node | v4 / v4.4.0 | [49933ea5288caeca8642d1e84afbd3f7d6820020](https://github.com/actions/setup-node/commit/49933ea5288caeca8642d1e84afbd3f7d6820020) |
| dtolnay/rust-toolchain | master | [683b588c487a69d01f6c6ddf40f036ff682cc2c8](https://github.com/dtolnay/rust-toolchain/commit/683b588c487a69d01f6c6ddf40f036ff682cc2c8) |

Reviewed each pinned `action.yml`, commit metadata and changed-file inventory;
checkout's main and input handling; setup-node's main and cache restore; and
Rust's complete composite installer. Checkout and setup-node use Node 20 for
the action itself; setup-node still selects Node 24 for our checks. Checkout
retains default event-based self-checkout and post-job credential cleanup.
Setup-node retains npm lockfile-keyed caching and failure reporting. Rust
accepts the explicit 1.100.0 input and installs rustfmt/clippy; its latest commit
only changes the upstream maintenance script's patch-release list. None of the
three manifests references another action. This is a bounded compatibility and
provenance review, not an audit of all upstream bundled JavaScript or downloads.
The chosen SHAs freeze the existing refs as resolved, rather than upgrading
major action versions.

## Update route and automation limit

[GitHub's documented configuration](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/auto-update-actions)
supports monthly GitHub Actions proposals. Configuration is actions-only; no
application dependency updates or auto-merge are added. Actual proposals from
this configuration have not been observed and cannot be verified before it
reaches the default branch.

Inspected Dependabot's
[updater at 13987a86518585c1b2fd84124ab23391dd5a318b](https://github.com/dependabot/dependabot-core/blob/13987a86518585c1b2fd84124ab23391dd5a318b/github_actions/lib/dependabot/github_actions/update_checker.rb#L355-L369):
`latest_commit_sha` returns early without `latest_version_tag`, before its
branch-head fallback. The upstream
[untagged-pin report](https://github.com/dependabot/dependabot-core/issues/15577)
is consistent with that limitation but is for a path-based action, not a
successful or failed Metten update run. Branch-derived updates for this exact
Rust pin remain unproven; they are not treated as reliable automation.

Named maintenance owner: Connor Moulton (@moultonlavah). Cadence: first of every
month, manually review the Rust master pin; also verify tagged-action proposals
or manually check those pins if proposals are absent. The contributor guide
records the review and full-workflow acceptance procedure. Updating the Rust
installer must not change compiler 1.100.0; compiler changes need a separate
issue and renewed resource-proof assessment.

## Acceptance evidence

No new test framework or application test is added for literal SHA substitutions.
The required regression is the existing complete Foundation checks workflow,
including native checks, release-mode full-hour tests and the packaged app build.
TypeScript checking and Rust clippy are wired into that workflow; there is no
separate JavaScript lint gate. Final commit and actual run outcomes will be
recorded on issue #28 after review and execution. Until then, CI acceptance is
pending, not passing.
