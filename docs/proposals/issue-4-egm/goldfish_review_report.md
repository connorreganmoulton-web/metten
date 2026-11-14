# Goldfish Annotated Review Report

Status: resolved

**Resolution:** Connor replied “Auto-accept”; all 12 PROPOSE FIX document edits were batch-applied. This status records textual resolution, not implementation proof or independent re-review.  
**Target:** [strategy-rules-design.md](strategy-rules-design.md) (`full file`, original lines 1–664).  
**Baseline SHA-256:** `3f6f8c23492a41131f56cb81e899860da5565a28519f61ac79ffbe081731c853`.  
**FindingsList:** [consolidated review](consolidated_review_report.md), CR-01 through CR-12, with all 17 raw identities preserved below.

## Owner-facing scope check

Issue #4's current body was re-read without changes: strict bounded rule semantics; generated explanations; controlled checkpoint/side/reset/gap examples; immutable versions; matching field reference and one learning file. All proposed edits fit those acceptance criteria or the local import/verification slice needed to deliver them.

The consequential repairs define the **controlled engine interface**, its source/contract identity, and bounded **local strategy requests**. They do not implement source adapters, market discovery, subscriptions, an execution scheduler, paper/live purchases, order/holding recovery, daily accounting, qualifications, concurrent runs or measured production capacity. Those remain #5 onward. The existing desktop Start/Pause still controls only its non-trading foundation. No author JSON feature, provider, strategy, dependency or broader market is added.

The three implementation phases below describe later separately authorized coding. No phase is executed in this session. This report initially proposed the changes. Following Auto-accept, only the target, synchronized draft, preserved original snapshot and this resolution record were written. All 12 consolidated items are PROPOSE FIX, including four Nits worth fixing for reader clarity. No contest, escalation or decline is needed; Connor is not being asked to design arithmetic, bindings or worker mechanics.

## Prioritized Findings & AI Suggestions

### CR-01 — Major — make long calculations responsive to Stop

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** R9-1. Target lines 438–439, 458, 525–534.

The plan promises a control check after bounded work but exposes only a synchronous call returning the final decision. It does not define how that call can suspend, retain its continuation or receive a later Stop. A small input batch can still represent millions of logical calculation steps.

* **Rationale:** specify one concrete bounded/resumable processing contract, who owns its continuation, how control interrupts are inspected, when processing is admitted/refused, and why no intermediate/stale purchase candidate can escape. Keep the pure engine independent of Tauri and provider clients. Do not add another service.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Valid readiness gap. The repair specifies a pure controlled continuation API, not a scheduler, running bot or desktop strategy-control integration.
* ***Proposed Edit:***

Location: Detailed Implementation item 3 public API, item 6 service-slice paragraph, and the matching item 8 verification rows. Replace the single final-result `Evaluator::advance` contract with the following text, and update references to that API consistently:

> `Evaluator::begin(batch) -> Result<(), EvidenceError>` admits one owned bounded batch only while idle. A second batch returns `busy`; a stopped evaluator returns `stopped`; an oversized batch returns `resource_limit`. These refusals do not mutate accepted evidence or queue an unbounded task. Beginning a batch clears any previously returned current candidate and creates private continuation state; it returns no Decision.
>
> `Evaluator::process_slice(control, objective: Option<ObjectiveView>) -> SliceResult` performs at most 4096 primitive validation, staging, numeric, condition, or deque-entry operations per call. `SliceResult` is `Pending { phase, evaluated_through_ms }`, `Complete(Decision)`, `Stopped`, or `Rejected(EvidenceError)`. `phase` is `validating`, `staging`, or `evaluating`. The evaluator owns the continuation, including clock/node/transaction/member/deque cursors; callers cannot clone or resume it twice. A deque operation spanning slices retains its cursor without exposing a partially applied candidate. Validation/staging is also sliced; a large input must not hide an unsliced validation loop. Complete batch validation precedes mutation of accepted evaluation state.
>
> `control` is `Continue` or `Stop`, checked before doing work in each slice. Stop cancels the pending continuation and makes that evaluator terminal without processing the remaining catch-up ticks. It returns no candidate and preserves the last completely published checkpoint/progress snapshot and all separately stored immutable history. Private partial work is not presented as a completed observation. A later Go creates a new evaluator under the existing no-automatic-resume rule. This internal cancellation is not the product's ordinary “Stop after this market”; that runtime mapping remains later work.
>
> While Pending, the evaluator reports processing unavailable for current purchase eligibility. It does not label valid source history unsafe or fabricate a provider gap. Only Complete can expose one decision for the final target, after checking current admission and the supplied active-market objective. ObjectiveView is required only when a market is active; None during collection cannot authorize a purchase. An invalid/regressing active objective or failed current admission yields no candidate. Numeric failures retain their existing dependency-local unavailable behavior.
>
> The controlled caller performs one slice, checks control before the next slice and again before using a Complete candidate, and never uses an intermediate catch-up candidate. Stop therefore takes effect at the next bounded slice boundary, not after the whole batch. No wall-clock response-time promise is inferred from an operation budget. A newer relevant observation or changed objective invalidates a candidate before use; actual execution and numerical age profiles remain later work.
>
> Pending input plus any queued normalized transactions share the existing 4096-transaction/8-MiB ingress budget; these are total bounds, not a fresh allowance for each waiting batch. Admission exceeding either bound returns resource_limit without waiting or silently dropping events. The controlled caller blocks affected current eligibility and requires trustworthy resynchronization when ingress completeness is lost. Existing completed checkpoints retain their defined lifetime. No arbitrary disconnect or catch-up timeout is added to JSON.
>
> Issue #4 implements and tests this pure resumable contract with a deterministic caller that injects Stop between slices. It does not add a scheduler, connect this evaluator to desktop Start/Pause, or build a paper/live run loop.

