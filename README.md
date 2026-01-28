# Polytunnel

[한국어](README_KR.md) | [CI](../../actions) | [Codecov](https://codecov.io/gh/SteelCrab/polytunnel)

Fast Java dependency manager written in Rust (uv/ruff style).

## Features

- **Fast**: Rust-based for speed
- **Parallel**: Concurrent dependency resolution and downloads
- **Simple**: Intuitive CLI and configuration
- **Build**: Direct javac compilation
- **Test**: Auto-detect JUnit 5/4, TestNG
- **Cross-Platform**: Windows, macOS, Linux (x86_64, ARM64)

## Comparison

| Feature | Maven | Gradle | Polytunnel |
|---------|-------|--------|------------|
| Speed | Slow | Medium | **Instant** |
| Config | XML | Groovy/Kotlin | **TOML** |
| Scope | All-in-one | DSL | **Focused** |
| Size | Large | Large | **~5MB** |

## Architecture

| Crate | Description |
|-------|-------------|
| `polytunnel` | CLI binary (`pt` command) |
| `polytunnel-core` | Core types, config parsing |
| `polytunnel-maven` | Maven Central API client |
| `polytunnel-resolver` | Dependency resolution |
| `polytunnel-build` | Build and test execution |

## Installation

```bash
cargo install polytunnel
```

## Quick Start

```bash
# Initialize project
pt init my-java-app

# Build (compile & test)
pt build

# Run tests only
pt test
```

See `examples/hello-java` for a complete example.

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
| `pt init` | Initialize project | Working |
| `pt build` | Compile and run tests | Working |
| `pt test` | Run tests only | Working |
| `pt add` | Add dependency | Planned |
| `pt remove` | Remove dependency | Planned |
| `pt sync` | Sync dependencies | Planned |
| `pt tree` | Show dependency tree | Planned |

## Build & Test

```bash
# Build
pt build              # Full build
pt build --clean      # Clean rebuild
pt build --skip-tests # No tests
pt build -v           # Verbose

# Test
pt test           # All tests
pt test MyClass   # Specific test
pt test -v        # Verbose
pt test --fail-fast
```

## Directory Structure

Standard Maven layout:

```
project-root/
├── polytunnel.toml
├── src/main/java/
├── src/test/java/
└── target/
```

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

## License

Apache-2.0
