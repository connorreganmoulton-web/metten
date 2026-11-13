# Metten

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/brand/exports/svg/metten-a-lockup-reversed.svg">
  <img src="assets/brand/exports/svg/metten-a-lockup-primary.svg" alt="Metten: three offset rounded pieces with a cobalt center, beside the lowercase metten wordmark" width="256">
</picture>

A local Mac application foundation with an independent Rust engine and a Tauri interface. The planned product runs supported Kalshi strategies.

**Status: early development. The app controls a non-trading test session and saves its history. No trading or public release is available.**

Metten separates strategy rules from connections, purchasing, accounting, and recovery. The planned design uses validated JSON rules, with the shared engine responsible for execution. Gen 1 targets Bitcoin 15-minute markets, one live run, and independent paper runs. Paper is the default; real-money validation requires separate explicit authorization.

- [Product specification](METTEN_DESIGN_DRAFT_2026-11-12.md)
- [Architecture decision](docs/adr/0001-rust-engine-and-tauri-boundaries.md)
- [Project vocabulary](GLOSSARY.md)
- [Implementation tracking](https://github.com/connorreganmoulton-web/metten/issues)

The application is intended to keep credentials in local-only Mac Keychain and nonsecret structured data in SQLite, with no Metten account or telemetry. These are requirements, not implemented guarantees. No profitability claim is made.

## Development

Implementation tickets are tracked in [GitHub Issues](https://github.com/connorreganmoulton-web/metten/issues); see the [implementation plan](docs/implementation-plan.md) for dependencies. Each ticket must deliver a verifiable behavior, state its dependencies, and preserve the product specification. Follow the accepted architecture decision; do not add strategy-specific purchase or recovery paths.

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, checks, and contribution boundaries, and [SECURITY.md](SECURITY.md) to report a vulnerability privately.

## Repository map

- [`src/`](src/) — application interface.
- [`src-tauri/`](src-tauri/) — native Mac integration.
- [`crates/engine/`](crates/engine/) — shared engine and its tests.
- [`tests/`](tests/) — interface and integration checks.
- [`docs/`](docs/) — architecture decisions, implementation plan, and verification records.
- [`scripts/`](scripts/) — development and verification tools.
- [`assets/brand/`](assets/brand/README.md) — logo artwork and usage guidance.

## Run the non-trading foundation

Install Node.js 24 or later, the pinned Rust toolchain from `rust-toolchain.toml`, and Apple's Xcode Command Line Tools. Then:

```sh
npm ci
npm run tauri dev
```

Start the controlled session, advance its manual test clock, and pause. Now and History display committed SQLite records. Reopening retains history and starts stopped. This is a test session, not paper trading; it has no market feeds, orders or financial accounting. Credential storage has been tested with dummy credentials only; provider connections are not available.

Build a local Mac application with `npm run tauri build -- --bundles app`; open `target/release/bundle/macos/Metten.app`. This local bundle is not a signed/notarized public release.

The default workspace is the operating system's application-data directory for `com.metten.foundation`. For isolated checks, launch the bundle's `Contents/MacOS/metten` executable with `METTEN_TEST_WORKSPACE` set to a fresh temporary directory. Never point checks at another application's workspace. No workspace path is accepted from the screen.

```sh
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run check
npm run build
```

See [the issue #2 verification record](docs/verification/issue-2.md) for exact local evidence and limitations.

## License

Licensed under the [MIT License](LICENSE).

Brand artwork and font provenance are documented in the [brand guide](assets/brand/README.md); those rights are not established by the software license.
