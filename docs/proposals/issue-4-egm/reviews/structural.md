# Raw Goldfish report — maintainability

Reviewer id: `/root/egm_synthesis/structural`  
Role: Maintainability-Brainstormer (`egm-structural`)  
Status: completed; actual native child final received.  
Read attestation: included verbatim in the raw report below.  
Mode: BRAINSTORM. No rewriting of the child's report follows.

---

Completed the read-only BRAINSTORM review using the Structural Goldfish maintainability rubric. No additional standard was selected. The concept supports the intended separation between uploaded rules and the shared runtime; I found no maintainability blocker requiring Connor to change the adopted product decisions.

1. **Major — Long-Term Tech Debt & Maintainability: immutable files also need stable meanings.**

   Evidence: `working_hypothesis.md`, “Preserved boundaries”; handoff, controlled example 12; specification §§10 and 12; glossary, “Strategy version.”

   Keeping an old JSON file unchanged will not preserve its original meaning if later engine changes alter rounding, crossing, rolling-window or observation rules. Generated explanations could also describe a different meaning after an update.

   The design should identify the semantics under which a saved strategy was validated and define how future changes preserve or explicitly migrate that meaning. Evaluation, explanation and the matching reference should use that same contract. This is an engineering definition, not a request for Connor to design version machinery.

   The opposing alternative is to apply the newest interpretation to all saved files. That is simpler initially, but conflicts with preserving original meanings unless compatibility is demonstrated. Missing proof: a controlled compatibility example showing an earlier version, its explanation and its recorded meaning remain stable after a contract change.

2. **Major — Long-Term Tech Debt & Maintainability: validity must follow the condition that needs it.**

   Evidence: handoff decisions B and E, controlled examples 1, 3 and 7; specification §§4, 6 and 16; ADR 0001.

   Nested alternatives, completed checkpoints and rolling calculations have different current dependencies. A single “all sources healthy” switch would accumulate exceptions and incorrectly block independent options. Putting those exceptions in provider adapters would couple connections to strategies.

   The design should keep named-source evidence and coverage separate from rule evaluation, and derive a condition’s current requirements from the validated rules. A completed checkpoint retains its recorded evidence; an unfinished duration or historical calculation retains its own outstanding requirements. Common purchase safeguards still apply.

   The opposing alternative is global blocking whenever any named source is unavailable. It is easier to implement but contradicts the adopted independent-alternative behavior. Missing proof: controlled timelines demonstrating that a failed dependency blocks exactly the affected conditions without treating a present snapshot as repaired history. Actual source capabilities and production validity limits remain later integration evidence.

3. **Major — Long-Term Tech Debt & Maintainability: state needs explicit lifetimes and one evaluation order.**

   Evidence: handoff decisions B–D and controlled examples 4–6 and 8–11; specification §§5–6; glossary definitions for market window, checkpoint and purchase objective.

   The concept combines run-wide history, market-local checkpoints, unfinished durations, ordered alternatives and purchase outcomes. If each feature resets or advances its own state independently, combinations will become brittle: a market change could erase useful underlying history, an option could accidentally retain a spent opportunity, or a timer could overlook an intervening failure.

   The design should name each state lifetime and define one authoritative sequence for accepting evidence, updating derived measurements and checkpoints, evaluating options, and applying objective restrictions. Initial collection followed by the next full market must remain distinct from temporary recovery within an active market. Process restart retains the specification’s explicit recovery/start controls.

   The opposing alternative is a small independent state machine for every rule or option. Local code may look simpler, but interactions become the maintenance burden. Missing proof: the twelve controlled examples expressed against the same public evaluation contract, including simultaneous derived results and checkpoint-before-purchase ordering. This defines rule behavior in #4 without implementing later purchasing or recovery integrations.

These are requirements to carry into drafting, not evidence that the concept is defective or that the existing unfinished code must be retained.

Read attestation: I fully read the actual `agents/structural.md` persona and `docs/codex.md` in `/Users/connormoulton/.codex/plugins/cache/egm/egm/0.1.1/`, plus all six required sources: `docs/proposals/issue-4-egm/working_hypothesis.md`, the complete `docs/proposals/issue-4-egm-handoff.md` including A–E and all twelve examples, `METTEN_DESIGN_DRAFT_2026-11-12.md`, `GLOSSARY.md`, `docs/adr/0001-rust-engine-and-tauri-boundaries.md`, and `docs/implementation-plan.md`. Truncated combined output was followed by complete separate reads of the affected documents.

Isolation observation: no earlier parent conversation turns or unique parent-only marker were visible. The delegation, host-provided standing/project instructions, environment information and Ponytail instructions were visible. This observation does not prove host instruction-file exclusion; that remains unverified and was not separately tested.

No source inspection, writes, tests, builds, browsing, provider/account access or operations were performed. Provider feasibility and live behavior remain unproven.
