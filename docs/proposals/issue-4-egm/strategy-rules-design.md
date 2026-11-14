# Import, explain and preserve supported strategy rules

* **Author:** Connor Moulton
* **Status:** Document corrections complete — the focused independent recheck and four owner-authorized follow-up corrections are recorded in [focused resolution](focused_resolution_report.md). Targeted document checks passed; the final corrections have not received another independent review. Implementation and publication remain unauthorized.
* **Date:** 2026-11-13
* **Scope:** GitHub issue #4, using the [approved concept brief](concepts_brief.md), [product specification](../../../METTEN_DESIGN_DRAFT_2026-11-12.md), [adopted owner decisions](../issue-4-egm-handoff.md) and [ADR 0001](../../adr/0001-rust-engine-and-tauri-boundaries.md).

## Overview

Metten's intended experience is upload a JSON file into the application, choose a saved strategy version, configure the run and press Go. The file describes supported purchase conditions. Metten provides the shared engine for data connections, purchasing, safeguards, accounting and recovery. Authors can combine supported rules without maintaining a separate bot or changing that engine for each strategy.

Issue #4 establishes this separation through the strategy contract and local import experience: validate a file, preview its generated explanation, save a version, inspect saved versions and edit a copy. It also defines and demonstrates how supported rules behave using controlled observations, time and validity inputs. Later issues connect that foundation to actual data sources and trading.

Creating the JSON must be understandable too. The repository will include a complete agent authoring guide and a plain-English human guide, readable directly on GitHub, backed by the same field reference and worked example. Together they explain how to turn an idea into supported rules, check the file and understand its limitations. These documents also provide the foundation for a possible future strategy-authoring helper or skill.

The generated explanation must describe the same rules the engine evaluates. Saving or editing creates an immutable version: earlier records keep their original meaning. A valid import confirms supported rules; it does not confirm provider access, authorize trading or establish profitability.

The long-term product goal includes all Kalshi 15-minute markets, not just Bitcoin. Reusable rule names and evaluation logic should therefore describe the underlying asset or contract; Bitcoin is the initial supported instrument, not the definition of the strategy language. Future use on another market must establish explicit compatible input bindings and market rules. This keeps the accepted BTC-first release scope while preserving the intended reuse of strategy logic.

## The Problem

Connor's experience maintaining Kalshi-Bot motivates this project: strategy logic and the runtime became difficult to maintain together. Metten must make the boundary enforceable. New combinations of supported rules belong in JSON; connections, order handling, limits and recovery remain shared application responsibilities.

A file importer alone cannot establish that boundary. A narrow example that needs engine changes for each new combination would leave the underlying maintenance problem unresolved. Separate implementations for evaluation, explanations or provider-specific strategies could also disagree about what a file means. Application updates could silently change earlier results if they reinterpret saved rules.

The design must therefore define a coherent supported rule language, preserve its meanings, and make acceptance, rejection and waiting reasons understandable. Flexibility still requires justified limits on resource use. Missing information must block the conditions that depend on it without erasing remembered evidence or blocking independent alternatives.

An exhaustive field list alone is insufficient. People need a learning path, and agents need enough explicit instruction to create valid files without relying on the owner's private conversation or inventing unsupported capabilities. Human and agent guidance must teach the same contract so they do not become competing explanations of the product.

## Goals

- **Direct local import and understandable review.** Let a user select JSON in the application, receive actionable validation errors or a generated preview, and save a supported version. Preserve the correction path when validation fails. Show saved versions and create a new version when editing a copy. Require no terminal commands or external explanation service for this flow.
- **A reusable supported rule contract.** Define explicit sources and price types, comparisons and Boolean combinations, price differences and changes, crossings, rolling averages/minimums/maximums/ranges, author-selected durations and distinct-update confirmations. Include multiple checkpoints, remembered same/opposite sides, and ordered nested purchase alternatives. Preserve the named-source decisions and all capability families in the approved brief. New inputs or operators require an explicit capability extension.
- **One consistent meaning.** Evaluation, generated explanations, the complete author/agent field reference and the learning example must agree. Define exact units, arithmetic, boundaries, timing and observation requirements in Detailed Implementation items 1–3. Unsupported or excessive rules must fail clearly rather than be ignored or silently changed.
- **A complete repository guide for agents.** Provide a clearly discoverable document that explains how to translate an idea into strategy JSON: identify supported sources and measurements, choose and combine conditions, define checkpoints and purchase priority, account for timing and gaps, validate and repair the file, and inspect its generated explanation. Include all supported fields through an exact linked reference covering types, units, defaults, required/optional fields, valid combinations, limits and version meanings. State what belongs in run settings, what cannot be expressed, and when clarification or a capability extension is needed. Never invent fields, substitute sources or treat rule validity as trading readiness. An agent must be able to follow the guide without this conversation.
- **Understandable documentation for people.** Provide an accessible entry page and step-by-step authoring guide, readable in the GitHub repository as the initial wiki-like documentation. Explain JSON and Metten terms before using them, build up the worked learning example in small steps, and show importing, reading the preview, saving, choosing a saved version and editing a copy. Explain startup waiting, unavailable evidence, common validation errors and the separation of author rules from user settings. Link to the complete reference for detail. Distinguish the import experience delivered by #4 from running paper or live strategies delivered later. Plan a walkthrough using the guide to create and validate a file, alongside checks that its example matches the supported contract.
- **Trustworthy evidence and lifecycle behavior.** Preserve market-local checkpoints, continuous applicable underlying history, contract identity, distinct observations and complete required historical coverage. Distinguish verified quiet periods from genuine gaps. Initial collection begins at Go, waits for every required history period and then the next full market; recovery within an active market follows its separately agreed rules.
- **Predictable purchase eligibility.** Define priority and retry eligibility within one original quantity objective per market. Confirmed fills lock the side; confirmed zero-fill outcomes allow reconsideration; unknown outcomes block another attempt. User quantity, permitted prices and common safeguards remain engine/run responsibilities. Demonstrate these restrictions with controlled inputs without implementing exchange submission here.
- **Preserved versions and existing records.** Saving changed rules must not alter earlier versions, explanations or history. Define safe handling of older supported and unsupported formats and workspace changes. Active runs do not switch strategy versions.
- **Evidence tied to this issue.** Specify meaningful public-engine behavior checks for all twelve adopted timelines, explanation/error checks, version preservation and focused desktop import/storage checks. Provide one worked learning example without claiming profitability or reproducing historical strategies. Planning examples become executed evidence only after implementation is separately authorized.

## Non-Goals

- Actual provider connections, entitlement/licensing checks, production freshness thresholds, paper fills, order submission, settlement, recovery integration or live validation. These remain later tracked work in the [implementation plan](../../implementation-plan.md).
- Multiple independent purchase objectives, both-side purchases, per-option quantities, selling, exits, hedging or resting orders. Gen 1 retains IOC purchases held through settlement.
- Arbitrary executable uploads, a visual strategy builder, strategy-specific provider branches, an AI explanation service, remote monitoring or new infrastructure.
- A separately published wiki, hosted documentation service or packaged strategy-authoring skill in this issue. The repository guides are required deliverables and the reusable foundation for a helper; a dedicated skill remains a possible follow-up, not a prerequisite to creating JSON or using Metten.
- Confidence models, volatility, crossing counts, richer volume/depth indicators, or historical D/E/F strategy definitions and thresholds.
- Replacing unfinished source changes with assumed requirements, rewriting unrelated work, or treating design approval as permission to publish, change GitHub or implement.

Human and agent documentation must teach strategy JSON without relying on this conversation. Detailed Implementation items 5, 7 and 8 assign its paths, responsibilities and checks. A packaged helper/skill remains outside issue #4.

## Open Questions and Evidence Still Needed

The approved brief was reconciled without an unresolved owner decision. Formal review identified the documented planning gaps; this resolution applies their accepted text fixes. Section approval and textual resolution do not establish technical readiness. The outstanding evidence is:

- Whether actual adapters can supply the named inputs, compatible precision, normalized distinct observations, trustworthy quiet coverage and recovery behavior. Access, licensing/cost, reference definitions and production timing profiles remain later feed acceptance gates.
- Whether the implemented allocation plan and bounded processing keep the intended workload responsive on supported Macs. The byte calculations are design checks; actual engine/run capacity is unmeasured.
- Whether the public engine, migration and native import flow implement the specified meanings, preserve previous records and fail truthfully under faults. No implementation/package result follows from this design.
- Whether people and agents can author and understand files using only the guides, reference and example. The required independent walkthrough remains unperformed.

## Critical User Journeys

These restate approved behavior so reviewers can assess the document without the private conversation. They are proposed acceptance journeys, not executed results.

### Import, understand and repair a file

- **Given:** the local application and a user-selected JSON file, without provider/account access.
- **When:** the user imports and previews it.
- **Then:** invalid rules produce bounded field-specific repair guidance and preserve editable text; valid rules produce the engine-generated explanation and requirements without saving. Source access remains explicitly unverified.

### Save, inspect and edit an immutable version

- **Given:** a successful preview of the current exact editor text.
- **When:** the user saves, reopens and edits a copy.
- **Then:** saving commits a complete new version; earlier JSON, explanation/history and active-version identity stay intact. Changing text requires another preview. A list refresh failure cannot negate a confirmed save, and legacy versions remain inspectable with honest execution restrictions.

### Express and understand composed measurements

- **Given:** a human or agent using the repository guides, exact reference and explicit supported source bindings.
- **When:** the author defines a five-minute average and its one-minute change or another average.
- **Then:** the same named measurement is reused with defined precision, cumulative six-minute history, complete gap handling and a preflight workload result. The author supplies no scheduler/storage code and receives no implied provider or trading approval.

### Preserve observed meaning through gaps and priority changes

- **Given:** the public controlled engine, a fixed version, identified market, controlled observations/coverage and objective state.
- **When:** checkpoints latch, a source disappears, options change priority or controlled outcomes report zero/partial/unknown fill.
- **Then:** established checkpoints persist until market reset; only dependent current conditions become unavailable; ordered options obey one original quantity and filled-side lock. No missing observation, crossing or fill is invented. These are controlled eligibility checks; connected purchasing and live acceptance remain later work.

## Technical Plan (Architecture)

### One rule engine, with separate responsibilities

Keep the accepted Rust/Tauri boundary and the existing local application. Rust owns supported rule meanings, validation, evaluation, generated explanations and version storage. The interface sends bounded requests and displays engine results. Trusted adapters will eventually supply named-source evidence and execution outcomes; they must not know strategy names. No network service, separate strategy process, executable plugin or second evaluator is introduced.

```text
Local JSON file or edited copy
         |
         v
Interface -- preview request --> Rust validation --> validated rules
         |                              |                 |
         |<-- errors or explanation ----+                 |
         |                                                |
         +-- save exact reviewed text --> revalidate --> SQLite version
                                                          |
Controlled evidence/time/outcomes --> shared evaluator <--+
                                          |
                            eligibility + specific reasons
                                          |
                         future execution/limits/recovery

Human guide + agent guide --> shared rule reference + checked example
```

The diagram's execution boundary is deliberately later work. A qualifying result is permission to consider an attempt under all remaining engine safeguards, not proof of a submission or fill. Existing Start/Pause controls still operate the non-trading foundation. This issue must not relabel them as running a strategy.

### Proposed rule document and validation

The owner requires the JSON itself to be intuitive: a reader should understand what its variables mean and follow a simple strategy without repeatedly decoding names. Documentation supplies precise definitions; it must not compensate for obscure public names. This applies to field names, operation names, author-chosen identifiers in examples, nesting and references, not merely to the generated explanation.

Use this naming and readability standard when fixing the grammar:

- Use descriptive English names consistently, with lowercase words separated by underscores. Prefer `less_than` over `lt`, and `lookback_seconds` over an unexplained `n` or `window`. These illustrate the naming standard; they are not a complete schema.
- Make meaning and units clear together. A price must identify whether it is a buyable ask, bid, last trade or an explicitly named underlying/reference value. Time must identify seconds and its reference point; percentages must be distinguishable from dollar or contract-price differences. Do not rely on an author knowing a programmer's abbreviation.
- Use the same terms for the same concept throughout the file, preview, errors and guides. Keep necessary official source names such as BRTI, with their meaning explained at their first appearance in the guides. Explicit source identity remains required.
- Show the simplest supported strategy simply. Do not require empty checkpoint sections, unnecessary groups, dummy measurements or an alias for every constant. Add nesting to express actual condition/priority structure, and named reusable measurements when they clarify the strategy or avoid repetition.
- In shipped examples and agent instructions, use identifiers that explain the fact being represented, such as `average_underlying_price` or `early_yes_checkpoint`, with source, instrument and lookback explicitly defined nearby. Reserve asset-specific names for intentionally asset-specific inputs, such as an explicitly chosen Bitcoin comparison signal. Avoid `x`, `m1` or strategy-letter shorthand. Author identifiers remain identifiers, not executable instructions or a new natural-language parser.
- Read purchase options in their priority order, with conditions and the resulting side visibly connected. A reader should be able to explain what is checked, what is remembered, when purchasing may qualify and which alternative comes next without tracing through unnecessary references.

Before finalizing the field reference, show the complete learning JSON alongside its plain-English meaning and conduct a readability walkthrough. Check that a reader can explain its variables, units, timing, checkpoint and purchase order from the file; consult the reference for precise edge behavior. If the file needs continual translation, revise the public names or structure. The agent guide must teach these conventions and use the same example. This is a usability check, not a restriction on the agreed authoring capabilities.

Reserve a new explicit format revision for the agreed contract. The unfinished format 1 remains a legacy draft; its narrow grammar does not define the new one. Every supported format has fixed operator meanings. Changing those meanings requires another format revision, rather than silently changing previously saved files.

The rule document uses the following logical sections. Detailed Implementation item 1 enumerates the external fields and tagged variants.

