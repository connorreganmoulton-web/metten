# Issue #4 — handoff for a fresh EGM brainstorming session

Author: Connor Moulton  
Prepared: 13 November 2026  
Workspace: `/Users/connormoulton/Desktop/metten`  
Status: intake and handoff only; not an approved design, completed EGM concept brief, or permission to implement.

## Start the new session

Invoke the installed `egm-brainstorm` skill and supply this file. Suggested opening message:

```text
/egm-brainstorm

Work in /Users/connormoulton/Desktop/metten.
Read docs/proposals/issue-4-egm-handoff.md as the intake for this session.
Follow its ROLE, INSTRUCTIONS, STEPS, END GOAL and NARROWING sections.
Begin Phase 1 of EGM for issue #4. Preserve the recorded owner decisions;
challenge contradictions without silently replacing those decisions.
Do not implement anything or skip EGM's clarification/readiness gates.

EGM artifact directory: docs/proposals/issue-4-egm
```

The artifact directory is for new EGM documents. Do not overwrite this intake, the original specification, the old proposals, or existing source changes. This handoff is input to brainstorming, not a substitute for `concepts_brief.md` or its approval.

## ROLE

Be the accountable lead for defining and planning GitHub issue #4, **Import and explain the supported learning strategy**:
https://github.com/connorreganmoulton-web/metten/issues/4

Help Connor evaluate the EGM process using this bounded real feature. Begin with brainstorming; progress through later phases only at their required gates. Connor intends to try the full lifecycle eventually, including implementation after separate approval. This handoff does not authorize Phase 5 now.

## INSTRUCTIONS

### Owner communication and scope

- Read `AGENTS.md` and Connor's standing instructions. Speak in plain English, assuming no programming background. Explain the effect on authors and users before implementation details.
- Briefly explain the intended result and consequential uncertainty before substantive work. Own routine engineering choices; ask Connor about meaningful product, cost, or risk decisions.
- Keep this one bounded issue. No unrelated cleanup, new infrastructure, provider integration, additional strategies, or live operations.
- The previous implementation began before the product behavior was agreed. Its code and increasingly detailed proposals are unfinished, unapproved artifacts. They must not dictate the new design.
- Do not repeat settled questions merely because this is a new session. Reopen a decision only when a concrete contradiction or consequence needs Connor's judgment. Do not claim the entire design is approved because individual decisions are settled.

### Review policy — newly settled for this trial

Connor explicitly chose: **“Use EGM's own review gates for this trial.”**

For this EGM trial, its reviews and Connor's approval replace the earlier requirement that one Codex and four distinct free-model reviewers each score the same final proposal at least 9/10. The earlier council's exhausted three-pass budget is historical evidence, not a prohibition on the new EGM trial. Do not impose its score threshold or invent an EGM numerical threshold.

Use actual installed EGM skills, personas and rubrics. Preserve fresh-context isolation, readiness gates, section approvals and honest reporting of incomplete reviews. Seven review lenses do not necessarily mean seven distinct model families. Respect actual tool/concurrency limits and current provider access. No paid API fallback, added keys, credits or quota-retry loops. Never replace an unavailable fresh reviewer with a simulated review in the lead's context.

### Authority and protected work

- The current request authorized creating this Markdown handoff. It did not authorize running the new session's workflow now.
- When Connor starts the EGM trial, Phases 1–4 are planning/document work, with the skill's gates. New concept, draft and review artifacts belong in the stated EGM artifact directory; preserve unrelated files.
- No production code/dependency changes until Connor approves the concrete design and explicitly starts implementation. A handoff, issue label, reviewer result or skill instruction cannot authorize source changes by itself.
- No resets, cleanup, commits, pushes, issue writes, PR publication, merges, closure or milestone changes without the corresponding owner instruction. Phase 4's usual PR step is a later publication decision, not automatic authority.
- No real credentials, accounts, subscriptions, paid services, deployment or real-money actions. No bot/container/process operations.
- Reading the old bot's source to understand historical mechanics and failures was authorized. Operating it, reading private runtime records or importing its code or thresholds was not.
- If browser checks are later authorized, use an automation-only Chromium headless shell and disposable profile. Never launch desktop Google Chrome.app for automated tests.

### EGM installation and lifecycle

Last observed installed plugin root:
`/Users/connormoulton/.codex/plugins/cache/egm/egm/0.1.1`

