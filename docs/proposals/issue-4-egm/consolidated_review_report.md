# Issue #4 — consolidated EGM design review

Date: 2026-11-13  
Mode: full uninterrupted review, selected by Connor. Both linked public Kalshi pages were included as authorized.  
Outcome: **review complete; design needs targeted revision before implementation.** No implementation or publication is authorized.

## What Connor needs to know

The shared-engine approach survived both architecture reviews. The reviewers did not call for a new service, another evaluator, a new dependency or a broad redesign. They did find engineering instructions that need to be completed before someone starts coding.

The most consequential repairs concern responsive Stop controls during long calculations, the exact contract to which collected history belongs, and a limit on simultaneous desktop strategy requests. The plan also needs a safe build sequence, one missing test-harness file, validation of rules that can never finish in their market, and several document corrections. These are engineering work for the lead to resolve; Connor does not need to design the mechanisms.

All five stages completed through **10 actual independent native review runs**. No reviewer assigned a Blocker label. There are **17 raw findings: eight Major and nine Nit**. After combining overlap, they form **12 repair items: eight Major and four Nit**. These are severity labels from the installed personas, not numerical scores or proof of trading safety. The zero-consequential-decision readiness requirement remains unmet.

Recommended next EGM step: resolve these findings in the design, preserve the approved product behavior, and run only the affected closure checks/reviews. Keep implementation out of this session. The approved review target was not edited while being reviewed.

## Target and completed stages

Target: [strategy-rules-design.md](strategy-rules-design.md), 664 lines.  
SHA-256 before and after review: `3f6f8c23492a41131f56cb81e899860da5565a28519f61ac79ffbe081731c853`.  
FeatureRange: 1–324. DetailedRange: 325–664. The draft copy remains identical to this target.

| Stage | Native runs | Result | Stage report |
| --- | --- | --- | --- |
| 5 — Initial Zoom Out | `/root/egm5_zoom`, `/root/egm5_inventive` | No findings; no false compromise detected; reuse searches completed | [Initial architecture review](zoom_out_report.md) |
| 6 — Comprehension | `/root/egm6_comprehension` | All sections could be paraphrased; one Major self-containment gap | [Complete paraphrase and finding](comprehension_report.md) |
| 7 — Critic | `/root/egm7_design`, `/root/egm7_structure`, `/root/egm7_style` | Two Major and four Nit raw findings | [Critic report](critic_report.md) |
| 8 — Intermediate Zoom Out | `/root/egm8_zoom` | No new architectural findings; inherited findings remain open | [Architecture recheck](intermediate_zoom_out_report.md) |
| 9 — Implementation Readiness | `/root/egm9_readiness`, `/root/egm9_codebase`, `/root/egm9_style` | Five Major and five Nit raw findings; not implementation-ready | [Readiness report](implementation_readiness_report.md) |

Stages ran in this order. Roles within stages 5, 7 and 9 ran independently in parallel within the native concurrency limit. Direct reviewer dispatch followed the installed review skill's uninterrupted subflow; the lead synthesized actual completed results. No resolution loop, automatic design edits or implementation ran. Earlier brainstorm and targeted consultations were context, not substitutes for these stages.

## Prioritized repair list

### CR-01 — Major — make long calculations responsive to Stop

**Sources:** R9-1. Target lines 438–439, 458, 525–534.

The plan promises a control check after bounded work but exposes only a synchronous call returning the final decision. It does not define how that call can suspend, retain its continuation or receive a later Stop. A small input batch can still represent millions of logical calculation steps.

**Required design repair:** specify one concrete bounded/resumable processing contract, who owns its continuation, how control interrupts are inspected, when processing is admitted/refused, and why no intermediate/stale purchase candidate can escape. Keep the pure engine independent of Tauri and provider clients. Do not add another service.

**Closure evidence:** a controlled long quiet-interval catch-up with Stop delivered between work slices suppresses candidates without completing the whole catch-up first; document exact continuation/cancel/refusal states. This is a required future behavior check, not an executed result. Measured production speed remains later evidence.

### CR-02 — Major — define upcoming-contract binding and history preservation

**Sources:** R9-2. Target lines 348–352, 438, 444–450, 460.

An abstract input binding is not enough to identify an upcoming contract before entry. The plan allows feasible long contract history but supplies contract identity at the market boundary and also resets contract-specific state there. A developer must otherwise invent how the same contract's pre-open history survives while a different contract's book cannot be substituted.

**Required design repair:** define the concrete binding map and collection lifecycle keyed to actual market/contract identity, the earliest permitted post-Go collection, and the preservation/reset rule at entry. Preserve checkpoints' per-market reset and continuous compatible underlying history. Do not infer listing lifetime from the 15-minute outcome period or add provider integration.

**Closure evidence:** controlled pre-open history for the same contract remains usable on its entry; a different contract's data cannot satisfy it; pre-Go history is rejected. Actual source feasibility stays a later adapter gate.

### CR-03 — Major — bound native strategy-operation admission

**Sources:** B9-2. Target lines 464–477, 494, 520–526, 646; source details in the raw Codebase report.