| Section | Responsibility |
| --- | --- |
| `format`, `name` | Identify fixed rule meanings and the user-facing strategy name. Names never select executable behavior. |
| `inputs` | Bind author-chosen identifiers to explicit source, instrument, price field and unit. Contract fields identify the evaluated contract/side; underlying values remain scalar. |
| `measurements` | Name derived values such as a difference, change or rolling average. References form a finite, acyclic dependency graph. |
| `checkpoints` | Name market-local latched conditions, permitted intervals and any side-selection rule. The section may be empty. |
| `purchase_options` | An ordered tree of conditions and purchase choices. Evaluate depth-first in author order; a group gates its descendants. Choose the first currently eligible leaf. |

There are no quantity, account credentials, arbitrary URLs, executable expressions or user purchase-price settings in this document. Author price conditions remain rules and must also satisfy the user's permitted prices. Checkpoints are optional; a current-price purchase must not require invented Bitcoin direction or a checkpoint merely to fit the grammar.

Inputs explicitly distinguish BRTI, Coinbase BTC/USD and Kraken BTC/USD, as well as supported Kalshi contract/reference fields. Declaring a source is an identity requirement, not a claim that an adapter, entitlement or particular field is available. Unsupported field/source/instrument combinations fail validation. A later adapter must establish capability before a run can use that input; another feed cannot silently substitute.

Keep source/instrument binding separate from reusable measurement names. For example, `average_underlying_price` refers to the input explicitly declared for that strategy, rather than always meaning Bitcoin or whatever feed happens to be available. Its operations can remain unchanged when a future supported market supplies compatible inputs. A generic name must not conceal the bound asset, provider, quote currency, physical quantity/unit, price meaning or lookback in the JSON and preview. Dollar prices alone do not establish compatible instruments. BRTI remains a specifically identified Bitcoin source, not a generic name for every future market's reference price.

The future target is reuse of one strategy's rules across supported 15-minute markets. Exact same-file binding to multiple assets is not implemented by choosing a generic variable name: the later market-support work must define explicit compatible bindings, preserve the selected run/version meaning, and check market-specific timing, reference/settlement definitions and source capability. Issue #4 does not add an automatic asset/source substitution system or require future providers now. The agent and human guides must distinguish reusable logic from these actual compatibility requirements.

Validation checks byte and nesting bounds before constructing the supported rule structure, rejects duplicate object keys and unknown fields, checks unique identifiers and references, rejects cycles, validates compatible units and meaningful intervals, and accounts for required retained state. Errors contain a field path, a finite reason and an accepted form or corrective action. Error messages must not echo entire uploaded documents or expose secrets accidentally supplied in them. Preserve editable text locally for correction; never interpret it as HTML.

Use the installed Rust serialization and SQLite dependencies; no schema framework or expression-language dependency is proposed. The Rust parser and semantic validator remain authoritative. The field reference documents that contract; documentation checks cannot replace runtime validation. Resource limits are part of the published format support profile and need an explicit workload rationale, including long underlying lookbacks. File size alone is insufficient. Detailed Implementation item 6 specifies the proposed support profile; actual allocation/responsiveness evidence remains required. These limits do not inherit old strategy thresholds or waive approved capabilities.

### Measurement and condition meanings

Keep raw input identity separate from derived values. A measurement declares its operands, operation, units and any lookback. Derived measurements can refer to earlier named inputs or measurements after validation establishes a dependency order. Both operands of a comparison are explicit compatible measurements or typed constants; no generic unqualified price is allowed.

| Capability | Proposed meaning |
| --- | --- |
| Signed / absolute difference | Left minus right, or the absolute value of that result, in compatible units. |
| Percentage difference | One hundred times the signed difference divided by an explicitly named reference operand. Zero denominator makes the value unavailable; it does not produce zero or infinity. |
| Change over time | Current value minus the value at the declared lookback boundary; percentage change uses the prior value as its reference. Required boundary evidence must exist. |
| Rolling average / minimum / maximum / range | Operate on complete value coverage over the declared trailing time window. Average is time-weighted over verified value intervals; range is maximum minus minimum. Verified unchanged values cover time without inventing new updates. |
| Current comparison | `<`, `<=`, `==`, `>=` or `>` over compatible values. Strict comparisons exclude equality. |
| Crossing | Below requires observed prior `left >= right`, followed by observed `left < right`; crossing above is symmetric. No crossing is inferred from the first observation, a timer, a duplicate or a gap. |
| Duration | A condition must remain true with continuous required evidence for the selected elapsed time. False or unavailable resets an unfinished duration. A timer may complete it without a new price if current evidence remains valid. |
| Distinct-update confirmation | The author names the input whose distinct observations count and the required count. Rechecks and duplicates add nothing. Each counted observation re-evaluates the complete condition; false or unavailable breaks the confirming sequence. Identical values may count when observation identity is genuinely distinct. |

Detailed Implementation item 2 defines the boundary/numeric examples; the future shared reference must reproduce them. Actual adapter capability remains subject to the later source-acceptance evidence.

Boolean combinations use true, false and unavailable. For `all`, a known false makes the result false; otherwise missing required evidence makes it unavailable. For `any`, a known true makes the result true; otherwise missing evidence makes it unavailable. Track the evidence supporting the resulting truth so that an independently sufficient branch does not acquire unnecessary dependencies. Source problems cannot be hidden by stale retained numbers.

All measurements use six decimal places and a complete one-millisecond calculation clock, with normalization at each named operation; Detailed Implementation item 2 defines the arithmetic. Use checked decimal operations, not floating-point equality or unchecked rounding. Complete time weighting, dimensional compatibility and all required coverage remain mandatory. Standalone design-oracle checks are recorded in the [targeted evidence](detailed-implementation-adversarial-review.md); engine verification remains required.

### Evidence, state lifetimes and ordering

Replace the draft's complete-global-observation assumption with an engine input contract that carries source/instrument identity, observation identity, effective event time, validated receipt time, and explicit current-validity and historical-coverage evidence. Controlled inputs supply these claims for #4. Later trusted adapters must substantiate them; JSON cannot assert freshness, manufacture observations or configure away a safety requirement.

An accepted value change and evidence that a value is still current are separate events. A verified quiet interval may extend coverage but never creates another confirmation or a crossing. A current snapshot may restore present state while leaving an earlier history hole unresolved. New input from another stream cannot repair that hole. Invalid, duplicate, replayed or out-of-order evidence cannot advance temporal rules; inconsistent evidence makes affected requirements unavailable with a reason. Distinguish a provider continuity gap from local decision sampling.

The evaluator advances in an authoritative sequence:

1. Validate and accept a controlled input batch, validity transition, market boundary, outcome transition or timer event. Preserve all decision-relevant observations in their established order; a batch is not permission to coalesce an intervening failure away.
2. Apply market resets and evidence changes. Recompute all affected derived values in dependency order before evaluating conditions. Two results due from the same accepted event are both new before dependent evaluation; equal timestamps on independent arriving feeds do not prove simultaneous delivery.
3. Advance temporal conditions and establish newly observed checkpoints. A checkpoint used only as a Boolean can latch without a side. Side-dependent checkpoints evaluate the declared side rule; if both sides qualify without an explicit choice, report ambiguity and do not invent a remembered side. Authors can choose a fixed side or an unambiguous selector. Side references must target a checkpoint that can establish a side.
4. Evaluate ordered purchase options using the newly established checkpoints. Unless a rule explicitly requires a later distinct observation, same-event checkpoint establishment can authorize eligibility. Resolve unavailable branches locally and continue to independently eligible alternatives.
5. Select the first eligible leaf that also fits shared objective restrictions supplied by the engine: remaining original quantity, filled-side lock and unresolved-attempt block. A preferred opposite-side leaf after a fill cannot prevent a lower-priority same-side leaf from completing the objective. An unresolved attempt blocks every leaf. Return a candidate and reasons; no exchange request is made here.

| State lifetime | What survives and what resets |
| --- | --- |
| Strategy version | Fixed rules, meaning and generated explanation; edits create another version. |
| Run | Fixed version/settings, Go boundary and applicable continuous underlying history. No pre-Go prefill or active version switching. |
| Market / contract | Checkpoints, remembered sides and objective reset per entering market. Historical cells are keyed to exact registered market/contract: preserve the entering contract's own pre-open history; never transfer the old book under the new identity. |
| Condition evidence | Crossings, confirmation sequences, duration starts and rolling coverage change only according to their own requirements. Completed checkpoints outlive temporary loss of their establishment source. |
| Attempt | A prior attempt must be confirmed finished before another can qualify. Any fill fixes the objective's side; only the remaining original quantity may proceed. |

Collect required initial history from Go across all alternatives, then enter only the next full market. Display the required period and progress, followed by the agreed ready/waiting label. Reaching the collection duration alone cannot falsely claim complete coverage. New-contract measurements require that exact contract's evidence. Entry retains its own admitted pre-open history, resets market-local conditions/checkpoints/objective, and neither manufactures history nor erases usable underlying history. Once a market is active, eligible conditions can recover within it without another startup wait. Crash/restart remains stopped pending separate recovery and the user's next Go.

Document history eligibility by identified instrument and collection scope. Reject only demonstrably impossible requirements; never infer contract listing lifetime from the 15-minute outcome window or stitch contract books together. Production source timing limits and capabilities remain later gates.

### Public engine and desktop requests

Keep the pure rules interface within `crates/engine/src/strategy.rs`, exposed through the engine crate. It accepts a validated document and controlled evidence/time/outcome inputs, and returns checkpoint state, per-option eligibility, selected candidate and compact structured reasons. Detailed Implementation specifies the controlled public types and their responsibilities. It receives no Tauri objects, credentials or network clients. Existing controlled session behavior remains a separate test facility.

Evolve the existing strategy command into explicitly distinguished preview, save and list operations using a typed request. Preview returns the engine-generated explanation and requirements without writing a version. Save sends the exact reviewed text and revalidates it in Rust; it cannot accept a frontend claim of validity. The interface disables save after an edit until the new preview succeeds, and displays success only after a confirmed committed result. Listing and editing a copy use stored version identity; saving another version never changes an active run's selection. A list failure after a successful commit must report the committed version identity rather than falsely claim nothing was saved.

Native requests stay bounded and run storage work away from the interface thread, following the existing pattern. The interface cannot supply a database path, raw SQL, shell command or provider destination. Engine validation applies even if frontend checks are bypassed. Reuse the existing permission for the strategy command; this design adds no broad filesystem, shell or network permission. File selection reads only the user-selected file through the existing local control.

### Version storage and safe compatibility

Reuse append-only SQLite version storage and its existing workspace lock and durable writer settings. New rows retain the reviewed JSON, fixed format identity, engine-generated explanation snapshot and recording time. Validation/explanation precede the write; the save transaction inserts a complete version or none. Unknown or unsupported earlier rows remain independently inspectable and cannot prevent supported rows from listing.

Extend storage atomically with nullable metadata for legacy rows and a new database revision. Preserve all existing strategy and session records; do not backfill by rewriting immutable rows. New versions require complete metadata. Existing older applications already refuse database revisions newer than they understand; increase that revision when the new contents require it, even when SQLite could otherwise open the tables.

Legacy format 1 is read-only history, not an approved new trading format. Preserve any existing stored explanation; when one was never stored, show available original JSON and identify that absence rather than claim a reconstructed explanation proves historical meaning. Unsupported rows remain viewable, with a clear reason they cannot execute. Editing a legacy file requires an explicit valid new version, not silent conversion. Define the exact migration and mixed-version response shapes in Detailed Implementation. No general downgrade framework or access to a private trading workspace is needed.

### Human and agent documentation

Use repository Markdown, readable directly on GitHub, with one shared field/semantics reference. Proposed responsibilities are:

| Proposed path | Reader and purpose |
| --- | --- |
| `docs/strategies/README.md` | Human entry page: what a strategy file does, how to choose a reading path, and what currently works. |
| `docs/strategies/authoring.md` | Plain-English tutorial building the learning example in steps; import, preview, save, choose a version, edit a copy and troubleshoot. Explain future run steps with honest availability labels. |
| `docs/strategies/reference.md` | Complete exact field, source, unit, operation, timing, priority, validity, limit and version contract; link each concept to the worked example or a controlled boundary example. |
| `docs/strategies/agent-authoring.md` | Self-contained task recipe for agents: map intent to supported capabilities, ask only consequential missing questions, produce JSON, validate/repair it and explain limitations. Link the full reference rather than duplicate its grammar. |
| `examples/learning.json` | One validated learning strategy shared by both guides; no profitability or historical D/E/F claim. |
| `README.md` | Discoverable links to human documentation and the agent authoring entry point. |

The documentation must explain the separation of author purchase conditions from run settings and engine safeguards. Agent guidance must prohibit invented inputs, secret-bearing JSON, source substitution and claims that validation proves trading readiness. Both guides use the same checked example and generated explanation. A future helper or skill may consume these guides without becoming required runtime machinery.

Verification will include a documentation-led authoring exercise using only the guides and a stated example intent, followed by actual engine validation and explanation checks after implementation. Exact expected assertions and the review procedure belong in Detailed Implementation. Broken links, unsupported example fields or inconsistent rule meanings are failures; merely creating documents is not sufficient.

### Existing files affected and evidence still needed

The main changes reach `crates/engine/src/strategy.rs` (rules, evaluation and explanations), `lib.rs` (public version/preview operations), `storage.rs` (version metadata and compatibility), `src-tauri/src/main.rs` (typed native operations), `src/contract.ts` and `src/main.ts` (requests and display), and `index.html`/`src/style.css` (preview/save/version experience). Existing engine and interface tests, the learning JSON and the proposed documentation must change with them. Detailed Implementation item 5 enumerates the complete Create/Modify/generated-output allowlist; this section is not permission to edit any of those files.

Detailed Implementation now specifies the grammar/types, side selectors and ties, numeric representation/precision, temporal boundaries, history calculation/coverage, normalized observation/batch identity, resource bounds, response types and migration definitions. These are section-approved drafting proposals with the accepted formal-review document fixes applied, not proven implementation. Existing test/build tools are reused. Standalone design arithmetic/document checks are recorded in the targeted review record; historical passing tests do not validate the unfinished rule changes. The changed definitions retain their affected closure/review requirements; publication and implementation gates remain separate.