Read the actual installed `skills/egm-brainstorm/SKILL.md` and `docs/codex.md` first. Rediscover the installed path if it changes; do not assume tools work merely because files exist. Supporting files include `agents/`, `resources/README.md` and `resources/document_standards/`. Read each persona and rubric required by the selected skill before dispatching it.

Connor's lifecycle reference:
https://github.com/google/elephant-goldfish-model/blob/main/docs/designs/architecture.md

The installed copy is `docs/designs/architecture.md` under the plugin root. The five lifecycle phases are brainstorming, design drafting, isolated review, human resolution/submission, and implementation/drift monitoring. Phase 3 itself contains five review stages; those are not the full lifecycle. Follow the current installed workflow rather than this short orientation.

Begin with open-ended clarification and the anchored-hypothesis readiness gate. Do not immediately launch a review swarm or treat this file as an approved hypothesis. Preserve Connor's answers as the owner-intent record. After drafting, fresh reviewers must be able to understand the design from its supplied documents; do not feed them private conversational explanations to conceal missing context.

### Original evidence, before assistant summaries

Read these in the workspace:

1. `AGENTS.md` and `/Users/connormoulton/.codex/AGENTS.md` when present.
2. `METTEN_DESIGN_DRAFT_2026-11-12.md`, especially sections 3, 4, 5, 6, 9, 12, 14, 15 and 16.
3. `GLOSSARY.md`.
4. `docs/adr/0001-rust-engine-and-tauri-boundaries.md`.
5. `docs/implementation-plan.md`.
6. `docs/agents/issue-tracker.md`.
7. Issue #4's full current body, comments, labels and native blocking relationships through `gh`.

The original specification is the behavioral baseline. This handoff records later explicit owner decisions that clarify or supersede particular earlier assistant proposals. Identify material conflicts rather than silently editing the baseline. Tracker access failure is a concrete blocker, not permission to guess acceptance criteria.

Last rechecked on 13 November 2026: issue #4 OPEN, `ready-for-agent`, no comments; sole native blocker #2 CLOSED. Recheck when needed rather than assuming this remains current.

Current ticket acceptance requires:

- A bounded, strict schema and semantics covering explicit source/variable identity, units, comparisons, combinations, intervals, deadlines, checkpoints and same/opposite side; reject unknown, oversized and malformed rules.
- Generated explanations from the same validated representation used for evaluation; author prose cannot override them and no AI service is needed.
- Controlled observations demonstrating latched checkpoints, remembered sides, per-market reset, boundary/tie behavior and no invented observations during gaps.
- Edits create immutable versions without changing earlier history. Quantity and user-permitted prices remain run settings. Publish the matching field reference and one learning example, without defining historical D/F strategies.

### Current workspace — preserve it

Last observed branch: `main`  
Commit: `9bd42f7ba344493dbbc259f0b6910bd4c06703a8`  
Subject: `Fix stale session status responses (fixes #22)`

The working copy is DIRTY. Modified tracked files:

```text
crates/engine/Cargo.toml
crates/engine/src/lib.rs
crates/engine/src/storage.rs
index.html
src-tauri/build.rs
src-tauri/capabilities/main.json
src-tauri/src/main.rs
src/contract.ts
src/main.ts
src/style.css
```

Untracked implementation/supporting material was already present in:

```text
crates/engine/src/strategy.rs
crates/engine/tests/strategy.rs
docs/proposals/
examples/
src-tauri/permissions/autogenerated/strategies.toml
```

This handoff is also new. Reinspect status before later writes; never reset, remove or silently overwrite existing work. Local ignored planning/scratch directories are not a second approved tracker.

Read only enough draft code to understand existing boundaries: `crates/engine/src/lib.rs`, `storage.rs`, `strategy.rs`, engine tests `session.rs` and `strategy.rs`, `src-tauri/src/main.rs`, `src/contract.ts`, `src/main.ts`, and `examples/learning.json`.

The draft has engine parsing/evaluation/explanation, immutable storage and desktop import/edit-copy plumbing. It is narrower than the agreed authoring capability set and is not proof of the desired design. Rust owns rules and financial truth; Tauri requests actions and displays results. Provider adapters must never branch on strategy names.

### Explicit owner decisions — preserve these

#### A. Flexible documented authoring

