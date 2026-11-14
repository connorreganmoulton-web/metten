COMPLETE — focused Stage 9 readiness re-audit. One remaining Major clarification prevents a zero-decision readiness pass. No Blockers or additional Nits.

**RR9-1 — Major — specify the pending evaluator’s state after rejection**

**Affected scope:** `strategy-rules-design.md:463–475`, particularly line 465; related verification at line 714.

**Exact passages:**

- Line 463: “admits one owned bounded batch only while idle.”
- Line 465: “SliceResult is Pending … Complete(Decision), Stopped, or Rejected(EvidenceError).”
- Line 465: “Continue with no accepted pending batch returns Rejected(no_pending_batch) without changing state.”
- Line 465: “Complete batch validation precedes mutation of accepted evaluation state.”
- Line 467 explicitly defines Stop’s terminal state; no corresponding transition is specified for rejection of an owned pending batch.

**Remaining choice:** A bounded batch can pass `begin`’s admission checks, then fail the sliced semantic validation. The plan does not say whether `Rejected` clears that continuation and restores idle admission, leaves the evaluator busy, or terminates it. These produce materially different recovery behavior. The no-pending special case defines only rejection when there was no owned batch.

**Smallest resolution:** State that a pending validation rejection discards the entire rejected continuation, preserves previously accepted evaluation state, exposes no candidate and returns the evaluator to idle. State whether any later rejection can occur after accepted-state mutation; if not, make that explicit. Keep numeric failures dependency-local as already specified.

**Required future assertion:** Admit a bounded malformed batch, process it to `Rejected`, then successfully admit and complete a valid batch on the same evaluator. Check that accepted history/checkpoints remain unchanged by the rejected batch. This is a proposed check, not executed evidence.

The principal accepted repairs are otherwise concrete:

- **CR-01:** evaluator-owned continuation, bounded sliced work, control inspection, terminal cancellation and suppression of intermediate candidates are specified.
- **CR-02:** concrete binding identity, post-Go collection, same-contract history preservation and bounded registration refusal are specified.
- **CR-04:** three ordered checkpoints identify exact files, compatibility retention/removal and applicable checks.
- **CR-06:** exclusive deadlines, recursive earliest-completion bounds, checkpoint dependencies and long-lookback preservation are specified.

I did not treat finite registration capacity as a missing unlimited-collection feature. I also did not raise a separate current-admission finding: lines 471/509 require current checks before candidate use, while production age integration remains explicitly later work.

**Read attestation:** Read the installed implementation-readiness persona and standard, plugin `docs/codex.md`, the complete 755-line target, annotated and consolidated review reports, prior Stage 9 readiness report, concepts brief, issue-4 handoff, product specification, glossary, ADR 0001 and implementation plan. Formal audited scope was DetailedRange **325–755**, focused on CR-01/02/04/06 and their interactions; earlier sections supplied definitions. Verified before/after target SHA-256:

`741a728267de62ab7bc2856672071f49b3e0169bab00f5012b53e4363aac48d7`

No files changed; no builds, tests, external research, private records, provider/account operations or subagents. Implementation correctness, usability, actual capacity and packaged behavior remain unverified.
