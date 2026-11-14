# Raw Goldfish report — comprehension

Reviewer id: `/root/egm_synthesis/comprehension`  
Role: UX and Ergonomics-Brainstormer (`egm-comprehension`)  
Status: completed; actual native child final received.  
Read attestation: included verbatim in the raw report below.  
Mode: BRAINSTORM. No rewriting of the child's report follows.

---

Status: completed. BRAINSTORM review using the UX, ergonomics and cognitive-overhead rubric. No blocker found in the anchored product direction; three requirements need clearer treatment as the concept becomes a design.

1. **Major — UX & Developer Ergonomics: some contract-history requests may be impossible to satisfy.**

Evidence: the working hypothesis’s “Preserved boundaries” retains rolling measurements on contract prices and author-selected durations. Handoff decisions A and D preserve long underlying lookbacks while keeping contract data tied to its contract; controlled example 11 forbids using the previous contract’s book. The baseline limits Gen 1 to 15-minute markets (§2).

Consequence: an author could reasonably assume that an hour-long rolling measurement is supported on either Bitcoin or contract prices. Initial prefill cannot make an hour of history available for a newly opened 15-minute contract. A generic “collecting data” screen could imply eventual readiness when readiness is impossible.

**Ergonomic requirement:** document which source/instrument and history combinations can actually become eligible, and reject demonstrably impossible combinations with a precise explanation before saving or starting. Preserve the agreed long underlying-history capability and all feasible contract-price measurements. Engineering should establish these compatibility rules; Connor should not have to design them.

Opposing alternative: accept every well-formed duration and leave the condition permanently unavailable. This preserves syntactic flexibility but creates a misleading authoring experience. Combining books from successive contracts would instead introduce an unapproved capability.

Missing proof: concrete history identity, lookback eligibility and boundary examples. No provider capability is established by this review.

2. **Major — UX & Developer Ergonomics: generated explanations must preserve purchase priority, evidence requirements and quantity limits together.**

Evidence: the working hypothesis promises explanations from validated rules. Handoff B–E and controlled examples 1, 3, 8–10 establish checkpoint memory, local dependency blocking, priority reconsideration, same-update eligibility and partial-fill side locking. The baseline requires concise dashboard reasons (§11) and matching reference/example explanations (§12).

Consequence: a flat list of individually correct conditions could still give the wrong overall impression—for example, that the alternative can create another purchase after four YES contracts fill, or that a lost checkpoint source invalidates the remembered checkpoint. Nested options increase that risk.

**Ergonomic requirement:** use generated structure that communicates “try this first, otherwise…” and separately states the shared one-objective quantity ceiling and side lock. Explanations should distinguish remembered facts from evidence that must be current. Carry this through the worked example and controlled explanation checks, without implementing the later trading dashboard in #4.

Opposing alternative: render the rule tree literally. That is mechanically traceable but may force ordinary users to interpret authoring syntax. Author-written prose alone is already excluded because it can disagree with evaluation.

Missing proof: a representative explanation containing nested options, a remembered side and a partial-fill continuation. This is an engineering presentation requirement, not a request to revise the adopted purchase behavior.

3. **Major — UX & Developer Ergonomics: validation needs a repair path, and imported validity must remain distinct from connected-source readiness.**

Evidence: the hypothesis promises direct JSON upload and strict supported capabilities. The handoff’s “Engineering definition and future evidence still needed” explicitly leaves error messages, exact fields and resource bounds unresolved. The baseline §§3, 9 and 12 separates valid strategy import from actual source access and requires actionable troubleshooting.

Consequence: “unsupported strategy” gives authors no practical way to repair a file. Conversely, “valid strategy” can sound like confirmation that BRTI or another named source will work, despite source access and adapters remaining unverified later work.

**Ergonomic requirement:** make rejection identify the offending field or rule, explain the accepted form or supported capability, and preserve the imported text for correction. Preview/save should clearly mean that the rules are supported by the contract; later source readiness must be reported separately. Document justified resource limits and explain a rejected limit in author terms. No new editor, CLI or schema framework is necessary to satisfy this.

Opposing alternative: a short generic rejection minimizes UI work but transfers diagnosis to the user. Requiring real source connections during import would blur #4 with later integration work and make authoring unnecessarily dependent on account access.

Missing proof: example diagnostics for an unknown operator, incompatible units/source pairing, malformed input and a justified size/resource limit. Actual entitlement, licensing and usable source data remain unverified outside #4.

The adopted initial prefill rule is understandable: collect all required startup history from Go, then wait for the next full market. It must remain visibly different from recovery during an active market; the hypothesis and handoff already preserve that distinction. I found no reason to reopen that owner decision or reduce the supported capabilities.

Read attestation: read fully the installed `agents/comprehension.md` and `docs/codex.md`, plus all six assigned sources: `docs/proposals/issue-4-egm/working_hypothesis.md`; `docs/proposals/issue-4-egm-handoff.md`, including A–E and all twelve examples; `METTEN_DESIGN_DRAFT_2026-11-12.md`; `GLOSSARY.md`; `docs/adr/0001-rust-engine-and-tauri-boundaries.md`; and `docs/implementation-plan.md`. No additional standard was selected.

Isolation observation: no parent conversation transcript or identifiable parent-only test marker was visible. The visible user context did include the supplied standing and project `AGENTS.md` instructions for `/Users/connormoulton/Desktop/metten`, including communication, scope, browser and repository rules. This observation does not prove host instruction-file exclusion, which remains unverified.

Skipped source-code inspection, execution, external access and writes. Findings identify missing design proof, not established implementation defects or trading safety results.
