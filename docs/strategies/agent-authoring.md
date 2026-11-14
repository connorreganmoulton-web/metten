# Author a strategy with an agent

Use this recipe with the user's stated intent, [the shared field reference](reference.md) and [the shared learning file](../../examples/learning.json). The [human guide](authoring.md) explains the same file and local buttons. This is a repository guide, not a required installed skill or runtime service.

## Bounded task recipe

1. Restate the intended purchase conditions in plain English. Separate author thresholds from quantity, permitted user prices, daily limits and mode. Ask only when a consequential condition, source choice, side, time boundary or priority is missing. Do not invent profitability assumptions, D/F rules, confidence models or default provider substitutions.
2. Bind every variable to an exact supported source/instrument/field/unit and, for contracts, side. Distinguish buyable ask, bid, last trade, underlying prices, opening reference and settlement reference. The capability matrix proves language support, not entitlement, source access, connected execution or source-history feasibility. Never replace BRTI with Coinbase/Kraken silently.
3. Map intent to the reference's supported operations and condition variants. Use descriptive identifiers and an acyclic graph; compare compatible units. Keep side binding explicit. Decide strict versus inclusive thresholds, half-open intervals, unique-side ties, checkpoint latching and same/opposite selection. A continuing below-threshold condition uses `held_for` around `compare`, not around a crossing event. An explicitly later distinct observation uses `after_checkpoint_update`.
4. Explain the shared learning file: selected-side Kalshi buyable ask in cents; unique side below 40 in `[0,60)`; opposite first at most 30 in `[60,120)`, then remembered side at most 30. No unique checkpoint means no side; equality at 40 fails while 30 passes; time 120 is excluded. Completed memory survives gaps but cannot supply fresh purchase data. The array order determines priority.
5. Plan all post-Go source histories, including every alternative. Use the [complete reference examples](reference.md#complete-teaching-examples). A one-hour underlying average requires 3600000 ms. A five-minute average plus its one-minute change and one-minute average requires 360000 ms for the underlying input, because the delayed inner value itself needs five minutes. Check generated startup milliseconds, per-node/shared/per-market bytes, profile and implicit asks. No sampling, scheduling or storage code belongs in the JSON.
6. Produce UTF-8 format-2 JSON with only supported fields. Decimal quantities are strings; integer seconds/counts/format use integer tokens. No credentials, secrets, private runtime records, URLs, executable code, author-written explanation or run settings. Respect combined support limits, not just each independent ceiling.
7. Validate through the app's **Preview current text**, or through the public Rust engine interface below. Repair finite field-path errors using the reference, then preview again. Compare generated explanation against intent, including all boundaries, sources, selectors, priority and safeguards. Never claim that a plausible JSON file or an agent's explanation is validated evidence.
8. Present exact JSON, actual validation outcome, generated explanation/requirements, unresolved source limitations and the user's needed decisions. Save only in an authorized workspace. Follow [the human save/inspection/edit-copy steps](authoring.md#import-review-and-save): save reviewed bytes, inspect original, edit one field in a copy, preview/save as another identity, then prove the first JSON/explanation/id remain unchanged. Preview alone writes nothing. On uncertain save, inspect before retrying.

## Precision, startup and missing evidence

Calculations use six decimal places and one-millisecond time steps. Every named calculation normalizes with nearest-even rounding before reuse. The [four-feed regression](reference.md#four-feed-difference-rounding-regression) produces 0.001 percent for each intermediate percentage and the four-second average; do not average unnormalized fractions or rewrite the graph algebraically. Historical calculations cover the full grid, not the arriving-event count.

The controlled evaluator collects all required post-Go history, then waits for the next full market; readiness exactly at open waits for the following open. No pre-Go history or stitched contracts. Underlying history persists across compatible markets; exact contract history cannot transfer to a different contract. Market entry resets checkpoints, temporal conditions and objective. A genuine gap invalidates dependent calculations and downstream windows; a fresh snapshot cannot repair missing past cells. Verified quiet coverage supplies duration without observations, confirmations or crossings. Completed checkpoints survive gaps; independent paths can remain eligible. Active-market recovery does not restart initial prefill.

Unknown/in-flight attempts block further purchasing; confirmed zero fill allows reselection, any fill locks the side and leaves only the original remaining quantity. Current ask, permitted prices, funds, limits, recording and current admission still apply. Supported JSON cannot waive them. This app's Start/Pause controls only the existing non-trading session; strategy-run selection, source integration, paper qualification and live execution are later work.

## Public engine validation for a local exercise

`metten_engine::ValidatedStrategy::parse(&text)` returns a validated structure or finite `RuleError` records. On success, use `json()`, `explanation()` and `requirements()`; preserve the exact reviewed text. `requirements().startup_ms` is per input, in milliseconds. The same public `Engine::strategies` boundary supports `StrategyRequest::Preview { json }`, `Save { json, reviewed_json }`, `List { before_id }` and `Detail { version_id }`. Public response types are exported by the engine crate. Use a fresh disposable SQLite path for an authorized exercise; do not modify an existing workspace or read credentials. Build any temporary harness outside repository source, reusing the existing engine dependency. Local engine evidence does not substitute for packaged WebView verification.

## Completion record

Record authored JSON, generated explanation and requirements, actual validation errors/corrections, original/new version identities and unchanged original details. Explain each variable, unit, boundary, remembered side, priority, startup requirement, gap and common safeguard. Name every clarification needed beyond these guides: that is a documentation failure to correct. Do not claim source access, qualification, capacity or live fills from these checks.

### Minimal disposable validation harness

For an authorized local exercise, create a temporary Cargo binary outside the repository. Its manifest needs `metten-engine = { path = "/absolute/path/to/metten/crates/engine" }`. The following is API plumbing, not a strategy answer. Supply your own authored file and a fresh SQLite path as the two command arguments. Do not reuse an existing workspace. Use the pinned repository toolchain and `/Users/connormoulton/.cargo/bin/cargo` on Connor's Mac.

```rust
use metten_engine::{Engine, StrategyRequest as Request, StrategyResponse as Response};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&args[1]).unwrap();
    let engine = Engine::open(&args[2]).unwrap();
    match engine.strategies(Request::Preview { json: text.clone() }).unwrap() {
        Response::Preview { explanation, requirements, .. } => {
            println!("{explanation}\nRequirements: {requirements:?}");
        }
        Response::RuleErrors { errors, .. } => panic!("{errors:?}"),
        other => panic!("Unexpected response: {other:?}"),
    }
    let saved = match engine.strategies(Request::Save {
        json: text.clone(), reviewed_json: text,
    }).unwrap() {
        Response::Saved { version } => version,
        other => panic!("Unexpected response: {other:?}"),
    };
    let detail = match engine.strategies(Request::Detail {
        version_id: saved.id.parse().unwrap(),
    }).unwrap() {
        Response::Detail { version } => version,
        other => panic!("Unexpected response: {other:?}"),
    };
    println!("Saved {}: {detail:?}", saved.id);
    engine.stop_and_save().unwrap();
}
```

`StrategyDetail` has `summary`, optional `json`, optional `explanation`, optional `meaning_id` and `notice`. `summary` has string `id`, `recorded_at`, optional `display_name`, optional `format_revision` and `status`. Keep a clone of original detail JSON/explanation/id, save the one-field copy through the same preview/save sequence, and request the original detail again to assert equality. After `stop_and_save()`, drop the engine before reopening the same SQLite path; `snapshot().running` must be false. `Request::List { before_id: None }` returns `Response::Versions { versions, next_cursor }`; parsing the returned string id to the native integer is appropriate only in this Rust API, while IPC retains strings.