Add required controlled assertions: a long quiet catch-up returns Pending after its first bounded slice; Stop on the next call returns Stopped without finishing catch-up; no candidate was exposed; oversized/second pending admissions fail deterministically without erasing accepted state; an unchanged accepted batch yields the same final values regardless of slice boundaries. Charge continuation/cursor/snapshot storage to the existing working-memory reserve. No new dependency or new production file.

### CR-02 — Major — define upcoming-contract binding and history preservation

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** R9-2. Target lines 348–352, 438, 444–450, 460.

An abstract input binding is not enough to identify an upcoming contract before entry. The plan allows feasible long contract history but supplies contract identity at the market boundary and also resets contract-specific state there. A developer must otherwise invent how the same contract's pre-open history survives while a different contract's book cannot be substituted.

* **Rationale:** define the concrete binding map and collection lifecycle keyed to actual market/contract identity, the earliest permitted post-Go collection, and the preservation/reset rule at entry. Preserve checkpoints' per-market reset and continuous compatible underlying history. Do not infer listing lifetime from the 15-minute outcome period or add provider integration.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Valid identity/lifetime gap within the required explicit-source semantics and controlled per-market-reset checks. Concrete identity/resource admission is specified; real market discovery and collection policy stay in later issues.
* ***Proposed Edit:***

Location: item 3 RunContext and normalized binding/MarketBoundary definitions, item 6 admission wording, and item 8 history checks. Insert these definitions and replace the ambiguous “contract-specific state resets” wording wherever it could erase the new contract's own prepared history:

