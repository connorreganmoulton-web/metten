# Raw Goldfish report — design

Reviewer id: `/root/egm_synthesis/design`  
Role: Design Critic-Brainstormer (`egm-design`)  
Status: completed; actual native child final received.  
Read attestation: included verbatim in the raw report below.  
Mode: BRAINSTORM. No rewriting of the child's report follows.

---

Completed the bounded BRAINSTORM review. No concept-level blocker found. The shared engine direction fits the stated purpose; three safeguards should remain explicit as the concept develops.

1. **Major — resource and quota economics: flexible history needs a bounded operating envelope.**  
   Evidence: `working_hypothesis.md`, “Preserved boundaries”; handoff decisions A, D and E; specification §§2, 10 and 16. Authors may combine nested alternatives, multiple checkpoints, distinct-update confirmations and long lookbacks. Underlying history may cross market boundaries, while later releases support concurrent paper runs. Resource demand therefore depends on observations, duration, rule complexity and simultaneous runs—not merely uploaded file size.

   Consequence: accepting a small JSON file could commit the application to excessive memory or evaluation work. Persisting every observation as the workaround would conflict with the decision against continuous raw-feed storage.

   Mitigation for the concept brief: require justified bounds on accepted rule complexity and retained state, explicit rejection when an otherwise valid strategy exceeds the supported envelope, and later measured capacity that protects live controls. Preserve the full agreed capability families and hour-long-history example. Engineering should choose and justify the bounds; Connor should only see consequential restrictions.

   Opposing alternative: unrestricted author-selected resource use maximizes flexibility but leaves local reliability uncontrolled. Arbitrary inherited node counts or a less-than-one-market lookback cap would be easier to enforce but would contradict the agreed direction. Missing proof: workload measurements and concrete bounds; these are not established by this review.

2. **Major — architectural bottleneck: observation fidelity and coherent evaluation must survive bounded processing.**  
   Evidence: handoff decision E and controlled examples 4, 5, 7 and 10; specification §16; ADR 0001.

   Consequence: retaining only the latest value could erase a brief failure, crossing or qualifying observation. Independently updating derived measurements could also evaluate a purchase using one new result and one old result. Conversely, treating every sampled-counter jump as a provider gap could unnecessarily disable valid conditions.

   Mitigation for the concept brief: distinguish validated source receipt, retained evidence and purchase evaluation. Require coherent evaluation of affected derived values and checkpoints before dependent purchase conditions. Overload must make affected evidence unavailable explicitly; it must not silently fabricate continuity or discard decision-relevant observations. This belongs in #4’s controlled-input semantics; actual adapter ordering and overload acceptance remain later integration evidence.

   Opposing alternative: a latest-value-only queue simplifies processing but cannot support all agreed historical and checkpoint behavior. One global source-health gate simplifies blocking but wrongly disables independent alternatives. Missing proof: the engineering ordering contract and controlled timelines demonstrating it. The twelve planning examples identify the expected outcomes but have not been executed here.

3. **Major — architectural compatibility: immutable JSON alone does not preserve a strategy’s meaning.**  
   Evidence: `working_hypothesis.md`, import/explanation/versioning boundaries; handoff example 12; glossary “Strategy version”; specification §§4, 10 and 12.

   Consequence: unchanged saved JSON could acquire different behavior after an application update changes an operator’s definition, rounding or time boundaries. An updated explanation could then describe a meaning different from the one used by an earlier run, even though the stored strategy was never edited.

   Mitigation for the concept brief: require saved versions and historical explanations to remain tied to their original validated rule meanings. The detailed design should choose the smallest explicit compatibility mechanism and reject unsupported meanings clearly. This is an engineering definition of the existing immutability requirement, not a new authoring feature.

   Opposing alternative: always reinterpret old JSON with the newest definitions is simpler, but conflicts with preserving earlier meanings. Keeping every historical evaluator forever may be unnecessary; a bounded compatibility policy could suffice. Missing proof: the chosen policy and checks across a semantics change.

Provider economics remain an acknowledged later gate, not a newly discovered blocker for #4. Explicit source identity must survive validation without implying that BRTI, Coinbase or Kraken access, licensing, fields, quotas or production continuity have been proven. No external-source or account verification occurred. Generated template explanations require no LLM service, so no runtime token budget is implicated.

Initial prefill is also already settled: collect all required initial history from Go, then wait for the next full market. Restoring evidence within an already-active market must not automatically impose that startup wait again. Completed checkpoints and independent alternatives retain their agreed treatment.

Read attestation: fully read the actual installed `agents/design_critic.md` and `docs/codex.md`, plus all six assigned sources:

- `/Users/connormoulton/Desktop/metten/docs/proposals/issue-4-egm/working_hypothesis.md`
- `/Users/connormoulton/Desktop/metten/docs/proposals/issue-4-egm-handoff.md`, including decisions A–E and all twelve examples
- `/Users/connormoulton/Desktop/metten/METTEN_DESIGN_DRAFT_2026-11-12.md`
- `/Users/connormoulton/Desktop/metten/GLOSSARY.md`
- `/Users/connormoulton/Desktop/metten/docs/adr/0001-rust-engine-and-tauri-boundaries.md`
- `/Users/connormoulton/Desktop/metten/docs/implementation-plan.md`

Isolation observation: I saw the assigned dispatch and host-supplied instructions, including the user-provided AGENTS instructions and environment context. I observed no earlier parent dialogue or parent-only marker. This observation does not verify host instruction-file exclusion; that remains untested.

Skipped implementation/source inspection, tests, builds, external access and all writes. These findings identify concept safeguards; they establish no provider, performance or live-trading acceptance.
