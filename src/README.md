# Controlled-session interface

## Overview

This interface displays the saved state of a controlled, non-trading session and requests actions from the native application. It does not determine recording success or own session state. The [product specification](../METTEN_DESIGN_DRAFT_2026-11-12.md) describes the broader intended product; [ADR 0001](../docs/adr/0001-rust-engine-and-tauri-boundaries.md) assigns authoritative state to Rust and presentation to the interface.

This contract covers the existing session and isolated dummy-credential interface. Issue #4 strategy additions in `main.ts`, `contract.ts`, `style.css`, and the root HTML are excluded from this bootstrap.

## Interface

| Boundary | Contract |
| --- | --- |
| [contract.ts](./contract.ts): `Action`, `Record`, `Snapshot` | Actions are `start`, `pause`, and `advance`. Records contain ID, stopped/running/paused status, test tick, reason, and recorded timestamp. Snapshots carry running/pending state, a nullable recording error, history, and actual SQLite writer settings. |
| [main.ts](./main.ts): `element`, `render`, `refresh`, `control`, `poll` | Private entry-point functions bind required HTML IDs, render snapshots, invoke native `snapshot`/`control`, and refresh the display. Loading this module registers handlers and begins polling. |
| [view.mjs](./view.mjs): `presentation(snapshot)` | Exported pure function returns status/recording messages and the three control-disabled flags. [view.d.mts](./view.d.mts) declares that result for TypeScript callers. |
| [main.ts](./main.ts): credential operation handlers | Invoke native `credential` with `save`, `replace`, `remove`, or `check`; include `value` only for save/replace. Display native status or a filtered failure message. |
| [index.html](../index.html): session, credential, history, and details sections | Supplies required IDs, labelled sections, buttons, password input, status regions, error alert, and expandable writer details. The page identifies dummy verification and no exchange connection. |
| [style.css](./style.css): `.controls`, `button:disabled`, `:focus-visible`, `#error` | Wraps controls, distinguishes disabled buttons, outlines focused buttons/summary, and preserves multiline errors. [vite-env.d.ts](./vite-env.d.ts) references Vite client declarations. |

## Invariants

- `presentation` and `render` expect a nonempty `snapshot.history`; they read its first record. A recording error takes precedence over running/pending status, blocks start/advance, and never reports the requested change as saved. Pending work also blocks start/advance, with Pause available while saving. These are display rules, not a substitute for native validation.
- `refreshSequence` accepts only the newest requested refresh result, including failures. A failed current read says state is unavailable, disables start/advance, and permits Pause; it does not assert that the session stopped.
- `poll` waits for `refresh` to finish before scheduling its next 200 ms delay. A control-triggered refresh can overlap a poll; sequence checking prevents older results from replacing newer ones.
- `control` clears the prior request error, disables start/advance, requests the action, and refreshes afterward. `render` gives recording errors priority over request errors.
- Credential handlers clear the password input immediately, disable all four credential buttons until completion, and suppress unexpected framework errors. Only failures matching the explicit native-message prefix allowlist are displayed verbatim. This does not establish secret protection outside the interface.
- Dynamic session/history/status text uses `textContent`; history replacement uses created elements. Accessibility structure comes from the labelled HTML, status/alert roles, password label, and stylesheet focus treatment; this is source evidence, not an accessibility audit.

## Side Effects

- `main.ts` mutates page text, history nodes, input values, and button state; registers click handlers; and schedules recurring browser timers.
- Native `invoke` calls cross the desktop boundary. This interface requests session changes and dummy-credential operations; actual persistence and protected storage belong to native code. `presentation` performs no I/O.

## Verification

From the repository root, `npm run check` performs TypeScript checking and runs the three JavaScript test files. `npm run build` checks types and builds the Vite interface; it does not establish native packaging or Keychain behavior. See [test boundaries](../tests/README.md). No tests, build, browser, native app, or credential operations were run for this documentation change.