The current native bridge starts blocking work for each request. Per-request size limits and disabled frontend buttons do not bound concurrently queued direct IPC requests, and strategy storage shares a recorder mutex with session recording.

**Required design repair:** define a native admission guard before spawning work. The source reviewer recommends one in-flight strategy operation, nonblocking acquisition, a finite Busy response for excess requests, and guaranteed release on every completion/failure. Keep quit handling and existing permissions. This needs no dependency or wider architecture change.

**Closure evidence:** direct IPC attempts during an active operation receive Busy; after success/error the next allowed operation works. Confirm that rejected calls do not allocate unbounded waiting tasks or retain arbitrary payloads.

### CR-04 — Major — prescribe buildable implementation phases

**Sources:** R9-3. Target lines 329–649, especially 483–512 and 627–649.

The file list and final checks are detailed, but the transition sequence across engine, storage, native requests and frontend callers is not defined. An implementer still has to decide when interfaces change and how callers stay buildable.

**Required design repair:** add a short ordered phase table with exact files, completed behavior, caller compatibility/removal rules and applicable checks for each stable checkpoint. Keep it within one bounded issue and existing tools. No builds need to run during document resolution.

**Closure evidence:** each phase's described state is buildable and testable without undefined temporary semantics; the final phase removes any explicitly temporary compatibility path. Reuse passing checks where unchanged.

### CR-05 — Major — include the required existing refresh-test harness repair

**Sources:** B9-1. Target lines 485–499; source `tests/refresh.test.mjs:64–77` and `src/main.ts:94,121`.

The existing harness groups every non-control request into its deferred snapshot list. Strategy startup therefore occupies a snapshot slot. The proposed tagged strategy-list request needs a separate response, but the supposedly complete file allowlist omits this existing test file.

**Required design repair:** add **Modify `tests/refresh.test.mjs`** with a command-specific strategy-list mock; only snapshot requests enter the deferred snapshot array. Preserve the existing stale-response assertions. Do not add unrelated credential-test cleanup: the source reviewer found no equally necessary edit there.

**Closure evidence:** the intended before/after-Pause snapshot ordering remains the thing being tested, alongside independent strategy-list startup. No test was executed during this audit; this is a source-grounded planning finding.

### CR-06 — Major — reject demonstrably unreachable market-local holds

**Sources:** D7-1. Feature lines 152, 168, 195–199; detailed definitions 395, 429, 523.

A hold that begins no earlier than market open and lasts 900 seconds cannot finish before an exclusive 900-second close. A one-hour hold cannot complete in that market either. This is different from an hour-long underlying lookback, which may span markets legitimately.

**Required design repair:** define field-specific reachability validation and repair guidance for holds that cannot finish before their enclosing effective deadline. The smallest route is rejection of demonstrably unreachable rules rather than silently accepting an option that can never qualify. Do not impose a market-length cap on historical lookbacks or attempt a general theorem prover for arbitrary strategy profitability/reachability.

**Closure evidence:** `[0,900)` plus a 900-second hold is diagnosed; a hold completing strictly before the end can qualify under the existing evidence rules; legitimate hour-long history remains supported.

### CR-07 — Major — make the arithmetic regression fixture self-contained

**Sources:** C6-1, T9-4. Target lines 407–409, 621, 638.

The standalone target refers to four earlier percentage values and a 147-bit-denominator counterexample without containing its operands. The supporting targeted record has the numbers, but the document-only reviewer correctly could not reconstruct the required verification from the target alone.

**Required design repair:** include a small named controlled fixture, its exact graph, normalization boundaries and expected results. Existing evidence supplies Coinbase BTC/USD fixed at `100001` and four equal one-second Kraken BTC/USD values: `100000.000003`, `100000.000019`, `100000.000057`, `100000.000063`. Compute percentage difference against Kraken, then a four-second time average. The revised per-node six-place results are `0.001 percent` and their average is `0.001 percent`. Do not replace the already selected decimal policy with another arithmetic framework.

**Closure evidence:** a fresh reader can reproduce all operands/intermediate/final results without prior conversation or review history; later engine checks exercise that exact fixture. The existing standalone arithmetic check is design evidence, not an engine test.

### CR-08 — Major — remove stale operative drafting instructions

**Sources:** T7-1, S7-1, T9-3. Feature lines 34, 51, 134, 154, 238, 286, 320–322; detailed lines 651, 664.

The finalized target still directs the reader to draft or approve already completed sections. Statements that formal review has not started also need their historical finalization date, now that this review is complete.

**Required design repair:** use current cross-references to completed definitions; keep approval chronology in provenance; date historical handoff claims. Do not erase genuinely missing implementation/operational evidence or turn section approval into implementation authority.

**Closure evidence:** there is one unambiguous current workflow state, and no imperative to redo completed drafting. Updating the reviewed target creates a new target hash; these reports continue to identify the original snapshot.

### CR-09 — Nit — consolidate chronology and repeated authority scaffolding

**Sources:** T7-2, T9-1. Feature lines 51, 171, 175, 244, 320; detailed lines 327–329, 651, 658–664.

