# Metten

A planned local Mac application for running supported Kalshi strategies, with a Rust trading engine and a Tauri interface.

**Status: design and planning. There is no runnable application or release yet.**

Metten separates strategy rules from connections, purchasing, accounting, and recovery. Authors supply validated JSON rules; the shared engine handles their execution. Gen 1 targets Bitcoin 15-minute markets, one live run, and independent paper runs. Paper is the default; real-money validation requires separate explicit authorization.

- [Product specification](METTEN_DESIGN_DRAFT_2026-11-12.md)
- [Architecture decision](docs/adr/0001-rust-engine-and-tauri-boundaries.md)
- [Project vocabulary](GLOSSARY.md)
- [Implementation tracking](https://github.com/connorreganmoulton-web/metten/issues)

The application is intended to keep credentials in local-only Mac Keychain and nonsecret structured data in SQLite, with no Metten account or telemetry. These are requirements, not implemented guarantees. No profitability claim is made.

## Development

Implementation tickets are tracked in [GitHub Issues](https://github.com/connorreganmoulton-web/metten/issues); see the [implementation plan](docs/implementation-plan.md) for dependencies. Each ticket must deliver a verifiable behavior, state its dependencies, and preserve the product specification. Follow the accepted architecture decision; do not add strategy-specific purchase or recovery paths.

## License

Licensed under the [MIT License](LICENSE).

The approved Metten brand asset pack has not been imported. Check its provenance and any third-party asset or font licenses when adding it; those rights are not established by this repository license.
