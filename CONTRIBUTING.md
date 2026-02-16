# Contributing to Polytunnel

## Scope

Thank you for contributing. This repository is in active development (workspace version: `0.1.0`) and focuses on a small, stable core CLI: `pt init`, `pt build`, and `pt test`.

## Development setup

- Install Rust toolchain and dependencies with standard Cargo tooling.
- Install pre-commit hooks:
  - `pre-commit install`
- Recommended daily checks:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo llvm-cov --workspace --fail-under-lines 85`

## Repository workflow

- Use small, focused commits by intent.
- Keep behavior changes covered by tests under `/crates/*/tests` when possible.
- Avoid placeholder or fake tests; prefer executable behavior tests.

## Coding standards

- Use workspace dependencies via `workspace` entries where applicable.
- Respect existing module conventions and naming in each crate.
- Keep error handling explicit and test failure paths at least once in integration coverage for CLI-level behavior.

## Pull requests

- Use clear titles: `<type>: <summary>`.
- Link related issues or tasks.
- Include test output summary, and call out any skipped tests or environment constraints (e.g., Java toolchain availability).

## Commit convention

- Preferred format: `type(scope): short description`
- Use prefixes such as `feat`, `fix`, `chore`, `test`, `docs`, `refactor`.
