# Polytunnel

[![CI](https://github.com/SteelCrab/polytunnel/workflows/CI/badge.svg)](../../actions)
[![codecov](https://codecov.io/gh/SteelCrab/polytunnel/graph/badge.svg?branch=main)](https://codecov.io/gh/SteelCrab/polytunnel?branch=main)
[![English](https://img.shields.io/badge/lang-English-blue.svg)](README.md)

**Rust로 작성된 빠른 Java 의존성 관리 도구** — `uv`, `ruff`에서 영감. 선언형 TOML 설정, 네이티브 바이너리, JVM 데몬 없음.

```bash
pt init my-app && cd my-app
pt add com.google.guava:guava:33.0.0-jre
pt build
pt run com.example.App
```

---

## 왜 Polytunnel인가?

|  | Maven | Gradle | **Polytunnel** |
|---|---|---|---|
| 시작 속도 | JVM 느림 | JVM 느림 (+데몬) | **즉시 실행 (네이티브)** |
| 설정 | XML (장황) | Groovy/Kotlin (스크립트) | **TOML (선언형)** |
| 바이너리 크기 | ~10MB + JVM | ~100MB + JVM | **~6MB, JVM 불필요** |
| 피크 메모리 | 500MB+ | 500MB~2GB | **~50-200MB 목표** |
| 설치 | `brew install maven` | `brew install gradle` | **바이너리 한 개** |

## 설치

원하는 방법으로 선택하세요.

### 방법 1 — 사전 빌드 바이너리 (권장, Rust 불필요)

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
# pt.exe를 PATH에 포함된 디렉토리로 이동
```

### 방법 2 — 체크섬 검증 포함 아카이브 다운로드

바이너리 실행 전에 SHA-256 검증을 하고 싶을 때.

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

`TARGET`은 다음 중 하나: `linux-x86_64`, `linux-aarch64`, `linux-musl`, `linux-aarch64-musl`, `macos-aarch64`, `windows-x86_64`, `windows-aarch64`.

### 방법 3 — Cargo (Rust 1.75+ 필요)

```bash
cargo install polytunnel
```

### 방법 4 — 소스에서 빌드

```bash
git clone https://github.com/SteelCrab/polytunnel.git
cd polytunnel
cargo build --release
./target/release/pt --version
```

### 사전 요구 사항

Polytunnel은 Java 프로젝트를 관리하므로 작동하는 JDK가 필요합니다:

- **Java 17+** (JDK, JRE 아님) — `javac`와 `java`가 PATH에 있어야 함
- 확인: `javac --version && java --version`

---

## 빠른 시작

```bash
# 1. 새 프로젝트 생성
pt init my-app
cd my-app

# 2. 의존성 추가
pt add com.google.guava:guava:33.0.0-jre

# 3. 테스트 의존성 추가
pt add org.junit.jupiter:junit-jupiter:5.10.1 --scope test

# 4. 빌드 (의존성 다운로드 + 컴파일 + 테스트)
pt build

# 5. 애플리케이션 실행
pt run com.example.App
pt run com.example.App -- --port 8080 --debug   # `--` 이후는 앱 인자

# 6. 의존성 트리 확인
pt tree
```

실행 가능한 전체 예제: [`examples/hello-java`](examples/hello-java).

## 명령어

아래 모든 명령어는 현재 사용 가능합니다.

| 명령어 | 설명 |
|---|---|
| `pt init [name]` | `polytunnel.toml`로 새 프로젝트 초기화 |
| `pt add <groupId:artifactId:version> [--scope <compile\|runtime\|test\|provided>]` | 의존성 추가 |
| `pt remove <groupId:artifactId>` | 의존성 제거 |
| `pt sync [-v]` | 선언된 의존성 전부 다운로드/해석 |
| `pt tree [-v]` | 의존성 트리 출력 |
| `pt build [--clean] [--skip-tests] [-v]` | 소스 컴파일 + 테스트 실행 |
| `pt test [PATTERN] [-v] [--fail-fast]` | 테스트만 실행 |
| `pt run <MAIN_CLASS> [args...] [-v]` | Java 메인 클래스 실행 |
| `pt vscode` | IntelliSense용 `.vscode/` 설정 생성 |

세부 옵션은 `pt <command> --help`로 확인.

## 설정

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
test_framework = "auto"          # JUnit 5/4와 TestNG 자동 감지

[dependencies]
"com.google.guava:guava" = "33.0.0-jre"
"org.junit.jupiter:junit-jupiter" = { version = "5.10.1", scope = "test" }

[[repositories]]
name = "central"
url = "https://repo1.maven.org/maven2/"
```

## 프로젝트 구조

Maven 표준 레이아웃을 따릅니다:

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

## 아키텍처 (기여자용)

Cargo workspace `crates/`:

| 크레이트 | 역할 |
|---|---|
| `polytunnel` | CLI 바이너리 (`pt`) |
| `polytunnel-core` | 설정 파싱, 공통 타입 |
| `polytunnel-maven` | Maven Central HTTP 클라이언트, POM 파서 |
| `polytunnel-resolver` | 동시 의존성 해석 |
| `polytunnel-build` | javac 컴파일, 테스트 러너 |
| `polytunnel-ide` | VS Code 통합 |

## 개발

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

PR 전 체크: build → clippy → fmt → test (전부 통과해야 함).

## 후원

☕ [ko-fi.com/pistacrab](https://ko-fi.com/pistacrab)

## 라이선스

Apache-2.0