> `MarketDescriptor` is trusted metadata `{ series_id, market_id, contract_id, open_ms: i64, close_ms: i64 }`. Its open/close are relative to Go and may describe a market already open at Go; their checked difference must be 900000 ms for the selected BTC15m series. Metadata alone is not an observation and supplies no history, checkpoint or eligibility. Identifiers are opaque nonempty strings of at most 128 UTF-8 bytes without control characters.
>
> `BindingDescriptor` is `{ binding_id: u32, input_id, scope }`. Input source/instrument/field/unit and any fixed side come from the validated document; metadata cannot override them. Scope is `Underlying`, `Market { market_id }`, or `Contract { market_id, contract_id, side }`. `selected_side` expands to explicit YES and NO bindings. The tuple including the validated input identity and concrete scope is the historical identity. Underlying bindings remain independent of market changes.
>
> RunContext supplies an initial bounded descriptor/binding set. A controlled `RegisterMarket { descriptor, bindings }` metadata transaction can add another identified market before its open. Registration must be admitted and fully validated before any evidence references its binding ids. A binding id is never remapped to another tuple during a run; same exact registration is idempotent, conflicting reuse is an error. Retired ids are not reused. Price/current-validity/coverage/observation-number state begins empty; first accepted evidence must be post-Go. Descriptors do not import pre-Go history.
>
> Market-dependent measurement state is keyed by concrete market/contract, so pre-open evidence can warm only that exact contract's calculations. The same named rule definition can have separately accounted concrete instances; no old book is renamed to current_contract. Checkpoints, holds, confirmations and purchase eligibility remain inactive before their own market entry, even when a historical measurement is collecting.
>
> `MarketBoundary { market_id }` activates the already registered descriptor at its declared open, subject to initial readiness and next-full-market rules. It resets the new market's checkpoint/temporal/objective state but preserves historical cells and measurement warmup already collected for that exact entering contract. It cannot preserve the previous contract's cells under a new identity. Continuous applicable underlying history is retained. Unknown/mismatched descriptors or evidence bindings cannot authorize entry or contribute history; return a finite unregistered_binding/incompatible_market error.
>
> Registration is bounded by the existing 128 expanded numeric-instance and 256-MiB working-memory limits, counting every concrete market-dependent instance and its retained buffers. The preview reports shared underlying cost and per-market replication cost. Constructor admission reserves the complete initial binding set before Go; later registration must reserve its entire additional planned state before collection for that new binding begins. Refusal leaves existing registered history intact and does not falsely start a collecting progress bar. No admitted series grows beyond its planned capacity as it fills. Descriptors without price state are capped at 128 and charged to the fixed reserve. This refines resource accounting; it neither increases the budget nor promises unlimited upcoming-contract collection.
>
> Issue #4 only supplies this controlled metadata/evidence contract and its deterministic tests. Selecting/discovering actual future contracts, subscribing to books, collecting real feeds and source-access/listing feasibility remain #5 and subsequent integration work. No provider watchlist/collector or desktop running-strategy flow is added.

Add explicit controlled cases: register one future contract at Go, accept its own hour of complete post-Go history, and preserve that history when the same id enters a later full market; a different contract's identical prices cannot satisfy it; no pre-open checkpoint can latch; conflicting binding-id reuse and pre-Go observations are rejected; an over-budget registration refuses before accepting new evidence and preserves existing histories. Use a future open after readiness, not the exact readiness instant that must wait for the following full market.

### CR-03 — Major — bound native strategy-operation admission

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** B9-2. Target lines 464–477, 494, 520–526, 646; source details in the raw Codebase report.

The current native bridge starts blocking work for each request. Per-request size limits and disabled frontend buttons do not bound concurrently queued direct IPC requests, and strategy storage shares a recorder mutex with session recording.

* **Rationale:** define a native admission guard before spawning work. The source reviewer recommends one in-flight strategy operation, nonblocking acquisition, a finite Busy response for excess requests, and guaranteed release on every completion/failure. Keep quit handling and existing permissions. This needs no dependency or wider architecture change.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Valid local import-boundary gap. A standard-library one-operation guard is the smallest bounded native fix and reuses the existing command/permission.
* ***Proposed Edit:***

Location: item 4 native requests and item 8 native checks. Insert:

> The existing `strategies` command permits one native in-flight strategy operation, covering preview, save, list and detail. After bounded request decoding and the quit check, acquire a nonblocking operation guard before calling spawn_blocking. If occupied, return the finite error `{ code: busy, message: Strategy action already in progress. Wait and retry. }`; do not spawn or queue another waiting task. No caller-provided text appears in that error.
>
> The guard is owned by the accepted task and releases on success, validation/storage error, task rejection, cancellation before execution, and panic unwinding. Use the existing standard-library ownership/atomic pattern, without another executor/dependency. Do not hold an interface-state or credential lock across blocking work. Recheck quitting before beginning storage mutation; preserve committed-save-versus-refresh-failure reporting.
>
> Busy preserves editor content and any still-current preview; it is not a failed commit and triggers no automatic save retry. Matching Rust/TypeScript result unions include this finite native error. Permissions stay limited to the existing local main-window strategies command.

Add native tests: hold one accepted operation at a controlled barrier, directly invoke each other operation and receive Busy with no queued task, then release via success and via failure and verify the next operation can run. These are future tests in the already planned `src-tauri/src/main.rs`; no actual credential/provider request is needed.

### CR-04 — Major — prescribe buildable implementation phases

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** R9-3. Target lines 329–649, especially 483–512 and 627–649.

The file list and final checks are detailed, but the transition sequence across engine, storage, native requests and frontend callers is not defined. An implementer still has to decide when interfaces change and how callers stay buildable.

* **Rationale:** add a short ordered phase table with exact files, completed behavior, caller compatibility/removal rules and applicable checks for each stable checkpoint. Keep it within one bounded issue and existing tools. No builds need to run during document resolution.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Valid planning gap under the supplied readiness standard. Three stable checkpoints avoid a new compatibility framework or a deployment requirement.
* ***Proposed Edit:***