Move repeated conversation history and approval quotations into the existing provenance record. Keep operative requirements and technical rationale where they are used, a short opening authority reference, and relevant verification limits beside their checks. Preserve money, data and no-implementation boundaries.

### CR-10 — Nit — narrow the public instrument-list claim

**Sources:** D7-2, T7-3. Target line 318.

The preserved captures support the multiple-series context but do not independently establish the full named BTC/ETH/Gold/Silver enumeration. Narrow to the supported observation or preserve the actual indexed evidence before relying on that enumeration. The smaller route is wording correction; broader adapter support remains outside this issue.

### CR-11 — Nit — name the walkthrough accurately

**Sources:** T9-2. Target line 625.

Change “documentation-only walkthrough” to “independent documentation-led authoring walkthrough.” Documentation is the participant's instruction source, while engine preview/save checks remain part of the exercise. Do not drop those required executable checks.

### CR-12 — Nit — separate summary-count and byte limits

**Sources:** T9-5. Target line 526.

Replace compressed `50/32 KiB` notation with at most 50 summaries and 32 KiB per page, explicitly stating that the byte ceiling applies to the complete serialized response. Preserve bounded paging rather than silently truncating summaries.

## Raw-finding traceability

Each raw finding maps to one consolidated repair. Stage 8's inherited list is confirmation of open items, not new independent findings.

| Raw finding | Original severity | Consolidated repair | Raw report |
| --- | --- | --- | --- |
| C6-1 | Major | CR-07 | [Comprehension](comprehension_report.md) |
| D7-1 | Major | CR-06 | [Design](reviews/stage-7-design.md) |
| D7-2 | Nit | CR-10 | [Design](reviews/stage-7-design.md) |
| S7-1 | Nit | CR-08 | [Structure](reviews/stage-7-structure.md) |
| T7-1 | Major | CR-08 | [Feature style](reviews/stage-7-style.md) |
| T7-2 | Nit | CR-09 | [Feature style](reviews/stage-7-style.md) |
| T7-3 | Nit | CR-10 | [Feature style](reviews/stage-7-style.md) |
| R9-1 | Major | CR-01 | [Readiness](reviews/stage-9-readiness.md) |
| R9-2 | Major | CR-02 | [Readiness](reviews/stage-9-readiness.md) |
| R9-3 | Major | CR-04 | [Readiness](reviews/stage-9-readiness.md) |
| B9-1 | Major | CR-05 | [Codebase](reviews/stage-9-codebase.md) |
| B9-2 | Major | CR-03 | [Codebase](reviews/stage-9-codebase.md) |
| T9-1 | Nit | CR-09 | [Detailed style](reviews/stage-9-style.md) |
| T9-2 | Nit | CR-11 | [Detailed style](reviews/stage-9-style.md) |
| T9-3 | Nit | CR-08 | [Detailed style](reviews/stage-9-style.md) |
| T9-4 | Nit | CR-07 | [Detailed style](reviews/stage-9-style.md) |
| T9-5 | Nit | CR-12 | [Detailed style](reviews/stage-9-style.md) |

## Evidence, source limitations and review isolation

All 10 native review runs completed; no required read was reported missing. Each reviewer read its bundled persona and integration instructions, its assigned target scope, and applicable standards. Stage 6 received no linked project context and retained both required paraphrases in its full report. All other stages received the eight-file context in [review setup](review_setup.md); Stage 8 additionally read the completed earlier stage reports as prescribed. Code-reuse/current-state findings include actual source searches and file inspections, not a claim that planned files already exist.

Every formal reviewer used `fork_turns: none`; parent conversation turns were not supplied. Host instructions were visible to reviewers, and their exclusion is unverified. No separate marker probe was conducted. Do not claim full filesystem/instruction isolation, distinct model families, a five-model council or numerical scoring. No memory/conversation-history retrieval was requested or reported.

The approved public sources were saved as [market-list page extraction](review-sources/kalshi-15-minute-markets.md) and [15-minute guide extraction](review-sources/kalshi-15-minute-guide.md). The web reader returned both; direct raw-HTML downloads returned HTTP 429 and were not retried. The captures preserve their representation limits. The listing extraction is a dynamic shell; no browser or account was used to populate it. These sources do not establish Metten adapter capability, licensing, entitlement or compatible cross-market execution. No further external URLs were fetched for this review.

No Metten source/dependency files, approved target text, credentials or runtime records were changed by the review. No code tests, builds, browser checks, provider/account/SQLite operations, paid services, GitHub mutations, commits or publication ran. Reviewers examined source and proposed definitions; findings are not observed live failures or proof of an unsafe running bot. Actual implementation correctness, packaging, usability, field access, source validity profiles and measured capacity remain required later evidence.

## Stopping point

The requested full consolidated review is complete. Findings are open; no remediation is represented as already applied. The next authorized work should be EGM human resolution/document revision, with the lead owning routine engineering choices and surfacing only consequential scope/cost/risk decisions. Preserve the approved behavior and earlier passing evidence. Apply affected closure checks rather than restarting brainstorming or the entire review by default; respect the installed workflow's re-audit caps. Implementation remains excluded from this session.
