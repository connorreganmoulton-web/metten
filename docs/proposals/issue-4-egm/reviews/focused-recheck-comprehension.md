## What this proposes

Issue #4 would let users import supported strategy JSON, preview its meaning, save immutable versions, inspect earlier versions and edit copies. One Rust evaluator would own validation, calculations, remembered conditions and purchase eligibility; the desktop interface would display its results. Calculations use six decimal places and a complete one-millisecond timeline, with explicit evidence, history and resource requirements. Human and agent guides would teach the same contract, while actual feeds, trading and recovery integration remain later work.

## How the existing system works, per this document

Metten already has a local Rust/Tauri application, append-only SQLite strategy storage, workspace protection and controlled session functions. Its unfinished format-1 strategy draft is narrower than the agreed capabilities, and existing Start/Pause controls operate a non-trading foundation. Existing stored strategy and session records must survive the proposed changes. The document identifies unfinished source changes and historical checks but does not present them as verified implementation of this design.

## Findings

No Blocker, Major or Nit comprehension findings.

The arithmetic fixture is self-contained and reproducible as a specification:

- **Lines 421–423:** Identify both feeds, exact input values, four half-open intervals, complete controlled coverage, the percentage formula, the four-second average and its evaluation time.
- **Lines 409–415:** Define normalization boundaries, scaled arithmetic and nearest-even rounding, so a reader need not consult the rejected arithmetic proposal.
- **Lines 437–439:** Explain which completed cells enter the average, including why a new endpoint value cannot alter the window ending at 4000 ms.
- **Lines 423–425:** Give authoritative intermediate and final expected values and distinguish the historical fraction rationale from format-2 behavior.
- **Lines 697 and 717:** Explicitly require the fixture in the shared reference and later engine verification.

The remaining sections connect the proposed grammar, evidence ordering, state lifetimes, bounded processing, storage migration, interface flow and verification requirements sufficiently to explain their intended behavior without opening supporting documents.

## Read and scope attestation

Read only:

1. `/Users/connormoulton/.codex/plugins/cache/egm/egm/0.1.1/agents/comprehension.md`
2. `/Users/connormoulton/Desktop/metten/docs/proposals/issue-4-egm/strategy-rules-design.md`, all lines 1–755.

Reviewed FeatureRange 1–324 and DetailedRange 325–755 in REVIEW mode. Verified target SHA-256: `741a728267de62ab7bc2856672071f49b3e0169bab00f5012b53e4363aac48d7`. No additional standard was supplied or applied.

No supporting files, prior reviews, source, history, memory or links were consulted; no writes, builds, tests, external retrieval or delegation occurred. This document-only comprehension result does not establish implemented arithmetic correctness, independent authoring usability, performance or trading readiness.
