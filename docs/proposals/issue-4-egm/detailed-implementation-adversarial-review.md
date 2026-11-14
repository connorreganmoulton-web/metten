# Issue #4 — targeted challenge of Detailed Implementation

Date: 2026-11-13  
Requested by: Connor Moulton, during Detailed Implementation review  
Outcome: revise before approval. This is targeted adversarial consultation, not the formal EGM design review, section approval, implementation authority or executed engine acceptance.

## Owner direction

Connor asked what the historical-nesting restriction meant and requested adversarial subagents. His framing: “the goal is this to be as simple as possible but still be powerful” and “an abstraction language for what the actual metten product will do.” The current request explicitly authorized these consultations while the section approval remains pending. No reviewer treated that authorization as approval of the proposed restriction or permission to implement.

## What the restriction means

A calculation is nested when it uses the result of another calculation. Comparing a one-minute average with a five-minute average already works in the proposed grammar. Asking how much the five-minute average changed over the last minute is rejected by the draft because both operations look back through time. Asking for a one-minute average of that five-minute average is rejected too.

These are different problems. The change needs the inner average at two times; the second average needs the inner average's path throughout an interval. The draft used the harder second problem to justify banning the easier first one. That is unnecessarily restrictive for Metten's intended language.

## Completed independent consultations

Each agent received no parent conversation turns (`fork_turns: none`), a bounded read-only task, source paths and the owner's simplicity/power criterion. All three returned actual final reports with full read attestations. Host instruction-file exclusion was not tested; no claim of full filesystem/instruction isolation is made. No further agents were launched.

| Native task | Lens and required full reading | Result |
| --- | --- | --- |
| `/root/language_power` | Author capability and language composition; AGENTS, concepts brief, full design draft and product specification | Replace blanket ban with endpoint composition; distinguish real aggregation cost. Revise derived-time handling and cumulative history. |
| `/root/temporal_math` | Temporal mathematics and evidence; AGENTS, concepts brief, full design draft and full handoff | Model derived measurements as time-dependent values; endpoint composition is feasible. A step/straight-line curve boundary could admit richer rolling composition, but unrestricted exact closure is not a small system. |
| `/root/simplicity_challenge` | Whole-section simplicity and hidden restrictions; AGENTS, concepts brief, full design draft, product specification and ADR 0001 | Do not approve as written: ordinary fraction growth and unsupported workload assumptions are design gaps. Normalize adapter evidence once rather than carrying overlapping identities. |

These are targeted consultants, not bundled EGM persona reports. No EGM numerical score or full-review completion is claimed. Their native final reports are in the session record; the findings below consolidate them without representing an agreed final replacement design.

## Findings and lead disposition

### 1. The historical-composition ban is too broad — major

Affected draft: Detailed Implementation items 1, 2 and 8.

Useful examples lost include change of a moving average and average of raw-price changes. Naming every operation does not preserve composability if ordinary combinations require engine changes.

Recommendation: allow `change` and `percentage_change` to refer to supported derived measurements. Evaluate the underlying definition at the two exact times from verified evidence, rather than comparing conveniently sampled outputs. A one-minute change in a five-minute average requires six minutes of underlying coverage. A cross-feed expression carries each source's own cumulative requirement. Preserve complete intervening coverage, market/contract identity, checkpoint lifetime and initial-prefill rules.

Lead disposition: accept the finding. Remove the blanket prohibition during the section revision. Do not replace it with a promise that every imaginable combination already works. Further rolling aggregation needs an explicit coherent calculation model; the current restriction is not a settled owner requirement.

### 2. Derived measurements can change while the latest price stays still — major

Affected draft: item 2's blanket piecewise-constant series statement and item 3's timer/condition processing.

Raw verified prices may be constant between updates. A moving average can nevertheless change as older prices leave its window. Holding the average constant until the next trade can miss a threshold failure and falsely report that a condition remained true for its full duration. This issue exists even without nested history.

Recommendation: distinguish raw evidence intervals from the time-dependent results calculated from them. Define how Metten accounts for time-driven changes and intervening failures, independently of purchase-loop frequency. Timers must not become new provider observations or confirmations. Preserve the adopted observation-based crossing definition unless separately changed by the owner.

Lead disposition: accept. This is a correctness gap in the proposed design, not evidence that the running product misbehaved; no running strategy was examined. A replacement temporal model and controlled examples are required before section approval.

### 3. The proposed exact-number representation can fail on ordinary inputs — blocker

Affected draft: item 2, signed 128-bit exact fractions.

Averages of percentage differences can accumulate unrelated denominators. This is not confined to extreme raw prices or maliciously deep expressions. Intermediate cancellation does not help when the final exact fraction itself is too large.

The lead independently checked four equally weighted one-second percentage differences using Python's standard-library `Fraction`: one feed fixed at 100001, the other at 100000.000003, 100000.000019, 100000.000057 and 100000.000063. Each percentage difference is about 0.001 percent and fits the proposed raw precision. Their exact average denominator needs **147 bits**, beyond a positive signed 128-bit integer. This is a standalone arithmetic counterexample, not a Metten engine test.

