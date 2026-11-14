# Metten

Terms used by strategy authors, operators, and the trading engine.

## Language

**Strategy version**:
An immutable set of supported purchase rules. Importing changed rules creates a new version; earlier results remain attached to the original version.

**Run**:
An execution of one strategy version with fixed mode and user settings, covering successive eligible markets until stopped.

**Market window**:
The interval of one supported 15-minute market. Checkpoints reset for each new market window.

**Checkpoint**:
A condition observed true during its permitted interval and remembered for the rest of the market window, including which side caused it when relevant. A remembered checkpoint does not establish that current data is fresh.

**Purchase objective**:
The user's requested contract quantity for one market. Confirmed fills lock its side; further attempts can only complete the original quantity.

**Purchase attempt**:
One order submission toward a purchase objective. Its actual outcome must be established before another attempt is allowed.

**Fill**:
A confirmed executed quantity from an attempt. A partial fill purchases less than the amount requested in that attempt.

**Settlement**:
The exchange-confirmed resolution of purchased contracts. Pending settlement is distinct from settled profit or loss.

**Qualification**:
The recorded eligibility of a strategy version for live use after the required paper purchase, resolved accounting, and settlement. Qualification does not prove live API acceptance or future fill quality.

**Trading day**:
A midnight-to-midnight UTC day. A market's result belongs to the day of its close, even if settlement is confirmed later.

**Input**:
An explicitly named source, instrument, field, unit and applicable side. It is a language binding, not proof of source access.

**Measurement**:
A named supported calculation over inputs or other measurements. Historical composition needs cumulative source coverage.

**Evidence and coverage**:
Trusted current validity and complete historical intervals for an exact binding. Verified quiet coverage adds time without inventing observations; a snapshot cannot repair a historical gap.

**Purchase option**:
An ordered eligibility condition and side choice, or a group of alternatives. The first eligible leaf wins subject to the shared objective and safeguards.

**Format meaning**:
The fixed interpretation attached to a saved version: format 2 uses `metten_rules_2`. A newer application cannot silently reinterpret old saved rules.