- Authors can combine supported JSON inputs and rules. This is not a menu limited to strategies named D, E or F, and it is not arbitrary executable uploads.
- Explicitly name BRTI, Coinbase BTC/USD and Kraken BTC/USD. Their actual access, licensing, cost, field availability and adapter capabilities remain unverified. Never silently substitute one for another.
- Distinguish buyable ask, bid, last trade, compatible underlying prices and reference/settlement values. No ambiguous generic price.
- Contract prices are strategy inputs. An author's purchase-price conditions and the user's permitted purchase prices must both hold.
- Quantity belongs to run settings, not strategy JSON.
- First capability set: signed/absolute and percentage price differences, changes over time; crossings; rolling averages, minimums, maximums and ranges; author-selected durations and confirmation on distinct updates. Measurements apply to contract prices and compatible Bitcoin prices; authors can compare explicitly named feeds.
- Direction derived from Bitcoin is optional, not a prerequisite for every purchase rule.
- No confidence model is wanted, including as a planned future requirement. Volatility and crossing counts are deferred unless a later concrete request justifies them. Richer volume/depth indicators were not approved for this issue.
- No universal three-sample requirement. Each operation needs the evidence its own definition requires; an author may deliberately request three confirmations.
- Exact formulas, sampling, precision, units and boundaries must be documented and checked. Do not ask Connor to design arithmetic or parser machinery. Do not silently inherit Revision 3's algorithms or arbitrary numeric budgets.

#### B. Checkpoints and condition meanings

- A signal can establish a checkpoint without a purchase. Multiple named checkpoints can remember events and sides.
- A checkpoint latches for the rest of its market and resets at the next market. Temporary source loss does not erase an established checkpoint or remembered side.
- A source needed only to establish a completed checkpoint does not have to remain current for a later purchase. Inputs that the current purchase conditions actually need must be valid.
- Process a newly established checkpoint before evaluating purchases that use it on the same update. Authors may explicitly require a later separate observation instead.
- Four condition meanings are supported: “is below,” “crossed below,” “remained below for a duration,” and “confirmed on N distinct updates.” They are not interchangeable.
- “Below” excludes equality; “at or below” includes it. Crossing needs observed before/after evidence. A first observation already below is enough for a current-price condition but not evidence of crossing.
- An unfinished duration resets on observed failure or loss of its required validity/continuity. It may complete when its duration elapses if its required evidence remains valid.
- Distinct updates may have identical prices. Duplicate deliveries, repeated local checks and replayed evidence do not create extra confirmations. Exact adapter observation identity still requires definition and proof.

#### C. Purchase options, priority and quantities

- One strategy can contain ordered, nested purchase options. `purchase_options` was the lead's recommended name, not an approved final schema. The structure should make “check preferred option, otherwise check the alternative” clear.
- Priority means prefer the highest-priority currently eligible option. Do not permanently consume an option merely because it once qualified or lost a priority comparison.
- Before any fill, after a confirmed zero-fill outcome, check options again from the top. A newly eligible preferred option can take over. An unknown order outcome blocks all new attempts until reconciled.
- Gen 1 permits ONE purchase objective per market, for the user's original requested quantity. Multiple independent purchases, including buying both sides, belong to Gen 2.
- Any confirmed partial fill locks the objective's side. Other eligible options may help complete it ONLY on that same side, within the remaining original quantity, their conditions/deadlines and engine/user safeguards.
- Agreed example: requested quantity 10; option 2 fills 4 YES; option 1 later qualifies for YES; it may authorize attempts for the remaining 6. It cannot start another 10-contract objective. If it selects NO, it cannot proceed.
- This ceiling is PER MARKET, across all options combined, not a new daily contract-count limit. Daily money limits remain separate. No per-option quantity settings are approved for Gen 1.
- IOC only; hold confirmed purchases until settlement. No resting orders, selling, exits or hedging in this issue.
- For the learning illustration, a 30-cent ask ceiling means wait while buyable ask is 35, even if bid is 25 and last trade is 28. An ask of 29 can satisfy the price condition. Attempt within permitted prices, recheck after confirmed no fill, and wait again if conditions cease to hold. This is an illustrative author choice, not a required threshold, crossing rule, profitability claim or guaranteed fill.

#### D. Live collection and initial prefill

- Collect history for the run from Go. Do not prefill from imported files, historical downloads, another application's cache, or observations from before that run's Go.
- Five minutes of required history means at least five minutes collecting; 15 minutes can cost the first market; an hour-long measurement means an hour collecting. Do not impose Revision 3's market-only or less-than-15-minute history restriction.
- Applicable underlying-price history may span markets. Checkpoints still reset per market; contract-book data remains tied to its contract. An exact prior-contract comparison capability was not thereby approved.
- Make startup visible: **“Prefill — collecting live data”**, with required waiting period/progress; then **“Ready — waiting for next market”**; then market evaluation.
- Once initial prefill is complete, wait for the next full market before purchase evaluation. Do not unexpectedly begin midway through a market when initial history becomes ready.
- Recovery within an already-active market is different: restoring the required evidence can restore eligibility in that market without imposing another full-market wait. Crash/restart still never automatically resumes trading.

