# Metten Gen 1 implementation tickets — draft for review

These are proposed GitHub issues, not authorization to implement or trade. The product specification and ADR 0001 remain authoritative. Numbers below are draft identifiers; real issue numbers and native blocking relationships will be assigned after approval. GitHub labels describe readiness of the brief, not permission to act on blocked work or access credentials.

Each implementation ticket includes relevant UI, storage, tests and documentation for the behavior it introduces. Earlier slices are developer proofs, not public trading releases; production submission remains unavailable until the integration and release gates are satisfied. Tests should primarily exercise the engine's public controls with controlled clock/source/execution inputs; packaged UI, Keychain and exchange contracts receive focused integration checks. This is the proposed test surface for review.

Engineering owns routine interface and schema details. Do not invent D/F strategies, import old-bot code, or add unrelated markets/features. Live account access, paid services and real-money actions need exact separate authority. All intermediate evidence must be labeled honestly.

## Proposed sequence

1. **Choose the permissive license** — blocked by None. Confirm the specific permissive license before accepting outside code or distributing releases.
2. **Open the Mac app and control a persisted engine session** — blocked by None. Open a minimal Tauri app, start and pause a controlled non-trading session, and see the persisted result after reopening.
3. **Save, replace and remove a connection credential securely** — blocked by 2. Walk through connection setup using a dummy credential stored only in local-only Mac Keychain.
4. **Import and explain the supported learning strategy** — blocked by 2. Import one learning-example JSON strategy, inspect its generated explanation, and save immutable versions.
5. **Connect named sources and display trustworthy market data** — blocked by 3, 4. Use guided setup to show the supported BTC market and the exact data sources the learning strategy needs.
6. **Complete one paper purchase through settlement and history** — blocked by 4, 5. Run the learning strategy in paper mode from the next full market boundary to an accounted-for settlement.
7. **Complete partial paper purchases without exceeding the objective** — blocked by 6. Handle partial fills and eligible retries toward the original requested quantity.
8. **Recover an uncertain purchase without submitting a duplicate** — blocked by 7. Show an honest paused state after a lost purchase response or crash, then reconcile the original attempt.
9. **Preserve checkpoints and purchasing safety through feed interruptions** — blocked by 8. Recover from missing updates, reconnects and sleep/wake without losing observed checkpoints or buying on stale information.
10. **Enforce daily limits across settlement, midnight and restart** — blocked by 8. Display trustworthy Metten-only daily results and stop or resume runs according to the agreed limits.
11. **Record paper qualification and authorize future live transitions** — blocked by 7, 8, 10. Explain whether a strategy version is live-eligible and capture explicit settings for an optional paper-to-live transition.
12. **Execute and reconcile Kalshi IOC requests through the shared engine** — blocked by 5, 8, 9, 10, 11. Connect the shared engine to the current Kalshi order contract with controlled transport and explicit integration evidence.
13. **Run independent paper tests without slowing live control** — blocked by 7, 9, 10, 11. Run several paper sessions with independent balances and create a fresh test without deleting history.
14. **Export, back up and restore history without losing recovery state** — blocked by 8, 10, 11. Inspect storage use, export history, and create or restore a consistent secret-free workspace backup.
15. **Finish the branded setup and authoring walkthrough** — blocked by 3, 4, 6, 11, 14. Use the approved Metten identity through welcome, source setup, strategy import, Now, History and settings, with complete user and agent guides.
16. **Measure engine responsiveness and enforce supported capacity** — blocked by 12, 13, 14. Publish measured limits and keep the engine truthful and responsive under supported Mac load.
17. **Verify security and failure recovery independently** — blocked by 12, 13, 14, 15. Produce an evidence-backed review of malformed input, credential boundaries and purchase recovery before distribution.
18. **Install and update a signed Mac release candidate** — blocked by 1, 15, 16, 17. Install the release candidate on a clean supported Mac and update it while preserving local state safely.
19. **Perform separately authorized live acceptance and record release evidence** — blocked by 18. Demonstrate the specified clean-setup to paper qualification to live purchase to settlement path under exact owner authorization.

## Ticket bodies

### 1. Choose the permissive license

**What to build:** Confirm the specific permissive license before accepting outside code or distributing releases.