## Alternatives Considered

These alternatives come from the approved concept brief and the subsequent documentation/readability clarifications. Owner decisions are identified explicitly; other dispositions are engineering choices carried through the approved Technical Plan. None is attributed to an upstream maintainer. Approval of a direction does not establish that its implementation works.

### Limit strategies to the existing learning example, or implement each strategy separately

- **Benefit:** the current narrow draft could ship sooner; individual strategy code could express behavior without designing reusable rules.
- **Problem:** limiting the product to that example excludes agreed capabilities. Separate strategy runtimes duplicate execution, limits and recovery and recreate the maintenance problem motivating Metten.
- **Disposition:** rejected by author: supported strategies must be combinations of documented JSON rules, with Metten providing the shared runtime. Reuse the existing import/storage boundaries while replacing restrictions that conflict with the adopted capabilities. Arbitrary executable uploads remain outside scope.
- **Cost of the selected approach:** the rule contract and shared evaluator need precise definitions and combination tests. A genuinely new input or operation still needs an explicit engine capability extension.

### Short variable names and a general expression language

- **Benefit:** abbreviated names reduce file length; executable-looking expressions could make complex calculations concise for programmers.
- **Problem:** opaque abbreviations, implicit units and expression syntax make simple strategies harder to read. A general expression language adds parsing, safety and documentation work beyond the supported operations.
- **Disposition:** rejected by author: public variables and strategy JSON must be intuitive and simple to read. Use descriptive English names, explicit meaning/units and only necessary structure. The selected design uses supported data and operations rather than uploaded code or a new expression language.
- **Cost of the selected approach:** some names are longer, and the supported contract cannot express every imaginable calculation. The complete example must demonstrate readability; naming conventions alone do not prove it.

### Hard-code Bitcoin into reusable rules, or build universal market support now

- **Benefit:** Bitcoin-specific names can appear concrete in the first example; full market support now could provide immediate reuse across instruments.
- **Problem:** Bitcoin-specific public concepts would make a reusable strategy appear tied to one asset and invite strategy-specific engine changes. Implementing every market's data, reference, schedule and execution compatibility now exceeds the agreed BTC-first issue and lacks the necessary evidence.
- **Disposition:** use descriptive asset-neutral rule names such as `average_underlying_price` with explicitly identified source/instrument bindings. Preserve intentionally asset-specific signals as explicit choices. Record all Kalshi 15-minute markets as the eventual support goal while implementing only the approved initial scope. This carries Connor's portability clarification into the existing shared-runtime design.
- **Cost of the selected approach:** generic names do not prove cross-market compatibility or make the exact same file run unchanged everywhere. Later market-support work must establish compatible bindings and market rules; it must never silently treat a different asset/feed as the original one.

### One global source-health gate or a latest-value-only evaluator

- **Benefit:** one health flag and one current value per input would simplify state and evaluation.
- **Problem:** global blocking prevents independent alternatives from qualifying and can make a completed checkpoint depend unnecessarily on its former source. Retaining only the latest sampled value can erase an intervening failure, crossing or qualifying observation.
- **Disposition:** do not adopt, because these shortcuts conflict with adopted decisions B/E and controlled examples 1, 3–5 and 7 in the handoff. Keep dependency-specific validity/coverage and preserve decision-relevant evidence in established order.
- **Cost of the selected approach:** the engine needs explicit evidence/state lifetimes and coherent ordering. Bounded processing must report unavailable evidence when overloaded; it cannot silently drop required observations.

### Reinterpret saved JSON with the newest engine, or keep every old evaluator executable

- **Benefit:** reinterpretation avoids compatibility code; keeping every evaluator would maximize execution compatibility.
- **Problem:** reinterpretation can change historical meanings or explanations despite unchanged files. Preserving every unfinished evaluator indefinitely would create an unjustified maintenance commitment and could imply support never approved for the draft.
- **Disposition:** preserve original rules, fixed format meanings and stored explanations. Keep unsupported/legacy rows inspectable with an explicit execution restriction; conversion creates a new version. This follows the approved version-preservation direction without promising indefinite execution support.
- **Cost of the selected approach:** format identity and historical inspection need explicit handling. A legacy explanation that was never stored cannot be claimed as recovered historical truth. Compatibility checks must cover mixed supported and unsupported records.

### Unlimited workloads or inherited arbitrary caps

- **Benefit:** unrestricted rules maximize syntactic flexibility; fixed inherited caps are quick to enforce.
- **Problem:** a small document can require large retained histories or expensive evaluation. Old limits could silently exclude agreed capabilities, including hour-long underlying history, without evidence.
- **Disposition:** justify and publish limits on the actual supported workload. Preserve the agreed capability families and reject excess with actionable errors. Do not impose the old proposal's market-length history restriction or assume a contract's listing lifetime from the outcome-window length.
- **Cost of the selected approach:** resource accounting and concrete supported bounds are specified in Detailed Implementation item 6; verifying the actual allocation remains required. Later measured run capacity remains separately tracked; this design cannot promise unlimited duration, complexity or concurrency.

### Continuous raw-feed recording, remote tracing or an AI explanation service

- **Benefit:** full feeds could aid replay and debugging; external tools could provide rich tracing or flexible prose.
- **Problem:** these add retention, privacy, cost and service dependencies. AI-generated explanations could disagree with evaluation, and full raw-feed recording conflicts with the approved storage policy.
- **Disposition:** use engine-generated explanations, compact local decision transitions and bounded diagnostics. Preserve necessary in-memory history/evidence without promising continuous durable feed replay. This follows the local-only and deterministic-explanation baseline.
- **Cost of the selected approach:** diagnostics must retain the relevant decision context deliberately. They cannot promise exact full-market replay or replace durable financial recovery records in later work.

### A general workspace downgrade framework

- **Benefit:** users could open newer data with more older application versions.
- **Problem:** reversible transformations would add migration paths and could erase new meaning that an older application cannot represent.
- **Disposition:** use atomic preservation and an explicit compatibility policy. An incompatible older application may refuse the newer workspace with an actionable explanation. A general downgrade system is unnecessary for issue #4.
- **Cost of the selected approach:** returning to an older application is not guaranteed. Interrupted migration and refusal behavior must be checked against disposable workspaces; private workspace contents must not be used as fixtures.

### Start an easy option during initial collection, or repeat startup waiting after recovery

- **Benefit:** an easy current-price alternative could begin sooner; a universal startup gate could simplify recovery handling.
- **Problem:** early evaluation could enter a market unexpectedly when one timer completes, while repeating startup waiting could unnecessarily abandon an already-active market.
- **Disposition:** rejected by author: collect all required initial history, then start at the next full market. Active-market recovery retains its separately adopted behavior and common safeguards. No history is imported from before Go, and no missing path is inferred from a current snapshot.
- **Cost of the selected approach:** startup deliberately delays purchasing and may miss markets. The engine must distinguish initial readiness, market-specific evidence and later recovery.

### Separate wiki copies, a visual builder or a required authoring skill

- **Benefit:** a standalone wiki could offer another entry point; a builder or skill could help create files.
- **Problem:** independent reference copies can drift, and new tooling introduces work before the rule contract is defined. Requiring a helper would make JSON authoring depend on an additional product.
- **Disposition:** deliver the required human and agent repository guides, one shared field reference and one checked learning file, with discoverable GitHub links. A separately published wiki, builder or packaged skill is not required for this issue. The owner identified a skill as a possible later helper, not a replacement for understandable files and documentation.
- **Cost of the selected approach:** both guides still require maintenance and a documentation-led walkthrough. Repository Markdown offers fewer interactive conveniences than a dedicated tool. A later helper must consume the same contract and cannot invent capabilities.

### Public evidence for the portability clarification

Kalshi's [15-minute market guide](https://kalshi.com/pro/help/fifteen-minute-markets) describes selecting among active 15-minute series and displaying underlying data where available. The captured [market-list page](https://kalshi.com/tag/15-min-markets) is a dynamic shell and does not independently verify the available instruments. These public sources support the broader-market context, not Metten adapter capability, entitlement, reference/settlement compatibility or cross-market qualification. The owner’s eventual compatible-market goal remains; issue #4 remains BTC-first.

Drafting sections are approved. Formal EGM review is complete; this resolution revision applies its accepted document fixes. The final handoff records approval chronology and remaining verification limits.

*Drafted with EGM. Current resolution status, prior approvals and verification limits are recorded in the final handoff.*


## Detailed Implementation

> **Issue #4 resolution revision.** Accepted fixes from the [annotated report](goldfish_review_report.md) are incorporated below.

This section specifies the proposed issue-4 changes and their verification. Approval and implementation authority are recorded in the final handoff; no implementation is authorized in this session.

### 1. Freeze the file contract and its vocabulary

Use integer `format: 2`, meaning `metten_rules_2`. Format 1 is the unfinished historical draft. All objects reject unknown and duplicate keys, including duplicates inside tagged variants. No comments, trailing commas, null values, numeric coercion or author-written explanation field. UTF-8 JSON only. Object key order has no meaning; array order matters for purchase priority and diagnostics. No source name or strategy name selects executable strategy code.

Top-level fields are exactly:

| Field | Required form and meaning |
| --- | --- |
| `format` | Required JSON integer 2. |
| `name` | Required nonempty display string, at most 120 Unicode scalar values, no control characters. It never changes evaluation. |
| `inputs` | Required array of 1–32 input definitions. |
| `measurements` | Optional array, default empty, at most 64 named calculations. |
| `checkpoints` | Optional array, default empty, at most 32 checkpoint definitions. |
| `purchase_options` | Required nonempty ordered array of option nodes; at least one purchase leaf overall. |

Identifiers use `[a-z][a-z0-9_]{0,63}`. Input, measurement and checkpoint ids share one namespace; option ids are unique in their own namespace. Names are case-sensitive. Forward references are allowed; unknown references and dependency cycles are rejected with the reference path. Checkpoint dependency order follows references, not author array order. No checkpoint may depend on a purchase result. Nodes, references and collection arrays must obey the support profile in item 6.

**Input objects.** Common fields are required `id`, `source`, `instrument`, `field`, `unit`. Only contract fields additionally require `side`, one of `yes`, `no`, `selected_side`. A `selected_side` input is evaluated for the side being considered by that condition/option; it is not a global mutable binding. Compile its required data for both sides when selection can use either. No other fields are accepted.

| Source | Instrument | Supported fields | Unit and identity |
| --- | --- | --- | --- |
| `kalshi` | `current_contract` | `buyable_ask`, `bid`, `last_trade` | `contract_cents`; side required; exact market and contract bound by the evaluator. Missing ask/bid/trade is unavailable, never zero. |
| `brti` | `btc_usd` | `index_value` | `usd_per_btc`; explicitly BRTI, with delivery transport left to the adapter. |
| `coinbase` | `btc_usd` | `last_trade` | `usd_per_btc`; Coinbase BTC/USD trades. |
| `kraken` | `btc_usd` | `last_trade` | `usd_per_btc`; Kraken BTC/USD trades. |
| `kalshi` | `current_market` | `opening_reference`, `settlement_reference` | `usd_per_btc`; market-rule-defined values, without a side. Unavailable until actually established; opening and settlement references are distinct. |

This is a language capability matrix, not a verified provider mapping. Particularly, the two reference fields cannot be populated by a convenient feed value. Later #5 must establish their definitions/access or mark the requested capability unsupported. A strategy importing successfully must show this distinction. Arbitrary URLs, synthetic provider names and implicit “best available” bindings are rejected. Tests supply controlled evidence for these same declarations, rather than making a synthetic source part of shipped JSON.

Units additionally include `percent` for derived percentage results/constants. USD-per-BTC operands may be compared across the explicitly named BTC/USD sources and with compatible market references. Contract cents may be compared across explicitly bound sides of the same current contract. No comparison across physical instruments, quote currencies or contract identities is inferred from a shared currency label. BTC-only import bindings are deliberate: future compatible 15-minute assets require an explicit contract extension and new binding/version, never silently replacing `btc_usd` or BRTI. Generic names such as `average_underlying_price` remain reusable without claiming same-file universal execution.

**Values and measurements.** A value is exactly either `{ "reference": "identifier" }` naming an input/measurement, or `{ "value": "decimal", "unit": "unit_name" }`. A measurement has required `id`, `operation` and the operands below; no implicit units or extra operands. Its unit is inferred and shown in the preview.

| `operation` | Other required fields | Definition |
| --- | --- | --- |
| `difference`, `absolute_difference` | `left`, `right` values | Left minus right, optionally absolute; both operands have the same compatible unit/identity. |
| `percentage_difference` | `left`, `reference` values | `100 × (left − reference) / reference`; denominator nonzero, otherwise unavailable. Output percent. |
| `change`, `percentage_change` | `of` value, `lookback_seconds` | Current minus boundary value; percentage form divides by the nonzero boundary value, retaining its sign, and multiplies by 100. |
| `rolling_average`, `rolling_minimum`, `rolling_maximum`, `rolling_range` | `of` value, `lookback_seconds` | Complete trailing-window calculation; range is maximum minus minimum. |

Historical `of` must reference an input or measurement, not a constant. Every supported operation may use another supported measurement, including a historical one, within compatible units, an acyclic graph and the resource plan. There is no historical-operation blacklist or operator-pair exception table. Define `average_underlying_price` once, then use it in a comparison, a change, another rolling calculation or a percentage expression. The engine resolves the shared graph; authors do not write sampling, storage or scheduling code. Measurements cannot read checkpoint or purchase state.

The uniform meaning is the calculation series defined in item 2: all measurements have fixed decimal values on a complete one-millisecond timeline. Aggregating a moving average means aggregating that timeline of computed values, not attempting an unrestricted continuous mathematical integral. This is an explicit format-wide time/precision definition, not silently averaging whichever samples happened to arrive. It supports average-of-average, change-of-average and average-of-percentage with the same existing operations, no symbolic integrator, arbitrary-size fractions, extra dependencies or new author-facing sampling options. A given definition is shared and evaluated once per relevant event/tick and side binding.

