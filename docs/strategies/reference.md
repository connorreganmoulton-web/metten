# Strategy field reference

This is the shared author reference for format `2`, meaning `metten_rules_2`, support profile `rules_2_initial`. It describes the validated rules and controlled evaluator, not a connected trading service. Source access is unverified. The Mac app currently previews and saves versions; Start/Pause control only the separate non-trading session.

Read [the human guide](authoring.md) or [the agent recipe](agent-authoring.md) with [the shared learning file](../../examples/learning.json). Rules contain no credentials, private records, quantity, permitted run-price range or daily limits. A rule threshold is an author condition, not the user's price setting.

## Fields and variants

Use integer `format: 2`, meaning `metten_rules_2`. Format 1 is the unfinished historical draft. All objects reject unknown and duplicate keys, including duplicates inside tagged variants. No comments, trailing commas, null values, numeric coercion or author-written explanation field. UTF-8 JSON only. Object key order has no meaning; array order matters for purchase priority and diagnostics. No source name or strategy name selects executable strategy code.

Top-level fields are exactly:

| Field | Required form and meaning |
| --- | --- |
| `format` | Required JSON integer 2. |
| `name` | Required display string containing a non-whitespace character, at most 120 Unicode scalar values, no control characters. It never changes evaluation. |
| `inputs` | Required array of 1–32 input definitions. |
| `measurements` | Optional array, default empty, at most 64 named calculations. |
| `checkpoints` | Optional array, default empty, at most 32 checkpoint definitions. |
| `purchase_options` | Required nonempty ordered array of option nodes; at least one purchase leaf overall. |

Identifiers use `[a-z][a-z0-9_]{0,63}`. Input, measurement and checkpoint ids share one namespace; option ids are unique in their own namespace. Names are case-sensitive. Forward references are allowed; unknown references and dependency cycles are rejected with the reference path. Checkpoint dependency order follows references, not author array order. No checkpoint may depend on a purchase result. Nodes, references and collection arrays must obey the support profile below.

**Input objects.** Common fields are required `id`, `source`, `instrument`, `field`, `unit`. Only contract fields additionally require `side`, one of `yes`, `no`, `selected_side`. A `selected_side` input is evaluated for the side being considered by that condition/option; it is not a global mutable binding. The engine accounts for required data for both sides when selection can use either. No other fields are accepted.

| Source | Instrument | Supported fields | Unit and identity |
| --- | --- | --- | --- |
| `kalshi` | `current_contract` | `buyable_ask`, `bid`, `last_trade` | `contract_cents`; side required; exact market and contract bound by the evaluator. Missing ask/bid/trade is unavailable, never zero. |
| `brti` | `btc_usd` | `index_value` | `usd_per_btc`; explicitly BRTI, with delivery transport left to the adapter. |
| `coinbase` | `btc_usd` | `last_trade` | `usd_per_btc`; Coinbase BTC/USD trades. |
| `kraken` | `btc_usd` | `last_trade` | `usd_per_btc`; Kraken BTC/USD trades. |
| `kalshi` | `current_market` | `opening_reference`, `settlement_reference` | `usd_per_btc`; market-rule-defined values, without a side. Unavailable until actually established; opening and settlement references are distinct. |

This is a language capability matrix, not a verified provider mapping. Particularly, the two reference fields cannot be populated by a convenient feed value. Connected source discovery remains later work. A strategy importing successfully does not establish access. Arbitrary URLs, synthetic provider names and implicit “best available” bindings are rejected. Tests supply controlled evidence for these same declarations, rather than making a synthetic source part of shipped JSON.

Units additionally include `percent` for derived percentage results/constants. USD-per-BTC operands may be compared across the explicitly named BTC/USD sources and with compatible market references. Contract cents may be compared across explicitly bound sides of the same current contract. No comparison across physical instruments, quote currencies or contract identities is inferred from a shared currency label. BTC-only import bindings are deliberate: future compatible 15-minute assets require an explicit contract extension and new binding/version, never silently replacing `btc_usd` or BRTI. Generic names such as `average_underlying_price` remain reusable without claiming same-file universal execution.