**Spec coverage:** 2, 15.

**Blocked by:** None (can start immediately).

**Proposed label:** `ready-for-human`.

- [ ] Owner selects a specific license consistent with the accepted permission for commercial closed-source derivatives; do not infer a legal grant from public visibility.
- [ ] Add the approved license text and update README licensing status. Record branding and third-party asset licensing separately.
- [ ] No application or trading behavior changes.

### 2. Open the Mac app and control a persisted engine session

**What to build:** Open a minimal Tauri app, start and pause a controlled non-trading session, and see the persisted result after reopening.

**Spec coverage:** 2, 6, 10, 11; ADR 0001.

**Blocked by:** None (can start immediately).

**Proposed label:** `ready-for-agent`.

- [ ] Rust engine is independent of Tauri rendering; UI requests cross a small validated interface and cannot issue raw exchange requests, SQL or shell commands.
- [ ] A controlled clock/event source drives a session; Now and History show state from SQLite rather than UI guesses. Restart is stopped, never automatically active.
- [ ] Use proposed durable SQLite settings, verify effective writer settings, and show an actionable recording failure. No credentials or external network calls are needed.
- [ ] Rust and interface checks run in CI; one meaningful engine-to-storage behavior test and a packaged local smoke check exercise the path. Record any packaging limitations.

### 3. Save, replace and remove a connection credential securely

**What to build:** Walk through connection setup using a dummy credential stored only in local-only Mac Keychain.

**Spec coverage:** 3, 9, 11.

**Blocked by:** 2.

**Proposed label:** `ready-for-agent`.

- [ ] Use a native Keychain integration with nonsynchronizing attributes; demonstrate add/read-for-provider-use/replace/delete in the actual packaged app.
- [ ] Credential value never returns to the renderer after entry or appears in SQLite, logs, errors, history, exports or backups. No generic secret-read command.
- [ ] Unavailable or denied Keychain access blocks saving with an actionable explanation; test lock/unlock behavior and record signing/entitlement requirements.
- [ ] Use dummy secrets only. Real account access or paid signing prerequisites require separately established authority.

### 4. Import and explain the supported learning strategy

**What to build:** Import one learning-example JSON strategy, inspect its generated explanation, and save immutable versions.

**Spec coverage:** 3, 4, 12.

**Blocked by:** 2.

**Proposed label:** `ready-for-agent`.

- [ ] Define bounded, strict schema and semantics for explicit source/variable identity, units, comparisons, combinations, intervals, deadlines, checkpoints and same/opposite side; reject unknown, oversized or malformed rules.
- [ ] Explanation is generated from the same validated rule representation used by evaluation. Author text cannot override it; no AI service is required.
- [ ] Controlled observations prove latched checkpoints, remembered sides, per-market reset, boundary/tie behavior and no invented observations during a gap.
- [ ] Editing creates a new version without changing earlier history. Quantity and permitted prices remain run settings. Publish the matching field reference and one learning example, with no D/F rules.

### 5. Connect named sources and display trustworthy market data

**What to build:** Use guided setup to show the supported BTC market and the exact data sources the learning strategy needs.

**Spec coverage:** 3, 4, 9, 11.

**Blocked by:** 3, 4.

**Proposed label:** `ready-for-agent`.

- [ ] Connection adapters preserve source, units, timestamps, subscription identity and sequence. Snapshot/delta state is maintained correctly; receipt of an acknowledgment alone is not healthy data.
- [ ] Sources view distinguishes missing credentials, missing entitlement, stale data and disconnected state. Never substitute another source silently.
- [ ] Confirm required source availability and retention/licensing constraints from official evidence; if BRTI is required, actual access is a separate authorized account check, not inferred from login.
- [ ] Adapter fixtures exercise healthy data, wrong source, absent data and denied access. No purchase endpoint can be invoked by this read-only slice.

### 6. Complete one paper purchase through settlement and history

**What to build:** Run the learning strategy in paper mode from the next full market boundary to an accounted-for settlement.

**Spec coverage:** 3–8, 10–12.

**Blocked by:** 4, 5.

**Proposed label:** `ready-for-agent`.

