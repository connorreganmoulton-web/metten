# Raw Goldfish report — migration and compatibility

Reviewer id: `/root/egm_synthesis/implementation_readiness`  
Role: Migration and Compatibility-Brainstormer (`egm-implementation-readiness`)  
Status: completed; actual native child final received.  
Read attestation: included verbatim in the raw report below.  
Mode: BRAINSTORM. No rewriting of the child's report follows.

---

Status: completed. BRAINSTORM review only; this is not an implementation-readiness audit.

Two bounded compatibility risks should carry into the concept brief.

1. **Major — Migration & Backward Compatibility: saved rules must keep their original meaning.**

The working hypothesis requires immutable versions and preserved earlier records. Handoff decision E and controlled example 12 reinforce this; product specification sections 7 and 10 attach qualification and history to exact strategy versions.

The unfinished storage code illustrates a concrete hazard: `crates/engine/src/storage.rs`, `strategy_versions()`, reparses every saved JSON document using the current parser and regenerates its explanation. `crates/engine/src/strategy.rs` currently accepts only `format: 1`. Changing that format's accepted fields, arithmetic or temporal meanings could change an old version's explanation or make one saved row prevent the entire version list from loading. Whether any such rows actually exist is unverified.

The concept brief should require an identifiable rule-contract revision for each saved version, retained original JSON and stable interpretation of earlier records. Unknown or unsupported revisions should remain inspectable with a clear explanation of why they cannot run. Converting an old strategy must create another immutable version; it must not rewrite the original.

Two reasonable alternatives exist: retain the old evaluator where execution compatibility is genuinely needed, or retain read-only access to unsupported historical versions and require an explicit new-version import before execution. The latter is smaller for an unreleased application. Choosing between them is an engineering compatibility decision after inspecting actual persisted formats, rather than a reason to make Connor design a parser.

Missing proof: which earlier formats were saved, which meanings need preserving, and whether existing records can still be listed when another record is unsupported. Draft code is evidence of the hazard, not an approved format that must remain executable.

2. **Major — Migration & Backward Compatibility: a failed upgrade or return to an older application must preserve the workspace.**

The product specification, sections 6 and 10, requires history preservation through restart and storage changes. The handoff's future implementation coverage also explicitly includes append-only versions and preservation of existing records. Neither the hypothesis nor handoff currently states the bounded upgrade/rollback promise.

The unfinished `Recorder::open()` in `crates/engine/src/storage.rs` already uses a database revision and rejects revisions newer than it supports. That is a useful existing pattern. However, a richer strategy contract could alter stored data without a corresponding database revision, leaving an older application able to open a workspace whose strategy documents it cannot understand.

The concept brief should require atomic storage changes, preservation of existing session/history records, and a deliberate compatibility decision whenever a change makes an older application unable to understand newly written data. Returning to an older application may safely refuse the newer workspace; it must not silently reinterpret, downgrade or erase it. The required newer application or supported recovery route should be explained plainly.

The smallest alternative is to keep storage additions compatible where practical and document that downgrading a changed workspace is unsupported. Building a general downgrade mechanism is unnecessary for this issue. A migration/rollback check with disposable existing-workspace fixtures belongs in the later design and implementation evidence; operating a trading run is unnecessary.

Missing proof: the final stored representation, affected prior database revisions, interruption behavior during migration, and the actual older-application refusal behavior.

The concept already handles several rollout boundaries correctly. Initial prefill and waiting for the next full market are distinct from recovery during an active market. Actual source capability and numerical production validity limits remain later integration gates. The implementation plan keeps connected feeds, purchasing, uncertain outcomes and interruption recovery in later issues. Those boundaries should remain explicit; I found no basis to require provider access, live rollout, or a production migration system in issue #4.

Owner decisions establish preserved versions, fixed active-run settings, full supported rule capabilities and recovery behavior. Contract revision identity, migration transactions and unsupported-version presentation are engineering definitions. Provider compatibility, packaged migration behavior and execution integration require later evidence.

Read attestation:

- Read fully: `/Users/connormoulton/.codex/plugins/cache/egm/egm/0.1.1/agents/implementation_readiness.md`.
- Read fully: `/Users/connormoulton/.codex/plugins/cache/egm/egm/0.1.1/docs/codex.md`.
- Read fully: `/Users/connormoulton/Desktop/metten/docs/proposals/issue-4-egm/working_hypothesis.md`.
- Read fully: `/Users/connormoulton/Desktop/metten/docs/proposals/issue-4-egm-handoff.md`, including A–E and all twelve controlled examples.
- Read fully: `/Users/connormoulton/Desktop/metten/METTEN_DESIGN_DRAFT_2026-11-12.md`.
- Read fully: `/Users/connormoulton/Desktop/metten/GLOSSARY.md`.
- Read fully: `/Users/connormoulton/Desktop/metten/docs/adr/0001-rust-engine-and-tauri-boundaries.md`.
- Read fully: `/Users/connormoulton/Desktop/metten/docs/implementation-plan.md`.
- Additionally inspected narrow storage/parser/version interfaces in the current unfinished source. No additional standard was selected.

Isolation observation: no parent conversation transcript or unique parent-only marker was visible. The delegation task was visible. Standing instructions and the Metten `AGENTS.md` instructions were also visible in the supplied context. Parent-turn isolation therefore must not be described as proof of host instruction-file exclusion; that exclusion remains unverified. I retrieved no memory or conversation history.

No files changed, checks executed, accounts accessed or external sources browsed. Actual persisted workspace contents and migration behavior remain unverified.
