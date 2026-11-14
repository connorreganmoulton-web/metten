# Import, explain and preserve supported strategy rules

* **Author:** Connor Moulton
* **Date:** 2026-11-13
* **Status:** Concept approved by Connor for design drafting. Detailed design, publication and implementation remain unapproved.

## Overview

Metten should let an author express supported purchase rules in JSON and let a user import them, understand their generated explanation, choose a saved version, configure a run and press Go. The same engine supplies connections, purchasing, limits, accounting and recovery. A new combination of supported rules must not require another bot or changes to the shared runtime.

Issue #4 establishes the supported rule contract, import/preview/save/edit-copy experience, immutable versions, matching field reference and one worked learning example. Controlled inputs establish rule behavior. Later tracked issues deliver connected sources and trading. Supported JSON is bounded data, not executable uploads or permission to bypass safeguards.

This brief preserves the [working hypothesis](working_hypothesis.md), [adopted decisions A–E and twelve examples](../issue-4-egm-handoff.md), [product baseline](../../../METTEN_DESIGN_DRAFT_2026-11-12.md) and [engine boundary](../../adr/0001-rust-engine-and-tauri-boundaries.md). Connor approved this conceptual direction and proceeding to drafting. The recommended engineering dispositions are drafting inputs; detailed mechanisms still need definition and review.

## Analysis

The most consequential gap is preserving a strategy's meaning across application changes. Keeping its file unchanged is insufficient if an updated engine interprets it differently or regenerates a different explanation. The draft storage reparses saved JSON using the current parser. The concrete design should identify each version's rule meaning, preserve historical records and explanations, and keep unsupported versions inspectable. Converting rules creates another version. This does not require keeping every unfinished evaluator executable forever.

Evidence and remembered state also need distinct lifetimes. A completed checkpoint survives a temporary source loss; an unfinished duration or rolling calculation still needs its own evidence. Only dependent conditions become unavailable. The engine must process observations and derived results coherently, establish checkpoints before dependent purchases on the same update, and apply priority and quantity restrictions consistently. This preserves the agreed behavior without a separate engine for each strategy or global blocking whenever one source is unavailable.

Flexible rules need justified resource limits and useful errors. A small file can demand substantial retained history or evaluation work. Engineering should define bounds and history eligibility, preserve the full agreed capability families, and explain demonstrated unsupported combinations before they become endless waits. A 15-minute market window does not establish how long its contract book can be observed; do not impose a contract-history limit from that assumption or join different contracts' books. Actual source capabilities remain unverified.

Explanations must make the combined behavior understandable: try the preferred option first, otherwise an eligible alternative, within one shared quantity objective. Distinguish remembered facts from evidence needed now. Import errors should identify the offending field or rule and how to repair it. A supported import does not mean a provider is connected or accessible. Controlled decision reasons should distinguish unmet conditions, missing evidence, priority and shared safeguards; later work connects them to runtime history. Compact local transitions suffice; continuous raw feeds or external monitoring are unnecessary.

The inventive reviewer found **no false compromise** needing a new architecture. Existing separation of rules, evidence and purchase authority costs explicit state bookkeeping. Validating before use and collecting history before starting cost time and bounded resources. The shared engine avoids competing purchase authority but needs coherent ordering. The ideal result remains one local application that uses one validated meaning for evaluation, explanation and documentation, retains only necessary evidence, and blocks precisely the conditions whose evidence is unavailable. Missing history and unknown order outcomes cannot safely be inferred away.

## Goals vs. Non-Goals

**In-scope goals, preserving adopted decisions:**

- Combine documented inputs and rules freely within the supported contract. Explicitly identify BRTI, Coinbase BTC/USD and Kraken BTC/USD; distinguish ask, bid, last trade, compatible underlying prices and reference/settlement values. Never silently substitute sources. Actual access is a later gate.
- Support signed/absolute and percentage differences, changes over time, crossings, rolling averages/minimums/maximums/ranges, author-selected durations and confirmation on distinct updates, for compatible contract and Bitcoin measurements. Bitcoin-derived direction is optional. Engineering defines formulas, units, precision, boundaries and observation identity; no universal sample count is imposed.
- Support multiple named, market-local checkpoints with remembered sides, same/opposite-side rules and ordered nested purchase alternatives. Preserve completed checkpoints after source loss. Current-state, crossing, duration and distinct-confirmation rules retain their different meanings; duplicates do not create confirmations.
- Define one purchase objective per market using the user's original quantity. Highest-priority currently eligible options are reconsidered after confirmed zero fill; unknown outcomes block another attempt. Any fill locks the side; eligible same-side options can complete only the remaining original quantity. Quantity and permitted prices remain run settings. Author conditions and user safeguards must both hold. IOC purchases only; hold through settlement.
- Collect initial history from the run's Go, wait for all required startup history across alternatives, then wait for the next full market. Show “Prefill — collecting live data,” then “Ready — waiting for next market.” Applicable underlying history spans markets; checkpoints and contract-specific state reset appropriately. Active-market recovery does not impose that initial wait again; crash/restart retains explicit recovery/start controls.
- Preserve verified quiet periods and trustworthy observations. A current snapshot cannot repair a missing historical path. Complete required rolling coverage may remain unavailable until a genuine hole leaves the window. Independent alternatives may proceed when their own evidence and common safeguards permit; no relaxed averaging mode is added.
- Strictly validate imports, generate explanations from the evaluated meaning, preserve earlier versions/history, prevent active-run version switching, and provide the matching field reference and one learning example. Demonstrate the twelve controlled examples through the public engine boundary when implementation is authorized.

