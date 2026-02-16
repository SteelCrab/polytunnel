# Polytunnel Roadmap

This roadmap is based on workspace version **0.1.0** and is split into a 12-week plan with three milestones.

## Milestone: v0.1.0 (In Progress)
[x] Week 1: Initial command set shipped (`init`, `build`, `test`).
[x] Week 2: Baseline release packaging and repository publication preparation.
  [x] Publish the baseline non-container distribution specification in `/package-spec.md`.
  [x] Define artifact format, file naming, checksum policy, and installation flow in `/package-spec.md`.
  [x] Confirm minimum target matrix in `/package-spec.md`: linux-x86_64, linux-aarch64, linux-musl, macos-aarch64, windows-x86_64 (windows-arm64 tracked in v0.2.0).
  [x] Define repository publication contents in `/package-spec.md` (binary, examples, compatibility notes, signatures policy).
  [x] Update README and README_KR release section with checksum + install verification steps.
  [x] Keep container distribution tasks to v0.2.0 only (container package and smoke tests).
[ ] Week 3: CI/build/test matrix stabilization for initial release.
[ ] Week 4: Contributor-level docs and release notes finalized in README.

## Milestone: v0.1.1 (Weeks 1-4) — Reliability & Test hardening
[ ] Week 1: Baseline error-path audit for `build` and `test` command flows.
[ ] Week 1: Add guard and skip tests where Java tools are missing.
[ ] Week 2: Expand real integration tests for maven client happy-path and bad-status handling.
[ ] Week 2: Add test coverage for non-2xx response mapping in client layer.
[ ] Week 3: Tighten error mapping across `build`/`vscode` commands and align user-facing messages.
[ ] Week 3: Improve command output for common failure classes.
[ ] Week 4: Reduce flaky integration risk; stabilize CI behavior under restricted toolchains.
[ ] Week 4: Validate coverage trend and fix remaining gaps in patched modules.

## Milestone: v0.1.2 (Weeks 5-8) — CLI Completeness
[ ] Week 5: Implement `pt add` command + unit/integration tests.
[ ] Week 5: Add validation around dependency duplicates and invalid coordinates.
[ ] Week 6: Implement `pt remove` command + tests.
[ ] Week 6: Handle file-backed rollback for partial remove failures.
[ ] Week 7: Implement `pt sync` command + success/error behavior checks.
[ ] Week 7: Add tests for lockless and partially synced states.
[ ] Week 8: Implement `pt tree` command and formatting output.
[ ] Week 8: Implement `pt run` command for running user-specified entry points.
[ ] Week 8: Add `pt <-> gradlew` migration story with opt-in compatibility mode and clear mapping of equivalent commands/options.
[ ] Week 8: Add docs for command usage and exit-code expectations.

## Milestone: v0.2.0 (Weeks 9-12) — Workflow and Developer Experience
[ ] Week 9: Improve diagnostics and structured logs for common failure classes.
[ ] Week 9: Add developer-oriented debug hints in command output.
[ ] Week 9: Design Docker packaging architecture (Dockerfile, base image, non-root container, timezone/JAVA_HOME/cache path policies).
[ ] Week 10: Add `CHANGELOG` scaffolding and release notes template.
[ ] Week 10: Add CI-driven multi-platform image build/push (`linux/amd64`, `linux/arm64`) and tag strategy.
[ ] Week 10: Add migration and upgrade guidance draft.
[ ] Week 10: Add non-container `windows-arm64` release artifact target (`polytunnel-<version>-windows-arm64.zip`) and naming rule.
[ ] Week 10: Extend release CI matrix for `windows-arm64` build, checksum generation, and publish verification.
[ ] Week 11: Add container runtime smoke checks (`docker run --rm <image> --help`, `--version`, sample build).
[ ] Week 11: Add reproducibility checks and local cache invalidation diagnostics.
[ ] Week 11: Run contributor onboarding validation with a fresh setup.
[ ] Week 12: Update README and README_KR with `docker run` usage and mount examples.
[ ] Week 12: Final feature-to-doc synchronization sweep across README / ROADMAP / user-facing docs.
[ ] Week 12: Sign-off release readiness checklist and PR review playbook.