**Values and measurements.** A value is exactly either `{ "reference": "identifier" }` naming an input/measurement, or `{ "value": "decimal", "unit": "unit_name" }`. A measurement has required `id`, `operation` and the operands below; no implicit units or extra operands. Its unit is inferred from the operands; percentage operations produce `percent`.

| `operation` | Other required fields | Definition |
| --- | --- | --- |
| `difference`, `absolute_difference` | `left`, `right` values | Left minus right, optionally absolute; both operands have the same compatible unit/identity. |
| `percentage_difference` | `left`, `reference` values | `100 × (left − reference) / reference`; denominator nonzero, otherwise unavailable. Output percent. |
| `change`, `percentage_change` | `of` value, `lookback_seconds` | Current minus boundary value; percentage form divides by the nonzero boundary value, retaining its sign, and multiplies by 100. |
| `rolling_average`, `rolling_minimum`, `rolling_maximum`, `rolling_range` | `of` value, `lookback_seconds` | Complete trailing-window calculation; range is maximum minus minimum. |

Historical `of` must reference an input or measurement, not a constant. Every supported operation may use another supported measurement, including a historical one, within compatible units, an acyclic graph and the support profile below. There is no historical-operation blacklist or operator-pair exception table. Define `average_underlying_price` once, then use it in a comparison, a change, another rolling calculation or a percentage expression. The engine resolves the shared graph; authors do not write sampling, storage or scheduling code. Measurements cannot read checkpoint or purchase state.

The uniform meaning is the calculation series defined below: all measurements have fixed decimal values on a complete one-millisecond timeline. Aggregating a moving average means aggregating that timeline of computed values, not attempting an unrestricted continuous mathematical integral. This is an explicit format-wide time/precision definition, not silently averaging whichever samples happened to arrive. It supports average-of-average, change-of-average and average-of-percentage with the same existing operations, no symbolic integrator, arbitrary-size fractions, extra dependencies or new author-facing sampling options. A given definition is shared and evaluated once per relevant event/tick and side binding.

**Conditions.** Conditions are tagged by required `type`. The table lists every variant and its remaining required/optional fields; no unlisted fields or defaults apply. Child arrays are nonempty.

| `type` | Fields | Meaning |
| --- | --- | --- |
| `compare` | `left`, `comparison`, `right` | Compatible values; comparison is `less_than`, `at_most`, `equal_to`, `at_least`, `greater_than`. |
| `all`, `any` | `conditions` array | Three-state Boolean combination using the truth rules below. |
| `crossed_below`, `crossed_above` | `left`, `right` | Observed strict crossing of compatible values. |
| `held_for` | `condition`, `duration_seconds` | Continuous truth and required evidence for the duration. |
| `confirmed_updates` | `condition`, `count`, `count_input` | Consecutive true evaluations on distinct observations of the explicitly named input. |
| `checkpoint_reached` | `checkpoint` | Latched Boolean; false until established. Establishment dependencies are no longer current dependencies. |
| `after_checkpoint_update` | `checkpoint`, `count_input` | Latched checkpoint plus a distinct observation of the named input accepted strictly after its establishing event. |

`count_input` names a raw input, not a measurement. A later update must also be later than the identity high-water mark for that input when the checkpoint latched; a duplicate or same-event batch member cannot satisfy it. A confirmation count is an integer 1–10000. An unrelated update or timer cannot increment it, but can reset it on false/unavailable evidence. The count saturates; further true observations preserve success until failure, unavailability or market reset. A crossing is true only for its causing accepted observation event and cannot be manufactured or prolonged by a timer. Consequently holding a crossing true over time does not mean holding the crossed price relation; use `held_for(compare(...))` for that intent. Side-dependent conditions maintain separate state for YES and NO; scalar ones can share state. Conditions stay updated even in currently losing priority branches, so a past failure is never hidden by short-circuit evaluation.

**Side selectors, checkpoints and options.** A selector is exactly `{ "type": "fixed", "side": "yes" | "no" }`, `{ "type": "checkpoint_side", "checkpoint": "id", "relation": "same" | "opposite" }`, or `{ "type": "unique_side" }`. In the last form, evaluate the enclosing condition for YES and NO: select only if exactly one is true and the other false; both true is `ambiguous_side`, either unavailable prevents proof of uniqueness. Do not break ties by iteration order. Fixed or remembered selectors evaluate their conditions only for the selected side. A checkpoint-side reference must name a side-bearing checkpoint; a Boolean-only checkpoint cannot be used for side selection.

