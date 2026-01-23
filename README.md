# Polytunnel

[![한국어](https://img.shields.io/badge/lang-한국어-blue.svg)](README_KR.md)
[![CI](https://github.com/SteelCrab/polytunnel/workflows/CI/badge.svg)](../../actions)
[![codecov](https://codecov.io/gh/SteelCrab/polytunnel/graph/badge.svg)](https://codecov.io/gh/SteelCrab/polytunnel)

> Fast Java dependency manager written in Rust (uv/ruff style)

## Features

- ⚡ **Fast** - Rust-based for speed
- 🚀 **Parallel** - Concurrent dependency resolution and downloads
- 🎯 **Simple** - Intuitive CLI and config
- 🔒 **Reproducible** - Lock file support (planned)
- 🛠️ **Build** - Direct javac compilation
- 🧪 **Test** - Auto-detect JUnit 5/4, TestNG
- 🖥️ **Cross-Platform** - Windows (amd64, ARM64), macOS (x86_64, ARM64), Linux (all architectures)

## Comparison with Existing Tools

| Feature | Maven | Gradle | Polytunnel |
|---------|-------|--------|------------|
| **Speed** | Slow (JVM startup) | Medium (JVM + Daemon) | **Instant** (Native binary) |
| **Config** | Verbose XML | Complex Groovy/Kotlin | **Simple TOML** |
| **Scope** | Everything | Limitless (DSL) | **Focused** (Build/Deps) |
| **Size** | ~10MB + JVM | ~100MB + JVM | **~5MB** (Standalone) |

*(Note: Polytunnel is currently in early development and lacks many advanced features of mature tools)*


## Architecture

| Crate | Description |
|-------|-------------|
| `polytunnel` | CLI binary (`pt` command) |
| `polytunnel-core` | Core types, config parsing, error handling |
| `polytunnel-maven` | Maven Central API client, POM parser |
| `polytunnel-resolver` | Dependency resolution algorithm |
| `polytunnel-build` | Build and test execution engine |

## Installation

```bash
cargo install polytunnel
```

## Quick Start

```bash
# Initialize project
pt init my-java-app

# Build the project (compiles and runs tests)
pt build

# Run tests only
pt test
```

See `examples/hello-java` for a complete working example.

## Configuration

`polytunnel.toml`:

```toml
[project]
name = "my-java-app"
java_version = "17"

[build]
source_dirs = ["src/main/java"]
test_source_dirs = ["src/test/java"]
output_dir = "target/classes"
test_output_dir = "target/test-classes"
compiler_args = ["-encoding", "UTF-8", "-g"]
test_framework = "auto"

[dependencies]
"com.google.guava:guava" = "33.0.0-jre"
"org.junit.jupiter:junit-jupiter" = { version = "5.10.1", scope = "test" }

[[repositories]]
name = "central"
url = "https://repo1.maven.org/maven2/"
```

## Commands

| Command | Description | Status |
|---------|-------------|--------|
| `pt init [name]` | Initialize project | ✅ Working |
| `pt build` | Compile and run tests | ✅ Working |
| `pt test [PATTERN]` | Run tests only | ✅ Working |
| `pt add <dep>` | Add dependency | 🚧 Phase 3 |
| `pt remove <dep>` | Remove dependency | 🚧 Phase 3 |
| `pt sync` | Sync dependencies | 🚧 Phase 3 |
| `pt tree` | Show dependency tree | 🚧 Phase 3 |

## Build Commands

```bash
pt build              # Full build with tests
pt build --clean      # Clean rebuild
pt build --skip-tests # Build without tests
pt build -v           # Verbose output
```

## Test Commands

```bash
pt test           # Run all tests
pt test MyTest    # Run specific test class
pt test -v        # Verbose output
pt test --fail-fast
```

## Supported Test Frameworks

- **JUnit 5 (Jupiter)** - Modern testing framework
- **JUnit 4** - Legacy but widely used
- **TestNG** - Alternative with advanced features

## Directory Structure

```
project-root/
├── polytunnel.toml
├── src/
│   ├── main/java/     # Main source files
│   └── test/java/     # Test source files
├── target/
│   ├── classes/       # Compiled main classes
│   └── test-classes/  # Compiled test classes
└── .polytunnel/
    ├── cache/         # Downloaded JARs
    └── build-cache.json
```

## Continuous Integration

Polytunnel uses GitHub Actions for automated testing and building across multiple platforms and architectures.

### CI Jobs

| Job | Purpose | Platform(s) | Status |
|-----|---------|-------------|--------|
| **Build** | Debug build verification | Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x86_64) | ✅ |
| **Test** | Unit and documentation tests | Linux (x86_64) | ✅ |
| **Lint** | Clippy static analysis with strict warnings | Linux (x86_64) | ✅ |
| **Format** | Code formatting check (rustfmt) | Linux (x86_64) | ✅ |
| **Release Build** | Optimized release binaries | Linux (x86_64), macOS (aarch64), Windows (x86_64) | ✅ |
| **Coverage** | Code coverage with llvm-cov (uploaded to Codecov) | Linux (x86_64) | ✅ |

### Supported Platforms for Building

| Platform | Arch | Build | Release |
|----------|------|-------|---------|
| Linux | x86_64 | ✅ | ✅ |
| Linux | aarch64 | ✅ | ⏳ |
| macOS | x86_64 | ✅ | ⏳ |
| macOS | aarch64 (Apple Silicon) | ✅ | ✅ |
| Windows | x86_64 | ✅ | ✅ |

### Triggers

CI runs automatically on:
- All branches (push)
- All pull requests

### View Workflows

Check the latest CI runs at: [GitHub Actions](../../actions)

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

## Roadmap

- ✅ **Phase 1** - Project setup (Rust workspace, CLI, config)
- ✅ **Phase 2a** - Maven Central integration (API client, POM parser)
- ✅ **Phase 2b** - Build & test engine (javac, test execution)
- 🚧 **Phase 3** - Dependency management (`add`, `remove`, `sync`, `tree`)
- ✅ **Phase 3.5** - Parallel dependency resolution and concurrent downloads
- ⏳ **Phase 4** - Advanced features (cache, lock files)

## License

Apache-2.0
