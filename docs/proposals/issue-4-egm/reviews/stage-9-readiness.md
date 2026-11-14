Readiness review is complete. The plan still leaves three consequential implementation choices unresolved.

- **R9-1 — Major — lines 438–439, 458 and 525–534: Define how bounded processing yields and receives control.**  
  `advance` returns a final `Decision` synchronously, while the plan requires processing to yield every 4096 operations, inspect control/queue state and stop immediately. No continuation result, control input during processing, or worker contract supplies that behavior. A small batch advancing through a long quiet interval can require millions of ticks while a subsequent `Stop` cannot enter through the documented interface. Queue byte/count limits do not decide when catch-up becomes processing-unavailable.  
  **Smallest correction:** specify one resumable processing contract, including continuation ownership, control inspection, candidate suppression and the deterministic admission/refusal condition. Add one controlled long-catch-up/Stop assertion. Production timing measurements can remain later evidence.

- **R9-2 — Major — lines 348–352, 438, 444–450 and 460: Define contract binding and collection before market entry.**  
  The design permits long contract lookbacks when an actual contract can supply them, but observations carry only `binding_id`; its mapping to actual market/contract identity is not specified. `RunContext` fixes a series, and `MarketBoundary` supplies contract identity when entering a market. The document does not define how controlled evidence binds an upcoming contract before its open, or how that same contract’s collected history survives entry while the previous contract’s state resets.  
  **Failure example:** a one-hour `current_contract` average has sufficient controlled evidence before its market opens. An implementer must invent how to represent that evidence and decide whether the boundary preserves or discards it.  
  **Smallest correction:** specify the binding map, when binding/collection may begin, and the boundary preservation/reset rule keyed to actual contract identity. Include a controlled same-contract pre-open-history case and a different-contract rejection case. This requires no provider feasibility proof now.

- **R9-3 — Major — lines 329–649, particularly 483–512 and 627–649: Add incremental, buildable implementation phases.**  
  The numbered sections define responsibilities and final verification, but do not partition implementation into stable checkpoints. Replacing the existing strategy interface, migrating storage and updating native/frontend callers crosses several coupled contracts; the implementer must choose the transition sequence and temporary compatibility behavior.  
  **Failure example:** exporting the new request/result types before updating storage and desktop callers leaves the workspace unbuildable between steps, with no prescribed checkpoint or safe ordering.  
  **Smallest correction:** add a short ordered phase table identifying each phase’s files, completed behavior and applicable checks. Specify how existing callers remain buildable until their replacement lands. No builds need to run during this review.

The grammar, fixed arithmetic, complete calculation clock, cumulative history, allocation formulas, immutable storage policy and required behavior checks are substantially specified. I found no additional independent readiness finding requiring expansion of this review.

**Read attestation:** I read the full target, the implementation-readiness persona and standard, plugin `docs/codex.md`, and all eight supplied context documents: concepts brief, product specification, handoff, ADR 0001, implementation plan, targeted adversarial record and both Kalshi captures. Verified target SHA-256: `3f6f8c23492a41131f56cb81e899860da5565a28519f61ac79ffbe081731c853`. Review scope was lines 325–664, with the full document used for definitions.

No files changed; no builds, tests, network, provider, account, database or publication operations ran. Actual implementation, usability, performance and provider evidence remain unverified.