Location: item 5, after the complete file allowlist. Insert this ordered implementation phase table; it describes future authorized work, not work to execute now:

| Phase | Exact planned files | Stable checkpoint and checks |
| --- | --- | --- |
| 1 — pure rule contract | `crates/engine/src/strategy.rs`, `crates/engine/src/lib.rs`, `crates/engine/tests/strategy.rs` | Add format-2 validation, explanations and controlled resumable evaluation; expose the new public types. Keep the old import/list entry points and their format-1 parser/explanation only as temporary development compatibility for existing storage/native callers. Do not add a second connected evaluator. New format-2 tests use inline controlled fixtures until the shared example switches. Existing session behavior remains unchanged. Run affected public-engine tests, existing session regression tests, fmt and clippy. |
| 2 — coordinated local import/version slice | `crates/engine/src/strategy.rs`, `crates/engine/src/lib.rs`, `crates/engine/src/storage.rs`, `crates/engine/tests/strategy.rs`, `crates/engine/tests/strategy_storage.rs`, `src-tauri/src/main.rs`, `src/contract.ts`, `src/main.ts`, `src/view.mjs`, `src/view.d.mts`, `index.html`, `src/style.css`, `tests/strategy.test.mjs`, `tests/refresh.test.mjs`, `examples/learning.json` | Change the engine/storage/native/frontend contract together in this phase, including revision-3 migration, tagged operations/admission, preview/save/version flow and harness repair. Switch the shared example and all its consumers together. Remove the temporary format-1 parser/explanation/import compatibility after all callers are replaced; legacy rows are inspected as stored, not executed/reinterpreted. End-of-phase has one format-2 evaluator and one strategy-command protocol. Run workspace Rust tests, fmt/clippy, npm check/build and focused IPC/storage/UI checks. |
| 3 — documentation and focused packaged proof | `docs/strategies/README.md`, `docs/strategies/authoring.md`, `docs/strategies/reference.md`, `docs/strategies/agent-authoring.md`, `README.md`, `GLOSSARY.md`, `docs/verification/issue-4.md` | Complete the linked human/agent/reference guides against the now-checked grammar/example. Perform the independent documentation-led authoring exercise and existing focused unsigned packaged import/reopen/edit-copy checks on a disposable workspace; record results and missing evidence. Reuse unaffected passing checks. No credential/provider/trading integration, signing or publication. |

All coupled caller/type changes in phase 2 form one buildable checkpoint rather than separately shipping incompatible intermediate contracts. The temporary phase-1 compatibility is neither a supported new format nor a deployment. Phase order adds no commits/PR/build authority in this design-only session. Design/review records are updated separately as documentation under EGM gates, not by expanding these source phases.

### CR-05 — Major — include the required existing refresh-test harness repair

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** B9-1. Target lines 485–499; source `tests/refresh.test.mjs:64–77` and `src/main.ts:94,121`.

The existing harness groups every non-control request into its deferred snapshot list. Strategy startup therefore occupies a snapshot slot. The proposed tagged strategy-list request needs a separate response, but the supposedly complete file allowlist omits this existing test file.

* **Rationale:** add **Modify `tests/refresh.test.mjs`** with a command-specific strategy-list mock; only snapshot requests enter the deferred snapshot array. Preserve the existing stale-response assertions. Do not add unrelated credential-test cleanup: the source reviewer found no equally necessary edit there.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Valid caller-fixture omission confirmed in the existing source. Only the specifically affected refresh harness is added.
* ***Proposed Edit:***

Location: item 5 complete allowlist. Add exactly:

| Modify `tests/refresh.test.mjs` | Route invoke mocks by command: only snapshot calls enter deferred snapshot reads; strategies list gets its own correctly tagged result; control remains independent. Preserve every existing before/after-Pause stale-response assertion. Necessary caller-fixture correction, not unrelated test cleanup. |

Location: item 8 interface verification. Add: “The entry-point harness must start a strategy-list request without consuming either deferred snapshot response; resolving an older snapshot afterward cannot overwrite the newer Pause/status result.” Do not add `tests/credential.test.mjs` to the change list without new concrete evidence.

### CR-06 — Major — reject demonstrably unreachable market-local holds

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** D7-1. Feature lines 152, 168, 195–199; detailed definitions 395, 429, 523.