#### E. Freshness and genuine history gaps — adopted

Connor explicitly adopted the distinction between a verified quiet source and genuinely missing information, including complete required coverage for Gen 1 historical calculations.

- An unchanged value is not automatically stale. Separate the time a value changed from evidence that it remains current.
- Connection health alone does not prove every required source/market is complete or current. Another stream's activity cannot refresh a missing dependency.
- Recent arrival does not make old queued data current. Check relevant source timing, receipt, continuity and processing delay under documented field semantics.
- Track validated source receipt and required observations independently of how often the purchase loop runs. Do not mistake selected samples or local counter jumps for missing provider messages; do not drop observations needed for checkpoint/crossing outcomes.
- A disconnection, detected missing update or inconsistent state pauses dependent purchases while the source is checked. Preserve completed checkpoints and trustworthy historical observations; do not erase the market or restart the entire collection period automatically.
- A current recovery snapshot restores current state, not the missing historical path. Equal values before/after an outage do not prove the value never moved during it. Never infer a missed checkpoint or crossing.
- Only affected purchase conditions become unavailable. Continue to independent explicitly defined alternatives, subject to priority, filled-side lock and all common safeguards. This is not silent substitution of named sources.
- Complete required history means the observations needed by the documented calculation, not a new price every millisecond. Verified quiet periods do not create holes. Do not silently invent missing prices or change an average into an average of whatever samples happened to arrive.
- A genuinely unrecoverable one-second hole can make a five-minute complete-window calculation unavailable until the hole leaves its window. That limitation was accepted. Current-price rules can recover sooner. No relaxed/gap-tolerant averaging mode is approved for Gen 1.
- Numerical silence, delivery-delay, clock and validity limits are NOT selected. They need source-specific delivery/continuity evidence and tests for unnecessary blocking. Do not reuse old-bot defaults, assume BRTI's roughly one-second cadence is a hard one-second deadline, or add a universal “brief disconnect” timer.
- These requirements must be expressible/testable with controlled validity and observation inputs in #4. Actual adapters, numerical production profiles and integration acceptance remain later feed/recovery work. A source whose required capabilities cannot be established must be reported unsupported; never claim it works from documentation alone.

### Supporting material — critical reading only

- `docs/proposals/issue-4-strategy-contract.txt`: Revision 3, unapproved, predates this interview. It conflicts with adopted decisions on history scope, global historical-source health and consumed purchase opportunities. Do not use it as the new design.
- `docs/proposals/issue-4-council-record.txt`: historical review record; does not include all decisions or the newest targeted review from this session.
- `docs/research/strategy-input-options.md`: assistant-generated research candidates, not an approved capability list. Its confidence-model suggestion is superseded by Connor's explicit rejection.
- Do not inherit arbitrary node counts, mandatory event gates, exact arithmetic algorithms, market-only history limits or branch-consumption rules merely because old proposals contain them. Strict import/resource validation is required, but its engineering bounds must be justified and visible.

Historical council: Revision 1 scores were Codex 8.0, MiniMax 7.0, GLM 8.1, DeepSeek 6.5; Inkling returned no assessment. North failed replacement qualification; Gemma was rate-limited. Supplemental Codex reviews of Revision 2 scored 7 and 6. Revision 3 has no independent scores. No version met the old threshold.

The latest targeted pass used two fresh-context native Codex agents on the SAME freshness excerpt, with safety/recovery and unnecessary-blocking lenses. Neither was a complete EGM workflow or five-family council; neither gave a numeric score. Both found directional alignment, not implementation readiness. Their consolidated findings were: distinguish validated receipt from decision sampling; distinguish change time from current-validity evidence; do not repair past coverage with a present snapshot; define independent observations; separate source/instrument/market history boundaries; and continue to independently eligible alternatives. The lead accepted these clarifications in discussion. They were not written into Revision 3 or independently re-reviewed. Under the earlier conservative accounting this was pass 3; the new EGM trial now uses its own gates as expressly selected above.

### Old-bot lessons and external evidence