- [ ] One shared engine evaluates rules, validates fixed run settings, builds requests, records intent before execution and records outcomes. Paper execution is an adapter, not a second engine.
- [ ] Initial finite-depth simulation handles one full-fill or zero-fill outcome, fees and settlement with documented assumptions; no fill is invented from intent alone.
- [ ] Basic funds, quantity, price and worst-case-loss validation exists from the first purchase; skip rather than resize. Exact money arithmetic and one objective per market.
- [ ] Go, Stop-after-market, Pause-now and quit have their specified distinct behavior; active settings are immutable. UI and history distinguish requested, filled, unsettled and settled. Live remains disabled.
- [ ] Use controlled exchange observations/clock for repeatable engine-interface tests and a paper walkthrough; no new independently implemented strategy path.

### 7. Complete partial paper purchases without exceeding the objective

**What to build:** Handle partial fills and eligible retries toward the original requested quantity.

**Spec coverage:** 5, 7, 8.

**Blocked by:** 6.

**Proposed label:** `ready-for-agent`.

- [ ] Confirm the prior attempt is terminal before another; compute remaining quantity from deduplicated confirmed fills and never exceed the objective.
- [ ] Any fill locks the side. Before any fill, side changes require a finished prior attempt and current eligibility; no hedge or second objective.
- [ ] Recheck rules, fresh data, deadline, prices, funds and reservations before each dispatch. Stop retrying when eligibility ends; retain purchased contracts.
- [ ] Simulator models finite quantity, partial/no fills and fees without granting the same liquidity repeatedly from unchanged data; test multiple partials and terminal zero-fill paths.

### 8. Recover an uncertain purchase without submitting a duplicate

**What to build:** Show an honest paused state after a lost purchase response or crash, then reconcile the original attempt.

**Spec coverage:** 5, 6, 10, 13.

**Blocked by:** 7.

**Proposed label:** `ready-for-agent`.

- [ ] Record objective, attempt identity, request and reservation durably before any dispatch; persist confirmed outcomes before retries.
- [ ] Inject interruptions around recording, sending, acceptance, response and outcome commit. Unknown is never treated as zero fill; reservations remain until reconciled.
- [ ] Engine startup reconciles holdings and attempts but never resumes purchases automatically. Old/missing/unreadable records cannot establish an empty account.
- [ ] Exercise duplicate/conflicting outcomes and not-found/incomplete results. A reused client order ID is not assumed to guarantee exchange deduplication.
- [ ] Now shows “Trading paused — checking a purchase” and later the actual result; preserved history explains the recovery.

### 9. Preserve checkpoints and purchasing safety through feed interruptions

**What to build:** Recover from missing updates, reconnects and sleep/wake without losing observed checkpoints or buying on stale information.

**Spec coverage:** 4, 6, 9, 13.

**Blocked by:** 8.

**Proposed label:** `ready-for-agent`.

- [ ] Apply required book changes and relevant checkpoint observations before any latest-only UI summarization; detect sequence gaps and overload instead of silently losing required events.
- [ ] Reconnect with fresh named-source state and order reconciliation. Continue in the same window only when all rules still qualify and no outcome is uncertain.
- [ ] Timers handle deadlines and market boundaries even without new quotes. Sleep/wake or clock changes do not replay missed purchase ticks or bypass full-boundary start.
- [ ] Test burst updates, transient crossings, delayed/stale results from an old subscription/market, silence and slow recording. Pause remains responsive and controls are serialized with dispatch.

### 10. Enforce daily limits across settlement, midnight and restart

**What to build:** Display trustworthy Metten-only daily results and stop or resume runs according to the agreed limits.

**Spec coverage:** 5, 6, 8, 11.

**Blocked by:** 8.

**Proposed label:** `ready-for-agent`.

- [ ] Reserve full worst-case objective cost including fees, in-flight attempts and unsettled holdings; reject unaffordable objectives rather than resize.
- [ ] Count only Metten-owned settled live results after fees, attributed to the market-close UTC day. Exclude unrelated account activity and paper activity; deduplicate settlement.
- [ ] Test delayed settlement across midnight, pending exposure, partials, restart and strategy changes without resetting results.
- [ ] Daily thresholds finish the current market under remaining safety checks, then block the next. Configured next-day resumption never overrides manual stop, crash recovery or paper-only mode.
- [ ] History and Now distinguish settled results, pending risk and blocking reason; do not promise an exact guaranteed daily loss cap.