A hold that begins no earlier than market open and lasts 900 seconds cannot finish before an exclusive 900-second close. A one-hour hold cannot complete in that market either. This is different from an hour-long underlying lookback, which may span markets legitimately.

* **Rationale:** define field-specific reachability validation and repair guidance for holds that cannot finish before their enclosing effective deadline. The smallest route is rejection of demonstrably unreachable rules rather than silently accepting an option that can never qualify. Do not impose a market-length cap on historical lookbacks or attempt a general theorem prover for arbitrary strategy profitability/reachability.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Valid semantic validation gap. Rejecting a provably impossible hold is issue-4 rule validation, not a new trading strategy or market-history restriction.
* ***Proposed Edit:***

Location: item 1 `during`/condition semantics and item 8 validation checks. Insert:

> Validation rejects a demonstrably unreachable market-local hold using `unreachable_duration` at its rule path. Compute the effective interval as the intersection of the node and ancestor intervals. Its end is exclusive. A held_for duration must have a possible completion strictly before that end: `[0,900)` with a 900-second duration is rejected, not accepted as a permanent wait. The existing broad numeric duration bound is only a syntax bound; semantic interval validation is mandatory.
>
> For this time-only reachability check, a current comparison/crossing/confirmation has earliest possible success at the effective interval's start; do not assume a provider rate or minimum millisecond spacing between genuine same-time transactions. `all` takes the maximum child lower bound, `any` the minimum, and held_for adds its duration to the child's lower bound. Checkpoint references use the referenced checkpoint's earliest possible establishment, and intervals impose their start. Process checkpoint dependencies in their existing acyclic order. If a held_for or enclosing condition has an earliest possible success at or beyond its exclusive end, return field-specific repair guidance. This is a conservative bound, not a theorem prover for all numerical conditions or market profitability.
>
> This check does not cap historical lookbacks at market length: applicable underlying and exactly identified pre-open contract history may already exist. It must not reject feasible hour-long measurements merely because the outcome window is 15 minutes.

Add assertions for 900 seconds at an exclusive 900-second end, a hold completing strictly before end, nested holds whose combined earliest completion misses end, and preservation of a feasible hour lookback.

### CR-07 — Major — make the arithmetic regression fixture self-contained

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** C6-1, T9-4. Target lines 407–409, 621, 638.

The standalone target refers to four earlier percentage values and a 147-bit-denominator counterexample without containing its operands. The supporting targeted record has the numbers, but the document-only reviewer correctly could not reconstruct the required verification from the target alone.

* **Rationale:** include a small named controlled fixture, its exact graph, normalization boundaries and expected results. Existing evidence supplies Coinbase BTC/USD fixed at `100001` and four equal one-second Kraken BTC/USD values: `100000.000003`, `100000.000019`, `100000.000057`, `100000.000063`. Compute percentage difference against Kraken, then a four-second time average. The revised per-node six-place results are `0.001 percent` and their average is `0.001 percent`. Do not replace the already selected decimal policy with another arithmetic framework.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Valid comprehension gap; it cannot be contested. The fixture is necessary for a human/agent to reproduce the advertised required check independently.
* ***Proposed Edit:***

Location: item 2 numeric examples, with references from items 7 and 8. Insert a named fixture “Four-feed-difference rounding regression”:

> Coinbase BTC/USD last_trade is `100001` throughout `[0,4000)` ms. Kraken BTC/USD last_trade is `100000.000003`, `100000.000019`, `100000.000057`, `100000.000063` over successive one-second intervals `[0,1000)`, `[1000,2000)`, `[2000,3000)`, `[3000,4000)`. All observations/coverage are controlled, post-Go, valid and complete.
>
> Define `feed_difference_percent = percentage_difference(left=coinbase_price, reference=kraken_price)`, then `average_feed_difference_percent = rolling_average(of=feed_difference_percent, lookback_seconds=4)`. Each percentage node normalizes to `0.001 percent` under the existing six-place nearest-even policy. At 4000 ms, its four-second average also normalizes to `0.001 percent`. These exact operands and output boundaries are the regression fixture. This is a calculation example, not another shipped trading strategy or a provider-access claim.
>
> Historical rationale only: averaging the four unnormalized exact fractions would require a 147-bit final denominator; that rejected implementation is not part of format 2. The expected format-2 intermediate/final values above are authoritative for the controlled behavior check.

Replace every “prior/previous counterexample” requirement with this fixture's name and a link to its section, so a reader needs no conversation or separate report to reproduce the oracle. No arithmetic policy change.

