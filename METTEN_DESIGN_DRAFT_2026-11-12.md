# Metten Gen 1 specification

Date: 12 Nov 2026.
Status: Product baseline and Rust/Tauri direction accepted; implementation not started.

This specification is the product source of truth for Metten Gen 1. GitHub issues track bounded implementation work against it; ADRs record architectural decisions. Repository creation and planning are authorized. Implementation and release evidence remain outstanding. Neither this document nor an issue authorizes credential access, paid services, real-money validation, or operation of another trading application.

## 1. Purpose

Metten is intended to become an open-source, local application for running compatible Kalshi strategies. Both strategy authors and people installing someone else's strategies are intended users. The central experience is choose a market, choose a strategy, configure the run, and press Go.

The priorities are dependable purchasing, clear recovery, and maintainability. Changing supported strategy rules must not require changes to connections, purchase execution, accounting, or recovery. Guided secure setup and a clear branded interface support that engine. Paper results are not proof of live execution, and there is no profitability promise.

## 2. Gen 1 scope and local operation

- Mac first, delivered as a normal application: download, open, and guided setup, without terminal commands or separately installed containers for the user.
- Bitcoin 15-minute markets (BTC15m) only. Portability across compatible series is a future aim, not broader Gen 1 market support.
- One live run. Multiple financially independent paper runs may operate simultaneously; capacity must be measured later and live responsiveness protected.
- Each paper run has its own simulated starting balance, quantity, price range, and daily limits. Paper activity cannot consume live funds or alter live limits.
- Local-only workspace: no online Metten account, telemetry, or Metten server collecting workspace data. Credentials, strategy files, settings, and trading history remain local; necessary direct calls to trusted exchanges and data providers are expected.
- Leave the application open on an awake, connected computer for unattended operation. Closing it stops its trading operation; there is no remote service keeping it trading.
- Supported JSON rules only; IOC purchases only; no selling/exits. Hold purchased contracts through settlement.
- SQLite is chosen for local structured nonsecret storage, not a database server. Following owner review, Rust is selected for the engine and Tauri for the Mac application; see [ADR 0001](docs/adr/0001-rust-engine-and-tauri-boundaries.md). MIT is the selected open-source license; see [LICENSE](LICENSE). Detailed packaging remains undecided and packaged integration is not yet proven.

## 3. First use and starting a run

1. Download and open the Mac application. Welcome to Metten, choose what to be called, connect Kalshi securely, and enter the local workspace. No separate Metten password is required in Gen 1.
2. Connect the strategy's explicitly named supported sources. Missing required sources block start; authentication alone is not proof that required data is accessible.
3. Choose the supported market series and a strategy version. Import and validate JSON; reject malformed or unsupported rules. Review the plain-English explanation generated from the validated rules.
4. Configure paper/live choice, contracts per market, permitted contract purchase price range, and daily limits. Quantity belongs to run settings, not strategy JSON. Paper-only is the default; live eligibility and any automatic transition require the safeguards in section 7.
5. Press Go. Wait for the next full 15-minute market, never start halfway through the current one. All required source, funds, limits, recording, and qualification checks still apply.

The author's purchase conditions and the user's permitted prices must both hold. Never silently change a requested trade to fit user limits. An active run's strategy and settings are immutable: stop the run before starting with changes.

## 4. Strategy contract, checkpoints, and relative sides

- Imported strategies are documented JSON rules, not arbitrary Python, Rust, or other executable uploads. Validate before running; never silently ignore unsupported rules.
- The contract and documentation must specify variable names, types, units, comparisons, combinations, time windows, data source identity, checkpoints, deadlines, and reset behavior. Exact JSON grammar remains engineering work.
- A checkpoint is a Boolean flag. Once observed true, it latches for the rest of that market window, including after its observation interval ends or required data temporarily disappears. It resets at every new market. Latching does not waive fresh-data requirements for a new purchase.
- Remember which side caused a checkpoint. A later purchase may target that side or its opposite without hard-coding YES or NO. Preserve the observed relative side within the window; do not infer an unobserved checkpoint during a data gap.
- Distinguish buyable contract price, bid, last trade, underlying asset prices, and settlement/reference values. An ambiguous generic “price” is insufficient.
- The author chooses purchase eligibility, deadlines, and strategy skips. Metten owns exchange integration, user limits, accounting, and recovery.
- Generate a plain-English explanation directly from validated JSON using templates for supported rules before Go. Do not use a separate AI service or rely on an author-maintained description that can drift.
- Use one documented learning-example JSON strategy to establish mechanics, without a profitability claim or treating it as an approved D/F definition.