### 11. Record paper qualification and authorize future live transitions

**What to build:** Explain whether a strategy version is live-eligible and capture explicit settings for an optional paper-to-live transition.

**Spec coverage:** 3, 6–8, 11.

**Blocked by:** 7, 8, 10.

**Proposed label:** `ready-for-agent`.

- [ ] Qualification requires a paper purchase (a confirmed partial qualifies), resolved errors/accounting and accounted-for settlement; no win is required. Version changes reset eligibility.
- [ ] Paper is default. Display quantity, price range and daily limits before explicit automatic-transition authorization; enter only a future full eligible market after qualification.
- [ ] Data gaps, manual stop, crash recovery and daily-limit precedence cannot create an unexpected mid-window live transition.
- [ ] Tested settings remain visible when quantity/prices later change. Shared state tests prove transition gating; production submission stays unavailable until the integration and release gates are satisfied.

### 12. Execute and reconcile Kalshi IOC requests through the shared engine

**What to build:** Connect the shared engine to the current Kalshi order contract with controlled transport and explicit integration evidence.

**Spec coverage:** 5–9, 13.

**Blocked by:** 5, 8, 9, 10, 11.

**Proposed label:** `ready-for-agent`.

- [ ] Encode both economic sides, IOC, exact fixed-point quantity/price, fees and the documented current request schema in the exchange adapter; no selling or resting fallback.
- [ ] Exercise acceptance, rejection, partial/no/full fill, lost reply, order/fill lookup, pagination and settlement with deterministic transport fixtures through the same lifecycle as paper.
- [ ] Do not rely on order-ID reuse after terminal outcomes for duplicate protection. Reconcile scoped identities and actual filled quantity before another attempt.
- [ ] If account/demo checks are separately authorized, report exact evidence and differences from fixtures; do not represent fixture tests as live integration.
- [ ] Production purchases require explicit qualified live authorization and remain unavailable in distributed builds until release gates are met. No real-money validation is authorized by this issue.

### 13. Run independent paper tests without slowing live control

**What to build:** Run several paper sessions with independent balances and create a fresh test without deleting history.

**Spec coverage:** 2, 7, 8, 11.

**Blocked by:** 7, 9, 10, 11.

**Proposed label:** `ready-for-agent`.

- [ ] Paper balances, reservations, settings and daily limits remain separate from each other and live accounting.
- [ ] One live run is enforced across duplicate app instances. Duplicate launch cannot create another purchase engine against the same workspace.
- [ ] Admission bounds and queue behavior protect live control/recovery work; each view clearly identifies mode and run.
- [ ] Test simultaneous paper sessions, stop/start and fresh tests against retained earlier history. Capacity is measured later, not advertised from a guessed constant.

### 14. Export, back up and restore history without losing recovery state

**What to build:** Inspect storage use, export history, and create or restore a consistent secret-free workspace backup.

**Spec coverage:** 9, 10, 11.

**Blocked by:** 8, 10, 11.

**Proposed label:** `ready-for-agent`.

- [ ] Use consistent SQLite backup/restore and bounded automatic rotation. Restore requires provider reconnection where appropriate and never automatically resumes trading.
- [ ] Preserve exact strategy/settings, qualification, accounting and unresolved obligations; old-backup or missing-record uncertainty blocks new purchases until reconciled.
- [ ] Diagnostic logs rotate at seven days or 100 MB, whichever first, and never substitute for durable recovery. Debug capture is opt-in and time/size bounded.
- [ ] Monitor database auxiliary files and free space; disk/full/write failure blocks new purchases while retaining obligations. Test export redaction, concurrent backup, restore and migration refusal/rollback behavior.
- [ ] Explicit history removal cannot silently discard records required for unresolved orders/holdings. Document safe restrictions.

### 15. Finish the branded setup and authoring walkthrough

**What to build:** Use the approved Metten identity through welcome, source setup, strategy import, Now, History and settings, with complete user and agent guides.

**Spec coverage:** 3, 4, 9, 11, 12.

**Blocked by:** 3, 4, 6, 11, 14.