### CR-08 — Major — remove stale operative drafting instructions

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** T7-1, S7-1, T9-3. Feature lines 34, 51, 134, 154, 238, 286, 320–322; detailed lines 651, 664.

The finalized target still directs the reader to draft or approve already completed sections. Statements that formal review has not started also need their historical finalization date, now that this review is complete.

* **Rationale:** use current cross-references to completed definitions; keep approval chronology in provenance; date historical handoff claims. Do not erase genuinely missing implementation/operational evidence or turn section approval into implementation authority.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Valid workflow-state ambiguity. Current instructions must not direct readers to repeat completed drafting or infer implementation authority.
* ***Proposed Edit:***

Location: feature/detailed operative status sentences and final handoff. Apply these precise substitutions while retaining pending implementation evidence:

- “during technical drafting” → “in Detailed Implementation items 1–3” where it directs completion of this now-written contract.
- “Their exact boundary and numerical examples must be included in the detailed rule reference.” → “Detailed Implementation item 2 defines the boundary/numeric examples; the future shared reference must reproduce them.”
- “Exact document paths, responsibilities and checks will be assigned during technical and implementation drafting.” → “The document paths, responsibilities and checks are assigned in Detailed Implementation items 5, 7 and 8.”
- “Concrete numbers remain a required engineering deliverable in Detailed Implementation” → “Detailed Implementation item 6 specifies the proposed support profile; actual allocation/responsiveness evidence remains required.”
- Delete the obsolete command to draft Detailed Implementation and stop at its approval gate. Preserve the underlying approval quotation only in provenance.
- “No formal Goldfish design review has run on this Detailed Implementation section” → “As of this target's initial finalization, formal EGM review had not started. The full review subsequently completed; its findings and target hash are recorded in the consolidated review report.”
- Replace the handoff's current “Next phase: egm-review” instruction with “Current phase: EGM document resolution. The full review is complete; accepted document fixes require affected closure checks before readiness is claimed. Implementation remains unauthorized in this session.”

Mark the new target “resolution revision” rather than claiming another full review already passed. Preserve the original reviewed version at `docs/proposals/issue-4-egm/strategy-rules-design.reviewed-2026-11-13.md` before changing the untracked working target; retain its original SHA-256 and review reports. Update both current target and draft together after acceptance so they remain synchronized. The archive is review evidence, not a new product deliverable.

### CR-09 — Nit — consolidate chronology and repeated authority scaffolding

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** T7-2, T9-1. Feature lines 51, 171, 175, 244, 320; detailed lines 327–329, 651, 658–664.

Move repeated conversation history and approval quotations into the existing provenance record. Keep operative requirements and technical rationale where they are used, a short opening authority reference, and relevant verification limits beside their checks. Preserve money, data and no-implementation boundaries.

* **Rationale:** The concrete reader/implementation decision is clarified by the exact edit below.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Nit worth fixing because Connor explicitly said the document is difficult to follow. Move chronology rather than remove required behavior or safety boundaries.
* ***Proposed Edit:***

Location: feature clarification/history paragraphs, Detailed Implementation opening and final provenance. Relocate Connor's documentation/readability/portability clarification chronology, the Alternatives approval quotation, and repeated authorization quotations into “Approval, Reconciliation and EGM Handoff.” Keep their resulting requirements in place.

Use this concise implementation opening: “This section specifies the proposed issue-4 changes and their verification. Approval and implementation authority are recorded in the final handoff; no implementation is authorized in this session.” Keep concrete source-access, raw-data, privacy and no-live-proof limitations at their relevant validation/run/check steps, rather than repeating the full boundary list in every section. Do not erase prior approvals, reviewer findings or unperformed required evidence.

### CR-10 — Nit — narrow the public instrument-list claim

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** D7-2, T7-3. Target line 318.

The preserved captures support the multiple-series context but do not independently establish the full named BTC/ETH/Gold/Silver enumeration. Narrow to the supported observation or preserve the actual indexed evidence before relying on that enumeration. The smaller route is wording correction; broader adapter support remains outside this issue.

* **Rationale:** The concrete reader/implementation decision is clarified by the exact edit below.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Nit worth fixing: calibrate the claim to preserved evidence without broadening research or provider scope.
* ***Proposed Edit:***

Location: “Public evidence for the portability clarification,” replace the paragraph containing the unsupported enumeration with:

> Kalshi's [15-minute market guide](https://kalshi.com/pro/help/fifteen-minute-markets) describes selecting among active 15-minute series and displaying underlying data where available. The captured [market-list page](https://kalshi.com/tag/15-min-markets) is a dynamic shell and does not independently verify the available instruments. These public sources support the broader-market context, not Metten adapter capability, entitlement, reference/settlement compatibility or cross-market qualification. The owner’s eventual compatible-market goal remains; issue #4 remains BTC-first.

Retain the existing local captures and their raw-HTML/extraction limitations. Do not perform extra research, provider access or broader-market implementation to repair a wording claim.

### CR-11 — Nit — name the walkthrough accurately

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** T9-2. Target line 625.

Change “documentation-only walkthrough” to “independent documentation-led authoring walkthrough.” Documentation is the participant's instruction source, while engine preview/save checks remain part of the exercise. Do not drop those required executable checks.

* **Rationale:** The concrete reader/implementation decision is clarified by the exact edit below.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Nit worth fixing with one exact phrase replacement, preserving the real verification requirement.
* ***Proposed Edit:***

Location: item 7 walkthrough paragraph and corresponding verification row. Replace “documentation-only walkthrough” with “independent documentation-led authoring walkthrough.” Keep all engine validation, generated explanation, preview/save/reopen/edit-copy and participant-identification requirements. This clarifies the source of instructions without removing any executable check.

### CR-12 — Nit — separate summary-count and byte limits

* **Affected Scope:** original target lines and source identifiers copied from the consolidated finding below.
* **Discrepancy Description:**

**Sources:** T9-5. Target line 526.

Replace compressed `50/32 KiB` notation with at most 50 summaries and 32 KiB per page, explicitly stating that the byte ceiling applies to the complete serialized response. Preserve bounded paging rather than silently truncating summaries.

* **Rationale:** The concrete reader/implementation decision is clarified by the exact edit below.
* ***AI Suggestion:*** PROPOSE FIX
* ***Justification:*** Nit worth fixing because the two limits and cursor semantics affect an implementer and repair a current request-boundary ambiguity.
* ***Proposed Edit:***

Location: item 6 output limits and item 4 list response. Replace `Version summary page at most 50/32 KiB` with:

> A list page contains at most 50 complete summaries and its complete serialized UTF-8 response is at most 32 KiB, including envelope/cursor fields. Add summaries newest-first only while both limits fit. If another row remains, the next cursor identifies the last included row so the following request neither skips nor repeats it. Do not truncate a summary or silently omit rows. The existing bounded summary schema/finite status text must allow every individual summary plus envelope to fit; malformed legacy display metadata is represented by the existing invalid/unknown summary status rather than echoed unbounded text.

Keep the existing one-MiB individual-detail limit. Add a boundary check that multibyte names can shorten a page before it reaches 50 summaries while the cursor still preserves complete ordering.

## Raw identity, severity and category preservation

| Raw finding | Original severity | Consolidated edit | Category |
| --- | --- | --- | --- |
| C6-1 | Major | CR-07 | PROPOSE FIX |
| D7-1 | Major | CR-06 | PROPOSE FIX |
| D7-2 | Nit | CR-10 | PROPOSE FIX |
| S7-1 | Nit | CR-08 | PROPOSE FIX |
| T7-1 | Major | CR-08 | PROPOSE FIX |
| T7-2 | Nit | CR-09 | PROPOSE FIX |
| T7-3 | Nit | CR-10 | PROPOSE FIX |
| R9-1 | Major | CR-01 | PROPOSE FIX |
| R9-2 | Major | CR-02 | PROPOSE FIX |
| R9-3 | Major | CR-04 | PROPOSE FIX |
| B9-1 | Major | CR-05 | PROPOSE FIX |
| B9-2 | Major | CR-03 | PROPOSE FIX |
| T9-1 | Nit | CR-09 | PROPOSE FIX |
| T9-2 | Nit | CR-11 | PROPOSE FIX |
| T9-3 | Nit | CR-08 | PROPOSE FIX |
| T9-4 | Nit | CR-07 | PROPOSE FIX |
| T9-5 | Nit | CR-12 | PROPOSE FIX |

No severity has been changed. Where a Major and Nit share one edit, their original labels remain here and in the raw reports; they are not counted as two separate fixes.

## Application and closure boundary

