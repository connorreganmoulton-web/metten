# Issue #4 — focused independent recheck

Date: 2026-11-13. Scope: review artifacts only; no implementation or automatic remediation.

## Recommendation

**The design still needs targeted revision before implementation.** All 12 accepted repairs are present, but two consequential gaps remain in their interactions: recovery after rejecting a pending calculation batch, and a verification instruction that can erase the contract history the repaired design says to preserve. Two small wording issues also remain. The shared-engine direction does not need to be reopened.

Four fresh-context reviewers completed actual reports. This is a focused recheck, not another full five-stage review or proof that any Rust behavior works. No finding below has been automatically accepted or fixed. Stop here after recording lead clarification; later document changes need their own authorization. Do not execute the future implementation phases.

## Exact target and scope

Reviewed [strategy-rules-design.md](strategy-rules-design.md), **755 lines**, SHA-256 **`741a728267de62ab7bc2856672071f49b3e0169bab00f5012b53e4363aac48d7`**. FeatureRange is **1–324**; DetailedRange is **325–755**. The draft is byte-identical. The preserved original [reviewed snapshot](strategy-rules-design.reviewed-2026-11-13.md) remains **664 lines**, SHA-256 **`3f6f8c23492a41131f56cb81e899860da5565a28519f61ac79ffbe081731c853`**. Fingerprints matched the supplied handoff before review and again after reviewer completion; no target drift required new ranges or invalidated the recorded document checks.

Read the repository and standing AGENTS instructions, installed EGM review skill/Codex integration, target, concepts brief, original issue-4 handoff, product specification, glossary, ADR 0001, implementation plan, annotated resolution and consolidated report. Original finding evidence consulted includes Comprehension, Stage 7 Design/Structure/Style, Stage 9 Readiness/Codebase/Style and the Stage 9 synthesis. Earlier targeted consultations remain historical evidence, not formal re-audits. Existing code was inspected only for affected callers and current boundaries; its unfinished definitions and Revision 3 proposals were not promoted to approved requirements. No new tracker status is asserted.

## Completed independent reviews

The installed bundle at `/Users/connormoulton/.codex/plugins/cache/egm/egm/0.1.1` supplied the actual personas and Implementation Readiness Standard. Native `spawn_agent` was available and every reviewer used **`fork_turns: none`**. Three reviewers ran concurrently; Style followed when a slot became available. All four returned complete outputs and read attestations; none was unavailable, failed or substituted by the lead.

| Native task | Scope and rubric | Result / complete output |
| --- | --- | --- |
| `/root/recheck_readiness` | DetailedRange; Implementation Readiness persona and standard; CR-01/02/04/06 interactions | One Major, RR9-1. [Complete readiness output](reviews/focused-recheck-readiness.md) |
| `/root/recheck_codebase` | DetailedRange; Codebase Details persona and Implementation Readiness Standard; CR-03/04/05 with actual source searches | No findings; assigned design gaps closed. [Complete codebase output](reviews/focused-recheck-codebase.md) |
| `/root/recheck_style` | Changed implementation wording; Style persona, no additional standard | One Major and two Nits. [Complete style output](reviews/focused-recheck-style.md) |
| `/root/recheck_comprehension` | Entire target; Comprehension persona, no standard or supporting answers | No findings; both required paraphrases retained. [Complete comprehension output](reviews/focused-recheck-comprehension.md) |

Comprehension received only the target, persona and ranges. It did not read resolution reports, supporting answers, source or thread history. Other reviewers received relevant baseline/original findings for closure. Lead clarifications were not sent to any independent reviewer. Parent-turn isolation is established by the dispatch setting; host instruction-file exclusion and full filesystem isolation remain unverified. These were not distinct-model council reviews or scored votes.

This consumes the one formal affected re-audit for Stage 6 and Stage 9. Style is part of that same Stage 9 re-audit, not another pass. The lead checked feature-side repair text without relaunching Stage 7. Unaffected initial/intermediate architecture and structural evidence was reused within its original scope. There is no automatic review/remediation loop, and the cap does not turn unresolved findings into accepted trade-offs under this request.

## CR-01–CR-12 closure matrix

“Closed” below means document-level closure only. Original raw identities and severities remain in the original reports.