**Out-of-scope non-goals:** real source adapters/access checks, production validity thresholds, purchasing/settlement/recovery integration, historical D/E/F reproduction, confidence models, volatility or crossing counts, richer depth/volume indicators, multiple independent objectives, per-option quantities, selling, hedging, arbitrary code, new services or an authoring platform. Later issues retain their existing acceptance and authority gates.

## Alternatives Weighed

The following dispositions originated as review recommendations and were accepted as inputs to drafting through concept approval. They are not claims that Connor selected detailed mechanisms or independently rejected each alternative during the review.

| Alternative | Proposed disposition and consequence |
| --- | --- |
| Restrict the product to the draft learning example | Do not adopt: it cannot express the already agreed combinations. Reuse its application boundaries and useful plumbing. |
| One global healthy/unhealthy switch or latest-value-only evidence | Do not adopt: it can block independent options or erase decision-relevant history. Explicit evidence bookkeeping costs more state but preserves the required outcomes. |
| Reinterpret every old version with the newest engine | Do not adopt: unchanged files could change meaning. Choose the smallest explicit compatibility policy; historical inspection need not imply ongoing execution support. |
| Unrestricted resource use or arbitrary inherited caps | Do not adopt either: justify concrete bounds without silently removing agreed capabilities. |
| Full raw-feed logging, remote tracing or a new explanation service | Do not adopt: compact local reasons and the shared validated representation meet this issue's need and fit the local-only baseline. |
| A general workspace downgrade system | Not needed for #4: atomic preservation and a clear refusal by incompatible older versions are a smaller route. |
| Begin an easy alternative immediately during initial prefill, or repeat prefill after ordinary recovery | Both conflict with already adopted startup/recovery distinctions. Preserve those decisions. |

## Key Decisions & Staging for Next Steps

The adopted product decisions remain unchanged. The review found no established contradiction requiring Connor to redesign scope. Source access, licensing or a consequential restriction discovered later could require a concrete owner decision; none is assumed here.

Recommended engineering dispositions for the next authorized phase are to define stable version meanings and safe workspace changes; specify evidence lifetimes and coherent evaluation; justify supported resource/history bounds; and make generated explanations and rejection reasons match that contract. Reuse the existing engine, local storage and desktop import boundaries while replacing draft restrictions that conflict with adopted behavior. Do not build later integrations to prove this rule contract.

Seven of seven requested native lenses completed with full read attestations and no failed/retried reviewers: architecture/economics, operational visibility, simplicity/reuse, UX, migration/compatibility, maintainability and inventive catalyst. [The review report](brainstorm_review_report.md) records every reviewer id, common reading list, deduplicated findings and links to each verbatim raw report. Dispatch used fresh parent-turn context. Reviewers saw host instructions; host instruction-file exclusion remains unverified, and no marker probe was conducted.

Missing evidence remains explicit: exact field and temporal definitions, controlled behavior/explanation checks, earlier-version compatibility and disposable workspace migration checks. No tests, builds, provider calls or trading checks ran during this brainstorm. Source feasibility, numerical production profiles, packaged behavior, capacity and live acceptance remain later evidence; paper or controlled outcomes cannot prove live fills.

Owner approval after reviewing this brief: “Yes, that reflects what I want. Let's move on to design drafting. Should we do that in a new session?” This confirms concept alignment and the choice to proceed to EGM drafting. The next gates are the drafting skill's section approvals; publication and implementation retain their separate authority requirements.

During review of the first draft section, Connor clarified that authoring documentation must include a complete repository guide for agents and understandable documentation for people, with a helper or skill as a possible future form. This clarification is recorded in `design_draft.md`; the original Goldfish reports predate it. It reinforces product specification section 12 rather than approving a new hosted wiki, skill package or publication action. The design must make these guides concrete alongside the shared field reference and worked example.

During Technical Plan review, Connor emphasized that strategy JSON itself must be simple to read and its variables intuitively named. The technical draft now includes descriptive naming, explicit units, minimal necessary nesting and a readability walkthrough of the complete learning file. This is a further owner clarification after brainstorming, not a finding claimed to have been reviewed by the original Goldfishes.

During Alternatives review, Connor clarified the eventual goal of all Kalshi 15-minute markets and carrying strategy logic across assets. Reusable names should therefore use terms such as `average_underlying_price`, with explicit instrument/source bindings, rather than hard-code Bitcoin. Initial BTC-only support remains the accepted release boundary; future market compatibility and any same-file binding mechanism still require explicit definition and evidence. The draft records public Kalshi listing evidence and the distinction between generic logic and actual compatible inputs. The original Goldfish reports predate this clarification.