Simplified D and F exact rules, required inputs, and keep/remove/undecided comparisons with the old bots are explicitly deferred to a future strategy-testing task (section 14). Approximate old prices are not approved rules.

## 5. Purchase and order lifecycle

- One purchase objective per market, for the user's requested quantity. After any confirmed fill, including a partial fill, the objective is locked to that side. Further partial fills/retries may only complete the original amount, never exceed it or become a second independent purchase.
- Before any fill, the side may change only after the prior attempt is confirmed finished. Buying both sides or hedging is outside Gen 1.
- IOC only: each attempt's unfilled remainder is canceled by the exchange. Resting orders and additional order methods are deferred. Purchased contracts and partial fills are retained through settlement; no selling or exits.
- Retry only the unfilled quantity until the author's deadline, while strategy conditions, fresh required data, available funds, user price settings, and risk limits permit. If conditions cease to hold, make no further attempts and keep confirmed purchases.
- Do not silently resize the overall objective to fit funds or limits. Skip an objective that cannot fit the remaining worst-case loss budget; completing its original amount after a partial fill is not a new objective.
- Confirm actual order outcomes before another attempt. No response does not mean no fill; uncertainty blocks further purchases until actual filled quantity is established.
- Track orders, fills, fees, holdings, and settlement truthfully, including after interruptions. Persist necessary order intent in SQLite before submission and confirmed outcomes before retries. Durable recovery cannot depend on disposable diagnostic logs.
- Exact exchange mapping, transaction boundaries, recovery identity, duplicate prevention, and retry mechanics remain engineering evidence requirements, not proven implementation.

## 6. Run controls and recovery

- Go starts at the next full market boundary. No strategy or settings edits while a run is active.
- Ordinary Stop means “Stop after this market”: finish the current window, allowing otherwise valid purchases/completion there, but enter no new market afterward.
- “Pause purchases now” immediately blocks further attempts while order and holding monitoring continues. It cannot undo submitted purchases or sell holdings.
- Normal quit warns that trading stops and offers keep running or stop and quit. Save state, check in-progress purchases, and explain unresolved outcomes.
- Stopping, changing a strategy, restarting, or quitting must not erase history or reset daily trading results.
- A connection interruption does not automatically discard the market. Reconnect, obtain fresh required information, and reconcile orders, fills, and holdings; continuation within the same window is allowed only if conditions still qualify and no purchase outcome remains uncertain.
- Preserve checkpoints and remembered sides already observed in that window. Never assume a checkpoint occurred during the gap. Reset at the next market.
- If an IOC outcome is uncertain, show “Trading paused — checking a purchase.” Report the actual recovery result when known and remain paused while uncertain, even if this misses an opportunity. Never risk a duplicate purchase.
- Crash or unexpected application restart must never automatically resume trading. Reconcile and show existing orders/holdings, then wait for the user's Go at a full market boundary. A next-day daily-limit setting cannot override crash recovery or a manual stop/restart.
- Connection health includes source freshness, missing updates, and ability to confirm orders, not merely a connected indicator. Freshness thresholds, retry pacing, recovery checks, and long-gap behavior require engineering investigation and proof.

## 7. Paper and live qualification

- Both modes use the same strategy file, rule evaluation, purchase-request construction, validation, limits, retry logic, and order lifecycle. Only final execution differs: live submits to the exchange; paper simulates exchange outcomes from observed market information.
- Paper must model fills, no fills, partial fills, finite available quantity, fees, and order outcomes as realistically as possible. An intent log alone is insufficient. Simulation assumptions must be documented.
- Every newly imported or edited strategy version must qualify before live eligibility: at least one paper market with a simulated purchase, including a confirmed partial fill, resolved order/accounting outcomes, and accounted-for settlement. No win is required. Unresolved errors prevent passing.
- Paper-only is default. Explicit “Paper first, then live automatically” authorization must display and authorize live quantity, price range, and daily limits up front. After qualifying paper settlement, transition only at the next full eligible market, never unexpectedly mid-window after a data gap.
- A qualified version need not repeat qualification on every start. Changing quantity or permitted prices does not force requalification, but requires stopping before editing and retains all current safety, funds, price, and limit checks. Keep actual paper-tested settings visible; different sizes may fill differently.
- Paper cannot prove live API acceptance or identical future fills. Live integration must be proven separately; every real-money validation requires its own exact, explicit permission. This design task grants none.
- Cross-series qualification scope is future work because Gen 1 is BTC-only, not a Gen 1 implementation requirement. Eligibility handling after data gaps remains engineering work within the agreed boundary rules.

## 8. Risk accounting and daily limits