| Repair / original findings | Recheck disposition | Concrete target evidence and limit |
| --- | --- | --- |
| CR-01 / R9-1 | **Partial — RR9-1 remains Major** | Lines 453–475 define owned continuation, 4096-operation slices across validation/staging/evaluation, Stop, busy/stopped/resource refusal and final-only candidates. Lines 609–615 account for cursors/work/memory; line 714 specifies quiet catch-up, Stop, refusal and slice-equivalence assertions. Pending rejection lifecycle is still unspecified. |
| CR-02 / R9-2 | **Partial — RS9-1 remains Major** | Lines 479–499 identify concrete descriptor/binding tuples, post-Go registration, exact-contract histories, inactive pre-open conditions and bounded reservation/refusal. Lines 489/715 preserve the entering contract's own history; line 713 still says to remove contract state. |
| CR-03 / B9-2 | **Closed** | Lines 524–530 specify one native operation, nonblocking admission before spawning, Busy, release paths and quit handling. Line 725 requires direct IPC Busy and success/error recovery. Actual source confirms the path being repaired; no execution result is claimed. |
| CR-04 / R9-3 | **Closed** | Lines 578–588 prescribe three stable checkpoints and exact files. Phase 1 retains old callers temporarily; Phase 2 changes coupled callers together and removes format-1 compatibility. Source searches found no necessary omitted consumer. Buildability remains a future check. |
| CR-05 / B9-1 | **Closed** | Line 560 adds the existing refresh harness to the allowlist; line 585 includes it in Phase 2; line 725 preserves independent strategy-list startup and stale snapshot assertions. Existing mock defect confirmed by source, not by running a failing test. |
| CR-06 / D7-1 | **Closed** | Lines 395–401 define conservative recursive earliest-completion bounds and exclusive deadlines, including checkpoints and nested holds. Line 716 requires unreachable 900-second/nested cases and feasible shorter holds. Historical lookbacks remain distinct; applicable hour history is permitted. |
| CR-07 / C6-1, T9-4 | **Closed** | Lines 419–425 contain operands, intervals, graph and normalized expected results; lines 697/717 reuse the named fixture. Fresh document-only reviewer reconstructed it. Standalone decimal arithmetic agrees; no engine arithmetic ran. |
| CR-08 / T7-1, S7-1, T9-3 | **Closed** | Opening status and lines 735–755 distinguish completed initial review, accepted text fixes and pending affected evidence. Earlier drafting instructions are historical, not commands to redo sections. This report does not rewrite that frozen provenance. |
| CR-09 / T7-2, T9-1 | **Partial — RS9-2 Nit** | Approval quotations/chronology are collected in the final provenance, but lines 327–329 repeat the authority/handoff instruction twice. Remove the duplicate in later revision. |
| CR-10 / D7-2, T7-3 | **Closed** | Public-evidence passage at lines 316–318 narrows to multiple-series context; dynamic-shell limitations remain explicit. Preserved captures do not establish instrument availability, entitlement or Metten support. |
| CR-11 / T9-2 | **Closed in substance; RS9-3 Nit** | Line 701 calls for an independent documentation-led walkthrough and retains actual preview/version checks and participant identity. Correct “a independent” to “an independent.” The walkthrough remains unperformed. |
| CR-12 / T9-5 | **Closed** | Lines 532–534 and 601 bound both 50 complete summaries and the entire serialized 32-KiB page, define cursor ordering and prevent silent truncation. Line 725 includes multibyte-name/page-boundary assertions. |

## Remaining findings

### RR9-1 — Major — rejected pending work has no defined recovery transition

At line 463, `begin` admits only while idle. Line 465 returns `Rejected(EvidenceError)` and specifically defines `Rejected(no_pending_batch)` as state-preserving; line 467 separately defines terminal Stop. No rule says what happens when an admitted batch fails sliced validation. A developer must decide whether it stays busy, stops forever, or returns idle, and what prior state survives.

Smallest recommended revision: define pending validation rejection as discarding the continuation, preserving previously accepted state, exposing no candidate and returning idle. Explicitly identify whether rejection can occur after accepted-state mutation and define that case without implying an unspecified rollback. Add the focused malformed-admitted-batch → rejection → successful valid batch assertion, checking prior history/checkpoints. Existing dependency-local numeric failure rules should remain. This is a missing design transition, not an observed engine failure.

### RS9-1 — Major — the reset test still gives conflicting history instructions

Line 713: **“Market reset removes checkpoints/contract state but retains compatible underlying history.”** Lines 489 and 715 explicitly preserve the entering contract's own admitted pre-open history. “Contract state” includes more than the market-local conditions meant to reset, leaving conflicting test instructions after CR-02.

Smallest recommended revision: state that reset clears market-local checkpoint/temporal/objective state, preserves the entering contract's own history and compatible underlying history, and never transfers another contract's history. No new history feature or collection policy is needed.

### RS9-2 and RS9-3 — Nits