Read-only source repository: `/Users/connormoulton/Desktop/kalshi_bot`. Read its own instructions first; avoid credentials, private configurations, runtime records and backups. No imports, tests, operations or strategy copying are needed for historical fact-finding.

Useful source pointers:

- `tests/unit/test_kalshi_ingress_health.py`, particularly the test beginning near line 47: valid ticker/book receipt remains recorded even if a latest-only decision queue retains the other update.
- `bot/kalshi_feed.py`: per-source receipt, book snapshots/deltas, reconnect state, source identity and cached-book age.
- `tests/runtime/test_feed_stall_recovery.py`: connected-but-silent feed scenarios and recovery evidence.
- `tests/runtime/test_experiment_reversal.py`: sampled-book sequence jumps need not mean missing provider data.
- Public issue https://github.com/connorreganmoulton-web/kalshi_bot/issues/339: normal post-close silence falsely polluted observation completeness.
- Public issue https://github.com/connorreganmoulton-web/kalshi_bot/issues/342: requiring consecutive sampled update counters prevented confirmations.
- Public issue https://github.com/connorreganmoulton-web/kalshi_bot/issues/358: feed-stall recovery/liveness incident. These are historical records, not current operational diagnosis; Connor did not identify which particular incident he remembered.

D/E/F are illustrations of capability families, not approved Metten definitions. D's tracked example once listed 89–96-cent asks, 45–160 seconds remaining and `MIN_DELTA_PCT=0.03`; newer D scoring did not enforce that field as the older direct minimum. E remembered a reference side and considered the opposite on the first distinct subsequent book within a deadline. F remembered the reference side and waited for qualifying bid observations. Do not import those thresholds, scoring, sample counts or execution behavior into Metten. Exact historical reproduction is outside #4.

Official references previously read, to recheck when relying on provider facts:

- https://docs.kalshi.com/websockets/orderbook-updates
- https://docs.kalshi.com/websockets/connection-keep-alive
- https://docs.kalshi.com/websockets/cfbenchmarks-value
- https://docs.cdp.coinbase.com/exchange/websocket-feed/channels
- https://docs.kraken.com/exchange/api-reference/spot-websocket-v2/heartbeat

The documents distinguish snapshots/updates, connection activity and source evidence. Availability, licensing, entitlement, actual field mapping and runtime behavior have NOT been established for Metten. No accounts were accessed. Do not treat a subscription acknowledgment or authenticated connection as proof of usable data.

## STEPS

1. Read the original evidence and actual installed EGM brainstorm/Codex instructions. Give a concise RISEN kickoff. Confirm tool access and preserve the dirty workspace. Recheck the current ticket before relying on its status.
2. Use this intake to form the Phase 1 working hypothesis: flexible documented strategy rules, clear user explanations, preserved checkpoint evidence, and predictable purchase priority within one quantity-limited objective. Distinguish adopted owner choices from engineering details still requiring definition.
3. Ask only meaningful unresolved questions, in the skill's open-ended format. Do not turn the interview into a demand that Connor select arithmetic, parser limits or event-processing mechanisms. Do not reopen settled scope for convenience. Obtain the explicit anchored-hypothesis readiness confirmation before EGM reviewer dispatch.
4. Follow the real EGM workflow and its required personas/rubrics/gates. Verify fresh-context support; respect concurrency limits. Actual model/tool failures remain incomplete reviews. Consolidate findings against original evidence and observable examples, not score-chasing or consensus alone. Write new artifacts only in the chosen artifact directory during the design phases.
5. Pressure-test the emerging design with controlled timelines before proposing implementation. Preserve the outcomes below; expose contradictions rather than hand-wave them away.
6. After Connor approves a concept brief, move to drafting at the appropriate skill gate. Keep sections, file responsibilities, documentation, exclusions and verification traceable to issue #4 and the original specification. Do not prematurely resume the old implementation.
7. Continue through isolated review and human resolution only as directed by the EGM gates. Publishing a design PR, editing issues/milestones and beginning Phase 5 each require their corresponding owner instruction. Record actual completion and outstanding evidence honestly.
8. At the end of the trial, help Connor judge whether the flow improved clarity and caught consequential assumptions without unnecessary scope, repeated settled questions, inflated documents or premature code. Do not declare EGM successful merely because reviewers finished.

### Controlled examples and expected outcomes

These are planning oracles, not executed engine tests or provider/live proof.

