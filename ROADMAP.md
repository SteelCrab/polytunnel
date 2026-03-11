# Polytunnel Roadmap

## Milestone: v0.1.0 — Initial Release (Released 2026-02-18)

[x] Week 1: Initial command set shipped (`init`, `build`, `test`).
[x] Week 2: Baseline release packaging and repository publication preparation.
  [x] Publish the baseline non-container distribution specification in `/package-spec.md`.
  [x] Define artifact format, file naming, checksum policy, and installation flow in `/package-spec.md`.
  [x] Confirm minimum target matrix in `/package-spec.md`: linux-x86_64, linux-aarch64, linux-musl, macos-aarch64, windows-x86_64, windows-aarch64, linux-aarch64-musl (7 platforms).
  [x] Define repository publication contents in `/package-spec.md` (binary, examples, compatibility notes, signatures policy).
  [x] Update README and README_KR release section with checksum + install verification steps.
[x] Week 3: CI/build/test matrix stabilization for initial release.
  [x] Add windows-aarch64 non-container release artifact target (`polytunnel-<version>-windows-aarch64.zip`) and naming rule to `package-spec.md`.
  [x] Add linux-aarch64-musl non-container release artifact target and naming rule to `package-spec.md`.
  [x] Extend release CI matrix for windows-aarch64 and linux-aarch64-musl: build, checksum generation, and publish verification.
  [x] Stabilize all 7-platform build matrix (linux-x86_64, linux-aarch64, linux-musl, linux-aarch64-musl, macos-aarch64, windows-x86_64, windows-aarch64).
[x] Week 4: Contributor-level docs and release automation finalized.
  [x] Add `CONTRIBUTING.md` with development setup, branch naming, and commit format guidelines.
  [x] Add `CHANGELOG.md` with initial v0.1.0 release notes.
  [x] Automate GitHub Releases via CI on tag push (release notes, artifact upload, SHA256SUMS).
  [x] Generate and publish `SHA256SUMS` checksum file alongside release artifacts.

---

## Milestone: v0.1.1 — CLI Completeness & Test Hardening

### Test Hardening (Largely Complete)
[x] Baseline error-path audit for `build` and `test` command flows.
[x] Add guard and skip tests where Java tools are missing.
[x] Expand real integration tests for maven client happy-path and bad-status handling.
[x] Add test coverage for non-2xx response mapping in client layer.
[x] Tighten error mapping across `build`/`vscode` commands and align user-facing messages.
[x] Improve command output for common failure classes.
[x] Reduce flaky integration risk; stabilize CI behavior under restricted toolchains.
[x] Validate coverage trend — achieved 91.14% line coverage.

### CLI Completeness
[x] Implement `pt sync` command: resolve and download all declared dependencies.
[x] Add tests for lockless and partially synced states.
[x] Implement `pt tree` command: display resolved dependency tree.
[x] Finalize `pt tree` output formatting.
[ ] Implement `pt add <coordinate>` command + unit/integration tests.
[ ] Add validation for dependency duplicates and invalid coordinates on `pt add`.
[ ] Implement `pt remove <coordinate>` command + tests.
[ ] Handle file-backed rollback for partial remove failures.
[ ] Implement `pt run <main-class>` command for running user-specified entry points.
[ ] Add docs for all new command usage and exit-code expectations.

---

## Milestone: v0.2.0 — Lock File, Cache & Incremental Builds

### Lock File
[ ] Generate and validate `polytunnel.lock` on `pt sync`.
[ ] Lock file records resolved coordinates, versions, and artifact checksums.
[ ] `pt sync --frozen`: fail if lock file is absent or stale (CI-safe mode).

### Cache
[ ] Implement `pt cache clean` and `pt cache stats` subcommands.
[ ] SHA-256 integrity verification for all cached artifacts on read.
[ ] Cache eviction policy and size reporting.

### Incremental Builds
[ ] Implement `BuildCache` keyed on source file hashes and classpath fingerprint.
[ ] Connect `BuildCache` to orchestrator: skip `javac` invocations for unchanged modules.
[ ] Expose `--no-cache` flag to force full rebuild.

### Reliability
[ ] Parallel download concurrency control (configurable worker count).
[ ] Retry logic with exponential backoff for transient network failures.
[ ] Improved diagnostics: structured error messages with debug hints (`pt --debug`).

---

## Milestone: v0.3.0 — Gradle Compatibility & Annotation Processors

### Gradle Support
[ ] Implement `polytunnel-gradle` crate: Gradle Plugin Portal HTTP client and POM resolution.
[ ] Parse `build.gradle` / `build.gradle.kts` dependency blocks (common subset).
[ ] Implement `pt migrate`: convert `build.gradle` to `polytunnel.toml`.

### Annotation Processors
[ ] Lombok annotation processor support (source-retention, `delombok` integration).
[ ] MapStruct code generation support.
[ ] Spring Boot BOM import and `bootJar` / `bootRun` equivalents.

---

## Milestone: v0.4.0 — Plugin System & Advanced IDE/Container Support

### Plugin System
[ ] WASM plugin runtime (wasmtime): define `Plugin` trait and sandboxed execution model.
[ ] Plugin registry: load plugins from `polytunnel.toml` `[plugins]` table.
[ ] Built-in plugins: SpotBugs, Checkstyle, PMD, JaCoCo.

### IDE Integration
[ ] VS Code extension: real-time dependency resolution feedback and error lens.
[ ] IntelliJ plugin: import `polytunnel.toml` as project model.

### Container Packaging
[ ] Design Docker packaging architecture (Dockerfile, base image, non-root container, TZ/JAVA_HOME/cache path policies).
[ ] CI-driven multi-platform image build/push (`linux/amd64`, `linux/arm64`) and tag strategy.
[ ] Container runtime smoke checks (`docker run --rm <image> --help`, `--version`, sample build).
[ ] Update README and README_KR with `docker run` usage and mount examples.

---

## Milestone: v1.0.0 — Production Ready

### Correctness & Completeness
[ ] JUnit 4 and TestNG test execution (full runner integration, not just detection).
[ ] Version conflict detection: warn on diamond dependency conflicts; `--strict` mode to fail on conflict.
[ ] Offline mode: resolve from cache only, fail fast on missing artifacts.

### Performance
[ ] Publish benchmark suite comparing cold/warm build times against Maven and Gradle.
[ ] Memory and allocation profiling; reduce peak RSS for large dependency graphs.

### Documentation
[ ] Complete user guide (getting started, command reference, configuration reference).
[ ] Migration guide: Maven → Polytunnel and Gradle → Polytunnel.
[ ] Contributor guide: architecture deep-dive, adding a new command, plugin authoring.

### Release Readiness
[ ] End-to-end release readiness checklist sign-off.
[ ] PR review playbook and merge criteria documented in `CONTRIBUTING.md`.
[ ] Announce v1.0.0 via GitHub Release with full changelog and migration notes.