**Conditions.** Conditions are tagged by required `type`. The table lists every variant and its remaining required/optional fields; no unlisted fields or defaults apply. Child arrays are nonempty.

| `type` | Fields | Meaning |
| --- | --- | --- |
| `compare` | `left`, `comparison`, `right` | Compatible values; comparison is `less_than`, `at_most`, `equal_to`, `at_least`, `greater_than`. |
| `all`, `any` | `conditions` array | Three-state Boolean combination using the Technical Plan truth tables. |
| `crossed_below`, `crossed_above` | `left`, `right` | Observed strict crossing of compatible values. |
| `held_for` | `condition`, `duration_seconds` | Continuous truth and required evidence for the duration. |
| `confirmed_updates` | `condition`, `count`, `count_input` | Consecutive true evaluations on distinct observations of the explicitly named input. |
| `checkpoint_reached` | `checkpoint` | Latched Boolean; false until established. Establishment dependencies are no longer current dependencies. |
| `after_checkpoint_update` | `checkpoint`, `count_input` | Latched checkpoint plus a distinct observation of the named input accepted strictly after its establishing event. |

`count_input` names a raw input, not a measurement. A later update must also be later than the identity high-water mark for that input when the checkpoint latched; a duplicate or same-event batch member cannot satisfy it. A confirmation count is an integer 1–10000. An unrelated update or timer cannot increment it, but can reset it on false/unavailable evidence. The count saturates; further true observations preserve success until failure, unavailability or market reset. A crossing is true only for its causing accepted observation event and cannot be manufactured or prolonged by a timer. Consequently holding a crossing true over time does not mean holding the crossed price relation; guides must show `held_for(compare(...))` for that intent. Side-dependent conditions maintain separate state for YES and NO; scalar ones can share state. Conditions stay updated even in currently losing priority branches, so a past failure is never hidden by short-circuit evaluation.

**Side selectors, checkpoints and options.** A selector is exactly `{ "type": "fixed", "side": "yes" | "no" }`, `{ "type": "checkpoint_side", "checkpoint": "id", "relation": "same" | "opposite" }`, or `{ "type": "unique_side" }`. In the last form, evaluate the enclosing condition for YES and NO: select only if exactly one is true and the other false; both true is `ambiguous_side`, either unavailable prevents proof of uniqueness. Do not break ties by iteration order. Fixed or remembered selectors evaluate their conditions only for the selected side. A checkpoint-side reference must name a side-bearing checkpoint; a Boolean-only checkpoint cannot be used for side selection.

Each checkpoint requires `id`, `when`, and may have `during`, `remember_side`. Without `remember_side`, latch a scalar Boolean; reject any unbound `selected_side` reference in its transitive conditions. With a selector, evaluate `when` for its selected side and latch both flag and side. Uniqueness uses `when` itself, without a second hidden predicate. If no unique side exists now, keep checking through the interval. Once established, never rewrite its time, event or side. Side/checkpoint dependency cycles are rejected.

Each option node requires `id`, `when`, and optionally `during`. It then has exactly one of `options` (nonempty ordered child nodes) or `side` (a selector, making it a purchase leaf). Evaluate groups and their leaf together for the leaf's candidate side; `unique_side` tests the full ancestor-and-leaf condition. The first eligible leaf in depth-first array order wins after common objective restrictions. Unavailable preferred branches do not globally block independent alternatives. No “used option” flag exists. Prior attempts and fills are engine-owned constraints, not strategy fields.

A `during` object has exactly required `start_seconds` and `end_seconds`, integers with `0 <= start < end <= 900`; omitted means `[0,900)`. Seconds are elapsed from the identified market open. End is an exclusive deadline. Nested intervals intersect; reject empty intersections. A checkpoint can be used after its own observation interval ends. Temporal tracking for a node begins no earlier than its effective interval; a pre-window duration/confirmation cannot prequalify it. Historical measurements still use their declared trailing data. Evaluate the expiry/boundary before same-time eligibility, so no event at close or exactly at an end can authorize a purchase. These are BTC15m boundaries; future market schedules need explicit compatible definitions.

**Reachability of market-local durations.**

Validation rejects a demonstrably unreachable market-local hold using `unreachable_duration` at its rule path. Compute the effective interval as the intersection of the node and ancestor intervals. Its end is exclusive. A held_for duration must have a possible completion strictly before that end: `[0,900)` with a 900-second duration is rejected, not accepted as a permanent wait. The existing broad numeric duration bound is only a syntax bound; semantic interval validation is mandatory.

For this time-only reachability check, a current comparison/crossing/confirmation has earliest possible success at the effective interval's start; do not assume a provider rate or minimum millisecond spacing between genuine same-time transactions. `all` takes the maximum child lower bound, `any` the minimum, and held_for adds its duration to the child's lower bound. Checkpoint references use the referenced checkpoint's earliest possible establishment, and intervals impose their start. Process checkpoint dependencies in their existing acyclic order. If a held_for or enclosing condition has an earliest possible success at or beyond its exclusive end, return field-specific repair guidance. This is a conservative bound, not a theorem prover for all numerical conditions or market profitability.

This check does not cap historical lookbacks at market length: applicable underlying and exactly identified pre-open contract history may already exist. It must not reject feasible hour-long measurements merely because the outcome window is 15 minutes.

### 2. Use one bounded decimal and time-series meaning

**Public precision.** All literal/raw decimal values are strings matching `-?(0|[1-9][0-9]*)(\.[0-9]{1,6})?`, with at most 12 digits before the point. No exponent, whitespace, plus sign, negative zero or NaN/infinity. Raw contract prices are 0–100 inclusive; raw USD-per-BTC prices are positive. Signed changes/differences and percent results/constants may be negative. JSON integer fields use integer tokens without fractions/exponents. Author durations and lookbacks are positive integer seconds.

Use six fractional decimal places for every input and measurement, represented as signed integer millionths in `i128`. Pad shorter literals exactly; reject input precision greater than six places rather than silently round a feed. Trusted adapters must establish that their field mapping fits this precision or report unsupported. Every derived operation produces its own six-place value, rounding to nearest with ties to even; all comparisons, including equality and crossing thresholds, use those values exactly. The generated preview/reference must say **“Calculations use six decimal places and one-millisecond time steps.”** A threshold at that precision is intentional, not a claim about a more precise continuous quantity. No hidden epsilon, binary floating-point equality, unchecked cast or UI-only rounding.

This explicitly replaces the unapproved exact-fraction proposal. It also resolves the earlier Technical Plan's open numeric choice: time weighting, formulas and dimensional compatibility remain; normalization is now defined, not silent. Round once at each operation's output, never per term inside that operation's sum. Named graph nodes define normalization boundaries, so algebraically rewriting a graph can change a last-place result; do not optimize by reassociating or eliminating those boundaries. Reusing a named measurement always reuses its same value. Display the exact normalized value, with unnecessary trailing zeroes omitted; there is no different hidden decision value.

**Arithmetic bounds.** Each derived value is limited to magnitude `10^18` in its declared unit (scaled magnitude `10^24`). Exceeding that limit yields `numeric_range_exceeded` for that value and its dependents; never saturate/wrap. This allows six whole orders of magnitude above the largest literal/raw value. A denominator must be nonzero before division; signed derived denominators are permitted and retain their algebraic sign, while raw USD-per-BTC inputs remain positive; a positive value that normalized to zero is zero for subsequent nodes and makes division unavailable. Constants can be prevalidated using these same rules. Arithmetic uses checked integer operations even where the bounds prove they fit.