- Daily profit/loss is the combined Metten-bot-only settled live result after fees, not account-wide balance changes, deposits, unrelated trades, or paper results. Per-run live limits are deferred.
- A user-chosen daily maximum live loss is required; a daily maximum profit/profit target is optional. Available account funds are an additional prerequisite.
- Skip a proposed purchase if the original objective cannot fit remaining worst-case loss allowance, accounting for fees, holdings, and in-flight attempts. Do not resize it. Example: $20 allowance, $15 already lost, and a proposed purchase could lose $10: skip rather than reduce it to $5.
- When a daily threshold triggers, finish the current market under otherwise valid purchase/completion checks, then enter no new market. Final results can exceed a threshold; never promise an exact guaranteed daily cap.
- Provide configurable next-day resumption after a daily-limit stop. It does not convert paper-only into live or override manual stops, crashes, or the explicit recovery/start requirements.
- Trading days are midnight-to-midnight UTC; store timestamps in UTC, optionally supplementing with local display. Assign a market's result to the UTC day of its market CLOSE time, even if settlement arrives later. Keep delayed settlement pending until confirmed; do not count it as settled P&L.
- Stops, changed strategies, restarts, and quits cannot reset daily results. Financially independent paper accounting stays separate; “Start a new test” may create fresh paper accounting without deleting earlier history.
- Engineering must specify exact loss reservations, fee treatment, unsettled holdings across daily resets, threshold interactions, and resumption details without changing skip-over-resize or current-market finish behavior.

## 9. Security, sources, and research references

### Credential and source boundaries

Strong credential security is required; open source alone is not security evidence.

- Store API secrets and signing keys only in protected Mac Keychain, the OS encrypted credential store, configured local-only/non-synchronizing. No homemade cryptography or plaintext fallback. If secure storage is unavailable, refuse credential saving and explain why. Full secret-manager integration is not required in Gen 1.
- Never put secrets in strategy JSON, SQLite, history, logs, exports, workspace backups, or the repository. Only trusted provider/exchange connection code may use credentials; strategies receive no secrets and can request only named supported data/actions, not arbitrary code or network destinations.
- Sources/settings supports replacing and removing provider-specific credentials. Strategies must identify explicit named sources; missing required access/data blocks start and unsafe new purchases. “All available” was discussed but discouraged because changing sources could change a strategy. Final syntax is unresolved.
- Never silently substitute Coinbase or Kraken for BRTI. Required source identity and actual access must be verified, not inferred from authentication or a subscription acknowledgment.
- OS credential storage protects secrets at rest, not against a compromised running app or OS. Runtime compromise boundaries, secure packaging/distribution, dependency risks, and security updates require evidence; they are not proven here.
- Independent verification must cover malformed uploads, secret leakage, credential protection, recovery records, and dependency/distribution risks before release. No security implementation, audit, or credential access test has occurred.

### Recorded primary-source research — 12 November 2026

The existing draft records the following research facts and links. They are retained as research references, not newly fetched or verified in this consolidation, and do not establish actual account access, licensing, implementation, or operational readiness.

- Kalshi documents a real-time BRTI WebSocket feed at roughly one update/second with short-window averages. An availability-list request discovers available indexes; a successful subscription alone does not prove BRTI is flowing.
  https://docs.kalshi.com/websockets/cfbenchmarks-value
- Kalshi also documents a 5 Hz feed; requiring it has not been decided.
  https://docs.kalshi.com/websockets/cfbenchmarks-value-5hz
- Historical/index REST passthrough uses existing Kalshi credentials without a separate CF key but requires account entitlement. Requests cost more read capacity than default requests.
  https://docs.kalshi.com/cfbenchmarks/rest-passthrough.md
- Crypto settlement reference values can differ from single-exchange prices; market rules govern.
  https://help.kalshi.com/en/articles/13823838-crypto-markets
- V2 order documentation includes IOC, immediate fill quantity, and cancellation of its unfilled remainder.
  https://docs.kalshi.com/api-reference/orders/create-order-v2
- Order-book streaming starts with a snapshot and subsequent updates; documentation includes gap detection and resynchronization.
  https://docs.kalshi.com/websockets/orderbook-updates

A prior public-source helper also investigated direct CF access/licensing, streaming frequencies, and reference averaging. Ordinary-account BRTI availability, entitlement criteria/cost, demo availability, redistribution/local retention permissions, exact settlement-average correspondence, and historical revisions remain unresolved. Research does not prove universal/free BRTI entitlement, caching rights, or access based only on authentication. No live account calls were made.