Recommendation: choose a numerical policy that supports the advertised ordinary calculations, with documented precision and decision-boundary behavior. A bounded precision policy cannot silently replace the previously proposed exact meanings; any change must be reconciled explicitly with the approved Technical Plan. Merely switching to arbitrary-size fractions also needs a resource argument and does not solve continuous nested integration.

Lead disposition: accept the blocker. The existing “overflow makes this measurement unavailable” rule is insufficient as the normal support story for this advertised calculation. No replacement numeric policy is approved or implemented.

### 4. Further rolling composition needs a bounded, intelligible model — major

Affected draft: historical operand definitions, time-weighted averages, extrema and numeric/resource policy.

The math consultant proposed an internal capability boundary based on the shape of the calculated values: constant steps and straight-line segments can be aggregated exactly. This can admit average-of-average and useful extrema/change combinations without an operator-pair exception for every case. Authors would still use ordinary named measurements; the engine would validate the actual calculation graph.

However, ratios of changing derived values can require curves whose exact integral is not a rational number. For example, averaging a value proportional to `1/(1+t)` introduces a logarithm. Rolling extrema of continuous values over a half-open interval also need explicit boundary-limit semantics. A general symbolic mathematics engine is a materially larger feature and is not recommended merely to make a blanket composability claim.

Lead disposition: retain this as a candidate, not an adopted implementation. The smallest revision should restore straightforward endpoint composition and establish which further aggregation can be implemented faithfully. Do not silently turn time-weighted averages into sampled averages. The remaining support boundary must be stated in plain English with examples, visible in validation and reviewed before approval. No new author-facing sampling options or numerical machinery have been authorized by this report.

### 5. Resource limits need a usable workload, not just ceilings — major

Affected draft: item 6.

The proposed 24-hour maximum and 262144-record retention cap do not describe a workload that can reliably reach readiness. One changing series at the illustrative ten updates per second requires 864000 raw intervals for a day, before side/measurement expansion. Not every individual maximum needs to fit simultaneously, but the support profile must say which combinations can complete rather than repeatedly exhaust memory during prefill.

Recommendation: account for the actual expanded dependency graph and retained history, keep only event identities still needed after rule processing, and establish a conservative controlled workload. Preserve refusal under unexpected overload. Distinguish supported-workload design from later measured provider throughput and concurrent-run capacity.

Lead disposition: accept. The numbers in the draft remain proposals; their presence does not close this gap. Do not promise 24-hour operational support from a per-field duration ceiling alone.

### 6. Normalize provider identity once — simplification opportunity

Affected draft: item 3's evidence interface.

The draft carries provider identities, normalized ordinals, epochs, event numbers and continuity lineage together. The evaluator needs enough trusted information to distinguish a genuinely new observation from a duplicate, resynchronization or gap. Passing overlapping provider bookkeeping does not independently verify the adapter's claim.

Recommendation: trusted adapters normalize provider-specific identity/replay rules once. Give the engine compact normalized binding/observation identities and explicit validity/coverage events. Retain every adopted duplicate/gap/order test. Require a concrete engine decision for each additional interface field.

Lead disposition: accept as a design simplification to work through. Actual provider mapping is still later evidence; this report does not remove continuity, replay protection or named-source boundaries.

## Next drafting step and stopping condition

Revise Detailed Implementation around named inputs, reusable measurements, conditions, checkpoints and ordered purchase options. Keep these author concepts few and consistent; let the shared engine manage calculation dependencies, evidence, resource accounting and execution safeguards. Preserve BTC-first bindings, eventual compatible-market reuse, no source substitution, immutable versions and the approved one-objective purchase lifecycle.

Before re-presenting section approval, resolve the numeric counterexample, derived-time semantics, cumulative-history requirements and the concrete initial composition/workload support boundary. Reconcile any changed meanings with the approved earlier sections explicitly. Tests listed in the existing draft remain proposed until implementation is separately authorized. No formal EGM review, finalization, publication or implementation follows automatically from these consultations.

No source/dependency changes, builds, provider/account calls, trading operations or GitHub mutations occurred. The standalone arithmetic check establishes one flaw in the proposal; it does not validate a replacement engine or prove live safety/performance.


## Authorized revision and targeted recheck

Connor subsequently instructed: “Go ahead and revise it. Keep it simple and powerful, and resolve those issues before asking for approval again.” The lead revised the Detailed Implementation section and added a proposed precision clarification to the Technical Plan's previously open numeric paragraph. Earlier approvals are preserved as history; these new meanings remain part of the revised section gate. No implementation authority is inferred.

### Resolved design dispositions

