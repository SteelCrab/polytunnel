# Changelog

All notable changes to this project will be documented here.

---

## 🚀 [v0.2.1] — 2026-04-19

> **Released:** 2026-04-19
> `pt add` / `pt remove` / `pt run` commands, `TestProject` integration-test harness, human-runnable e2e shell scenarios, modular CI, and `rustls-webpki` security patch.

---

### ✨ Added
- `pt add` / `pt remove` / `pt run` CLI commands
- `TestProject` integration-test harness + human-runnable `scripts/e2e-scenarios.sh`

### ♻️ Changed
- `do_run` returns exit code; classpath puts project classes before deps; `main_class` trimmed before JVM call
- CI workflow modularized; README / README_KR modernized; new `test-e2e-scenarios` job

### 🔒 Security
- `rustls-webpki` security patch and security-audit follow-ups

---

## 🚀 [v0.2.0] — TBD

> **Released:** TBD
> `pt sync` & `pt tree` commands, internal refactoring, security patch.

---

### ✨ Added

**🖥️ CLI (`pt` command)**
- `pt sync` — resolve and download all declared dependencies (reports JAR count + elapsed time)
- `pt tree` — cargo-tree style dependency tree output (duplicate nodes marked with `(*)`)
- `--verbose` flag: supported for `sync` and `tree` subcommands

**⚙️ Build Engine (`polytunnel-build`)**
- `format_classpath()` shared utility function extracted

**📚 Code Quality**
- `#![warn(missing_docs)]` enabled across all library crates, full public API doc comments added
- `sync`/`tree` unit tests, `polytunnel-build` test suite expanded

---

### ♻️ Changed

- `polytunnel-build` internal modules `pub mod` → `mod` (reduced API surface)
- `cmd_build()` → delegates to `orchestrator.build()` for simplicity
- `compile_sources()`/`compile_tests()`: `async fn` → `fn` (removed unnecessary async)
- `println!` → `eprintln!` in library crates (warnings now go to stderr)
- `std::fs::write` → `tokio::fs::write` (consistent async I/O in `download_jar`)
- `&PathBuf` → `&Path` (idiomatic Rust signature for `download_jar`)
- Added `Copy` derive to `DependencyScope`
- Deduplicated `default_packaging()` (single source of truth)
- Hardcoded version string → `env!("CARGO_PKG_VERSION")`
- Removed deprecated `AppError` type entirely
- Inline `#[cfg(test)]` → moved to `tests/` directory (`polytunnel-build`)
- Removed direct `polytunnel-resolver` dependency from CLI

---

### 🐛 Fixed

- Fixed JUnit classpath not resolved when running `pt test` standalone (missing dependency resolution)
- Fixed POM exclusion parsing bug (`in_exclusion` flag ignored → now collected into `Exclusion` struct)
- Replaced Resolver `unwrap()` chains → proper error propagation (handles mutex poisoning)

---

### 🔒 Security

- Bumped `quinn-proto` security patch (`0.11.13` → `0.11.14`)

---

## 🚀 [v0.1.0] — Initial Release

> **Released:** 2026-02-18
> First public release of `pt`, a fast Java dependency manager written in Rust.

---

### ✨ Added

**🖥️ CLI (`pt` command)**
- `pt init` — generate `polytunnel.toml` project config
- `pt build` — compile Java sources with javac + download dependencies
- `pt test` — run JUnit 5 tests with Cargo-style output formatting
- `pt add` / `pt remove` / `pt sync` / `pt tree` — dependency management commands
- `pt vscode` — auto-generate VS Code `.classpath`, `.project`, `settings.json`
- `--clean` flag to invalidate incremental build cache
- Download and build progress bar (`indicatif`)
- Verbose artifact download logging (`--verbose`)
- Platform info display (OS, architecture)

**🔍 Dependency Resolver (`polytunnel-resolver`)**
- Recursive transitive dependency resolution against Maven Central
- **Nearest Wins** strategy — first-encountered version per GA wins
- Async parallel DFS resolution via `BoxFuture`
- Version range, snapshot, and deduplication handling
- `provided` scope transitive resolution support
- `with_client()` constructor for injecting a custom `MavenClient`

**📦 Maven Client (`polytunnel-maven`)**
- Maven Central REST API client (`reqwest` + `rustls-tls`, no OpenSSL)
- POM XML parsing — parent inheritance, `<dependencyManagement>`, `${property}` substitution
- Non-2xx response handling and HTML error page detection
- Search query URL encoding

**⚙️ Build Engine (`polytunnel-build`)**
- `javac` parallel compilation orchestration
- Classpath construction and parallel JAR downloads (`try_join_all`)
- JUnit 5 test runner — test class discovery + result parsing
- Incremental build cache (source change detection)
- Build elapsed time display

**🧩 IDE Integration (`polytunnel-ide`)**
- Auto-generate VS Code Java project files
- Auto-append generated IDE files to `.gitignore`

**🌐 Platform**
- Windows ARM64 support (`aarch64-pc-windows-msvc`)
- `Os::Unknown` variant for explicit unsupported platform handling

**📚 Examples**
- `examples/hello-java` — minimal Java project example
- `examples/todo-server` — Javalin-based TODO REST API server (with kotlin-stdlib)

---

### 🐛 Fixed

- Switched `reqwest` to `rustls-tls` — eliminates OpenSSL dependency in aarch64 cross-builds
- Fixed missing `${property}` substitution in parent POM `<dependencyManagement>`
- Fixed `groupId` / `artifactId` property substitution and HTML error response detection
- Fixed `provided` scope dependencies being excluded from transitive resolution
- Fixed CLI test output trimming to use only the first line
- Unified binary name `polytunnel` → `pt` (corrected packaging paths across all 7 platforms)
- Fixed Windows release artifact paths (`pt.exe`)

---

### 🏗️ CI / Infrastructure

- **7-platform** debug build matrix:
  `linux-x86_64`, `linux-aarch64`, `linux-x86_64-musl`, `linux-aarch64-musl`, `macos-aarch64`, `windows-x86_64`, `windows-aarch64`
- **7-platform** release builds with artifact packaging (`.tar.gz` / `.zip`)
- `SHA256SUMS` auto-generated checksum file (`checksum` job)
- `cargo audit` security vulnerability scanning (`security` job)
- `cargo llvm-cov` coverage — measured **91.14%** line coverage (threshold: 87%)
- Codecov integration and coverage badge
- `cargo clippy -D warnings` + `cargo fmt --check` enforced on all PRs
- `docs` branch skips CI entirely
- `rust-cache` for build dependency caching

---

### 🔒 Security

- License changed from MIT to **Apache-2.0**
- Patched `bytes 1.11.0` integer overflow vulnerability → upgraded to `1.11.1` ([RUSTSEC-2026-0007](https://github.com/advisories/GHSA-434x-w66g-qw3r))

---

### 📋 Release Notes

| | |
|---|---|
| **Binary** | `pt` |
| **Platforms** | Linux x86_64/aarch64 (glibc + musl), macOS aarch64, Windows x86_64/aarch64 |
| **Requirements** | JDK 11+, Rust 1.80+ (build only) |
| **Artifacts** | `polytunnel-0.1.0-{target}.tar.gz` / `.zip` |
