# Issue #29: reviewed dependency maintenance

## Current evidence — 14 November 2026 UTC

Baseline: `6861d8e264b4a78cacbf91e89bb1145a8775b27d` (issue #28 merged).
Changes are limited to `.github/dependabot.yml`, the dependency-review guidance
in `CONTRIBUTING.md`, and this record. No application dependency, action SHA,
compiler, workflow, or trading setting changes.

The configuration requests weekly npm, Cargo and GitHub Actions version checks
at `/`. Each ecosystem groups minor/patch updates and leaves major upgrades
separate. The Cargo entry covers the root workspace's `crates/engine` and
`src-tauri` manifests and shared `Cargo.lock`; Dependabot's
[Cargo fetcher](https://github.com/dependabot/dependabot-core/blob/main/cargo/lib/dependabot/cargo/file_fetcher.rb)
loads workspace members and the root lockfile. No duplicate member jobs are
needed. GitHub's [options reference](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference)
defines the version-update grouping. No ignore rule suppresses security fixes.

## Actual repository settings

Checked and changed only the requested settings through the GitHub API for
`moultonlavah/metten`:

- Dependabot alerts: initially disabled (GET `vulnerability-alerts` returned
  404 with that explicit reason); enabled with PUT and verified by successful
  GET (204).
- Security-update PRs: initially disabled; enabled with PUT
  `automated-security-fixes`, then GET returned `enabled: true, paused: false`.
  Repository metadata also reports `dependabot_security_updates: enabled`.
- Dependency graph: SBOM export succeeded with 528 packages and npm, Cargo and
  GitHub Actions package identifiers. The graph lists seven manifests/lockfiles,
  including both Cargo members. GraphQL dependency counts returned zero, so the
  populated SBOM, not those counts, is the evidence of dependency detection.
- Secret scanning and push protection: enabled before and after, unchanged.
- Automatic merge: disabled, unchanged. Main requires `mac-foundation` with
  strict up-to-date checks and administrator enforcement; protection unchanged.

Security fixes use [GitHub's security-update processing](https://docs.github.com/en/code-security/concepts/supply-chain-security/dependabot-security-updates),
not the weekly version schedule. The configured groups apply only to version
updates. The existing first-of-month manual action-pin fallback from #28 remains
in force, particularly for the branch-derived Rust installer pin. Rust 1.100.0
is fixed; any newer-compiler requirement remains unmerged pending its own issue.

## Observed jobs and remaining acceptance

The previous actions-only configuration has produced real proposals:
[initial Actions job](https://github.com/moultonlavah/metten/actions/runs/43562939183)
completed successfully, producing
[setup-node #33](https://github.com/moultonlavah/metten/pull/33) and
[checkout #34](https://github.com/moultonlavah/metten/pull/34). No downgrade was
manufactured. On #33 head `48973897e0a8d77ec4c3d200f2d5a0767fee6f76`,
[Foundation checks](https://github.com/moultonlavah/metten/actions/runs/43562983066)
completed successfully. On #34 head
`095e54dddf9645c2d2e3420505dffaf2e8e312fc`,
[Foundation checks](https://github.com/moultonlavah/metten/actions/runs/43562988812)
also passed. Both include native tests, release full-hour acceptance and the
packaged Mac build. This confirms proposal check triggering and successful
checks, not upstream source-review clearance or approval of either upgrade.
Neither proposal is merged by this task. Their affected-evidence notes
([#33](https://github.com/moultonlavah/metten/pull/33#issuecomment-6872746824),
[#34](https://github.com/moultonlavah/metten/pull/34#issuecomment-6872746985)) identify
checkout/credential-cleanup or Node/cache behavior respectively and preserve
the #28 upstream source-review requirement before merge.

The new three-ecosystem weekly configuration is not on the default branch yet.
Initial successful npm, Cargo and Actions version jobs under this configuration
remain **pending after merge**. A Cargo security-fix job started after security
updates were enabled; it is not evidence of the weekly Cargo version job.
That [security job](https://github.com/moultonlavah/metten/actions/runs/43563396998)
failed because the existing dependency constraints allow glib only through
0.18.5 while the earliest fixed version is 0.20.0. Security automation is enabled
but cannot resolve this update automatically. Leave the alert open for a
separate dependency compatibility fix; do not broaden this configuration task
into application upgrades or suppress the alert.
Do not close #29 until the three initial jobs have been inspected and their
actual success, no-update result, or failure recorded. No available update and
an unobserved check must be reported separately.

Independent review of `113dae0`: Codex / GPT-6.1 Sol Standards review found no
findings. OpenCode Free Trial / DeepSeek V4.1 Flash Free Spec review found no
implementation defects or material scope creep; it identified pending activation
acceptance, missing affected-evidence notes on the existing proposals and a
missing YAML check result. The notes and result are recorded in this follow-up;
configuration is unchanged. Post-merge acceptance remains pending.

The focused Ruby YAML/assertion check passed: version 2, exactly three root
ecosystems, weekly scheduling, minor/patch version-only groups, no ignore rules
or alternate target branch. Whitespace validation passed. No new application
logic or test framework was introduced.

Exact-final-revision Foundation checks are pending. TypeScript
checking and Rust clippy are wired; no separate JavaScript lint or Dependabot
configuration lint gate is wired. Use the existing Ruby YAML parser for a
focused configuration check without adding a dependency or test framework.
The existing complete Foundation workflow supplies final regression evidence;
no claim is made that prior packaged-app evidence belongs to a new artifact.
