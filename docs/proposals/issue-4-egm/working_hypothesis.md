# Issue #4 — Phase 1 working hypothesis

Author: Connor Moulton  
Date: 2026-11-13  
Status: awaiting owner readiness confirmation; not a reviewed concept brief or approved design.

## Anchored hypothesis

Users will upload a supported JSON file directly into Metten, choose a saved strategy version, configure their run and press Go; Metten supplies the shared runtime for data connections, rule evaluation, purchases, limits, accounting and recovery. Authors can combine documented rules without changing that runtime for each strategy, while users receive explanations generated from the validated rules and retain immutable versions. Issue #4 establishes that strategy contract, import/explanation/versioning experience and controlled proof of its rule behavior; the later tracked issues deliver the connected trading experience.

## Why this exists and how to judge the design

Connor clarified the purpose in this session: keeping strategy rules separate from the runtime is the reason for Metten. In his Kalshi-Bot project, maintaining strategy and runtime together became difficult. This is owner-reported motivation, not a new technical diagnosis of that application's code.

The user-facing goal is upload JSON → choose strategy → configure run → Go. Users should not need to maintain a separate bot or edit Metten's source for a new combination of already-supported rules. JSON describes purchase conditions; Metten owns the backend and its safety responsibilities. New inputs or operators still require a deliberate supported capability extension. JSON is not arbitrary executable code.

Reviewers should challenge any proposal that requires strategy-name branches in provider adapters, duplicates execution/accounting/recovery for each strategy, or gives uploaded rules control over credentials or safeguards. A design that imports one example but cannot express new combinations of the agreed capabilities would not satisfy the purpose.

This follows [the product specification](../../../METTEN_DESIGN_DRAFT_2026-11-12.md), especially sections 1–5, 12 and 16, and [ADR 0001](../../adr/0001-rust-engine-and-tauri-boundaries.md). The [implementation plan](../../implementation-plan.md) separates issue #4's foundation from later source connections, paper execution and recovery work. The full eventual experience is the product goal; it does not expand #4 into implementing those later integrations.

## Preserved rule behavior

Rules can remember checkpoints and sides, use defined price and time measurements, and prioritize purchase options within one quantity-limited objective per market, while blocking only conditions whose required evidence is unavailable. Initial collection begins at Go: collect all history required by the strategy, then wait for the next full market before purchase evaluation; later recovery within an active market follows the separately agreed recovery rules.

## Owner clarification in this session

The question concerned a preferred option needing an hour of history and an alternative needing only current prices. The lead proposed waiting for all required initial history before either option could act, followed by the next full market.

Connor accepted:

> That works. Wait until all required history is collected, then start at the next full market. I don't want it to jump into a market immediately the second the timer goes off of enough data

This settles startup across alternatives. It does not impose a new full-market wait after recovery in an already-active market, erase checkpoints, or require a source used only for a completed checkpoint to remain current.

## Preserved boundaries

- Preserve every adopted decision in [the handoff](../issue-4-egm-handoff.md), sections A–E, and its twelve controlled examples. This note supplements that record; it does not replace it.
- Supported capabilities include explicitly named sources and price types; differences and changes; crossings; rolling averages, minimums, maximums and ranges; durations; distinct-update confirmations; multiple checkpoints; and ordered, nested purchase alternatives. Exact engineering definitions remain to be written and checked.
- One purchase objective per market uses the user's original quantity. Confirmed partial fills lock its side. Confirmed zero-fill outcomes allow reconsideration from the top; unknown outcomes block another attempt. Define rule behavior here without implementing later execution integrations.
- Preserve verified quiet periods and trustworthy history. Do not invent observations, fill historical holes from a current snapshot, or substitute named sources. Initial history comes only from the run's Go; applicable underlying history can span markets, while checkpoints and contract-specific state reset appropriately.
- Quantity and permitted prices remain run settings. IOC purchases only; hold through settlement. No selling, hedging, second objective, confidence model, or historical D/E/F reproduction.
- Keep engine evaluation, validation and generated explanations aligned with the matching field reference and one worked learning example. Preserve earlier versions and records through import and edit-copy behavior.
- Source access, licensing, actual adapters and production validity limits remain unproven later work. Controlled inputs can establish rule semantics in #4, not provider or live acceptance.
- Unfinished code and Revision 3 are evidence to inspect critically, not requirements. Preserve all existing workspace changes.

## Evidence and authority

Read the original product specification, glossary, ADR 0001, implementation plan, issue-tracker instructions, installed EGM brainstorming skill, Codex integration instructions and lifecycle architecture. Inspected the draft code boundaries and learning example without running them. Author resolved from Git configuration.

Read-only GitHub check in this session: issue #4 is OPEN, labeled `ready-for-agent`, has no comments, and its sole native blocker #2 is CLOSED. Its full acceptance criteria remain the issue's requirements; no GitHub changes were made.

EGM's own reviews and owner gates replace the earlier five-model score requirement for this trial. Native fresh-context dispatch is available through `spawn_agent` with `fork_turns: "none"`; no reviewers have been launched and actual isolation/review completion has not been demonstrated. Host instruction-file exclusion remains unverified as described in the installed Codex integration instructions.

## Next gate and trial outcome

Ask Connor to confirm this anchored hypothesis is ready for Goldfish review. Only after that confirmation, run the installed brainstorming review workflow using its actual personas and standards, within available concurrency. Record failures as incomplete evidence. Produce `concepts_brief.md` only as the workflow's synthesized output, then obtain its owner approval and the next-phase choice.

The trial aims to evaluate the full five-phase lifecycle: brainstorming, drafting, isolated review, human resolution/submission, and implementation/drift checking. Each later phase retains its approval gates. Publishing, GitHub changes and implementation need their corresponding owner instruction; this note authorizes none of them. Judge the trial by clearer requirements and consequential assumptions caught without repeated settled questions, unnecessary scope or premature code.

No code tests, builds, browser checks, provider calls, trading operations or live validation were performed. New writes in this session are limited to this planning artifact.
