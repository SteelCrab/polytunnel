# Changelog

All notable changes to this project will be documented here.

---

## ✨ [v0.2.0] — TBD

> **Released:** TBD
> `pt sync` & `pt tree` 명령어 추가, 내부 리팩토링, 보안 패치.

---

### ✨ Added

- `pt sync` — 선언된 모든 의존성 해석 및 다운로드 (JAR 수 + 소요시간 리포트)
- `pt tree` — cargo-tree 스타일 의존성 트리 출력 (중복 노드 `(*)` 표시)
- `--verbose` 플래그: `sync`, `tree` 서브커맨드 지원
- `format_classpath()` 공용 유틸리티 함수 추출 (`polytunnel-build`)
- `#![warn(missing_docs)]` 전체 라이브러리 크레이트에 적용, 전체 public API doc comment 추가
- `sync`/`tree` 유닛 테스트, `polytunnel-build` 테스트 스위트 확장

---

### ♻️ Changed

- `polytunnel-build` 내부 모듈 `pub mod` → `mod` (API 표면 축소)
- `cmd_build()` → `orchestrator.build()` 위임으로 단순화
- `compile_sources()`/`compile_tests()`: `async fn` → `fn` (불필요한 async 제거)
- `println!` → `eprintln!` 라이브러리 크레이트 전환 (stderr로 경고 출력)
- `std::fs::write` → `tokio::fs::write` (`download_jar` 비동기 I/O 일관성)
- `&PathBuf` → `&Path` (`download_jar` 시그니처 idiomatic Rust)
- `DependencyScope`에 `Copy` derive 추가
- `default_packaging()` 중복 제거 (단일 소스)
- 하드코딩 버전 문자열 → `env!("CARGO_PKG_VERSION")`
- deprecated `AppError` 타입 완전 제거
- 인라인 `#[cfg(test)]` → `tests/` 디렉토리 이동 (`polytunnel-build`)
- CLI에서 `polytunnel-resolver` 직접 의존성 제거

---

### 🐛 Fixed

- `pt test` 독립 실행 시 JUnit classpath 미해석 버그 수정 (의존성 해석 누락)
- POM exclusion 파싱 버그 (`in_exclusion` 무시 → `Exclusion` 구조체에 수집)
- Resolver `unwrap()` 체인 → proper error propagation (mutex poisoning 대응)

---

### 🔒 Security

- `quinn-proto` 보안 패치 (`0.11.13` → `0.11.14`)

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
