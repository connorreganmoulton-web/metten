---
status: accepted
---

# Separate the Rust trading engine from the Tauri interface and strategy rules

Connor selected Rust for Metten's engine and Tauri for its Mac application on 12 November 2026, with maintainable responsibility boundaries as the deciding requirement. Keep the engine independent of the desktop framework: supported JSON strategies describe conditions, while shared engine code owns purchase authorization, limits, execution state and recovery; trusted adapters own provider communication, persistence and Mac Keychain access, and the interface requests actions and displays results. These are modules within one local application, not separately operated services.

## Consequences

- A supported strategy-rule change requires only a new validated strategy version; connection, purchase, accounting and recovery code must not branch on strategy names.
- New inputs or operators require explicit capability extensions. Shared rules are evaluated consistently in paper and live modes; only final execution differs.
- Provider-specific message formats stay in adapters. The engine's decisions and recovery can be tested with controlled inputs without launching the interface or contacting an exchange.
- The interface owns neither credentials nor financial truth. SQLite durability and local-only Mac Keychain remain required; the bounded local packaged evidence is recorded below, with broader validation still outstanding.
- Review dependency boundaries and test behavior across them. Separate files alone do not establish independence; generic request/state types must not hide strategy-specific executable callbacks.
- Rust/Tauri introduces a language boundary and native integration work. TypeScript/Electron was a credible alternative; this selection does not claim measured speed superiority or guaranteed reliability.

This records the architecture direction, not authorization for live actions or proof of implementation readiness. Further implementation and integration evidence remain engineering work within separately authorized implementation scope.

Product contract: [Gen 1 specification](../../METTEN_DESIGN_DRAFT_2026-11-12.md). Public mechanism references: [Tauri process model](https://v2.tauri.app/concept/process-model/), [Tokio message passing](https://tokio.rs/tokio/tutorial/channels). These references establish available mechanisms, not Metten performance or reliability.

## Evidence update — 13 November 2026

[Issue #2](../verification/issue-2.md) records the local packaged controlled-session, SQLite settings and restart/failed-save checks. The finished section of [issue #3](../verification/issue-3.md) records isolated dummy-credential add/read/replace/delete, refusal and guarded-quit checks in a local ad-hoc-signed arm64 bundle. These results support the initial integration direction without changing this decision. They do not prove distribution signing/notarization, upgrade identity continuity, other Macs, physical power-loss durability, real provider/account integration or trading readiness. The records retain their exact tested versions and limitations; this documentation update runs no new acceptance checks.
