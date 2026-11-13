# Documentation map

## Overview

The [product specification](../METTEN_DESIGN_DRAFT_2026-11-12.md) owns intended behavior. Accepted [architecture](adr/0001-rust-engine-and-tauri-boundaries.md) and [licensing](adr/0002-mit-license.md) decisions explain binding choices. Code-grounded READMEs describe the implemented foundation and do not override those sources.

## Interface

- [EGM baseline and coverage](egm/README.md): component contracts, exclusions, reporting instructions and dated drift reports.
- [Implementation plan](implementation-plan.md): issue sequence; GitHub issue bodies own acceptance criteria and current task status.
- [Issue #2 verification](verification/issue-2.md) and [issue #3 verification](verification/issue-3.md): historical execution evidence with bounded results. Later checkpoints supersede earlier incomplete dispositions; neither is a current runtime or release guarantee.
- [Issue tracker](agents/issue-tracker.md), [triage labels](agents/triage-labels.md) and [domain conventions](agents/domain.md): repository working rules, complemented by root [AGENTS.md](../AGENTS.md) and [GLOSSARY.md](../GLOSSARY.md).
- `proposals/`: issue #4 planning and review artifacts, entirely excluded from this baseline. No files there are approved implemented contracts for this report.

## Invariants

Keep proposed, implemented and verified behavior distinct. Missing later-issue features are planned work; discrepancies within completed scope are drift. Preserve historical verification records and their limitations rather than silently rewriting them as fresh evidence. A contradiction with the product specification or an accepted decision must be surfaced before changing that requirement.

## Side Effects

These documents execute no code. Publishing tracker changes, executing verification or changing runtime behavior requires the corresponding task scope. This baseline changes documentation only.

## Verification

Check local links and compare contract statements with referenced code and completed-scope issue criteria. Use the reporting procedure in [egm/README.md](egm/README.md). No ignored private research, local planning or runtime records are included.