For scaled operands A and B, a difference is A−B; a percentage difference's scaled result is nearest-even `(A−B) × 100000000 / B`. A change uses the current and delayed scaled values; percentage change uses the delayed value as B. A rolling average is nearest-even `sum(scaled operand cells) / number_of_cells`. Absolute difference and min/max/range use scaled integer values. With the maximum lookback of 3600000 cells, an average sum's magnitude is at most `3.6 × 10^30`; the percentage intermediate is at most `2 × 10^32`. Both are well below signed `i128` capacity. Denominators do not accumulate between operations. The [Four-feed-difference rounding regression](#four-feed-difference-rounding-regression) specifies the previously rejected fraction case and its format-2 expected values.

Required precision examples: 1/3 becomes 0.333333, 2/3 becomes 0.666667; a tie at 1.2345645 becomes 1.234564 and a tie at 1.2345655 becomes 1.234566, with the symmetric rule for negatives. The [Four-feed-difference rounding regression](#four-feed-difference-rounding-regression) gives 0.001 percent after normalization without fraction-denominator accumulation. A normalized value equal to its threshold satisfies `equal_to`/`at_most`, not `less_than`. These explicitly chosen last-place boundaries must appear in the reference; the product does not claim greater precision.

#### Four-feed-difference rounding regression

Coinbase BTC/USD last_trade is `100001` throughout `[0,4000)` ms. Kraken BTC/USD last_trade is `100000.000003`, `100000.000019`, `100000.000057`, `100000.000063` over successive one-second intervals `[0,1000)`, `[1000,2000)`, `[2000,3000)`, `[3000,4000)`. All observations/coverage are controlled, post-Go, valid and complete.

Define `feed_difference_percent = percentage_difference(left=coinbase_price, reference=kraken_price)`, then `average_feed_difference_percent = rolling_average(of=feed_difference_percent, lookback_seconds=4)`. Each percentage node normalizes to `0.001 percent` under the existing six-place nearest-even policy. At 4000 ms, its four-second average also normalizes to `0.001 percent`. These exact operands and output boundaries are the regression fixture. This is a calculation example, not another shipped trading strategy or a provider-access claim.

Historical rationale only: averaging the four unnormalized exact fractions would require a 147-bit final denominator; that rejected implementation is not part of format 2. The expected format-2 intermediate/final values above are authoritative for the controlled behavior check.

#### Calculation clock and coverage

**One calculation clock.** Engine time is nondecreasing integer milliseconds since Go. Preserve accepted transaction order inside a millisecond. A raw value becomes the current value when its trusted normalized event is accepted; it is never backdated to authorize past actions. Provider effective/receipt timing is checked by the adapter before acceptance. UTC remains the recording/display time, not the duration clock. Advancing engine time processes every intervening logical millisecond. A clock step is a computation, not a provider observation, trade, confirmation or proof of source health.

Before advancing a batch, stage its trusted coverage/validity facts as specified in item 3 without applying future values or conditions early. At millisecond k, finalize cell `[k−1,k)` using its last accepted value and staged evidence. Apply expiry, gaps and market boundaries effective at k, reset the relevant state, and prepare all time-dependent measurement results for k. If value transactions are scheduled at k, apply the first complete atomic transaction and recompute affected values **before** evaluating conditions/checkpoints; process subsequent genuine transactions in order, each with its own coherent evaluation. Do not insert a separate pre-transaction clock eligibility evaluation at that same instant. If there are no value transactions at k, evaluate the clock-driven state after the expiry/boundary changes. After the last transaction, there is no extra provider observation. Conditions/checkpoints/purchases follow the same order at every historical catch-up instant and the final target instant.

This prevents a hold from completing on an old value immediately before a false update in the same accepted batch: a ten-second hold started at 4000 ms, with its condition false in the batch's 14000-ms value transaction, resets rather than latching. A timer batch already accepted and completed before a subsequently received observation is a different established event; later evidence cannot rewrite its outcome. Known invalidity/expiry at a timestamp always precedes eligibility there. Separate genuine observations with equal timestamps remain individually evaluated, so a true observation followed by a false observation can establish then preserve an already observed checkpoint, exactly as their accepted order requires.

All changes within a current cell remain visible to temporal/checkpoint/confirmation evaluation; the final state in the cell supplies its time-weighted value. Earlier same-millisecond values occupy zero duration at this clock resolution; they cannot be discarded before their event-based effects are evaluated. A false condition followed by true at the same millisecond still resets an unfinished hold/confirmation. Every invalidity during a cell marks its dependent historical cell unavailable even if the final state recovers. This conservative coverage bit prevents a very short outage from being hidden by a final snapshot.

A historical window at k contains exactly the W completed cells `[k−W,k)`, where W is 1000 × `lookback_seconds`. Each cell has equal one-millisecond duration. An average is the time-weighted sum divided by W, including every required cell. Min/max/range operate on those same cells. They have attained extrema in this discrete domain; no special continuous supremum meaning is needed. The value at k enters a subsequent window, never the one ending at k. `change` compares current with the finalized operand at k−W and additionally requires complete operand coverage from that boundary through current evaluation. Any missing cell makes the calculation unavailable; do not shrink its denominator or skip a missing point.

For a raw price stream timestamped to this precision, averaging these complete cells is time weighting of the verified value intervals. For a derived stream, it is explicitly the time average of the six-place values computed on this clock. It is not an event-count average and not the exact continuous integral of an unrounded derived curve. This single fixed resolution avoids a curve algebra, pair-specific exceptions and author-chosen sampling modes. It cannot distinguish the duration of two events inside one millisecond, but still processes both observation events. This precision limit is documented in the same reference and preview as the numeric limit.

A raw price unchanged under valid, complete evidence can cover many cells without new observations. A rolling average still gets recomputed as old cells leave its window. Derived thresholds and hold failures are checked at every logical millisecond even during quiet periods. Scheduling frequency, UI polling and batching cannot change the result. Implementation may skip a span only when it proves the exact same normalized values and condition transitions as the logical tick-by-tick contract; such an optimization is unnecessary for the initial implementation and requires an oracle check if later added. Current-only rules still react to every accepted observation without waiting for the next clock tick. Any loss of a crossing’s required continuity clears its prior observed baseline. Crossing compares successive genuine observation evaluations of its dependencies; clock changes do not advance that observation baseline or fabricate a crossing. Timers/clock steps may reset a confirming sequence or hold but never increment an update count.

**Coverage and cumulative history.** Define history requirements recursively: an input has zero warmup, pointwise operations take the maximum of their operand warmups, and a historical operation adds its lookback to its operand warmup. Compute this per bound source/contract, not just as a single strategy number. A one-minute change of a five-minute average needs six minutes of source coverage. Averaging a five-minute average over ten minutes needs fifteen minutes. The same rule propagates gap unavailability through nested calculations. A current snapshot starts present state; it cannot supply missing past cells. Finalized cells are immutable: late evidence cannot rewrite earlier calculated outputs or retroactively establish a checkpoint; an unrecoverable hole must leave the entire affected horizon before that path becomes available. Known earlier good cells remain intact. No pre-Go history and no joining different contracts.

Live adapters substantiate validity and completeness; this calculation clock does not certify either. A cell is usable only when its own bound dependencies have the required current-validity and complete-coverage evidence. Verified quiet cells contain the previously established value; genuinely missing evidence leaves unavailable cells. If later evidence contradicts a previously claimed complete interval, invalidate affected retained coverage and dependent calculations and report inconsistent evidence; do not silently preserve a usable history or recompute old checkpoints as though they had been observed differently. Rebuilding affected historical operator state is allowed solely from retained trusted cells and has no retroactive actions. Actual adapter correctness, latency/continuity profiles and their source-specific numerical limits remain #5/#9 evidence.

**Boundaries and demonstrable behavior.** A market is `[open,close)`; enter it only via its boundary event. A deadline at 120 seconds excludes 120000 ms and includes 119999 ms. A ten-second hold starting at 4000 ms first qualifies at 14000 ms if every required intermediate condition/evidence remained valid, including at completion. Process invalidity/expiry at an instant before eligibility at that instant. Initial readiness completed exactly at a market open still waits for the following open. Crash/restart remains stopped.

Required oracles: 100 for three seconds then 110 for two seconds has five-second average 104, min 100, max 110, range 10. Its point change at the endpoint enters only the next historical cell. Over the last three milliseconds, values 1,2,2 average to 1.666667, which is greater than 1.666666 and equal to 1.666667. A one-minute change in an average compares the actual normalized average cells, not a nearest convenient provider sample. An average-of-average incorporates each changing inner value on the same clock. A false excursion at any logical millisecond resets a hold even if values at the two surrounding observations were true. A gap invalidates the full affected calculation window and then its downstream windows. These checks establish this fixed finite-resolution language, not continuous market truth or live fills.

### 3. Define the controlled public engine boundary and evidence identity

Keep these types and pure evaluation in `crates/engine/src/strategy.rs`, exported through `lib.rs`; use Rust enums/structs rather than a JSON-shaped catch-all. The following is an interface specification, not source implementation:

- `ValidatedStrategy::parse(text) -> Result<ValidatedStrategy, Vec<RuleError>>`; immutable, owns exact accepted text, format identity, resolved dependency graph, support requirements and generated explanation. Constructible only by validation.
- `Evaluator::new(validated_strategy, RunContext) -> Result<Evaluator, RuleError>`; context fixes strategy version identity, run id, Go time, BTC15m series and the initial admitted descriptor/binding set. No mutable version setter.
- `Evaluator::begin(batch) -> Result<(), EvidenceError>` admits one bounded owned input batch and returns no decision.
- `Evaluator::process_slice(control, objective: Option<ObjectiveView>) -> SliceResult` processes a bounded continuation owned by the evaluator; see the contract below. Neither operation accepts disk, exchange, credentials, network clients or Tauri types.
- `Decision` contains phase/progress, checkpoint snapshots, ordered per-leaf results, optional candidate, and structured reasons. Candidate contains market id, option id, side and remaining quantity; no submitted/fill claim or executable callback. Reasons include rule path and relevant input/measurement/checkpoint identities, not entire source messages. Transition summaries change only when phase, checkpoint, selected candidate or reason set changes; repeated waits have a count. Later history wiring remains later work.

**Bounded controlled processing.**

`Evaluator::begin(batch) -> Result<(), EvidenceError>` admits one owned bounded batch only while idle. A second batch returns `busy`; a stopped evaluator returns `stopped`; an oversized batch returns `resource_limit`. These refusals do not mutate accepted evidence or queue an unbounded task. Beginning a batch clears any previously returned current candidate and creates private continuation state; it returns no Decision.

`Evaluator::process_slice(control, objective: Option<ObjectiveView>) -> SliceResult` performs at most 4096 primitive validation, staging, numeric, condition, or deque-entry operations per call. `SliceResult` is `Pending { phase, evaluated_through_ms }`, `Complete(Decision)`, `Stopped`, `Rejected(EvidenceError)`, or `Failed { code: internal_evaluation_failure }`. `phase` is `validating`, `staging`, or `evaluating`. The evaluator owns the continuation, including clock/node/transaction/member/deque cursors; callers cannot clone or resume it twice. Continue with no accepted pending batch returns Rejected(no_pending_batch) without changing state; a stopped instance always returns Stopped, and repeated Stop is idempotent. A deque operation spanning slices retains its cursor without exposing a partially applied candidate. Validation/staging is also sliced; a large input must not hide an unsliced validation loop. Complete batch validation precedes mutation of accepted evaluation state.

A pending batch rejected during validation or staging before accepted evaluation state changes returns `Rejected(EvidenceError)`: discard its continuation, release admission and return to idle so the next valid `begin` can proceed. Preserve the prior accepted frontier, bindings, history, checkpoints and observation counters. Return no candidate and do not revive the candidate cleared by `begin`. A `busy` refusal preserves the existing pending batch; `Rejected(no_pending_batch)` changes nothing; explicit Stop remains terminal. Rejected input supplies no coverage evidence; actual evidence loss still follows the existing invalidation/recovery rules.

After evaluation begins mutating accepted state, do not use `Rejected` to imply rollback. Expected numerical failures and evidence gaps retain their dependency-local unavailable outcomes; invalid objective views block candidates under the existing common restrictions. An unexpected internal failure that prevents coherent continuation returns `Failed { code: internal_evaluation_failure }`, discards continuation and makes the evaluator terminal. Suppress candidates and preserve the last completely published checkpoint/progress snapshot and separately stored immutable history; private partial work is not published or rolled back as though it never happened. Subsequent `process_slice` calls return the same finite Failed outcome and `begin` returns `internal_evaluation_failure`. No automatic recovery or restart; a later separately authorized Go creates a new evaluator under the existing start rules.

`control` is `Continue` or `Stop`, checked before doing work in each slice. Stop cancels the pending continuation and makes that evaluator terminal without processing the remaining catch-up ticks. It returns no candidate and preserves the last completely published checkpoint/progress snapshot and all separately stored immutable history. Private partial work is not presented as a completed observation. A later Go creates a new evaluator under the existing no-automatic-resume rule. This internal cancellation is not the product's ordinary “Stop after this market”; that runtime mapping remains later work.

While Pending, the evaluator reports processing unavailable for current purchase eligibility. It does not label valid source history unsafe or fabricate a provider gap. Only Complete can expose one decision for the final target, after checking current admission and the supplied active-market objective. ObjectiveView is required only when a market is active; None during collection cannot authorize a purchase. An invalid/regressing active objective or failed current admission yields no candidate. Numeric failures retain their existing dependency-local unavailable behavior.

The controlled caller performs one slice, checks control before the next slice and again before using a Complete candidate, and never uses an intermediate catch-up candidate. Stop therefore takes effect at the next bounded slice boundary, not after the whole batch. No wall-clock response-time promise is inferred from an operation budget. A newer relevant observation or changed objective invalidates a candidate before use; actual execution and numerical age profiles remain later work.

Pending input plus any queued normalized transactions share the existing 4096-transaction/8-MiB ingress budget; these are total bounds, not a fresh allowance for each waiting batch. Admission exceeding either bound returns resource_limit without waiting or silently dropping events. The controlled caller blocks affected current eligibility and requires trustworthy resynchronization when ingress completeness is lost. Existing completed checkpoints retain their defined lifetime. No arbitrary disconnect or catch-up timeout is added to JSON.

Issue #4 implements and tests this pure resumable contract with a deterministic caller that injects Stop between slices. It does not add a scheduler, connect this evaluator to desktop Start/Pause, or build a paper/live run loop.

`ObjectiveView` is `{market_id, requested_quantity, confirmed_quantity, locked_side?, attempt_state, safeguards}`. Quantities are checked unsigned integers; requested is positive, confirmed never exceeds requested. A positive fill requires a side; zero fills cannot impose one. Attempt state is `none`, `in_flight`, `unknown`, or `finished`. In-flight and unknown block all candidates. Safeguards contains a common `allowed`/`blocked` result covering pause, funds and risk/recording gates, plus side-specific allowed price ranges in contract cents. Evaluate each candidate’s current buyable ask against its side’s user range before priority selection; missing/currently invalid ask blocks that candidate, even if its author condition uses another field. All sides use the original requested quantity; do not resize to fit. These constraints cannot be supplied by strategy JSON. This boundary accepts controlled values in #4 and is integrated with real orders in #6–#10. Conflicting/regressing objective facts cause an error and no candidate. A zero-fill finished attempt permits top-priority reselection; any confirmed fill locks the side, and only remaining original quantity can qualify. Filled-side incompatibility skips that leaf before considering the next; completion of the objective blocks every leaf. Eligibility still requires all author price conditions plus user constraints. #4 makes no IOC retry scheduler or order submission.

**Concrete market and binding identity.**

`MarketDescriptor` is trusted metadata `{ series_id, market_id, contract_id, open_ms: i64, close_ms: i64 }`. Its open/close are relative to Go and may describe a market already open at Go; their checked difference must be 900000 ms for the selected BTC15m series. Metadata alone is not an observation and supplies no history, checkpoint or eligibility. Identifiers are opaque nonempty strings of at most 128 UTF-8 bytes without control characters.

`BindingDescriptor` is `{ binding_id: u32, input_id, scope }`. Input source/instrument/field/unit and any fixed side come from the validated document; metadata cannot override them. Scope is `Underlying`, `Market { market_id }`, or `Contract { market_id, contract_id, side }`. `selected_side` expands to explicit YES and NO bindings. The tuple including the validated input identity and concrete scope is the historical identity. Underlying bindings remain independent of market changes.

RunContext supplies an initial bounded descriptor/binding set. A controlled `RegisterMarket { descriptor, bindings }` metadata transaction can add another identified market before its open. Registration must be admitted and fully validated before any evidence references its binding ids. A binding id is never remapped to another tuple during a run; same exact registration is idempotent, conflicting reuse is an error. Retired ids are not reused. Price/current-validity/coverage/observation-number state begins empty; first accepted evidence must be post-Go. Descriptors do not import pre-Go history.

Market-dependent measurement state is keyed by concrete market/contract, so pre-open evidence can warm only that exact contract's calculations. The same named rule definition can have separately accounted concrete instances; no old book is renamed to current_contract. Checkpoints, holds, confirmations and purchase eligibility remain inactive before their own market entry, even when a historical measurement is collecting.

`MarketBoundary { market_id }` activates the already registered descriptor at its declared open, subject to initial readiness and next-full-market rules. It resets the new market's checkpoint/temporal/objective state but preserves historical cells and measurement warmup already collected for that exact entering contract. It cannot preserve the previous contract's cells under a new identity. Continuous applicable underlying history is retained. Unknown/mismatched descriptors or evidence bindings cannot authorize entry or contribute history; return a finite unregistered_binding/incompatible_market error.

Registration is bounded by the existing 128 expanded numeric-instance and 256-MiB working-memory limits, counting every concrete market-dependent instance and its retained buffers. The preview reports shared underlying cost and per-market replication cost. Constructor admission reserves the complete initial binding set before Go; later registration must reserve its entire additional planned state before collection for that new binding begins. Refusal leaves existing registered history intact and does not falsely start a collecting progress bar. No admitted series grows beyond its planned capacity as it fills. Descriptors without price state are capped at 128 and charged to the fixed reserve. This refines resource accounting; it neither increases the budget nor promises unlimited upcoming-contract collection.

Issue #4 only supplies this controlled metadata/evidence contract and its deterministic tests. Selecting/discovering actual future contracts, subscribing to books, collecting real feeds and source-access/listing feasibility remain #5 and subsequent integration work. No provider watchlist/collector or desktop running-strategy flow is added.

Normalize provider-specific bookkeeping once in the trusted adapter. The evaluator receives `binding_id`, a monotone `observation_number` for a genuinely new value observation, engine acceptance time, and typed `Observation`, `Resynchronize`, `Validity` or `Gap` evidence. Bindings resolve to the explicit provider/instrument/field/side and actual contract/market identity. Provider message keys, reconnect epochs, exchange clocks and transport sequences stay in the adapter. They must support its deduplication, source-order and validity claims, rather than being duplicated as overlapping evaluator identity systems. A provider transaction can update several fields atomically; carry one transaction number and apply all its members before derived recomputation. Separate transactions at equal timestamps remain separate and are evaluated in accepted order.

Each binding's normalized observation number is strictly increasing for the run, including across reconnects. The adapter emits no new number for a duplicate/replay. If distinctness cannot be established, emit resynchronization/current validity rather than a counting observation. The engine additionally rejects decreasing numbers or conflicting reuse of a number and value; an identical duplicate has no effect. No small dedup cache can eventually make an old message new. A projection from one provider message retains its normalized number for that binding. Repeated identical prices with genuinely new numbers can count. `after_checkpoint_update` compares against the normalized high-water mark captured when the checkpoint latched, and requires a later transaction, not another member of the establishing atomic transaction.

The controlled batch shape is `AdvanceBatch { received_through_ms: u64, target_ms: u64, transactions: Vec<NormalizedTransaction>, coverage: Vec<CoverageFact>, current_admission: Allowed | Blocked(reason) }`. A normalized transaction has `{ number: u64, at_ms: u64, changes: Vec<Observation | Resynchronize | Gap | Validity | RegisterMarket | MarketBoundary> }`. Observation members have `{binding_id, observation_number: u64, value}`; values use the unit/precision from their validated binding, never caller-chosen units. Resynchronize has `{binding_id, value}` without an observation number. Gap has `{binding_id, start_ms, end_ms, reason}`; validity has `{binding_id, valid_until_ms, reason?}`. CoverageFact has `{binding_id, start_ms, end_ms}` with half-open bounds. RegisterMarket has `{descriptor: MarketDescriptor, bindings: Vec<BindingDescriptor>}` as defined above; MarketBoundary has `{market_id}` and resolves the already registered descriptor. Registration and binding references are validated before their evidence is staged. Market identity/times must match the fixed series and descriptor. Binding identifiers refer only to the validated requirements; implicit buyable asks needed by the user-price gate are included in those engine requirements. Transaction numbers are strictly increasing; member bindings are unique per atomic transaction; times are nondecreasing and within the prior evaluated frontier and target, with target no later than receipt. Interval validity/completeness facts are staged before replay; a same-time Gap/expiry is applied before any value/eligibility evaluation. Resource bounds apply to the complete batch before staging. `current_admission` is supplied by the trusted engine's source/processing-age gate, never by JSON or the interface; Blocked suppresses candidates while evidence/hold/checkpoint state continues to be processed truthfully. The separate process-slice Stop control suppresses candidates at its next bounded control check and preserves published snapshots and separately stored history; it is not an ordinary Stop-after-this-market request.

**Staging before clock advancement.** Distinguish `received_through_ms` (the trusted runtime's current receipt horizon) from `evaluated_through_ms` (the last finalized logical calculation time). An `AdvanceBatch` targets a nondecreasing evaluation time no later than the received horizon. Validate and stage all its bounded coverage/validity intervals and ordered transactions **before** replaying logical clock steps from the old evaluated frontier to the target. Coverage may describe that not-yet-finalized interval even though its certificate was received at the end. This supplies proof for earlier cells without inserting observations, new prices or retroactive actions. Never apply a later staged value/market boundary before its own normalized timestamp/transaction order. Atomic members apply together only at their transaction. Coverage facts determine whether the value already established at that point covers that cell; they cannot substitute a later snapshot value for it.

Validity has exclusive `valid_until_ms` for current use plus certified complete intervals ending no later than the received horizon. Completion evidence cannot claim a future interval merely from a validity lease, and connection health alone proves neither. A verified quiet certificate extends coverage of the previously established value without new observations. Example: the last value at 0 is 100; one batch received through 1000 contains verified quiet coverage `[0,1000)` and targets 1000. Stage that coverage, replay 1000 logical cells, and obtain a complete one-second average of 100. There is no one-millisecond delivery requirement. The explicit **receipt → stage evidence → advance calculation → expose current decision** order is essential; do not finalize those cells as missing before reading the certificate that proves them complete.

The runtime batches only not-yet-evaluated evidence; the calculation clock is not a wall-clock polling callback. A timer batch asks trusted adapters for their currently provable coverage, stages it, then advances. A binding without coverage leaves unavailable cells only for dependent paths; independent alternatives may proceed. Once a batch has finalized cells as genuinely missing, later certificates/snapshots do not rewrite them. This preserves the adopted history-gap policy. Receipt/event times older than the finalized frontier cannot authorize earlier checkpoints/actions. If validated late data contradict prior claimed completeness, use the inconsistent-evidence invalidation/recovery policy in item 2 rather than retroactive eligibility. Actual adapters must prove both usable quiet coverage and safe current/processing age; delayed batches cannot emit stale candidates. Those source-specific numerical profiles remain later acceptance gates, not guessed one-millisecond delivery deadlines or author knobs.

A resynchronization establishes present state but does not count an observation, crossing, confirmation or historical repair. Any superseded candidate is discarded before processing a new transaction; a result is usable only at its exact final evaluated time with current admission allowed. A gap identifies the binding and affected time interval and invalidates dependent coverage. Contradictory/out-of-order source facts, future times or reversed intervals invalidate only affected requirements with a finite reason. Adapter resynchronization restores trustworthy current state; retained history and completed checkpoints keep their defined lifetimes. Required tests use the same normalized interface as later adapters. JSON cannot provide these claims or relax their validity rules.

For each batch, validate/stage evidence and execute the single per-instant ordering in item 2: finalize the preceding cell, apply expiry/gap/boundary, apply a scheduled atomic transaction if any, recompute all affected measurements, then update conditions, establish checkpoints and select options. Never evaluate a virtual timer before the batch’s same-time value transaction. Update affected temporal conditions even in losing priority branches. A newly latched checkpoint may qualify a purchase in that transaction unless `after_checkpoint_update` requires later evidence. Every millisecond with no observation still advances windows and durations without observation counts/crossings. `process_slice` exposes Complete only for the final evaluation target after checking it against the received/current horizon and trusted processing-age admission; intermediate catch-up candidates are not returned as present authority. Pending marks current processing unavailable until continuation completes. A busy, stopped or over-budget admission follows the exact bounded contract above; no intermediate candidate is returned. No stale candidate may be retained for later execution. Real current/processing-age admission remains engine/adapter work, never a strategy setting.

At Go, clear all prior-run observations, histories, normalized observation counters and temporal state; retain only the newly admitted immutable descriptors/binding definitions. They supply no price history. Derive maximum cumulative history requirements across **all** alternatives. Initial collection waits for complete required series coverage, reports per-input required/collected milliseconds, then enters `Ready — waiting for next market`. Until then use `Prefill — collecting live data`; mere elapsed time is insufficient. Duration/confirmation/checkpoint rules are market-local and do not have to latch during prefill. For contract lookbacks, collect only the identified contract actually supplied after Go, never consecutive books stitched together. Ready history does not transfer to a different contract. A new market may therefore wait for its own contract evidence even after run prefill. If provider listing/collection capabilities prove a requested contract history impossible, report `unsupported_history` rather than endless progress; do not infer impossibility merely from the 900-second outcome window. An ordinary active-market interruption does not return the run to initial prefill; evaluate independent alternatives and restore affected eligibility when evidence permits. Restart constructs a stopped session; no persisted live feed replay or automatic resume is added.

### 4. Preview, save, inspect and preserve versions

Keep the existing single native command `strategies` and existing permission, replacing its optional-text ambiguity with strict tagged requests. Preview/save/list/detail are separate operations; raw unknown operations/fields produce a finite sanitized request error. Define matching Rust and TypeScript tagged unions:

| Request fields | Result |
| --- | --- |
| `operation: preview`, `json: string` | `preview`: format/meaning identity, generated explanation, typed requirements, support-profile id and source-access-unverified notice; or bounded RuleErrors. No write. |
| `operation: save`, `json: string`, `reviewed_json: string` | Require byte-identical strings, then revalidate and generate explanation in Rust, append one complete row and return `saved` with version summary. The equality protects the reviewed-content flow; it is not a security claim that a UI really displayed it. |
| `operation: list`, optional `before_id: decimal string` | `versions`: newest-first summaries, at most 50, with optional next cursor. No full document/explanation expansion for the whole history. |
| `operation: detail`, `version_id: decimal string` | One original JSON, optional stored explanation/meaning metadata, recording time and inspection/execution status. |

**Native operation admission.**

The existing `strategies` command permits one native in-flight strategy operation, covering preview, save, list and detail. After bounded request decoding and the quit check, acquire a nonblocking operation guard before calling spawn_blocking. If occupied, return the finite error `{ code: busy, message: Strategy action already in progress. Wait and retry. }`; do not spawn or queue another waiting task. No caller-provided text appears in that error.

The guard is owned by the accepted task and releases on success, validation/storage error, task rejection, cancellation before execution, and panic unwinding. Use the existing standard-library ownership/atomic pattern, without another executor/dependency. Do not hold an interface-state or credential lock across blocking work. Recheck quitting before beginning storage mutation; preserve committed-save-versus-refresh-failure reporting.

Busy preserves editor content and any still-current preview; it is not a failed commit and triggers no automatic save retry. Matching Rust/TypeScript result unions include this finite native error. Permissions stay limited to the existing local main-window strategies command.

A list page contains at most 50 complete summaries and its complete serialized UTF-8 response is at most 32 KiB, including envelope/cursor fields. Add summaries newest-first only while both limits fit. If another row remains, the next cursor identifies the last included row so the following request neither skips nor repeats it. Do not truncate a summary or silently omit rows. The existing bounded summary schema/finite status text must allow every individual summary plus envelope to fit; malformed legacy display metadata is represented by the existing invalid/unknown summary status rather than echoed unbounded text.

Version ids are positive SQLite rowids represented as decimal strings across IPC, avoiding JavaScript integer precision loss. Cursors have the same form; unknown/nonpositive/overflow ids fail without quoting raw values. Summary includes id, recorded_at, display name when safely known, format when known, and status `supported`, `legacy_read_only`, `unsupported`, or `invalid_record`. Missing historical names are labelled unknown rather than reparsed as newly validated truth. Detail returns original stored content even for unsupported format, with bounds described below. Supported means supported **rules**, never live-qualified. Listing one unsupported/invalid row cannot make the whole list fail.

RuleError has `{path, code, message, accepted_form}`. Paths use known field names and array indices, never raw unknown keys. Codes cover malformed/duplicate/unknown, unsupported format/binding/operation, reference/cycle, incompatible units, range/interval, unreachable_duration, unbound side and resource limit. Native request errors additionally include busy; controlled evidence admission uses busy/stopped/resource_limit and unregistered_binding/incompatible_market where specified. At most 20 errors, with an explicit truncated count/flag. Stop byte/depth parsing failures immediately. Messages never interpolate submitted values or raw serializer/SQL errors. Unknown framework errors become a generic finite local-action failure. Requests/responses never log document contents. Supported string fields cannot detect a user disguising a secret as a name; guides must prohibit secrets and private data, and errors must not echo suspect values.

Preview text stays in frontend memory. Any text/file/edit-copy change invalidates it and disables save. Each asynchronous preview/file read carries a local generation number; discard results if the editor generation changed. Disable duplicate save while pending; capture the reviewed bytes on click. Display saved identity immediately from confirmed save; refresh the list separately. Refresh failure cannot replace committed success with “not saved.” A transport/commit uncertainty shows “Save not confirmed — inspect saved versions before retrying”; no automatic retry inserts another version. Selecting a version for inspection uses id; edit-copy loads its text without overwriting that row. Keep the controlled non-trading session labelled as such. Strategy-run selection/execution is delivered later; #4 must not connect the existing Start button to this evaluator.

For storage revision 3, migrate revisions 0/1/2 under the existing workspace lock in one immediate transaction. Preserve existing `strategy_versions` ids/json/recorded_at and session records. Add nullable `format_revision INTEGER`, `meaning_id TEXT`, `explanation TEXT`, `display_name TEXT`; existing rows retain NULL, without reparsing/backfilling immutable history. For new rows insert exact reviewed JSON, 2, `metten_rules_2`, generated explanation, name and UTC recording time together. Retain no-update/no-delete triggers and add an insert guard requiring complete format-2 metadata on future inserts. Do not drop triggers to rewrite history. Set `user_version=3` only in the successful migration transaction. A failure/termination rolls back to the previous revision; reopen must be safe and idempotent. Revisions greater than 3 refuse with an actionable newer-app message. Revision 2 apps already refuse 3; prove that on a disposable fixture, without launching an old app on the owner's workspace.

Unknown legacy rows remain inspectable with the explanation only if it was actually stored. Never regenerate a legacy explanation and label it historical. New format-2 execution loads require matching meaning identity and revalidation under that fixed meaning; an inconsistency marks the record invalid, not silently repaired. Future unsupported meanings retain inspection; no permanent execution promise for every historical evaluator. Conversion/editing requires a newly valid format-2 file and explicit preview/save; there is no silent converter. An active future run retains its immutable version id and rules regardless of later imports.

### 5. Assign every affected file before implementation

All paths below are repository-root relative. This is the complete planned change allowlist, not a request to perform it now. No deletions, dependency changes, lockfile changes or new services. Existing dirty work must be reconciled within these responsibilities, never reset wholesale.

| Action and file | Responsibility and reason |
| --- | --- |
| Modify `crates/engine/src/strategy.rs` | Replace narrow format-1 draft execution with format-2 validation, typed values, evidence/coverage, conditions, lifecycle, priority, explanations and finite errors; keep one meaning shared by preview/evaluation. Keep helpers local; no separate expression framework. |
| Modify `crates/engine/src/lib.rs` | Export the controlled rule interfaces and typed preview/save/list/detail results; adapt current import/list callers; preserve existing controlled session and stopped-on-reopen behavior. |
| Modify `crates/engine/src/storage.rs` | Atomic revision-3 migration, append-only complete metadata, paged summaries/details and mixed-version inspection; preserve locking/durability and every existing record. |
| Modify `crates/engine/tests/strategy.rs` | Public-interface tests for grammar, numeric/time semantics, controlled twelve timelines, explanations and limits; use inline focused fixtures rather than a duplicate fixture language. |
| Create `crates/engine/tests/strategy_storage.rs` | Synthetic prior-revision workspaces, mixed/unsupported rows, immutable snapshots, migration interruption/refusal and write failure; separate actual SQLite behavior from pure rule tests. |
| Modify `src-tauri/src/main.rs` | Strict sanitized strategy request decoding and bounded blocking storage operations; retain main-window-only access/quit checks; add focused IPC tests alongside existing native tests. |
| Modify `src/contract.ts` | Exact request/response/error/status unions and string ids; frontend receives facts rather than evaluating rules. |
| Modify `src/main.ts` | Bounded file read/editor flow, preview-generation guard, explicit save, paged list/detail and edit-copy; render text safely, preserve committed identity on refresh failure. |
| Modify `src/view.mjs`, `src/view.d.mts` | Add only small strategy-presentation/state helpers needed for testable pending/stale-preview/save outcomes, using the existing presentation module pattern. No frontend rule evaluator. |
| Modify `index.html`, `src/style.css` | Labelled preview/save controls, readable generated explanation, field-path errors, status/progress and version inspection; keyboard/focus/live-status accessibility. Preserve existing credentials/session controls. |
| Modify `tests/refresh.test.mjs` | Route invoke mocks by command: only snapshot calls enter deferred snapshot reads; strategies list gets its own correctly tagged result; control remains independent. Preserve every existing before/after-Pause stale-response assertion. Necessary caller-fixture correction, not unrelated test cleanup. |
| Create `tests/strategy.test.mjs` | Existing Node runner checks editor-generation/pending/save presentation behavior, text rendering and response handling. Existing credential/session tests remain regression checks. |
| Modify `examples/learning.json` | Replace unfinished format-1 example with the complete format-2 learning file in item 7; one shared teaching file. |
| Create `docs/strategies/README.md` | Human entry path, scope/current availability and links to tutorial/reference/agent guide. |
| Create `docs/strategies/authoring.md` | Plain-English learning path, readability walkthrough, repair examples, preview/save/version flow and honest later-run instructions. |
| Create `docs/strategies/reference.md` | Exact tables/variants/units/formulas, timing/identity/gaps, ceilings and compatibility rules from this section; sole normative author field reference backed by parser tests. |
| Create `docs/strategies/agent-authoring.md` | Full authoring recipe using the shared reference/example; prohibit invented bindings, substitutions, secret content and unsupported trading claims. This is a guide, not a new installed skill. |
| Modify `README.md` | Discoverable human and agent guide links; describe #4's local import support without claiming trading integration. |
| Modify `GLOSSARY.md` | Add concise evidence/coverage, input/measurement, purchase option and format-meaning terms needed by guides, preserving existing product definitions. No architecture decision changes. |
| Create `docs/verification/issue-4.md` | Record actual version, executed checks, synthetic fixture origins, walkthrough outcomes and missing evidence after authorized implementation. Distinguish planning oracles from results. |
| Create `docs/proposals/issue-4-egm/strategy-rules-design.reviewed-2026-11-13.md` | Preserve the exact original review target before applying the accepted resolution; historical documentation only. |
| Modify `docs/proposals/issue-4-egm/goldfish_review_report.md` | Record the accepted resolution, new target hash/ranges, actual document checks and outstanding verification. |
| Modify `docs/proposals/issue-4-egm/detailed-implementation-adversarial-review.md` | Append the dispositions and actual targeted revision checks while preserving the original findings; planning evidence only. |
| Create `docs/proposals/issue-4-egm/strategy-rules-design.md` | Finalized review target from the section-approved draft; documentation only, without implementation/publication authority. |
| Modify `docs/proposals/issue-4-egm/design_draft.md` | This design-only section and gate state now; later EGM resolutions only under their gates. Do not rewrite earlier approvals. |

`crates/engine/Cargo.toml` already has serde/serde_json/rusqlite; use them plus standard library checked arithmetic. `src-tauri/build.rs`, `src-tauri/capabilities/main.json` and generated `src-tauri/permissions/autogenerated/strategies.toml` must expose the single local main-window command. Phase-2 inspection found these inherited registrations absent in the selected worktree; restoring them is a necessary bounded addition to the phase-2 file list, without broader permissions. No additional command permission is needed for tagged operations. No change to credential implementation, Tauri configuration, provider code, implementation-plan sequencing or old proposals. If implementation demonstrates a missing file/dependency or a materially restrictive bound, update the design explicitly rather than silently expand this allowlist.

### Ordered implementation phases

These are stable checkpoints for later separately authorized implementation, not instructions to code in this session.

| Phase | Exact planned files | Stable checkpoint and checks |
| --- | --- | --- |
| 1 — pure rule contract | `crates/engine/src/strategy.rs`, `crates/engine/src/lib.rs`, `crates/engine/tests/strategy.rs` | Add format-2 validation, explanations and controlled resumable evaluation; expose the new public types. Keep the old import/list entry points and their format-1 parser/explanation only as temporary development compatibility for existing storage/native callers. Do not add a second connected evaluator. New format-2 tests use inline controlled fixtures until the shared example switches. Existing session behavior remains unchanged. Run affected public-engine tests, existing session regression tests, fmt and clippy. |
| 2 — coordinated local import/version slice | `crates/engine/src/strategy.rs`, `crates/engine/src/lib.rs`, `crates/engine/src/storage.rs`, `crates/engine/tests/strategy.rs`, `crates/engine/tests/strategy_storage.rs`, `src-tauri/src/main.rs`, `src/contract.ts`, `src/main.ts`, `src/view.mjs`, `src/view.d.mts`, `index.html`, `src/style.css`, `tests/strategy.test.mjs`, `tests/refresh.test.mjs`, `examples/learning.json` | Change the engine/storage/native/frontend contract together in this phase, including revision-3 migration, tagged operations/admission, preview/save/version flow and harness repair. Switch the shared example and all its consumers together. Remove the temporary format-1 parser/explanation/import compatibility after all callers are replaced; legacy rows are inspected as stored, not executed/reinterpreted. End-of-phase has one format-2 evaluator and one strategy-command protocol. Run workspace Rust tests, fmt/clippy, npm check/build and focused IPC/storage/UI checks. |
| 3 — documentation and focused packaged proof | `docs/strategies/README.md`, `docs/strategies/authoring.md`, `docs/strategies/reference.md`, `docs/strategies/agent-authoring.md`, `README.md`, `GLOSSARY.md`, `docs/verification/issue-4.md` | Complete the linked human/agent/reference guides against the now-checked grammar/example. Perform the independent documentation-led authoring exercise and existing focused unsigned packaged import/reopen/edit-copy checks on a disposable workspace; record results and missing evidence. Reuse unaffected passing checks. No credential/provider/trading integration, signing or publication. |

All coupled caller/type changes in phase 2 form one buildable checkpoint rather than separately shipping incompatible intermediate contracts. The temporary phase-1 compatibility is neither a supported new format nor a deployment. Phase order adds no commits/PR/build authority in this design-only session. Design/review records are updated separately as documentation under EGM gates, not by expanding these source phases.

### 6. Admit a complete bounded workload before saving

Use support profile `rules_2_initial` in previews/errors. The language keeps the same composable operations; the profile limits total work and retained state. Do not advertise each independent ceiling as a promise that all maxima fit together.

| Resource | Ceiling and accounting |
| --- | --- |
| Document | 256 KiB UTF-8, lexical JSON depth 48, decoded strings individually at most 1024 bytes except name's tighter bound. Check before unbounded allocation. |
| Rule structure | 512 condition/option nodes, condition depth 12, option depth 8; inputs/measurements/checkpoints as item 1. Count side-expanded condition state too, at most 1024 instances. |
| Calculation graph | At most 128 numeric instances after input/measurement/side and concrete market-binding expansion; acyclic measurement depth at most 12. Share identical resolved instances; references do not copy their history. This permits chained supported operations without unbounded work. |
| Time | Each lookback 1–3600 seconds; cumulative source warmup at most 14400 seconds. Each hold duration 1–86400 seconds, requiring only bounded temporal state. An hour is supported for every historical capability on a single bound series; longer collection can arise from admitted compositions. No limit derives from the 900-second market length. |
| Planned working memory | At most 256 MiB per evaluator, including retained values, deque/delay buffers, bitmaps if needed, graph/state, current bindings, scratch and decision buffers. Reject excess during preview/save, not after hours of prefill. Reserve 8 MiB for all non-history state within this total; validate those fixed-capacity structures fit this reserve. |
| Pending evidence | Pending plus queued normalized input totals at most 8 MiB and 4096 atomic transactions, whichever binds first. Do not grant a fresh allowance to each waiting batch. Actual adapter queue/scheduling integration remains #5/#9. |
| Output | Generated explanation at most 128 KiB without reference expansion; version summary page at most 50 complete summaries and 32 KiB for the entire serialized UTF-8 response, including envelope/cursor fields, single detail/preview at most 1 MiB. Legacy raw detail above 1 MiB remains preserved in storage and is reported too large to show completely, never silently truncated. |

**Concrete retention plan.** Materialize history only at nodes whose operation needs it; a current input/arithmetic result or completed confirmation does not retain an event log. With W=1000×lookback seconds, a change uses a W+1 delay ring of 16-byte scaled values; average uses W values plus one checked running sum; extrema use monotone deques of 32-byte `(value, tick)` entries, at most W+1 entries each; range has two such deques. Allocate fixed-capacity arrays, not individually allocated entries. Unavailable entries are zero placeholders for sums/delays plus bounded validity tracking: the most recent unavailable tick and the node's warmup start determine whether the required full window is usable. Change needs its boundary cell through current evidence; average/extrema need only their half-open historical cells. No missing value is exposed as zero. Coverage contradictions invalidate the affected node and clear/rebuild its affected state from trusted retained inputs, or restart its own window collection if those inputs are not retained; independent paths and already established checkpoints remain.

All byte arithmetic is checked at validation. The memory charge includes actual allocated capacity (including W+1 and alignment), both deques where required, side expansion, fixed metadata and the 8 MiB reserve. The preview reports per-node bytes, per-source startup horizons, shared underlying cost and per-market replication cost. `Evaluator::new` obtains the complete allocation for the initial concrete binding set before collecting. Later RegisterMarket admission reserves its full additional planned state before collection for those bindings begins; refusal preserves existing histories. Allocation failure is an immediate explicit refusal, never a partially started prefill. An admitted binding set cannot grow beyond its planned capacities as history fills. No lazy individually allocated per-observation history. Distinct confirmations retain counters/last normalized ids, not W old messages. Process each raw observation before retaining only its current-cell historical value, preserving crossings/checkpoints/failures even when several observations share a cell.

The planned examples are small enough even with dense one-millisecond history: one hour average uses `3600000×16 = 57600000` bytes plus reserve; one hour range's two worst-case deques use `2×3600001×32 = 230400064` bytes plus reserve, below 256 MiB. A one-minute change of a five-minute average uses approximately 5.76 MB of rings plus reserve and six minutes of startup coverage. An hour average of an hour average uses approximately 115.2 MB plus reserve, with two hours of startup coverage. Two independent hour averages likewise fit. Five independent hour averages require over 288 MB of rings and are rejected before save, with the named nodes and required/allowed byte counts. These are allocation calculations, not claims of measured run capacity or provider support. They provide a usable, explicit supported workload instead of the old contradictory day/record ceilings.

A logical tick updates at most the admitted 128 numeric/1024 condition instances, counting concrete market bindings and side expansion. Average/delay/arithmetic updates have constant bounded work. Extremum deques have amortized constant cell cost, but one cell may remove up to W entries. The private process-slice continuation carries unfinished deque/node/transaction cursors across calls. Validation, staging, evaluation and deque-entry work all use the same 4096-primitive-operation slice budget. The controlled caller checks Stop before every slice and before using Complete; only a final Complete decision can supply a candidate. Pending means processing unavailable for current eligibility, while accepted historical evidence remains truthful. Busy/stopped/resource_limit admissions follow item 3 rather than an unspecified catch-up timeout. No logical clock transition may be silently skipped. Sustainable Mac processing and later runtime/source-age admission still require their separately tracked evidence. No scheduler, strategy-running desktop flow or production speed claim is added in #4.

This uses ordinary fixed-size arrays, integer arithmetic and monotone deques inside the existing engine module, without a general curve/integration system. Measurements keep the same meaning at all accepted workloads. Resource admission is separate from field/source access: a fitting graph does not establish actual source timing, licensing or availability. Multiple run admission and measured concurrency remain #16. A larger profile can later admit longer/more graphs without changing format-2 arithmetic or silently changing sources; it requires evidence and explicit profile identification.

### 7. Use one readable learning file and both authoring guides

The following is the complete proposed content of `examples/learning.json`, shown here as a design specimen, not a changed source/example file. It intentionally uses contract asks only; optional underlying-derived direction and rich measurements remain in the reference and controlled tests, not mandatory ceremony in the first file.

```json
{
  "format": 2,
  "name": "Learning: remember a side, consider its opposite",
  "inputs": [
    {
      "id": "selected_side_ask",
      "source": "kalshi",
      "instrument": "current_contract",
      "field": "buyable_ask",
      "unit": "contract_cents",
      "side": "selected_side"
    }
  ],
  "checkpoints": [
    {
      "id": "early_side_checkpoint",
      "during": { "start_seconds": 0, "end_seconds": 60 },
      "remember_side": { "type": "unique_side" },
      "when": {
        "type": "compare",
        "left": { "reference": "selected_side_ask" },
        "comparison": "less_than",
        "right": { "value": "40", "unit": "contract_cents" }
      }
    }
  ],
  "purchase_options": [
    {
      "id": "opposite_side_first",
      "during": { "start_seconds": 60, "end_seconds": 120 },
      "side": {
        "type": "checkpoint_side",
        "checkpoint": "early_side_checkpoint",
        "relation": "opposite"
      },
      "when": {
        "type": "compare",
        "left": { "reference": "selected_side_ask" },
        "comparison": "at_most",
        "right": { "value": "30", "unit": "contract_cents" }
      }
    },
    {
      "id": "remembered_side_alternative",
      "during": { "start_seconds": 60, "end_seconds": 120 },
      "side": {
        "type": "checkpoint_side",
        "checkpoint": "early_side_checkpoint",
        "relation": "same"
      },
      "when": {
        "type": "compare",
        "left": { "reference": "selected_side_ask" },
        "comparison": "at_most",
        "right": { "value": "30", "unit": "contract_cents" }
      }
    }
  ]
}
```

Readability walkthrough: first identify the one named value, **the ask for the side being considered**, in cents. No bid/last trade can satisfy it. In the first 60 seconds, remember a side only when exactly one side has ask below 40; both qualifying is ambiguous, and no checkpoint means no remembered-side selection. From second 60 up to but excluding second 120, first consider buying the opposite side at ask at most 30; otherwise consider the remembered side at ask at most 30. The first eligible option wins, subject to the user's settings and one shared quantity objective. Any partial fill fixes the side; a later eligible opposite-side option cannot reverse it. Neither threshold is a recommended trading rule. This specimen demonstrates priority, not a claim that both options are economically likely to qualify together.

The engine-generated explanation must state those intervals, strict/inclusive thresholds, side tie behavior, priority and common restrictions, plus a source-readiness disclaimer. It must not turn this example into a crossing rule or require three observations. A separate short reference example defines `average_underlying_price` from an explicitly bound Coinbase BTC/USD input and `rolling_average` with `lookback_seconds: 3600`; tests parse that snippet too. Show BRTI/Kraken bindings and compatible cross-feed comparisons in reference examples without adding a second shipped trading strategy or claiming adapter access.

A second teaching snippet in the shared reference (not another trading strategy file) demonstrates the language's reuse directly. Given an explicitly declared `underlying_price` input, define:

```json
[
  { "id": "average_underlying_price", "operation": "rolling_average",
    "of": { "reference": "underlying_price" }, "lookback_seconds": 300 },
  { "id": "change_in_average", "operation": "change",
    "of": { "reference": "average_underlying_price" }, "lookback_seconds": 60 },
  { "id": "smoothed_average", "operation": "rolling_average",
    "of": { "reference": "average_underlying_price" }, "lookback_seconds": 60 }
]
```

Read it as “the five-minute average,” “how much that average changed in one minute,” and “the one-minute average of that five-minute average.” Both downstream values require six minutes of source coverage. The author selects only the supported operations and durations; Metten handles normalized arithmetic, dependency order, history, gaps and admission. Show its generated requirements and the meaning of six-place/millisecond precision in both guides. Include the [Four-feed-difference rounding regression](#four-feed-difference-rounding-regression) as the complete-grid percentage-series check.

The human guide teaches JSON objects/arrays, price identities, units, checkpoint memory, ordered options and errors before showing edge-case tables. It walks through import → preview → save → inspect → edit-copy, and explains that actual Go/trading is later work. The agent guide starts with the intended condition, binds each required value explicitly, maps it to supported operations, checks dimensional compatibility and evidence needs, writes descriptive ids, uses the engine preview to repair finite errors, and presents the generated meaning. It asks for owner decisions only when the trading intent/source choice is missing or unsupported. It never adds credentials/quantity to JSON, swaps providers, invents confidence models or treats controlled evidence as live qualification. Both guides link the same exact reference and example.

After implementation, perform an independent documentation-led authoring walkthrough from the stated learning intent using just these guides/reference, without the handoff or conversation. Produce a new valid file, preview it through the public engine and compare its meaning to the intent. Record any point requiring private explanation as a documentation failure. The walkthrough must explain each variable, unit, time boundary, remembered side, option order, startup wait, gap behavior and user safeguard, and make a one-field edit that creates a new immutable version. Do not claim independent usability evidence from the drafting lead reading their own example; a later authorized walkthrough must identify who performed it and what they needed clarified. The participant-led walkthrough remains future evidence; the completed design review does not substitute for it.

### 8. Verify meaning and preservation before claiming completion

After separate implementation authorization, use the existing test runners and disposable synthetic workspaces. No network/provider/account credentials or other application's records are needed. Tests primarily call the public engine interface, not private helpers. Each meaningful check addresses a named acceptance risk:

| Check | Required observation/assertion |
| --- | --- |
| Adopted timelines 1–3 | Checkpoint latches without purchase; opposite selection works; ask 35 blocks despite bid 25/trade 28, ask 29 may qualify; loss of completed-checkpoint-only source does not erase it or block an independent eligible option. |
| Timelines 4–5 | True at 0, false at 3, true at 4 cannot complete ten seconds at 10; complete at 14 with verified evidence. Atomic two-result update sees both new values; independently arriving equal timestamps are processed in order, including intervening failure. |
| Timelines 6–7 | Five-minute initial readiness at 10:05 waits to 10:15; unchanged verified intervals supply coverage, a real hole blocks the exact affected window, snapshot does not fill it, current-only alternatives recover sooner. |
| Timelines 8–10 | Ordered/nested priorities, confirmed zero-fill reselection, in-flight/unknown block; request 10 with 4 YES leaves at most 6 YES, NO skipped; checkpoint-before-purchase on one event versus explicit later-update requirement. These are controlled eligibility tests, not fill integration proof. |
| Timelines 11–12 | Market reset clears market-local checkpoints/temporal/objective state, preserves the entering contract’s own admitted history and compatible underlying history, and never transfers another contract’s history; version edits preserve old JSON/explanation/history and active evaluator meaning. |
| Bounded controlled continuation | A long quiet catch-up yields Pending after one bounded slice; Stop at the next slice returns Stopped without finishing catch-up or exposing a candidate. Busy/oversized admission preserves accepted state. Final values are independent of slice boundaries for the same admitted batch. A bounded malformed batch admitted by begin then rejected during validation/staging returns idle, preserves prior accepted frontier/bindings/history/checkpoints/counters and exposes no candidate, including the previously cleared one; a subsequent valid batch completes on the same evaluator. Separately inject an unexpected failure after state mutation: Failed is terminal, returns no candidate, preserves the last published snapshot and immutable history, and permits no automatic restart; subsequent process_slice/begin return the defined failure. |
| Registered pre-open contract history | Register an identified future contract at Go, collect its complete post-Go hour, then enter that exact contract at a later full boundary without discarding its history. Different contract ids cannot supply it; no pre-open checkpoint latches. Conflicting ids/pre-Go values reject; over-budget registration refuses before collection and preserves admitted histories. |
| Duration reachability | Reject a 900-second hold in `[0,900)`, including nested hold bounds that miss the end; allow completion strictly before end. This does not cap feasible hour-long historical lookbacks. |
| Grammar and arithmetic | Every tagged variant and supported source binding accepted where valid; duplicate/unknown/malformed/oversized/deep/cyclic/unit-incompatible/unbound-side rules rejected at a stable path. Fixed-decimal/tie/boundary examples in item 2, [Four-feed-difference rounding regression](#four-feed-difference-rounding-regression), genuine range-failure locality, both/no-side ties and deadline endpoints. No arbitrary three-sample gate. |
| Identity and temporal evidence | Equal prices with new ids count; duplicate/replayed/reconnected snapshots, quiet certificates and timers do not. A false update on another dependency resets confirmation/duration. First-below/gap recovery never fabricates a crossing; same-id conflicting contents invalidates affected inputs. |
| Revised composition/clock | Change-of-average and average-of-average run over the same complete clock with cumulative history. An uninterrupted source quiet from 1000 to 2000 ms can still produce a derived threshold failure in between; a hold must reset. Clock events never add confirmations or observed crossings. At a ten-second hold’s 14000-ms completion, a false same-batch value update resets it before any checkpoint latch. |
| Coverage staging | A value 100 at 0 plus a certificate received at 1000 covering `[0,1000)`, staged before advancing, produces a complete one-second average of 100. A true hole previously finalized cannot be repaired by that later certificate. Receipt time and finalized calculation frontier are distinct. |
| Startup and history | One-hour underlying prefill spans markets; change-of-average, average-of-average and percentage-history composition, cumulative source horizons, fixed full-clock semantics; no pre-Go data; exact-open readiness waits to next open; contract history never stitches contracts; demonstrably impossible capability reports unsupported rather than forever collecting. |
| Resource profile | Exact-cap and cap-plus-one import tests; side-expansion and required-history checks; complete dense allocation plan, admitted hour workloads and rejected five-hour-average graph; controlled ingress/allocation refusal returns no partially applied candidate and never quietly discards required evidence. Record actual allocated size and processing measurements without extrapolating to production concurrency. |
| Explanations and docs | Every operator/variant has an explanation check against the same validated structure; order, intervals, source bindings, six-place/one-millisecond calculation meaning, unavailable reasons and objective restrictions visible. Parse the complete learning JSON and reference snippets; verify relative links and perform item 7's walkthrough. No unsupported field may appear only in documentation. |
| Storage/compatibility | Fixtures for revisions 0/1/2 → 3, reopen idempotence, forced termination/failure before migration commit, mixed legacy/new/unknown rows, preserved session ids/content, no-update/no-delete/insert guards, old-version refusal, immutable explanation snapshots and read/write failures. A failed save leaves no incomplete new row; uncertain save directs inspection. |
| Native/interface | Bypass frontend and send invalid/oversized/unknown-operation bodies; bounded sanitized errors, local main-window-only access and no secret marker leakage. Preview writes nothing; changed/stale preview cannot enable save; save/list failure distinction, string ids, pagination/detail, edit-copy and safe literal rendering. The entry-point harness starts strategy-list independently of both deferred snapshot responses; an older snapshot cannot overwrite the newer Pause/status result. Direct IPC during a held native operation returns Busy without another task; success/error releases admission for the next call. Multibyte summary names can shorten a page below 50 while serialized bytes and cursor ordering stay correct. |
| Packaged app | On a disposable local workspace, choose the learning file, preview, save, reopen, inspect and edit-copy; confirm existing session history/control labels survive. Confirm no provider/Keychain access is triggered by strategy actions. Reuse unchanged credential evidence; no new credential operation is needed for #4. |

Required implementation checks: `cargo test --workspace --locked`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `npm run check`, `npm run build`, and an unsigned local `npm run tauri build -- --no-bundle` plus the focused packaged executable walkthrough above. No signed distribution or publishing. Reuse still-applicable passing checks; rerun affected evidence after fixes. Any automated browser check must first resolve an automation-only Chromium headless shell and disposable profile under Connor's standing instructions; unavailable safe browser access is a named blocked check, never a reason to launch desktop Chrome. Native packaged checks remain distinct from browser checks.

Record actual source state, command outcomes, synthetic fixture identities and missing checks in `docs/verification/issue-4.md` after authorized implementation. No listed engine/package check has run in this document-resolution session. Historical #2/#3 and standalone arithmetic evidence do not validate the dirty strategy implementation. The final handoff records the completed review and accepted text fixes; changed controlled mechanisms still require implementation and affected closure evidence.

Unresolved evidence is explicit: correctness of the proposed parser/evaluator/migration, the independent readability walkthrough, the allocation planner verified against actual capacities, bounded-service responsiveness and sustainable processing measured, and focused packaged behavior. Later #5/#9 must establish source access/licensing, identity/continuity mappings, reference definitions, history feasibility and production validity profiles. #6–#12 must establish purchasing/recovery and actual IOC behavior; #16 must measure run capacity; later release/security/live gates remain untouched. These are not proven by a concrete design, a supported import, paper outcomes or a closed prerequisite issue.


## Approval, Reconciliation and EGM Handoff

### Current resolution state and authority

Connor approved the revised Detailed Implementation, then selected the full consolidated EGM review and inclusion of both linked public Kalshi pages. The five-stage review completed through 10 native reviewer runs against the [preserved original target](strategy-rules-design.reviewed-2026-11-13.md), SHA-256 `3f6f8c23492a41131f56cb81e899860da5565a28519f61ac79ffbe081731c853`. Its 17 raw findings are mapped to 12 repairs in the [consolidated report](consolidated_review_report.md); those reports remain evidence for that original snapshot.

Connor instructed document resolution, reaffirmed the issue-4 boundary, and answered “Auto-accept” to the [annotated fixes](goldfish_review_report.md). This resolution revision incorporates all 12 accepted document fixes, covering all 17 raw findings. No finding was contested, escalated or declined. The annotated report records the new target hash, scope/ranges, actual document checks and unperformed verification. Applying text is not proof that the Rust mechanisms work or that an independent reviewer passed the changed target.

Current phase: bounded EGM document correction complete. The full initial review and focused independent recheck are complete; the four owner-authorized follow-up corrections and targeted document checks are recorded in [focused resolution](focused_resolution_report.md). No further review cycle was run. Implementation correctness remains unproven; reuse unaffected evidence rather than restarting brainstorming or the whole pipeline. No code/dependency change, implementation, build, actual feed collection, credential/account access, trading operation, paid service, GitHub mutation, commit or publication is authorized in this session. Issue #4 keeps desktop Start/Pause attached only to its controlled non-trading foundation.

### Recorded owner decisions and drafting history

- The concept, revised Overview & Goals, Technical Plan, Alternatives Considered and revised Detailed Implementation were approved in sequence. Their approval fixed the proposed design direction, not implementation authority.
- The owner required complete human and agent guides, readable JSON with intuitive names, and reusable asset-neutral logic while retaining BTC-first bindings and explicit source identity. A helper/skill and compatible future markets remain later possibilities rather than new issue-4 deliverables.
- The earlier Alternatives approval included: “Approve the revised Alternatives Considered. I don't know if I want you implementing the plan due to your context window”. The later instruction was: “Approve the revised Detailed Implementation. Let's move on to the next EGM step do not implement this in this session.” These are historical instructions; they are not commands to repeat completed drafting.
- EGM drafting Step 6.1 reconciliation against `concepts_brief.md` found no unresolved contradiction or unraised architecture/interface/dependency addition. Exact grammar, arithmetic, identity, bounds, compatibility and requests were reserved engineering definitions. This resolution supplies missing control/binding/admission and verification details within those issue-4 responsibilities. It adds no new author JSON operator, provider, strategy, dependency or market.
- At initial finalization, formal review had not started. It subsequently completed as recorded above; the original handoff's future review instructions are historical. Earlier brainstorm/targeted reports are preserved and are not represented as formal reviews of this changed revision.

### Evidence and handoff limits

The target and draft are synchronized at resolution. The original 664-line target is archived unchanged with its original relative links intact; the original stage/raw/source reports retain their hashes/scopes and severities. No directory sitemap existed and none was created. Documentation-led authoring usability, public-engine correctness, native admission, actual allocation/service responsiveness and packaged behavior remain required future evidence. Real source discovery/collection, source-specific validity profiles, paper/live execution/recovery and measured capacity stay with #5 onward. Do not treat this resolution as completion of those integrations or authorization to implement them.