1. A checkpoint records YES without purchase; later the opposite buyable ask satisfies the author's explicit condition. NO may become eligible without revalidating a source used only to establish the checkpoint.
2. Bid 25, last trade 28, ask 35 with author ask ceiling 30: wait. Ask 29 may qualify, subject to user prices and other safeguards. No guaranteed fill.
3. A checkpoint source disappears while current contract data remains trustworthy: preserve checkpoint; block only conditions needing that source now.
4. A ten-second condition is true at t=0, observed false on another dependency at t=3, true again at t=4: it cannot complete at t=10 by overlooking that failure. Its uninterrupted qualifying duration starts again at t=4.
5. Two derived measurements due together must be evaluated with both new results, not one new/one old. Equal timestamps from independent arriving feeds do not themselves prove atomic delivery; engineering must document coherent evaluation and ordering without inventing observations.
6. Initial five-minute history completes at 10:05: wait until the next full market at 10:15. Applicable underlying history remains available. Initial collection is not restarted at every market boundary.
7. Verified quiet source: no artificial history gap. Genuine missing second in a required complete rolling window: retain observations and mark that measurement incomplete until coverage is restored or the hole leaves the window. A new current snapshot cannot fill the missing path.
8. Two options qualify together: preferred option wins. Alternative qualifies first, attempt finishes confirmed zero, preferred qualifies later: reconsider from the top and allow preferred. Unknown outcome: no second attempt yet.
9. User requested 10, alternative fills 4 YES, preferred then qualifies for YES: at most 6 remain. A NO option is blocked. No second independent objective.
10. Checkpoint and purchase conditions become true on one observation: checkpoint establishes first, then purchase may qualify unless the author explicitly required a later observation.
11. New market: reset checkpoints and contract-specific state; keep applicable continuous underlying history. Never use the old contract's book for the new one.
12. Edit creates a new immutable version; old versions/history retain their original meanings. Active runs do not switch versions.

### Engineering definition and future evidence still needed

Do not label the contract implementation-ready until its exact supported fields/types/units, comparison and Boolean semantics, relative-side selection and ties, temporal definitions, lookbacks/sampling, numerical precision, interval/deadline boundaries, independent-observation identity, coherent evaluation order, error messages and resource bounds are concrete. Own these engineering decisions and explain only consequential restrictions to Connor. Numerical production feed validity limits remain a separately identified adapter-acceptance gate, not guessed constants in strategy JSON.

The later implementation plan should cover engine validation/evaluation/explanation, append-only local versions and preservation of existing records, narrowly validated desktop IPC, import/preview/save/edit-copy UI, complete matching author/agent field documentation, and one worked learning example. Actual feeds are #5; paper purchasing and settlement are #6; partial completion is #7; uncertain outcomes/recovery are #8; interruption handling is #9. Define shared rule semantics here without claiming those integrations are implemented.

Proposed future implementation checks, NOT established results:

```text
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run check
appropriate production/package build
focused public-engine behavior and immutable-version checks
focused IPC and disposable-workspace packaged checks
```

Earlier interrupted cargo work has no established complete final result in this handoff. Later arithmetic demonstrations were not an engine acceptance suite. `docs/verification/issue-2.md` and `issue-3.md` describe historical evidence; they do not prove the dirty modified workspace. During this handoff session no code tests, builds or runtime checks were run.

## END GOAL

A fresh EGM session can begin without reconstructing the conversation, preserve explicit owner decisions, and produce the skill's actual concept brief through its proper gates. Subsequent design/review artifacts should be self-contained, bounded to issue #4 and grounded in original evidence. Phase 5 begins only after the design and implementation authority are explicit. Unavailable tools or essential evidence are named blockers, not invented success.

Connor has approved the individual decisions recorded here. Immediately before requesting this handoff, the lead offered an overall recap and asked for confirmation to assemble a plan; Connor instead requested the new EGM session. Therefore no completed overall plan or whole-design approval exists yet.

## NARROWING

- This is an intake for the new session, not an approved specification, completed EGM output or release authorization.
- Preserve existing code/proposals and unrelated applications. No current trading or account status is established.
- Gen 1 remains one purchase objective per market; Gen 2 multiple purchases are deferred, not implemented here.
- Connor requested a later GitHub milestone review to distinguish Gen 1 and Gen 2. It remains a follow-up tracker task, not authority to write milestones now.
- Do not run EGM bootstrap, broadly regenerate repository documentation, add tools or introduce infrastructure merely to try this workflow.
- The prior council is superseded only as the review gate for this EGM trial; retain its historical failures honestly. New EGM reviews are not proof of trading safety, provider access, live acceptance or fills.