Storage references supplied for the SQLite decision, likewise not newly verified and not evidence of Metten implementation:
- T3 Code SQLite usage reference:
  https://github.com/pingdotgg/t3code/blob/main/.agents/skills/test-t3-app/references/sqlite-fixtures.md
- SQLite application-file-format reference:
  https://www.sqlite.org/appfileformat.html

## 10. SQLite, history, diagnostic logs, and backups

- SQLite is the selected local structured store for strategy versions, nonsecret settings, runs, checkpoint/run recovery state, purchase intents, orders, fills, fees, settlements, daily-limit accounting, per-market explanations/history, and paper qualification. It requires no separate database server.
- Selecting SQLite does not mean history is encrypted or storage size is automatically capped. Credentials belong only in protected Mac Keychain. Retained trade history can grow.
- Preserve compact trading history and event-based explanations until explicit user removal. Record checkpoint transitions, purchase attempts, confirmed fills, fees, outcomes, and relevant decision inputs. Summarize repeated waiting/reason messages instead of continuously duplicating them.
- Preserve exact strategy versions and settings for each run and market. Editing/uploading creates a version, never rewrites earlier results. Exact version identifiers and schema remain engineering work.
- Do not continuously store raw price/order-book feeds. No exact full-market feed replay is promised.
- Disposable diagnostic logs are separate rotating files. Owner-approved initial retention is seven days OR 100 MB, whichever comes first; these defaults need development measurement and are not validated capacity limits. Temporary debugging requires explicit opt-in and both size and time bounds.
- Diagnostics cleanup must never discard unresolved order, holding, or recovery records. Durable trading recovery does not depend on diagnostic retention.
- Provide a storage-usage view, local history export, workspace backup, and bounded automatic backup rotation. Exports/backups must be secret-free; restoring on another machine requires reconnecting providers. Exact formats, rotation bounds, transactions, cleanup, and recovery mechanics remain engineering-owned.
- Monitor disk space and SQLite auxiliary write files. Low disk or inability to durably record safe trading state blocks new purchases and preserves unresolved order/holding/recovery records.
- Database choice and retention-policy approval are resolved design decisions, not proof of technical readiness, security, performance, or implemented backups.

## 11. Dashboard

Three main views:
1. **Now:** mode, run status, current market, checkpoint progress, purchase progress, daily limits, and concise health.
2. **History:** market results and skip reasons, strategy version, run settings, confirmed purchases, fees, and settlements.
3. **Sources and settings:** connected providers, credentials management, preferences, and access to storage usage.

Keep detail behind “View details,” not a flood of raw events. Distinguish no signal, strategy skip, user-limit block, failed order, and no fill. Separate submitted requests from confirmed fills, settled P&L from money still at risk, and paper from live. Results remain attached to their exact strategy version and settings.

Make connection interruptions visible; concise local notifications were proposed, not finalized. “Start a new test” creates fresh paper accounting without deleting history. Do not claim causal strategy insights from facts showing only correlation or execution events.

## 12. User and agent documentation

Thorough documentation/wikis and a strong strategy-authoring guide for agents are part of Gen 1, not just a few examples.

Required coverage:
- Installation, secure credentials, source availability, paper/live setup, limits, stop/pause/quit, connection failures, recovery, and interpreting results.
- Complete JSON field reference: types, units, supported inputs, operators, combinations, checkpoints, relative sides, deadlines, and resets.
- One end-to-end worked learning-example strategy, without a profitability claim or invented D/F rules. Explain how template-generated plain English follows the validated JSON.
- Agent-facing how-to: take an idea, identify supported inputs, express JSON, validate, run paper, inspect outcomes, and explain limitations without inventing capabilities.
- Engine-versus-strategy responsibilities and how ordering actually works; paper simulation assumptions and limits; what qualification proves and does not prove.
- Troubleshooting with actionable reasons rather than manual source edits. Schema, reference, examples, and explanations must agree. Documentation tooling remains undecided.

## 13. First-release acceptance

The future milestone is clean setup → one complete BTC strategy → qualifying paper market and accounted-for settlement → a SEPARATELY AUTHORIZED live purchase → settlement → readable history, with connection recovery demonstrated. This describes future release evidence, not permission for live tests now.

Acceptance must also establish purchase/partial-fill/no-fill/settlement behavior, disconnect and crash transitions, duplicate prevention and unknown-outcome handling, accounting integrity, durable recording and disk-failure safeguards, credential protection and security verification, clean installation usability, and documentation walkthroughs. Engineering must define and produce the evidence, including separate live-integration proof. None is established by this specification alone.

## 14. Deferred features and strategy work

