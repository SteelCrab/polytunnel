# Polytunnel Crates

Rust workspace containing all Polytunnel components.

## Crate Overview

| Crate | Description |
|-------|-------------|
| [polytunnel](polytunnel/) | CLI binary (`pt` command) |
| [polytunnel-core](polytunnel-core/) | Core types, config, error handling |
| [polytunnel-maven](polytunnel-maven/) | Maven Central API, POM parser |
| [polytunnel-resolver](polytunnel-resolver/) | Dependency resolution algorithm |
| [polytunnel-build](polytunnel-build/) | Build engine, test runner |

## Dependency Graph

```
polytunnel (CLI)
├── polytunnel-core
├── polytunnel-build
│   ├── polytunnel-core
│   └── polytunnel-maven
└── polytunnel-resolver
    ├── polytunnel-core
    └── polytunnel-maven
```

## Building

```bash
# From workspace root
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Adding a New Crate

1. Create directory: `mkdir crates/polytunnel-new`
2. Add `Cargo.toml` with `version.workspace = true`
3. Workspace auto-discovers via `members = ["crates/*"]`
