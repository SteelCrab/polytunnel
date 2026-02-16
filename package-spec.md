# Non-Container Release Package Specification

## Scope
This document defines the baseline (non-container) distribution artifacts for `polytunnel` before container-based
distribution (`v0.2.0`) is introduced.

## 1. Artifact format
- Archive format: `.tar.gz` for Unix-like platforms, `.zip` for Windows.
- Archive contents:
  - `polytunnel` binary
  - `README.md`
  - `README_KR.md`
  - `LICENSE`
  - `examples/` (minimal reference examples)

## 2. Target matrix
Supported baseline targets are:
- `linux-x86_64`
- `linux-aarch64`
- `linux-musl`
- `macos-aarch64`
- `windows-x86_64`

## 3. File naming
- Base pattern: `polytunnel-<version>-<target>.<ext>`
- Version is semver (workspace version, currently `0.1.0`).
- Examples:
  - `polytunnel-0.1.0-linux-x86_64.tar.gz`
  - `polytunnel-0.1.0-linux-aarch64.tar.gz`
  - `polytunnel-0.1.0-linux-musl.tar.gz`
  - `polytunnel-0.1.0-macos-aarch64.tar.gz`
  - `polytunnel-0.1.0-windows-x86_64.zip`

## 4. Integrity policy
- `SHA256SUMS` is required for every release.
- `SHA256SUMS.asc` (or equivalent signature file) is optional for `0.1.0` and required to be added gradually in future releases.
- Verification flow:
  - Distribute checksums next to artifacts.
  - Consumers verify with `sha256sum`/`Get-FileHash`.
  - Signature verification is used when available.

## 5. Minimum install steps
1. Download the target artifact and checksum file from GitHub Releases.
2. Verify checksums:
   - Unix: `sha256sum -c SHA256SUMS`
   - Windows: `Get-FileHash <file> -Algorithm SHA256`
3. Extract archive and make `polytunnel` executable on Unix (`chmod +x`).
4. Run `./polytunnel --version` and a basic CLI command (`pt --version`).

## 6. Repository publication contents
- Binary artifact(s) per supported target.
- `package-spec.md` and changelog references.
- `README.md`, `README_KR.md`.
- License and example project(s) for quick validation.
- Compatibility notes (`Java version` and supported CLI command set at baseline).

## 7. Release-ready checklist
- `package-spec.md` exists and is versioned in repository.
- All target artifacts use the naming convention above.
- `SHA256SUMS` is included in each release.
- README/KR release section points to this specification.
- Packaging and checksums are reviewed before release merge.
