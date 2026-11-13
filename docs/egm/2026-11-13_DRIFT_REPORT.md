# Initial EGM drift report — 13 November 2026

Scope: [the non-issue-#4 coverage map](README.md), foundation commit `b1eb58c25fed5a2970bd5c40db9c05f673ff8d71` and unchanged in-scope working-copy behavior. This is a documentation/source comparison, not executed acceptance testing. Issue #4 source, tests, examples and proposals are excluded, including its portions of mixed files. Planned later features are not classified as regressions in completed issues #1–#3.

Component source and tests were inspected in three bounded bootstrap workers; the lead inspected supporting configuration, tooling and documentation and read back every generated contract. Coverage includes maintained tests, permissions, build/CI, configuration and static asset ownership. Static exports were inventoried without rerendering. No source/runtime changes, application launches, credentials, tests/builds, remote CI checks or independent adversarial review were performed. Prior evidence remains historical and version-bounded. The fingerprint check detected a concurrent change to the excluded issue #4 design draft; this task did not edit, inspect or restore it. All pre-existing files except the root README and that externally changed draft retained their starting fingerprints. See [issue #26](https://github.com/moultonlavah/metten/issues/26) for task scope.

## METTEN_DESIGN_DRAFT_2026-11-12.md

- **Stale implementation status:** the header says “implementation not started”; section 2 says packaged integration is not yet proven. The existing `Engine::{open, request, snapshot, stop_and_save}`, `Recorder` and native credential adapter implement the controlled foundation, and the [issue #2](../verification/issue-2.md) and [finished issue #3](../verification/issue-3.md) records contain bounded local packaged results. These statements no longer describe the completed foundation. Recommend a dated status/evidence pointer distinguishing that proof from still-unimplemented provider/trading/release work. Do not change the product requirements or claim full Gen 1 acceptance.

## docs/adr/0001-rust-engine-and-tauri-boundaries.md

- **Stale evidence pointer:** the consequences still describe concrete SQLite/Keychain packaged integration as needing proof, and the closing text describes the next packaged proof as future work. The issue #2/#3 records now supply limited local foundation/dummy-credential evidence. Recommend an evidence addendum linking those records and retaining their signing, hardware, power-loss and non-trading limits. The accepted engine/desktop decision does not need to change.

## README.md

No drift detected.

## crates/README.md

No drift detected.

## crates/engine/README.md

No drift detected.

## crates/engine/src/README.md

No drift detected.

## src-tauri/README.md

No drift detected.

## src-tauri/src/README.md

No drift detected.

## src/README.md

No drift detected.

## tests/README.md

No drift detected.

## scripts/README.md

No drift detected.

## .github/README.md

No drift detected.

## assets/README.md

No drift detected.

## assets/brand/README.md

No drift detected.

## docs/README.md

No drift detected.

## docs/egm/README.md

No drift detected.

## CONTRIBUTING.md

No drift detected.

## SECURITY.md

No drift detected.

## GLOSSARY.md

No drift detected.

## LICENSE

No drift detected.

## docs/implementation-plan.md

No drift detected.

## docs/agents/issue-tracker.md

No drift detected.

## docs/agents/triage-labels.md

No drift detected.

## docs/agents/domain.md

No drift detected.

## docs/adr/0002-mit-license.md

No drift detected.

## docs/verification/issue-2.md

No drift detected. Historical incomplete/CI dispositions are checkpoint-specific; they are not interpreted as current issue or PR state.

## docs/verification/issue-3.md

No drift detected. Its finished disposition supersedes earlier incomplete checkpoints; recorded results are not fresh verification of the unfinished working tree.

## Follow-up resolution — 13 November 2026

- **METTEN_DESIGN_DRAFT_2026-11-12.md — resolved:** updated the header, introductory status and section 2 packaging summary to link the completed local foundation/dummy-credential evidence and retain outstanding full-product, distribution, hardware and durability limits. No product behavior or issue #4 requirement changed.
- **docs/adr/0001-rust-engine-and-tauri-boundaries.md — resolved:** replaced the outdated future-proof wording and added a dated evidence note linking issues #2/#3. The accepted architecture decision and its requirements remain unchanged.

This follow-up checks only those status/evidence corrections and documentation links; the prior component coverage remains applicable. The original findings above are retained as history. No tests, builds, app/credential operations or issue #4 work were performed. Connor authorized committing the documentation baseline and these corrections locally; no push or PR is included.

## Follow-up drift check — 13 November 2026

Scope: the existing coverage map outside issue #4. The committed non-documentation files remain unchanged from foundation commit `b1eb58c`; the earlier component review is reused for those files. The root README, coverage map, project instructions and the two previously corrected status summaries were checked for changes since that baseline. The removed `.github/README.md` was replaced in current links by the workflow file; its entry above remains a historical observation.

Coverage limit: ten shared files have uncommitted changes associated with ongoing issue #4 work. This pass did not inspect those changes or independently establish that every in-scope behavior in those files is unchanged. The result is therefore partial, not a clean assessment of the entire current working copy. Issue #4 artifacts and their changes remain excluded. No tests, builds, app launches, credentials or trading operations were performed.

## README.md

No drift detected in the changed documentation links reviewed in this pass.

## AGENTS.md

No drift detected in the project-page safeguard reviewed in this pass.

## docs/egm/README.md

No drift detected in the changed CI reference reviewed in this pass.

## METTEN_DESIGN_DRAFT_2026-11-12.md

No drift detected in the previously corrected status summaries reviewed in this pass.

## docs/adr/0001-rust-engine-and-tauri-boundaries.md

No drift detected in the previously corrected evidence update reviewed in this pass.