After the owner's resolution response, apply accepted text only to the target and synchronized draft, first preserving the original reviewed snapshot. Keep all original stage/raw/source reports unchanged. Record new target hash and line ranges, structural/link/JSON-example checks, accepted/contested/escalated/declined counts and any remaining issues. Mark this report resolved only after that synchronization; acceptance of a proposal alone is not evidence of an applied or independently verified fix.

Required future behavior assertions described above are additions to the plan, not code tests to run now. Document-level closure can verify the exact definitions, fixture arithmetic, complete file/phase list and scope. Do not claim the new control/binding/admission mechanisms work in Rust or that a fresh reviewer passed the changed target without actual evidence. If a re-audit is used, limit it to affected scope/lenses under the existing EGM cap rather than restarting the whole pipeline or brainstorming. No source/dependency edit, build, credential/provider access, account action, trading operation, GitHub mutation, commit or publication is authorized.

## EGM resolution choice

The installed egm-resolve Step 4 requires a human resolution response before applying PROPOSE FIX items. Available responses are: comment on this file; **auto accept** to batch-apply proposed document fixes; **continue/no changes** to leave findings open; **interactive breakdown** for item-by-item discussion; or a custom instruction. There are no escalations requiring private knowledge.


## Post-resolution synchronization

- Owner decision: **Auto-accept**; the prior issue-4 scope reaffirmation remains in force.
- Accepted: **12 consolidated fixes**, covering **17 raw findings**. Contested: **0**. Escalated: **0**. Declined: **0**. Unapplied accepted edits: **0**.
- Exact original target preserved at [reviewed snapshot](strategy-rules-design.reviewed-2026-11-13.md), original SHA-256 `3f6f8c23492a41131f56cb81e899860da5565a28519f61ac79ffbe081731c853` (664 lines; original feature/detail split 1–324 / 325–664).
- Current target SHA-256: `741a728267de62ab7bc2856672071f49b3e0169bab00f5012b53e4363aac48d7`.
- New target ranges: FeatureRange **1–324**; DetailedRange **325–755**. Target and draft are byte-identical.
- Applied controlled-contract definitions, native admission, future buildable phases, missing refresh-test allowlist entry, hold reachability, self-contained arithmetic fixture, status/provenance/claim calibration and precise walkthrough/paging wording. Public author JSON specimens are unchanged.
- Document closure checks completed as recorded below. No engine/build/packaged tests or independent re-audit have run on the changed target. Future behavior assertions in the plan remain unexecuted.


### Completed document closure checks

- Archived baseline SHA-256 matches the original reviewed 664-line document; current target and draft are byte-identical.
- Both public author JSON specimens are byte-for-byte unchanged from the reviewed version and parse as JSON. No new author field/operator/source was introduced.
- Markdown fences, heading hierarchy and all 13 tables have valid structure; current local file/fixture-anchor links resolve.
- Every CR-01–CR-12 accepted mechanism/text repair is present; obsolete API, stale drafting instructions, ambiguous page-limit notation and unsupported instrument enumeration are absent.
- Phase source paths are all in the complete file allowlist, including the specifically affected refresh harness; no dependency or unrelated source path was added.
- Standalone standard-library rational calculations reproduce all four normalized fixture outputs and their average as 0.001 percent. Simple interval arithmetic confirms the exclusive-end 900-second/nested-hold cases while retaining long historical lookbacks. These are document/math checks, not Metten engine behavior tests.
- The application batch verified 201 other observed workspace files unchanged; subsequent synchronization wrote only the same target/draft/report paths. Original stage/raw/source reports were preserved.

**Resolution handoff:** 12 accepted consolidated fixes / 17 raw findings addressed in text; 0 contested, 0 escalated, 0 declined, 0 accepted edits left unapplied. No independent reviewer has passed this changed target. Controlled engine tests, packaged checks, implementation correctness, source feasibility and measured responsiveness remain unperformed evidence; textual resolution does not change their status. The changed control/binding/admission/reachability definitions are the scope for any required affected re-audit, not a reason to restart the whole pipeline. No source implementation, code test/build, provider/account/credential operation, trading action or publication ran.


## Focused follow-up correction — 2026-11-13

Connor subsequently authorized the four bounded corrections from the focused recheck. All four are applied and synchronized; see [focused resolution](focused_resolution_report.md) for exact dispositions, checks and current target SHA-256 `4f39df3e732b27d79a43bc3dc95048cf5368337634c15bcd71e454bb149dd8e5`. Original findings and review records above remain historical evidence. No further independent review, implementation or application checks ran.