Each checkpoint requires `id`, `when`, and may have `during`, `remember_side`. Without `remember_side`, latch a scalar Boolean; reject any unbound `selected_side` reference in its transitive conditions. With a selector, evaluate `when` for its selected side and latch both flag and side. Uniqueness uses `when` itself, without a second hidden predicate. If no unique side exists now, keep checking through the interval. Once established, never rewrite its time, event or side. Side/checkpoint dependency cycles are rejected.

Each option node requires `id`, `when`, and optionally `during`. It then has exactly one of `options` (nonempty ordered child nodes) or `side` (a selector, making it a purchase leaf). Evaluate groups and their leaf together for the leaf's candidate side; `unique_side` tests the full ancestor-and-leaf condition. The first eligible leaf in depth-first array order wins after common objective restrictions. Unavailable preferred branches do not globally block independent alternatives. No “used option” flag exists. Prior attempts and fills are engine-owned constraints, not strategy fields.

A `during` object has exactly required `start_seconds` and `end_seconds`, integers with `0 <= start < end <= 900`; omitted means `[0,900)`. Seconds are elapsed from the identified market open. End is an exclusive deadline. Nested intervals intersect; reject empty intersections. A checkpoint can be used after its own observation interval ends. Temporal tracking for a node begins no earlier than its effective interval; a pre-window duration/confirmation cannot prequalify it. Historical measurements still use their declared trailing data. Evaluate the expiry/boundary before same-time eligibility, so no event at close or exactly at an end can authorize a purchase. These are BTC15m boundaries; future market schedules need explicit compatible definitions.

**Reachability of market-local durations.**

Validation rejects a demonstrably unreachable market-local hold using `unreachable_duration` at its rule path. Compute the effective interval as the intersection of the node and ancestor intervals. Its end is exclusive. A held_for duration must have a possible completion strictly before that end: `[0,900)` with a 900-second duration is rejected, not accepted as a permanent wait. The existing broad numeric duration bound is only a syntax bound; semantic interval validation is mandatory.

For this time-only reachability check, a current comparison/crossing/confirmation has earliest possible success at the effective interval's start; do not assume a provider rate or minimum millisecond spacing between genuine same-time transactions. `all` takes the maximum child lower bound, `any` the minimum, and held_for adds its duration to the child's lower bound. Checkpoint references use the referenced checkpoint's earliest possible establishment, and intervals impose their start. Process checkpoint dependencies in their existing acyclic order. If a held_for or enclosing condition has an earliest possible success at or beyond its exclusive end, return field-specific repair guidance. This is a conservative bound, not a theorem prover for all numerical conditions or market profitability.

This check does not cap historical lookbacks at market length: applicable underlying and exactly identified pre-open contract history may already exist. It must not reject feasible hour-long measurements merely because the outcome window is 15 minutes.

## Decimal arithmetic and time

**Public precision.** All literal/raw decimal values are strings matching `-?(0|[1-9][0-9]*)(\.[0-9]{1,6})?`, with at most 12 digits before the point. No exponent, whitespace, plus sign, negative zero or NaN/infinity. Raw contract prices are 0–100 inclusive; raw USD-per-BTC prices are positive. Signed changes/differences and percent results/constants may be negative. JSON integer fields use integer tokens without fractions/exponents. Author durations and lookbacks are positive integer seconds.

Use six fractional decimal places for every input and measurement, represented as signed integer millionths in `i128`. Pad shorter literals exactly; reject input precision greater than six places rather than silently round a feed. Trusted adapters must establish that their field mapping fits this precision or report unsupported. Every derived operation produces its own six-place value, rounding to nearest with ties to even; all comparisons, including equality and crossing thresholds, use those values exactly. **Calculations use six decimal places and one-millisecond time steps.** A threshold at that precision is intentional, not a claim about a more precise continuous quantity. No hidden epsilon, binary floating-point equality, unchecked cast or UI-only rounding.

