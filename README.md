# Polytunnel

[![CI](https://github.com/SteelCrab/polytunnel/workflows/CI/badge.svg)](../../actions)
[![codecov](https://codecov.io/gh/SteelCrab/polytunnel/graph/badge.svg?branch=main)](https://codecov.io/gh/SteelCrab/polytunnel?branch=main)
[![한국어](https://img.shields.io/badge/lang-한국어-blue.svg)](README_KR.md)

**Fast Java dependency manager written in Rust** — inspired by `uv` and `ruff`. Declarative TOML config, native binary, no JVM daemon.

```bash
pt init my-app && cd my-app
pt add com.google.guava:guava:33.0.0-jre
pt build
pt run com.example.App
```

---

## Why Polytunnel?

|  | Maven | Gradle | **Polytunnel** |
|---|---|---|---|
| Startup | Slow JVM | Slow JVM (+daemon) | **Instant (native)** |
| Config | XML (verbose) | Groovy/Kotlin (scripted) | **TOML (declarative)** |
| Binary size | ~10MB + JVM | ~100MB + JVM | **~6MB, no JVM** |
| Peak RSS | 500MB+ | 500MB~2GB | **~50-200MB target** |
| Install | `brew install maven` | `brew install gradle` | **one binary** |

## Install

Pick the method that fits you.

### Option 1 — Pre-built binary (recommended, no Rust needed)

**macOS (Apple Silicon)**
```bash
curl -L https://github.com/SteelCrab/polytunnel/releases/latest/download/pt-macos-aarch64 -o pt
chmod +x pt && sudo mv pt /usr/local/bin/
pt --version
```

**Linux x86_64**
```bash
curl -L https://github.com/SteelCrab/polytunnel/releases/latest/download/pt-linux-x86_64 -o pt
chmod +x pt && sudo mv pt /usr/local/bin/
pt --version
```

**Linux aarch64**
```bash
curl -L https://github.com/SteelCrab/polytunnel/releases/latest/download/pt-linux-aarch64 -o pt
chmod +x pt && sudo mv pt /usr/local/bin/
```

**Linux musl (Alpine)**
```bash
curl -L https://github.com/SteelCrab/polytunnel/releases/latest/download/pt-linux-musl -o pt
chmod +x pt && sudo mv pt /usr/local/bin/
```

**Windows x86_64 (PowerShell)**
```powershell
Invoke-WebRequest `
  -Uri https://github.com/SteelCrab/polytunnel/releases/latest/download/pt-windows-x86_64.exe `
  -OutFile pt.exe
# Move pt.exe somewhere on your PATH
```

### Option 2 — Verified archive download

Use this if you want SHA-256 verification before running the binary.

```bash
VERSION=0.2.0
TARGET=linux-x86_64
curl -LO https://github.com/SteelCrab/polytunnel/releases/download/v${VERSION}/polytunnel-${VERSION}-${TARGET}.tar.gz
curl -LO https://github.com/SteelCrab/polytunnel/releases/download/v${VERSION}/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
tar -xzf polytunnel-${VERSION}-${TARGET}.tar.gz
sudo mv pt /usr/local/bin/
pt --version
```

Replace `TARGET` with one of: `linux-x86_64`, `linux-aarch64`, `linux-musl`, `linux-aarch64-musl`, `macos-aarch64`, `windows-x86_64`, `windows-aarch64`.

### Option 3 — Via Cargo (requires Rust 1.75+)

```bash
cargo install polytunnel
```

### Option 4 — Build from source

```bash
git clone https://github.com/SteelCrab/polytunnel.git
cd polytunnel
cargo build --release
./target/release/pt --version
```

### Prerequisites

Polytunnel manages Java projects, so you need a working JDK:

- **Java 17+** (JDK, not JRE) with `javac` and `java` on PATH
- Verify: `javac --version && java --version`

---

## Quick Start

```bash
# 1. Create a new project
pt init my-app
cd my-app

# 2. Add a dependency
pt add com.google.guava:guava:33.0.0-jre

# 3. Add a test dependency
pt add org.junit.jupiter:junit-jupiter:5.10.1 --scope test

# 4. Build (downloads deps, compiles sources, runs tests)
pt build

# 5. Run your application
pt run com.example.App
pt run com.example.App -- --port 8080 --debug   # pass args after `--`

# 6. Inspect dependency tree
pt tree
```

Full runnable example: [`examples/hello-java`](examples/hello-java).

## Commands

All commands below are shipped and working.

| Command | Description |
|---|---|
| `pt init [name]` | Initialize a new project with `polytunnel.toml` |
| `pt add <groupId:artifactId:version> [--scope <compile\|runtime\|test\|provided>]` | Add a dependency |
| `pt remove <groupId:artifactId>` | Remove a dependency |
| `pt sync [-v]` | Download/resolve all declared dependencies |
| `pt tree [-v]` | Print dependency tree |
| `pt build [--clean] [--skip-tests] [-v]` | Compile sources and run tests |
| `pt test [PATTERN] [-v] [--fail-fast]` | Run tests only |
| `pt run <MAIN_CLASS> [args...] [-v]` | Run a Java main class |
| `pt vscode` | Generate `.vscode/` config for IntelliSense |

Run `pt <command> --help` for detailed flags.

## Configuration

`polytunnel.toml`:

```toml
[project]
name = "my-app"
java_version = "17"

[build]
source_dirs = ["src/main/java"]
test_source_dirs = ["src/test/java"]
output_dir = "target/classes"
test_output_dir = "target/test-classes"
compiler_args = ["-encoding", "UTF-8", "-g"]
test_framework = "auto"          # JUnit 5/4 and TestNG are auto-detected

[dependencies]
"com.google.guava:guava" = "33.0.0-jre"
"org.junit.jupiter:junit-jupiter" = { version = "5.10.1", scope = "test" }

[[repositories]]
name = "central"
url = "https://repo1.maven.org/maven2/"
```

## Project Layout

Standard Maven layout:

```
my-app/
├── polytunnel.toml
├── src/
│   ├── main/java/
│   └── test/java/
└── target/
    ├── classes/
    └── test-classes/
```

## Architecture (for contributors)

Cargo workspace at `crates/`:

| Crate | Role |
|---|---|
| `polytunnel` | CLI binary (`pt`) |
| `polytunnel-core` | Config parsing, shared types |
| `polytunnel-maven` | Maven Central HTTP client, POM parser |
| `polytunnel-resolver` | Concurrent dependency resolution |
| `polytunnel-build` | javac compilation, test runner |
| `polytunnel-ide` | VS Code integration |

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

Before opening a PR: build → clippy → fmt → test (all must pass).

## Support

☕ [ko-fi.com/pistacrab](https://ko-fi.com/pistacrab)

## License

Apache-2.0
