# Changelog

All notable changes to this project will be documented here.

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