- More markets, simultaneous live runs, per-run live limits, selling/exits (discussed for Gen 2), hedging/both-side purchases, resting orders, additional order methods, and arbitrary executable strategies.
- Additional login/password options, advanced scheduling, automated strategy advice, and Windows after Mac-first validation.
- A strategy marketplace with payments and commission is outside initial scope, not committed. Local delivery of rules raises copying/IP questions.
- Simplified D AND F exact rules/inputs and keep/remove/undecided decisions versus the old bots are explicitly deferred until a future strategy-testing task. They are not blockers to this core specification; their later definition is required before implementing those strategies.
- Existing trading applications are not implementation dependencies. Do not import their code, thresholds, or behavior. Any future D/F support question belongs to its separately scoped strategy task.

## 15. Unresolved engineering evidence

Engineering owns technical discovery and proof within the agreed behavior:
- Detailed JSON grammar and named-source syntax, side-selection ties, multi-checkpoint scope, time boundaries, expired conditions, and actionable unsupported-input messages. Establish mechanics through the learning example without inventing deferred D/F inputs or rules.
- Provider feasibility and actual access checks, BRTI entitlement/cost, licensing and retention rights, missing-source experience, and all unresolved research facts in section 9.
- Worst-case reservations including fees, holdings, and in-flight attempts; daily accounting/reset with unsettled holdings; threshold interactions; and resumption mechanics. Preserve skip-over-resize and current-market finish behavior.
- IOC mapping, recovery identity, duplicate prevention, rejected orders, insufficient funds, unknown outcomes, retry/freshness rates and rate limits, and long-gap failures. Thresholds and rates are not chosen constants.
- Paper simulation assumptions and shared paper/live semantics, qualification after gaps without unexpected mid-window live transition, and separately proven live integration.
- SQLite schema/transactions, durable intent/outcome recording, recovery, loss of local files, credential removal, history/export/backup/retention, auxiliary-file and disk monitoring, and proof that unsafe recording blocks new purchases.
- Independent security verification for malformed uploads, leakage, credentials, recovery records, dependencies, distribution, updates, and runtime compromise boundaries. No such verification has been performed.
- Measured paper-run capacity while protecting live responsiveness; no performance limit has been selected.
- Rust engine and Tauri desktop integration proof, implementation/build plan, secure packaging/distribution and security updates. MIT is selected; the engine/framework direction is selected in ADR 0001; feasibility and implementation evidence remain outstanding.
- Concrete release evidence and tests for the complete milestone and required recovery, order, accounting, security, installation, and documentation checks.

The product baseline, SQLite/storage policy, and Rust/Tauri direction are settled. Engineering owns routine implementation details and evidence. Raise only material product, cost, or money tradeoffs with the owner. Deferred D/F definitions do not block the core application. Selecting an architecture is not proof of implementation readiness.

## 16. Architecture direction

Rust and Tauri are selected, with maintainable separation as the deciding requirement. [ADR 0001](docs/adr/0001-rust-engine-and-tauri-boundaries.md) records this decision and supersedes earlier notes that the stack was unselected. The remaining mechanisms below are engineering proposals requiring implementation proof.

Prefer an event-driven engine with timers and reconciliation checks: consume supported live feeds, respond to relevant updates, and separately schedule market boundaries, deadlines, freshness checks, and recovery. Keep one authoritative owner of purchase decisions and state transitions, while network operations run asynchronously. Bound queues and detect overload; never silently discard required order-book updates, checkpoint observations, or order outcomes. If the engine cannot maintain a trustworthy current state, block new purchases and recover.

Keep strategy evaluation independent of screen rendering and provider connection management. A supported JSON strategy supplies conditions; the engine owns data validity, limits, durable intent, execution, and recovery. New combinations of supported rules should require no engine edits; genuinely new inputs or operators require an explicit extension rather than an ad hoc strategy-specific patch.

Rust is selected for the engine and Tauri with a web-based interface for the Mac application; Tokio is the proposed asynchronous runtime. SQLite and local-only Mac Keychain remain the existing choices. A small integration proof, including secure credential storage and packaging, must validate the selected direction before a full build commitment. Measure data age, evaluation delay, durable-recording delay, and exchange round-trip time separately; no numerical performance promise has been approved.

Primary references: [Kalshi streaming order books](https://docs.kalshi.com/websockets/orderbook-updates), [Tokio networking runtime](https://tokio.rs/tokio/tutorial), [Tokio message passing](https://tokio.rs/tokio/tutorial/channels), [Rust concurrency checks](https://doc.rust-lang.org/book/ch16-00-concurrency.html), and [Tauri process separation](https://v2.tauri.app/concept/process-model/). These support the available mechanisms, not proof of Metten's future reliability.
