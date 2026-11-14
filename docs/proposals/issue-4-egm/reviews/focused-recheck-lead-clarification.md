# Originating lead clarification

Source: [Draft Issue 4 Implementation Design](t3-thread://v2/646f9e94-f87e-459e-b51a-e29a116396d6/2a8d4f3d-442c-43ca-96bc-4e009ee92bed), completed run ordinal 11, message position 359, 2026-11-13. Received after all independent reviewers completed. Verbatim answer follows; it is not part of the unchanged target.

Both findings are valid. Keep them open in the report: these clarifications are intended resolutions, not rules already established by the target.

1. **CR-01 — rejection lifecycle**

   For a pending batch rejected during validation or staging **before accepted evaluation state changes**:

   - Discard its continuation and release its admission.
   - Return to idle, allowing the next valid `begin`.
   - Preserve the prior accepted frontier, bindings, history, checkpoints and observation counters.
   - Return no candidate. Do not revive the candidate invalidated by `begin`.

   Exceptions remain explicit: `begin` returning `busy` preserves the existing pending batch; `Rejected(no_pending_batch)` changes nothing; an explicitly stopped evaluator remains terminal.

   **After evaluation begins mutating state, blanket rejection must not imply rollback.** Expected numerical failures, evidence gaps and invalid objective views should follow their defined unavailable/blocking outcomes, preserving trustworthy evidence and allowing unaffected alternatives where common safeguards permit.

   An unexpected internal failure that prevents coherent continuation should terminate that evaluator, suppress candidates and preserve its last completely published snapshot. No automatic recovery or restart. The terminal diagnostic outcome also needs explicit definition in the document.

   A rejected input must never be treated as proof of uninterrupted source coverage. Any actual loss of required evidence still requires the existing invalidation/recovery rules.

   Require the invalid-batch → idle → valid-batch recovery assertion, plus a separate post-mutation failure assertion.

2. **CR-02 — market reset**

   Confirmed: preserve the entering contract’s own admitted pre-open history. Clear market-local checkpoints, temporal conditions and objective state; never transfer another contract’s history.

   Your proposed replacement for [line 713](/Users/connormoulton/Desktop/metten/docs/proposals/issue-4-egm/strategy-rules-design.md:713) expresses that distinction. Preserve the row’s existing immutable-version assertions too.

No edits, implementation or tests ran. Both findings remain unresolved in the target pending document correction.
