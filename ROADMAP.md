# Polytunnel Roadmap

This roadmap is based on workspace version **0.1.0** and is split into a 12-week plan with three milestones.

## Historical Completed Work (Past Commits & Branches)
- [x] `feat/build-progress-bar` — progress bar and concurrent download orchestration.
- [x] `test/resolver` / `test/polytunnel-maven` — resolver, transitive dependency, and Maven client test hardening.
- [x] `test/commands-coverage` — command-level error-path and coverage gap closure for CLI modules.
- [x] `ci/codecov` — CI coverage pipeline stabilization and threshold tuning.
- [x] `docs/add-roadmap` — first roadmap draft and baseline planning docs.
- [x] `docs/update-roadmap-parallel` — roadmap refresh for implemented parallelism features.
- [x] `docs/simplify-readme` — README readability and command list cleanup.
- [x] `feat/error-reporting` — centralized runtime error reporting flow.
- [x] `feat/windows-arm64-support` — platform support updates for ARM64 on Windows.
- [x] `feat/refactor-resolve` — dependency resolver refactoring and maintenance improvements.
- [x] `fix/transitive-dependency-resolution` — transitive resolution correctness fixes.
- [x] `fix/ci-workflow-improvements` — CI reliability improvements.
- [x] `chore/license-apache-2.0` — repository licensing transition and docs alignment.
- [x] `chore/setup-precommit` — pre-commit tooling baseline.
- [x] `ci/codecov` + `test-coverage` — test instrumentation and coverage workflow modernization.

## Milestone: v0.1.0 (In Progress)
- [x] Week 1: Initial command set shipped (`init`, `build`, `test`).
- [ ] Week 2: Baseline release packaging and repository publication preparation.
  - Define release package requirements for container delivery (image name, tag format, supported tags).
  - Decide release artifact set (binary in image, runtime examples, quickstart usage).
  - Align README and docs with container run path and expected arguments.
- [ ] Week 3: CI/build/test matrix stabilization for initial release.
- [ ] Week 4: Contributor-level docs and release notes finalized in README.

### Implementation examples
- Command parsing skeleton: `crates/polytunnel/src/cli.rs` defines subcommands and option flags.
- Command entrypoint wiring: `crates/polytunnel/src/main.rs` → `commands::*` dispatch.
- Minimal MVP execution paths:
  - `pt init` generates project config from template and validates Java version.
  - `pt build` performs resolution + compilation pipeline and optional test execution.
  - `pt test` executes test-only flow with `--fail-fast` and pattern filtering.
- User experience:
  - Keep `help` output clear on each command.
  - Surface actionable errors when Java toolchain files/paths are missing.

## Milestone: v0.1.1 (Weeks 1-4) — Reliability & Test hardening
- [ ] Week 1: Baseline error-path audit for `build` and `test` command flows.
- [ ] Week 1: Add guard and skip tests where Java tools are missing.
- [ ] Week 2: Expand real integration tests for maven client happy-path and bad-status handling.
- [ ] Week 2: Add test coverage for non-2xx response mapping in client layer.
- [ ] Week 3: Tighten error mapping across `build`/`vscode` commands and align user-facing messages.
- [ ] Week 3: Improve command output for common failure classes.
- [ ] Week 4: Reduce flaky integration risk; stabilize CI behavior under restricted toolchains.
- [ ] Week 4: Validate coverage trend and fix remaining gaps in patched modules.
- [ ] Dependencies: existing CI workflows, `polytunnel-build`, `polytunnel-maven`.

### Implementation examples
- Error-path unit + integration pattern:
  - Add unit tests in command modules for `Result` propagation.
  - Add integration tests using local temp workspace + mocked command scripts.
- Test strategy:
  - `crates/polytunnel/tests/cli_coverage_tests.rs`: coverage-focused negative-path cases.
  - `crates/polytunnel/tests/cli_coverage_real.rs`: run-path tests gated on `java_tools_available()`.
- Example hardening snippet:
  - Wrap Java dependency checks early and return command-specific error variants instead of generic failures.

## Milestone: v0.1.2 (Weeks 5-8) — CLI Completeness
- [ ] Week 5: Implement `pt add` command + unit/integration tests.
- [ ] Week 5: Add validation around dependency duplicates and invalid coordinates.
- [ ] Week 6: Implement `pt remove` command + tests.
- [ ] Week 6: Handle file-backed rollback for partial remove failures.
- [ ] Week 7: Implement `pt sync` command + success/error behavior checks.
- [ ] Week 7: Add tests for lockless and partially synced states.
- [ ] Week 8: Implement `pt tree` command and formatting output.
- [ ] Week 8: Implement `pt run` command for running user-specified entry points.
- [ ] Week 8: Add docs for command usage and exit-code expectations.
- [ ] Dependencies: Resolver + Maven client contracts, CLI command parsing.

### Implementation examples
- `pt add`:
  - Parse dependency coordinate.
  - Update `polytunnel.toml` dependency table preserving comments/format as far as possible.
- `pt remove`:
  - Remove key from dependency map and validate no empty sections remain.
- `pt sync`:
  - Re-resolve dependency graph and rewrite lock-like metadata.
- `pt tree`:
  - Render dependency graph with indentation and scopes (`compile`/`test`/`runtime`).
- `pt run`:
  - Resolve main class candidate from project metadata and execute generated classes with JVM args.
- Quality gates:
  - Add positive/negative tests for each command; assert non-zero exit codes with message checks.

## Milestone: v0.2.0 (Weeks 9-12) — Workflow and Developer Experience
- [ ] Week 9: Improve diagnostics and structured logs for common failure classes.
- [ ] Week 9: Add developer-oriented debug hints in command output.
- [ ] Week 9: Design Docker packaging architecture (Dockerfile, base image, non-root container, timezone/JAVA_HOME/cache path policies).
- [ ] Week 10: Add `CHANGELOG` scaffolding and release notes template.
- [ ] Week 10: Add CI-driven multi-platform image build/push (`linux/amd64`, `linux/arm64`) and tag strategy.
- [ ] Week 10: Add migration and upgrade guidance draft.
- [ ] Week 11: Add container runtime smoke checks (`docker run --rm <image> --help`, `--version`, sample build).
- [ ] Week 11: Add reproducibility checks and local cache invalidation diagnostics.
- [ ] Week 11: Run contributor onboarding validation with a fresh setup.
- [ ] Week 12: Update README and README_KR with `docker run` usage and mount examples.
- [ ] Week 12: Final feature-to-doc synchronization sweep across README / ROADMAP / user-facing docs.
- [ ] Week 12: Sign-off release readiness checklist and PR review playbook.
- [ ] Dependencies: existing contributor workflows and CI pipelines.

### Implementation examples
- Local troubleshooting:
  - Add debug logs behind verbosity flags for build/test resolution decisions.
  - Include `JAVA_HOME`, chosen repository endpoint, and command cache location in errors.
- Container packaging:
  - Use multi-stage or minimal runtime Dockerfile to keep images small and reproducible.
  - Run container as non-root and document volume mount pattern: `./workspace:/workspace`.
  - Add a smoke-test command in CI to fail fast on image build/push and startup errors.
- Operational docs:
  - Add runbook with 5 most common CLI failures and expected fixes.
- Release checks:
  - Add reproducibility matrix script for clean clone → `cargo build` → `pt init` → `pt build`.

## Notable Out of Scope (for this cycle)
- Full IDE plugin integration.
- Advanced build graph caching beyond current build/test flow.
- Enterprise repository-level policy system.
