# Changelog

All notable changes to this project will be documented here.

## ✨ [Unreleased]

- Added baseline non-container release artifact specification (`package-spec.md`) and linked it from release guidance sections in `README.md` and `README_KR.md`.
- Expanded `ROADMAP.md` / `ROADMAP_KR.md` Week 2 checklist with executable release-package criteria (artifact naming, checksum policy, target matrix, install flow).

## 🚀 [0.1.0] - 초기 릴리즈

### Added
- Workspace-based multi-crate structure for core CLI, Maven client, resolver, build engine, and IDE helpers.
- Core commands in active development: `pt init`, `pt build`, `pt test`.

### Changed
- Coverage and CI process updates to maintain stable thresholds and reproducible Rust toolchain behavior.
- Contributor and governance documentation added for release preparation (`CONTRIBUTING.md`, `SECURITY.md`, roadmap documentation).

### Fixed
- Ongoing hardening work focused on command and integration test coverage.

## Release note

- This changelog is initializing while workspace version remains `0.1.0` (development stage).
- Historical releases are not published yet; entries will be frozen at first tagged release.
