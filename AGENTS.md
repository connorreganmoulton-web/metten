# Working on Metten

Read the product specification before implementation and ADR 0001 before changing the engine or desktop integration. The product specification is `METTEN_DESIGN_DRAFT_2026-11-12.md`; it is the source of truth for behavior. GitHub issues are the implementation tracker; the implementation plan links the approved sequence.

Implement one bounded issue at a time. Check its blockers and acceptance criteria before starting; `ready-for-agent` describes a prepared brief, not permission to bypass open blockers. Keep supported strategy changes in validated JSON. The shared Rust engine owns orders, limits, accounting and recovery; Tauri displays state and requests actions. Provider adapters must not branch on strategy names.

Use behavior tests through the engine's public interface with controlled inputs. Add focused packaged-app checks for credentials, storage and IPC. Reuse passing evidence for unchanged behavior; explain any missing required evidence. Never claim paper outcomes prove live acceptance or fills.

No issue or label authorizes real credentials, account access, paid services, deployment or real-money validation. Obtain exact owner authority for those actions. Do not operate another trading application or import its code or strategy thresholds. Keep secrets and private runtime records out of source, logs, issues and exports.

Speak to the owner in plain English. Own routine engineering details; ask only about consequential scope, cost or risk decisions. Keep this repository free of bloat: implement only approved requirements, reuse existing code and tools, and remove confirmed unused or duplicate code within the approved task. Add extra layers, dependencies or features only for a concrete current need. Preserve necessary safety checks, meaningful tests, evidence and unrelated work.

Preserve the root README as the GitHub project page. Do not create `.github/README.md`: GitHub displays it in preference to the root README. Document GitHub workflows in an existing guide or a differently named file.

## Agent skills

### Issue tracker

Track work in this repository's GitHub Issues. See `docs/agents/issue-tracker.md`.

### Triage labels

Use the five standard triage labels. See `docs/agents/triage-labels.md`.

### Domain docs

Use one root glossary and `docs/adr/` for accepted decisions. See `docs/agents/domain.md`.
