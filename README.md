<h1>
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/brand/exports/svg/metten-a-lockup-reversed.svg">
  <img src="assets/brand/exports/svg/metten-a-lockup-primary.svg" alt="Metten: three offset rounded pieces with a cobalt center, beside the lowercase metten wordmark" width="256">
</picture>
</h1>

Metten is an open-source Mac app in development for running supported Kalshi strategies locally. It is designed around a shared engine that keeps strategy rules separate from purchasing, accounting, and recovery.

> [!NOTE]
> **Early development:** Metten currently runs a non-trading test session and saves its history. Paper trading, live trading, and a public release are not yet available.

[Get started](#run-the-non-trading-foundation) · [Contributing](CONTRIBUTING.md) · [Product specification](METTEN_DESIGN_DRAFT_2026-11-12.md) · [Issues](https://github.com/moultonlavah/metten/issues)

## What we're building

The first version targets Bitcoin 15-minute markets, with independent paper runs and one live run. Strategies will use validated JSON rules, while the engine handles execution and safeguards. Paper is the default; real-money validation requires separate explicit authorization. Paper results do not prove live execution or future performance.

Metten is intended to keep credentials in local-only Mac Keychain and nonsecret structured data in SQLite, without a Metten account or telemetry. These are product requirements, not fully verified guarantees. See the [product specification](METTEN_DESIGN_DRAFT_2026-11-12.md) and [architecture decision](docs/adr/0001-rust-engine-and-tauri-boundaries.md) for details.

## Explore and contribute

Browse the [implementation plan](docs/implementation-plan.md) and [open issues](https://github.com/moultonlavah/metten/issues) to see what's planned and what is ready. The [project vocabulary](GLOSSARY.md) explains the terms used throughout the documentation.

See the [contribution guide](CONTRIBUTING.md) for setup, checks, and contribution boundaries. To report a vulnerability privately, use the [security policy](SECURITY.md).

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

## Implementation documentation and drift checks

The [EGM baseline](docs/egm/README.md) maps the maintained repository outside issue #4 to code-grounded component contracts and includes a repeatable drift-report request. Issue #4, including its additions in shared files, is explicitly excluded. These contracts describe the implemented non-trading foundation; the product specification and accepted ADRs remain authoritative for intended behavior.

### Interface

- [Rust crates](crates/README.md) own controlled session state and recording; [native desktop integration](src-tauri/README.md) selects storage, validates requests and hosts the isolated credential proof; [the interface](src/README.md) displays native state and requests actions.
- [Tests](tests/README.md), [native verification tooling](scripts/README.md), [CI](.github/README.md), [assets](assets/README.md), and [documentation](docs/README.md) explain supporting boundaries and evidence.
- `package.json` defines `dev`, `check`, `build` and `tauri`. Development uses loopback port 5173 with `--strictPort`; checking performs TypeScript validation and JavaScript tests; building runs TypeScript and Vite. `tsconfig.json` checks `src` with strict ES2022/DOM settings and emits no TypeScript output.
- `Cargo.toml` groups `crates/engine` and `src-tauri` with resolver 2. `rust-toolchain.toml` pins Rust 1.100.0 with rustfmt/clippy. `Cargo.lock` and `package-lock.json` record resolved dependencies; use locked Cargo checks and `npm ci` for reproducible installation.

### Invariants

The screen does not own saved truth. Accepted requests are not confirmed writes; errors preserve last confirmed history. The current foundation has no trading operation. Credentials are dummy-only and unavailable on an ordinary launch. Later product features and unfinished issue #4 work are outside this baseline's implementation claims.

### Side Effects

Application execution can create a local workspace, SQLite history and lock files. The isolated native verifier can create and remove owned disposable Keychains and workspaces. Build/install/check commands can download dependencies and create generated files, binaries and test resources. See the owning component contract before running them. `.gitignore` excludes private planning/research, credentials, runtime databases, dependency directories and build/schema output; no such records belong in drift reports.

### Verification

Use the existing relevant checks above and in CONTRIBUTING.md for implementation changes. Documentation-only drift checks inspect source, inventory, references and links; they do not require launching the application or repeating unchanged packaged evidence. The [initial report](docs/egm/2026-11-13_DRIFT_REPORT.md) records this baseline's findings and limits.