Round once at each operation's output, never per term inside that operation's sum. Named graph nodes define normalization boundaries, so algebraically rewriting a graph can change a last-place result; do not optimize by reassociating or eliminating those boundaries. Reusing a named measurement always reuses its same value. Display the exact normalized value, with unnecessary trailing zeroes omitted; there is no different hidden decision value.

**Arithmetic bounds.** Each derived value is limited to magnitude `10^18` in its declared unit (scaled magnitude `10^24`). Exceeding that limit yields `numeric_range_exceeded` for that value and its dependents; never saturate/wrap. This allows six whole orders of magnitude above the largest literal/raw value. A denominator must be nonzero before division; signed derived denominators are permitted and retain their algebraic sign, while raw USD-per-BTC inputs remain positive; a positive value that normalized to zero is zero for subsequent nodes and makes division unavailable. Constants can be prevalidated using these same rules. Arithmetic uses checked integer operations even where the bounds prove they fit.

For scaled operands A and B, a difference is A−B; a percentage difference's scaled result is nearest-even `(A−B) × 100000000 / B`. A change uses the current and delayed scaled values; percentage change uses the delayed value as B. A rolling average is nearest-even `sum(scaled operand cells) / number_of_cells`. Absolute difference and min/max/range use scaled integer values. With the maximum lookback of 3600000 cells, an average sum's magnitude is at most `3.6 × 10^30`; the percentage intermediate is at most `2 × 10^32`. Both are well below signed `i128` capacity. Denominators do not accumulate between operations. The four-feed example below gives exact expected values.

