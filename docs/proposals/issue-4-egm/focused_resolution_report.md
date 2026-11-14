# Issue #4 — focused document resolution

Status: resolved

Date: 2026-11-13. Owner authorized the bounded final document corrections after the focused recheck. Used the installed Google EGM Resolve skill and Codex integration; the existing approval covers these corrections, so no repeat approval gate or reviewer cycle was started.

## Result and boundary

All four focused findings are addressed in text. This closes the requested document-correction task; it does not establish implemented or trading readiness. Implementation and publication remain unauthorized. The original review, recheck and verbatim reviewer/lead outputs remain unchanged historical evidence for their recorded hashes.

Target: [strategy-rules-design.md](strategy-rules-design.md), synchronized byte-for-byte with [design_draft.md](design_draft.md).

- Before: SHA-256 `741a728267de62ab7bc2856672071f49b3e0169bab00f5012b53e4363aac48d7`, 755 lines; FeatureRange 1–324, DetailedRange 325–755.
- After: SHA-256 `4f39df3e732b27d79a43bc3dc95048cf5368337634c15bcd71e454bb149dd8e5`, 759 lines; FeatureRange 1–324, DetailedRange 325–759.
- Original archive remains SHA-256 `3f6f8c23492a41131f56cb81e899860da5565a28519f61ac79ffbe081731c853`.

## Findings and applied edits

| Finding / severity | EGM category | Applied resolution |
| --- | --- | --- |
| RR9-1 / Major / CR-01 | PROPOSE FIX, owner-authorized | Defined pre-mutation rejection: discard continuation, release admission, return idle, preserve prior accepted frontier/bindings/history/checkpoints/counters, expose no candidate and never revive the cleared one. Kept Busy/no-pending/Stop distinct. Defined post-mutation internal failure as a finite terminal Failed outcome rather than implied rollback; subsequent calls have explicit outcomes, the last published snapshot/history survive, and no automatic restart occurs. Expected numeric/gap/objective outcomes retain existing local-unavailable/blocking rules. Added required future rejection→valid-batch and separate post-mutation failure assertions. |
| RS9-1 / Major / CR-02 | PROPOSE FIX, owner-authorized | Replaced the conflicting reset verification row: clear market-local checkpoints/temporal/objective state, preserve entering contract's own history and compatible underlying history, never transfer another contract's history. Preserved immutable-version assertions. |
| RS9-2 / Nit / CR-09 | PROPOSE FIX, owner-authorized | Removed duplicated authority wording from the Detailed Implementation banner; kept the single concise authority paragraph and resolution link. |
| RS9-3 / Nit / CR-11 | PROPOSE FIX, owner-authorized | Corrected “a independent” to “an independent”; retained the documentation-led walkthrough and actual preview/version checks. |

The failure rules use the [originating lead's recorded clarification](reviews/focused-recheck-lead-clarification.md); they add no source adapter, scheduler, trading control or author JSON capability. Opening status/final handoff now record this completed correction rather than claiming the earlier version's open findings are still current. No independent pass of the final changed text is claimed.

Counts: 4 accepted/applied; 0 contested, escalated, declined or unapplied. No known unaddressed focused finding remains in text. The [focused recheck](focused_recheck_report.md) remains the unchanged pre-correction report.

## Targeted checks actually completed

- Confirmed the exact pre-edit target hash and identical draft; applied replacements only when each original passage matched once.
- Target/draft are synchronized; headings, paired fences and local links remain coherent. Both public JSON specimens are unchanged and parse as JSON.
- Checked the explicit rejection/idle, Busy/no-pending/Stop, terminal failure, candidate suppression and next-call rules against the recorded clarification; checked that required future recovery/failure assertions are present.
- Confirmed the reset assertion agrees with exact-contract history preservation and retains immutable-version behavior; corrected both wording findings.
- Reused unchanged arithmetic, phase/caller, public-capture and original review evidence within its recorded limits. No extra reviewers, broad audit or repeated arithmetic exercise was needed for these edits.

No builds, application tests, database/runtime operations, source collection, accounts, credentials, trades, external research, GitHub mutations, commits or publication. Only the target, synchronized draft, this closure report and an appended pointer in the annotated resolution report were written. Existing source changes and original reports are preserved.

## Stopping point

The bounded document correction is complete. Next is separately authorized issue-4 implementation using the existing phased plan, followed by its required behavior/integration checks. Those checks, packaged operation, authoring usability, allocation/performance and later source/trading integration remain unproven. Do not start implementation or another review loop from this closure record.
