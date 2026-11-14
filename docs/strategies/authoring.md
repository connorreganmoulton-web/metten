# Author your first strategy

You can inspect and save rules locally today. The app does not connect those rules to trading. Keep [the exact reference](reference.md) beside [the shared learning file](../../examples/learning.json); both the human and agent guides use these same sources.

## Read the file one piece at a time

JSON uses objects inside `{}` for named fields and arrays inside `[]` for ordered lists. Field names and strings use double quotes; commas separate entries, with no comma after the last entry. Save UTF-8 text with a `.json` extension. Do not add comments, duplicate keys, private records, credentials or an author-written explanation. Decimal prices are strings such as `"40"`; seconds, counts and format are integer numbers such as `60` and `2`.

Start with the learning intent: in the first minute, remember the one side whose buyable ask is below 40 cents. During the second minute, consider the opposite side at no more than 30 cents first, then the remembered side at no more than 30 cents. This is a teaching rule, not a recommendation.

1. `format: 2` fixes the rule meaning. `name` is only a display label.
2. `selected_side_ask` explicitly names Kalshi's current contract buyable ask, in `contract_cents`. It is evaluated separately for the side being considered. Bid, last trade and BTC/USD price are different values; they cannot replace it.
3. `early_side_checkpoint` observes `[0,60)` seconds from market open: 0 is included, 60 excluded. `less_than "40"` excludes 40. Its `unique_side` selector needs exactly one side true and the other false. Both qualifying is ambiguous; missing evidence cannot prove uniqueness. Once observed, the flag and side stay remembered until the next market, even through a gap.
4. The options observe `[60,120)`: 60 is included, 120 excluded. `at_most "30"` includes 30. `opposite_side_first` refers to the checkpoint's opposite side; `remembered_side_alternative` refers to its same side. The first eligible option in the array wins. With no remembered checkpoint, neither can select a side. The checkpoint does not require a crossing or three updates.
5. These conditions do not set quantity or the user's price range. Common engine safeguards still apply: a current buyable ask, permitted prices, funds, limits and durable recording. An unknown/in-flight purchase blocks another; zero fill permits reselection; any fill locks the side and only the original remaining quantity can be completed. These are controlled engine semantics; connected execution is later work.

The [reference](reference.md#fields-and-variants) explains how to add Boolean combinations, holds, distinct update confirmations, named calculations and nested priorities. To require a price to stay below a threshold, put a `compare` condition inside `held_for`; holding a one-event crossing is a different intent. Use `after_checkpoint_update` only if a genuinely later observation is required; ordinary checkpoint use can qualify in its establishing event.

## Import, review and save

1. In **Strategy versions**, choose the learning JSON with **Choose a strategy JSON file**. You may also paste JSON into **Strategy text**. The maximum is 256 KiB.
2. Click **Preview current text**. Preview writes nothing. Read the generated explanation and source-access notice. It must describe the named ask, strict/inclusive thresholds, intervals, remembered/opposite sides and option order above. The explanation comes from validated rules; it is not a separate author description or AI judgment.
3. Correct errors at their displayed field paths and preview again. Editing or choosing another file disables saving until the current text is reviewed. A slow reply from an older preview does not approve newer text.
4. Click **Save reviewed version**. Wait for the confirmed version identity. If saving is unconfirmed, inspect saved versions before retrying, because the write may have succeeded. A later list-refresh failure does not undo a confirmed save. **Refresh saved versions** and **More versions** retrieve the history.
5. Choose a saved version to inspect its original JSON, stored explanation, timestamp, meaning and status. **Supported** means supported rules, not live-qualified.
6. Click **Edit a copy**. Change only the first option's threshold from `"30"` to `"29"`. Preview again: the opposite-side condition must now say at most 29; the fallback remains 30. Save it and inspect both identities. The original must still contain 30 and its original explanation. Neither changing text nor saving a copy rewrites an earlier version.
7. Quit and reopen: saved versions and controlled-session history remain, while the session starts stopped. Choosing an inspected version does not select a trading run.

## History and gaps before any future run

The learning file needs no historical lookback. That does not authorize a mid-market start. In the controlled evaluator, Go first collects all required post-Go history across all alternatives, then waits for the next full market. If collection becomes ready exactly at open, entry waits for the following open. The present Start/Pause buttons only control the separate test session.

Use the [complete underlying and composition examples](reference.md#complete-teaching-examples) to learn calculations. The one-hour Coinbase average requires `underlying_price: 3600000` startup milliseconds. The five-minute average followed by a one-minute change or average requires `underlying_price: 360000` milliseconds: computing the average one minute ago already needs its preceding five minutes. The generated requirements show this cumulative six-minute coverage, plus shared/per-market history cost and implicit ask bindings. Both downstream calculations share the same inner definition.

Calculations use six decimal places and one-millisecond time steps. Each named operation rounds with ties to even before its output is reused. The [four-feed rounding example](reference.md#four-feed-difference-rounding-regression) yields 0.001 percent for each percentage and its four-second average. Rewriting a graph can change a last decimal place.

An unchanged source with verified complete coverage can supply time without supplying new observations. A real gap makes affected values unavailable until their full required history recovers; a snapshot cannot fill missing past cells. Independent paths may remain eligible, and completed checkpoints survive. Underlying history can span markets; one contract's history cannot become another's. Market entry resets checkpoint/temporal/objective state. Source access, exact reference definitions and feasible history collection still need separate provider integration evidence.

## Repair a rejected file

| Result | Next step |
| --- | --- |
| malformed / duplicate / unknown | Use valid JSON, one occurrence of each field, and only fields in the reference. |
| unsupported format, binding or operation | Use a listed capability; ask about missing intent instead of substituting a source. |
| reference / cycle / unbound side | Check identifier spelling, remove dependency cycles and give selected-side conditions an enclosing selector. |
| incompatible units / range | Compare compatible quantities; use quoted decimals with at most six fractional places and valid integer durations. |
| interval / unreachable duration | Keep start before exclusive end; a hold must finish strictly before end. A 900-second hold cannot fit `[0,900)`. |
| resource limit | Reduce the graph to fit the combined limits and preview requirements. Maxima do not all fit together. |
| Busy | Wait and retry the local action; keep your text and still-current preview. |
| storage / unconfirmed save | Inspect versions before repeating a save. Preserve the text while resolving the local recording problem. |

See [all errors, limits and legacy compatibility](reference.md#local-versions-and-finite-errors). No successful preview, saved version or local test establishes connected data access, production capacity, paper qualification, live acceptance or fills.
