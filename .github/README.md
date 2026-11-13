# Automated repository checks

## Overview

[workflows/checks.yml](workflows/checks.yml) defines the `Foundation checks` workflow for pushes and pull requests. It builds and checks the foundation on a GitHub-hosted Mac; it does not publish a release or validate trading.

## Interface

The `mac-foundation` job uses `macos-15`, Node 24 with npm caching, and Rust 1.100.0 with rustfmt/clippy. It checks out source, installs locked npm dependencies, runs `python3 tests/credential-runner.test.py`, `npm run check`, `npm run build`, `cargo fmt --all --check`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `npm run tauri build -- --bundles app`, in that order.

## Invariants

Workflow token permissions are `contents: read`. The workflow has no credential provisioning, packaged Accessibility/Keychain exercise, signing/notarization, deployment or live trading step. A configured check is not evidence that a particular remote run passed. This baseline excludes issue #4 behavior even though a future execution of workspace-wide checks will include whatever source is then present.

## Side Effects

A triggered run downloads actions, toolchains and dependencies and creates build/test output on the hosted runner. Nothing is pushed or deployed by these steps.

## Verification

Compare the job, commands and versions with [package.json](../package.json), [Cargo.toml](../Cargo.toml), [rust-toolchain.toml](../rust-toolchain.toml) and [CONTRIBUTING.md](../CONTRIBUTING.md). No remote workflow was triggered or newly verified by this documentation task.