**Proposed label:** `ready-for-agent`.

- [ ] Use approved current branding assets with verified license/provenance, accessible controls and keyboard navigation; do not substitute historical logo variants.
- [ ] Walk a new user through setup and paper execution without terminal commands. Distinguish no signal, strategy skip, user-limit block, failed order and no fill; detail stays behind an explicit view.
- [ ] Complete field reference and worked learning example agree with validation and generated explanations. Agent guide covers supported inputs, JSON creation, validation, paper outcomes and limitations without inventing features.
- [ ] Document Stop/Pause/quit, limits, qualification, security, recovery, storage and simulation assumptions. Record a walkthrough against the implemented version; do not claim live proof.

### 16. Measure engine responsiveness and enforce supported capacity

**What to build:** Publish measured limits and keep the engine truthful and responsive under supported Mac load.

**Spec coverage:** 2, 6, 13, 15.

**Blocked by:** 12, 13, 14.

**Proposed label:** `ready-for-agent`.

- [ ] Measure source age, evaluation, durable recording, send, exchange-response and UI delays separately, including tail behavior; controlled transport is labeled separately from real network observations.
- [ ] Exercise paper concurrency, bursts, slow disk, CPU pressure, renderer stalls and recovery load; choose freshness/admission/retry bounds from evidence.
- [ ] Overload blocks unsafe new purchases rather than silently dropping required observations or starving pause/reconciliation. Record supported Mac and test conditions.
- [ ] Report unresolved limits honestly. No speed claim or trading outcome guarantee follows from Rust/Tauri alone.

### 17. Verify security and failure recovery independently

**What to build:** Produce an evidence-backed review of malformed input, credential boundaries and purchase recovery before distribution.

**Spec coverage:** 9, 10, 13, 15.

**Blocked by:** 12, 13, 14, 15.

**Proposed label:** `ready-for-agent`.

- [ ] Independent review exercises oversized/malformed strategies, untrusted renderer messages, secret leakage, dependency risks and restricted command/navigation permissions.
- [ ] Verify packaged Keychain attributes, denied/locked access and credential continuity on signed updates. Report actual prerequisites or missing evidence.
- [ ] Check crash/write/disk failure, duplicate-attempt defenses and unresolved exposure against release requirements; distinguish process termination from real power-loss durability evidence.
- [ ] Record and resolve blocking findings in GitHub. No live calls or credentials are implied by this verification ticket.

### 18. Install and update a signed Mac release candidate

**What to build:** Install the release candidate on a clean supported Mac and update it while preserving local state safely.

**Spec coverage:** 2, 3, 9, 12, 13.

**Blocked by:** 1, 15, 16, 17.

**Proposed label:** `ready-for-agent`.

- [ ] Signed/notarized distribution and selected Mac architectures are verified through download/open/setup without separately installed runtimes or containers; paid prerequisites require owner approval.
- [ ] Update flow requires safe stop, verified artifact and preserved unresolved state; restart reconciles without automatically buying. Incompatible database rollback is refused safely.
- [ ] No telemetry, Metten account or remote trading service is introduced. Any update check is disclosed and does not upload workspace data.
- [ ] Guides match the candidate. Production trading stays gated pending the separately authorized acceptance task; no profitability/readiness claim without evidence.

### 19. Perform separately authorized live acceptance and record release evidence

**What to build:** Demonstrate the specified clean-setup to paper qualification to live purchase to settlement path under exact owner authorization.

**Spec coverage:** 7, 13.

**Blocked by:** 18.

**Proposed label:** `ready-for-human`.

- [ ] Before any real-account or real-money action, present the exact account/mode, market, quantity, prices, worst-case spend/fees, timing, stop conditions and recovery plan; obtain explicit permission for that test.
- [ ] Run the approved learning strategy through natural paper settlement and qualifying accounting, then only the precisely authorized live purchase. Paper success is not live proof.
- [ ] Confirm actual IOC outcome, fills, fees, holdings and settlement; demonstrate the required connection recovery and readable history without duplicate purchases.
- [ ] Reuse unchanged passing checks, attach the exact candidate/version and evidence, and leave release blocked on any unresolved required check. If permission or account access is absent, stop at the prepared test plan.
