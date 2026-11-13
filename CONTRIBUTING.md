# Contributing to Metten

Metten is in early development. The current foundation is non-trading; paper and live trading are not available. Read the [README](README.md) for setup and the [product specification](METTEN_DESIGN_DRAFT_2026-11-12.md) for intended behavior.

## Find work

Check [open issues](https://github.com/connorreganmoulton-web/metten/issues) and their blockers before starting. Discuss substantial changes in an issue first. The [implementation plan](docs/implementation-plan.md) describes the planned sequence, and [ADR 0001](docs/adr/0001-rust-engine-and-tauri-boundaries.md) explains the engine and desktop boundary.

## Set up and check changes

Use macOS, Node.js 24 or later, Apple's Xcode Command Line Tools, and the Rust toolchain pinned in `rust-toolchain.toml`. Run `npm ci` and `npm run tauri dev` to start the local foundation. The README explains its test session and isolated workspace.

Run checks relevant to your change. The current automated workflow runs these on macOS:

```sh
python3 tests/credential-runner.test.py
npm run check
npm run build
cargo fmt --all --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run tauri build -- --bundles app
```

Describe what you changed, link the issue, and report checks and any limitations in your pull request. Do not present paper or simulated results as proof of live behavior.

## Protect users and their data

Use controlled inputs and dummy credentials for tests. Never commit secrets, private runtime records, or account data. An issue or contribution does not authorize access to real credentials or accounts, paid services, deployment, or real-money validation. Do not operate another trading application or import its code or strategy thresholds. See the [product specification](METTEN_DESIGN_DRAFT_2026-11-12.md) for the intended safety boundaries.

Report vulnerabilities privately using the instructions in [SECURITY.md](SECURITY.md). Do not post secrets or exploitable vulnerability details in a public issue.
