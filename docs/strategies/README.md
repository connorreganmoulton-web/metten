# Write and inspect a strategy

A strategy is a local JSON file describing when a purchase could be considered. Metten validates it, generates an explanation and saves immutable versions. It does not run these rules against connected markets yet. Start/Pause operate only the controlled non-trading session.

- [Learn to author and save a strategy](authoring.md): a plain-English walkthrough.
- [Exact field reference](reference.md): supported fields, calculations, timing, evidence, limits and compatibility.
- [Author with an agent](agent-authoring.md): a task recipe using the same reference.
- [Shared learning file](../../examples/learning.json): download or copy this file as UTF-8 JSON.

The example teaches mechanics, not profitability or an approved D/F strategy. Rules contain no credentials or private records. Successful validation proves supported syntax and meaning; it does not prove source access, licensed data, connected execution, paper qualification, production capacity or live fills. See [the product specification](../../METTEN_DESIGN_DRAFT_2026-11-12.md) and [local evidence](../verification/issue-4.md).