| Original finding | Revised rule and result |
| --- | --- |
| Blanket composition ban | Removed. Every supported historical operation can consume a supported measurement in an acyclic, dimensionally compatible, admitted graph. The author adds no scheduling or sampling fields. The reference now shows change-of-average and average-of-average using the same named definition. |
| Incorrect quiet derived behavior | All measurements use one complete logical millisecond calculation clock, independent of UI/purchase-loop frequency. The engine evaluates intervening derived failures; clocks never add observations/confirmations/crossings. Same-millisecond raw events retain their event effects. |
| Fraction denominator overflow | Every operation outputs a six-decimal integer value, with explicit nearest-even normalization. Comparisons use exactly that normalized value. Scaled integer intermediates are bounded below signed128 capacity; unrelated fraction denominators do not accumulate. No arbitrary-precision or symbolic dependency. |
| Unclear further rolling composition | The same discrete, complete time-series meaning applies to every derived operation. Average-of-average and average-of-percentage have defined results; they are not continuous symbolic integrals. The precision/time meaning is shown in preview and reference, including last-place boundary effects. |
| Unusable resource profile | Complete per-node ring/deque allocation is planned before save/Go, including side expansion, metadata and reserve. 256 MiB bounds history working state; simple hour measurements fit, five independent hour averages fail admission with repairable errors. Prefill horizons are recursive and separate from stored buffer lengths. |
| Duplicate adapter/evaluator identity machinery | Provider keys, epochs and clocks are normalized in trusted adapters. Evaluator retains bound identities, monotone observation numbers, explicit coverage/validity and atomic transaction order. Reconnect does not manufacture new observations. |

This is a finite-resolution rule language. It deliberately defines six-place results and one-millisecond time weighting, with explicit normalization at every named operation. The earlier draft did not establish a product requirement for arbitrary exact fractions or continuous symbolic integration. The revision makes these previously open engineering choices visible; it does not claim a hidden exact continuous answer behind the displayed number. Real feeds must establish compatible field precision and evidence before support can be claimed. No old saved version is reinterpreted under the new proposal.

### Actual targeted rechecks

Two fresh-parent-context native agents reviewed the revised draft, concept brief and this report in full and attested those reads plus repository AGENTS. These are continuation checks of the owner's requested adversarial consultation, not formal EGM design review or section approval:

- `/root/revision_workload_check`: found the original composition/arithmetic/retention/identity issues resolved at design level, including the explicit hour allocation examples and chunked deque work. Found one remaining coverage-order contradiction.
- `/root/revision_math_check`: accepted integer arithmetic bounds, recursive warmup and the complete calculation clock. Found the same coverage-order issue, a same-time duration-completion hazard, and an unintended positive-only percentage-denominator restriction.

The lead applied these bounded corrections after their reports:

1. Separate receipt horizon from finalized calculation frontier. Validate/stage coverage for unfinalized intervals before advancing their logical clock cells. A quiet certificate received at 1000 can establish the prior complete second without inventing a source observation. Finalized genuine holes still cannot be silently rewritten.
2. Use one per-instant order: finalize the previous cell; apply expiry/gap/boundary; apply the scheduled atomic value transaction; then derive/evaluate/latch. Do not evaluate a virtual completion timer before a false observation at the same instant in that batch. Distinct already accepted events retain their actual order.
3. Permit signed nonzero derived denominators. A signed difference moving from −2 to −1 produces −50 percent under the previously proposed formula; only zero is unavailable. Raw USD-per-BTC input positivity remains.

These were small fixes to the named findings; no additional review swarm or claim of independent review of the final wording is made. The revised draft contains the concrete regression examples and controlled API fields for all three corrections.

### Executed standalone design checks

The lead ran temporary in-memory standard-library Python calculations, without writing application code or using the Metten engine. Assertions passed for:

- positive/negative nearest-even ties and one-third/two-thirds normalization;
- the earlier four-percentage counterexample: exact denominator 147 bits, revised normalized average 0.001 percent without fraction accumulation;
- signed percentage change −2 → −1 = −50 percent and integer intermediate upper bounds;
- complete-grid composition: a one-second average rising from 100 to 110 has one-second change 10; its next one-second average-of-average is 104.995, with two seconds of required source history;
- a verified quiet interval whose derived average is 100 at both surrounding endpoints but 102.5 between them; a `<101` duration must reset despite no new source observation;
- a missing raw cell at 500 ms: the one-second inner calculation first regains full coverage at 1501 ms, and its one-second outer average at 2501 ms;
- hour-average, hour-range and two-hour nested-average storage below the proposed cap, and five independent hour averages above it;
- both complete JSON specimens parse, Markdown fences balance, and the superseded blanket-ban/fraction/record-count text is absent from the current draft.

These checks establish internal arithmetic examples and allocation arithmetic, not executed Rust behavior, live source readiness or measured responsiveness. Coverage staging and same-time ordering were walked through against the explicit timelines and added as required public-engine tests; no engine test result is claimed. Remaining required evidence is implementation correctness, actual allocated capacities/service responsiveness, documentation usability, packaged behavior, provider acceptance and later measured run capacity. The original unresolved *design definitions* now have concrete proposed replacements, ready for Connor's section review; the formal EGM and implementation gates remain pending.