Precision examples: 1/3 becomes 0.333333, 2/3 becomes 0.666667; a tie at 1.2345645 becomes 1.234564 and a tie at 1.2345655 becomes 1.234566, with the symmetric rule for negatives. The [Four-feed-difference rounding regression](#four-feed-difference-rounding-regression) gives 0.001 percent after normalization without fraction-denominator accumulation. A normalized value equal to its threshold satisfies `equal_to`/`at_most`, not `less_than`. The product does not claim greater precision.

#### Four-feed-difference rounding regression

Coinbase BTC/USD last_trade is `100001` throughout `[0,4000)` ms. Kraken BTC/USD last_trade is `100000.000003`, `100000.000019`, `100000.000057`, `100000.000063` over successive one-second intervals `[0,1000)`, `[1000,2000)`, `[2000,3000)`, `[3000,4000)`. All observations/coverage are controlled, post-Go, valid and complete.

Define `feed_difference_percent = percentage_difference(left=coinbase_price, reference=kraken_price)`, then `average_feed_difference_percent = rolling_average(of=feed_difference_percent, lookback_seconds=4)`. Each percentage node normalizes to `0.001 percent` under the existing six-place nearest-even policy. At 4000 ms, its four-second average also normalizes to `0.001 percent`. These exact operands and output boundaries are the regression fixture. This is a calculation example, not another shipped trading strategy or a provider-access claim.

#### Calculation clock and coverage

**One calculation clock.** Engine time is nondecreasing integer milliseconds since Go. Preserve accepted transaction order inside a millisecond. A raw value becomes the current value when its trusted normalized event is accepted; it is never backdated to authorize past actions. Provider effective/receipt timing is checked by the adapter before acceptance. UTC remains the recording/display time, not the duration clock. Advancing engine time processes every intervening logical millisecond. A clock step is a computation, not a provider observation, trade, confirmation or proof of source health.

Before advancing a batch, stage its trusted coverage/validity facts as described below without applying future values or conditions early. At millisecond k, finalize cell `[k−1,k)` using its last accepted value and staged evidence. Apply expiry, gaps and market boundaries effective at k, reset the relevant state, and prepare all time-dependent measurement results for k. If value transactions are scheduled at k, apply the first complete atomic transaction and recompute affected values **before** evaluating conditions/checkpoints; process subsequent genuine transactions in order, each with its own coherent evaluation. Do not insert a separate pre-transaction clock eligibility evaluation at that same instant. If there are no value transactions at k, evaluate the clock-driven state after the expiry/boundary changes. After the last transaction, there is no extra provider observation. Conditions/checkpoints/purchases follow the same order at every historical catch-up instant and the final target instant.

This prevents a hold from completing on an old value immediately before a false update in the same accepted batch: a ten-second hold started at 4000 ms, with its condition false in the batch's 14000-ms value transaction, resets rather than latching. A timer batch already accepted and completed before a subsequently received observation is a different established event; later evidence cannot rewrite its outcome. Known invalidity/expiry at a timestamp always precedes eligibility there. Separate genuine observations with equal timestamps remain individually evaluated, so a true observation followed by a false observation can establish then preserve an already observed checkpoint, exactly as their accepted order requires.

All changes within a current cell remain visible to temporal/checkpoint/confirmation evaluation; the final state in the cell supplies its time-weighted value. Earlier same-millisecond values occupy zero duration at this clock resolution; they cannot be discarded before their event-based effects are evaluated. A false condition followed by true at the same millisecond still resets an unfinished hold/confirmation. Every invalidity during a cell marks its dependent historical cell unavailable even if the final state recovers. This conservative coverage bit prevents a very short outage from being hidden by a final snapshot.

A historical window at k contains exactly the W completed cells `[k−W,k)`, where W is 1000 × `lookback_seconds`. Each cell has equal one-millisecond duration. An average is the time-weighted sum divided by W, including every required cell. Min/max/range operate on those same cells. They have attained extrema in this discrete domain; no special continuous supremum meaning is needed. The value at k enters a subsequent window, never the one ending at k. `change` compares current with the finalized operand at k−W and additionally requires complete operand coverage from that boundary through current evaluation. Any missing cell makes the calculation unavailable; do not shrink its denominator or skip a missing point.

For a raw price stream timestamped to this precision, averaging these complete cells is time weighting of the verified value intervals. For a derived stream, it is explicitly the time average of the six-place values computed on this clock. It is not an event-count average and not the exact continuous integral of an unrounded derived curve. This single fixed resolution avoids a curve algebra, pair-specific exceptions and author-chosen sampling modes. It cannot distinguish the duration of two events inside one millisecond, but still processes both observation events. This precision limit is documented in the same reference and preview as the numeric limit.

A raw price unchanged under valid, complete evidence can cover many cells without new observations. A rolling average still gets recomputed as old cells leave its window. Derived thresholds and hold failures are checked at every logical millisecond even during quiet periods. Scheduling frequency, UI polling and batching cannot change the result. Implementation may skip a span only when it proves the exact same normalized values and condition transitions as the logical tick-by-tick contract; such an optimization is unnecessary for the initial implementation and requires an oracle check if later added. Current-only rules still react to every accepted observation without waiting for the next clock tick. Any loss of a crossing’s required continuity clears its prior observed baseline. Crossing compares successive genuine observation evaluations of its dependencies; clock changes do not advance that observation baseline or fabricate a crossing. Timers/clock steps may reset a confirming sequence or hold but never increment an update count.

**Coverage and cumulative history.** Define history requirements recursively: an input has zero warmup, pointwise operations take the maximum of their operand warmups, and a historical operation adds its lookback to its operand warmup. Compute this per bound source/contract, not just as a single strategy number. A one-minute change of a five-minute average needs six minutes of source coverage. Averaging a five-minute average over ten minutes needs fifteen minutes. The same rule propagates gap unavailability through nested calculations. A current snapshot starts present state; it cannot supply missing past cells. Finalized cells are immutable: late evidence cannot rewrite earlier calculated outputs or retroactively establish a checkpoint; an unrecoverable hole must leave the entire affected horizon before that path becomes available. Known earlier good cells remain intact. No pre-Go history and no joining different contracts.

Live adapters substantiate validity and completeness; this calculation clock does not certify either. A cell is usable only when its own bound dependencies have the required current-validity and complete-coverage evidence. Verified quiet cells contain the previously established value; genuinely missing evidence leaves unavailable cells. If later evidence contradicts a previously claimed complete interval, invalidate affected retained coverage and dependent calculations and report inconsistent evidence; do not silently preserve a usable history or recompute old checkpoints as though they had been observed differently. Rebuilding affected historical operator state is allowed solely from retained trusted cells and has no retroactive actions. Actual adapter correctness, latency/continuity profiles and their source-specific numerical limits remain #5/#9 evidence.

**Boundaries and demonstrable behavior.** A market is `[open,close)`; enter it only via its boundary event. A deadline at 120 seconds excludes 120000 ms and includes 119999 ms. A ten-second hold starting at 4000 ms first qualifies at 14000 ms if every required intermediate condition/evidence remained valid, including at completion. Process invalidity/expiry at an instant before eligibility at that instant. Initial readiness completed exactly at a market open still waits for the following open. Crash/restart remains stopped.

Boundary examples: 100 for three seconds then 110 for two seconds has five-second average 104, min 100, max 110, range 10. Its point change at the endpoint enters only the next historical cell. Over the last three milliseconds, values 1,2,2 average to 1.666667, which is greater than 1.666666 and equal to 1.666667. A one-minute change in an average compares the actual normalized average cells, not a nearest convenient provider sample. An average-of-average incorporates each changing inner value on the same clock. A false excursion at any logical millisecond resets a hold even if values at the two surrounding observations were true. A gap invalidates the full affected calculation window and then its downstream windows. These checks establish this fixed finite-resolution language, not continuous market truth or live fills.

## Support limits

The support profile is `rules_2_initial` in previews/errors. The language keeps the same composable operations; the profile limits total work and retained state. Independent ceilings do not promise that all maxima fit together.

| Resource | Ceiling and accounting |
| --- | --- |
| Document | 256 KiB UTF-8, lexical JSON depth 48, decoded strings individually at most 1024 bytes except name's tighter bound.  |
| Rule structure | 512 condition/option nodes, condition depth 12, option depth 8; inputs/measurements/checkpoints as above. Count side-expanded condition state too, at most 1024 instances. |
| Calculation graph | At most 128 numeric instances after input/measurement/side and concrete market-binding expansion; acyclic measurement depth at most 12. Share identical resolved instances; references do not copy their history. This permits chained supported operations without unbounded work. |
| Time | Each lookback 1–3600 seconds; cumulative source warmup at most 14400 seconds. Each hold duration 1–86400 seconds, requiring only bounded temporal state. An hour is supported for every historical capability on a single bound series; longer collection can arise from admitted compositions. No limit derives from the 900-second market length. |
| Planned working memory | At most 256 MiB per evaluator, including retained values, deque/delay buffers, bitmaps if needed, graph/state, current bindings, scratch and decision buffers. Reject excess during preview/save, not after hours of prefill. Reserve 8 MiB for all non-history state within this total; Fixed-capacity structures are charged within the reserve. |
| Pending evidence | Pending plus queued normalized input totals at most 8 MiB and 4096 atomic transactions, whichever binds first. Do not grant a fresh allowance to each waiting batch. Actual adapter queue/scheduling integration remains #5/#9. |
| Output | Generated explanation at most 128 KiB without reference expansion; version summary page at most 50 complete summaries and 32 KiB for the entire serialized UTF-8 response, including envelope/cursor fields, single detail/preview at most 1 MiB. Legacy raw detail above 1 MiB remains preserved in storage and is reported too large to show completely, never silently truncated. |

The current parser reserves two implicit ask slots within the 128 numeric-instance ceiling, even when asks are explicit; document input/measurement expansion must therefore be at most 126. Depth limits count the root as the first level. Preview reports per-node history bytes, shared underlying bytes, per-market bytes, planned bytes and per-input startup milliseconds. Combined graphs must fit all limits. Production throughput and concurrent capacity remain unverified.

## Truth, observations and current eligibility

Conditions have true, false and unavailable results. For `all`, any false makes false; all true makes true; otherwise unavailable. For `any`, any true makes true; all false makes false; otherwise unavailable. Unavailable preferred branches do not suppress independent eligible alternatives. A unique-side selector requires exactly true/false, never true/unavailable.

Trusted adapters bind a source to an exact field/side/market/contract and supply monotone observation numbers. Equal prices with genuinely new identities count; duplicates, replay, reconnect snapshots, quiet certificates and timers do not. An atomic transaction applies all members before evaluation; separate genuine transactions at the same millisecond retain order. `after_checkpoint_update` needs a later transaction and an observation beyond the checkpoint's captured high-water mark. A resynchronization restores present state, never historical coverage or an observed crossing.

Coverage and current validity are separate. Trusted coverage is staged before advancing the calculation clock, without applying future prices early. A value of 100 at time 0 plus certified quiet coverage `[0,1000)` gives a one-second average of 100. A current-validity lease does not certify future complete history. Finalized missing cells cannot later be repaired. Contradictions invalidate dependent history, never invent past checkpoint changes. These facts come from the engine/adapters, never from strategy JSON.

All required histories across all alternatives are collected after Go, then the evaluator waits for the next full market. Readiness exactly at open waits for the following open. Zero lookback means zero historical collection, not permission to enter midway. Underlying history can span markets; contract history belongs to the exact registered contract, including its own admitted pre-open data. New market entry resets checkpoints, temporal conditions and objective, retaining only applicable history. Active-market recovery does not repeat the initial run-wide startup wait. Provider feasibility for requested history remains unverified; an impossible binding must be reported unsupported rather than collected forever. Reopening the application is stopped.

Every candidate additionally needs a current buyable ask, permitted user price, funds/limits/recording and common admission safeguards. Unknown or in-flight attempts block all purchasing. Confirmed zero fill allows reselection; any confirmed fill locks the side and leaves only the original remaining quantity. The JSON cannot override these controls. Controlled candidates are not submitted orders, paper qualification or live fills.

## Local versions and finite errors

Preview validates exact editor text and generates its explanation and requirements without writing. Save requires byte-identical reviewed/current text, revalidates it and appends exact JSON, meaning, explanation, name and UTC timestamp atomically. Editing or choosing another file invalidates the preview. A confirmed saved identity remains saved even if refreshing its list fails. If save is unconfirmed, inspect versions before retrying; there is no automatic retry.

Versions are newest first. More versions pages at most 50 complete summaries and 32 KiB per serialized response; ids/cursors are positive decimal strings, not JavaScript numbers. Detail and preview are at most 1 MiB; oversized historical text is preserved but cannot be displayed completely. Edit a copy loads text for another preview/save; it never changes the inspected row. Strategy operations admit one native action at a time; Busy means wait and retry, keeping current text and a still-current preview.

Supported means the rules are understood, not live-qualified. `legacy_read_only` preserves original legacy JSON and only an explanation actually stored at the time. `unsupported` preserves inspection without promising execution of future meanings. `invalid_record` reports inconsistent stored data without repair. Missing historical names remain unknown. New format-2 versions require fixed meaning `metten_rules_2` and matching validated metadata. Conversion requires a newly valid file and explicit save. Storage revision 3 migrates prior rows without rewriting them; older apps refuse the newer workspace. There is no silent converter or permanent promise to execute every historical evaluator.

Rule errors contain `path`, `code`, `message`, `accepted_form`. Fix the reported field and preview again. Parsing may stop at the first error; the interface caps rule-error responses at 20 with a truncation flag. Codes include `malformed`, `duplicate`, `unknown`, `unsupported_format`, `unsupported_binding`, `unsupported_operation`, `reference`, `cycle`, `incompatible_units`, `range`, `interval`, `unreachable_duration`, `unbound_side`, `resource_limit`. Runtime values may report `zero_denominator`, `numeric_range_exceeded` or `unavailable`. Messages are finite and do not echo submitted values.

Local action errors include `invalid_request`, `busy`, `quitting`, `not_reviewed`, `not_found`, `resource_limit`, `storage`. Refresh after a missing version; preview after editing; wait for Busy/quitting; inspect versions before repeating an uncertain storage/save action. A too-large response does not delete history. No raw database/framework diagnostics or submitted content should appear in errors.

## Complete teaching examples

Each JSON block below is a complete importable document. They teach calculations and source identity, not additional recommended strategies. Both guides use these examples and the single shared learning file.

### One-hour underlying history and explicit cross-feed identities

```json
{
  "format": 2,
  "name": "Teaching: one hour underlying",
  "inputs": [
    {
      "id": "underlying_price",
      "source": "coinbase",
      "instrument": "btc_usd",
      "field": "last_trade",
      "unit": "usd_per_btc"
    },
    {
      "id": "brti_price",
      "source": "brti",
      "instrument": "btc_usd",
      "field": "index_value",
      "unit": "usd_per_btc"
    },
    {
      "id": "kraken_price",
      "source": "kraken",
      "instrument": "btc_usd",
      "field": "last_trade",
      "unit": "usd_per_btc"
    }
  ],
  "measurements": [
    {
      "id": "average_underlying_price",
      "operation": "rolling_average",
      "of": {
        "reference": "underlying_price"
      },
      "lookback_seconds": 3600
    }
  ],
  "purchase_options": [
    {
      "id": "consider_yes",
      "side": {
        "type": "fixed",
        "side": "yes"
      },
      "when": {
        "type": "all",
        "conditions": [
          {
            "type": "compare",
            "left": {
              "reference": "underlying_price"
            },
            "comparison": "at_least",
            "right": {
              "reference": "average_underlying_price"
            }
          },
          {
            "type": "compare",
            "left": {
              "reference": "brti_price"
            },
            "comparison": "at_least",
            "right": {
              "reference": "kraken_price"
            }
          }
        ]
      }
    }
  ]
}
```

### Composition and cumulative history

```json
{
  "format": 2,
  "name": "Teaching: composed history",
  "inputs": [
    {
      "id": "underlying_price",
      "source": "coinbase",
      "instrument": "btc_usd",
      "field": "last_trade",
      "unit": "usd_per_btc"
    }
  ],
  "measurements": [
    {
      "id": "average_underlying_price",
      "operation": "rolling_average",
      "of": {
        "reference": "underlying_price"
      },
      "lookback_seconds": 300
    },
    {
      "id": "change_in_average",
      "operation": "change",
      "of": {
        "reference": "average_underlying_price"
      },
      "lookback_seconds": 60
    },
    {
      "id": "smoothed_average",
      "operation": "rolling_average",
      "of": {
        "reference": "average_underlying_price"
      },
      "lookback_seconds": 60
    }
  ],
  "purchase_options": [
    {
      "id": "consider_yes",
      "side": {
        "type": "fixed",
        "side": "yes"
      },
      "when": {
        "type": "compare",
        "left": {
          "reference": "change_in_average"
        },
        "comparison": "at_least",
        "right": {
          "value": "0",
          "unit": "usd_per_btc"
        }
      }
    }
  ]
}
```

### Four-feed-difference rounding document

```json
{
  "format": 2,
  "name": "Teaching: rounded percentage series",
  "inputs": [
    {
      "id": "coinbase_price",
      "source": "coinbase",
      "instrument": "btc_usd",
      "field": "last_trade",
      "unit": "usd_per_btc"
    },
    {
      "id": "kraken_price",
      "source": "kraken",
      "instrument": "btc_usd",
      "field": "last_trade",
      "unit": "usd_per_btc"
    }
  ],
  "measurements": [
    {
      "id": "feed_difference_percent",
      "operation": "percentage_difference",
      "left": {
        "reference": "coinbase_price"
      },
      "reference": {
        "reference": "kraken_price"
      }
    },
    {
      "id": "average_feed_difference_percent",
      "operation": "rolling_average",
      "of": {
        "reference": "feed_difference_percent"
      },
      "lookback_seconds": 4
    }
  ],
  "purchase_options": [
    {
      "id": "consider_yes",
      "side": {
        "type": "fixed",
        "side": "yes"
      },
      "when": {
        "type": "compare",
        "left": {
          "reference": "average_feed_difference_percent"
        },
        "comparison": "at_least",
        "right": {
          "value": "0",
          "unit": "percent"
        }
      }
    }
  ]
}
```

The one-hour example needs `underlying_price: 3600000` startup milliseconds; BRTI and Kraken require current values. It also needs implicit YES/NO buyable asks for common price checks. No bid or last trade substitutes for a buyable ask.

The composition defines the five-minute average, its one-minute change and its one-minute smoothed average. Each downstream calculation needs six minutes of original source coverage: to compute an average one minute ago, its preceding five minutes must already exist. Requirements therefore report `underlying_price: 360000` milliseconds, not 300000 or 60000. Graph nodes retain six-place values on the same one-millisecond clock. See the four-feed controlled values above for rounding before aggregation.