Remove duplicate authority wording at lines 327–329; correct “a independent” at line 701. The full Style report preserves the original wording, severity and editorial actions. These are two separate Nits, not additional major behavior gaps.

## Lead clarification record

T3 read/send access was verified. The supplied reference resolved to [Draft Issue 4 Implementation Design](t3-thread://v2/646f9e94-f87e-459e-b51a-e29a116396d6/2a8d4f3d-442c-43ca-96bc-4e009ee92bed). Its returned link identifies thread `2a8d4f3d-442c-43ca-96bc-4e009ee92bed` on environment `646f9e94-f87e-459e-b51a-e29a116396d6`; the attachment reference id was not substituted for that thread id. History was read before messaging.

Two authorized `mode:auto` messages supplied finding IDs, exact passages, concrete uncertainty and smallest recommended resolutions. They explicitly requested clarification only, with no design edits or implementation. Independent reviews were complete without those answers. Clarification is recorded below once received; it cannot itself close a gap in the unchanged target.

**Actual answer received:** the originating lead completed run 11 and confirmed both findings are valid and must remain open. The [complete verbatim clarification](reviews/focused-recheck-lead-clarification.md) is preserved separately. Before accepted-state mutation, a rejected batch should release admission, return idle, retain prior accepted frontier/bindings/history/checkpoints/counters, and return no candidate; it must not revive the candidate cleared by `begin`. Busy, no-pending and explicit Stop retain their distinct rules. After mutation, expected gaps/numeric/objective outcomes use their existing local-unavailable/blocking rules; unexpected incoherent internal failure should terminate the evaluator, suppress candidates and retain its last published snapshot. That terminal diagnostic and a separate post-mutation failure assertion still need document definitions. No rollback or automatic restart is implied. Rejected input cannot prove uninterrupted coverage. The lead also confirmed the reset correction must preserve the row's immutable-version assertions. This answer narrows the required future revision; it does not repair the reviewed document or establish a passing re-audit.

## Document/source evidence and limits

Reused the recorded Markdown structure/table/anchor checks for unchanged bytes. Fresh read-only checks confirmed synchronized copies, fingerprints, line counts, paired fences, existing local link paths and two byte-unchanged, parseable JSON specimens. Every one of the 19 phase file paths occurs in the allowlist. An initial checker missed second filenames in combined table cells; corrected extraction passed, so that was a checker error, not a design finding. Standalone high-precision decimal arithmetic reproduced four normalized outputs and their average as **0.001 percent**. Simple duration/allocation arithmetic agrees with the documented exclusive deadline and resource examples. These checks ran no application code and establish no Rust performance or behavior.

Source-grounded caller inspection covers engine import/list/storage, native strategy spawning and registration, frontend startup/import calls, shared example consumers and refresh/credential test harnesses. In particular, native `main.rs:45–62` currently spawns each strategy operation; engine `lib.rs:313–325` serializes storage on the recorder also used at line 176; `tests/refresh.test.mjs:70–77` groups non-control calls while frontend `main.ts:94,121` starts a strategy request. Those observations support the prescribed repairs. They are not executed failures. Existing command permissions are present and need no expansion for the tagged protocol.

The existing public captures were read locally and retained unchanged. No new external research or raw-HTML retry occurred. Their dynamic-shell/429 limitations remain; no inference about account access, listing feasibility, licensing or adapter capability follows.

Still required after separate implementation authority: public-engine behavior for the twelve adopted timelines, exact arithmetic/coverage/identity/continuation/refusal/rejection cases, resource planner versus actual allocations, storage migration and immutable mixed-version records, direct-IPC admission/release and stale-preview/save handling, coordinated caller builds, an independent documentation-led authoring exercise, focused disposable-workspace packaged import/reopen/edit-copy checks and measured responsiveness. Later integrations must separately prove source availability/continuity/age and order/recovery/accounting behavior. A paper result cannot establish live acceptance or fills.

No source/dependency changes, application tests/builds, browser/runtime operations, database opening, source collection, provider adapters, scheduler, real credentials, account/paid-service access, trades, GitHub mutations, commits or publication were performed. Only new review artifacts were written. The requested stopping point is this report plus necessary lead clarification, with the two Majors and two Nits left visible and the design unchanged.

Final preservation check compared 72 previously fingerprinted source/test/example/review files: none changed. Only the new focused report and clearly named focused review/clarification artifacts were added. Workspace status retains the pre-existing dirty implementation; no cleanup was attempted. Target/draft/archive fingerprints remain as stated above. All four assigned reviews and necessary lead clarification are complete. **Stop: targeted design revision is recommended; implementation readiness is not established.**
