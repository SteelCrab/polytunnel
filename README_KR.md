# Polytunnel

> Rust로 작성된 빠른 Java 의존성 관리 도구 (uv/ruff 스타일)
>
> Fast Java dependency manager written in Rust

## 기능

- ⚡ **속도** - Rust 기반으로 빠른 처리 속도
- 🚀 **병렬 처리** - 동시 의존성 해결 및 다운로드
- 🎯 **단순함** - 직관적인 CLI와 설정 파일
- 🔒 **재현성** - Lock 파일 지원으로 빌드 재현 보장 (계획 중)
- 🛠️ **빌드** - javac 직접 컴파일 지원
- 🧪 **테스트** - JUnit 5/4, TestNG 자동 감지

## 기존 도구와의 비교

| 특징 | Maven | Gradle | Polytunnel |
|-----|-------|--------|------------|
| **속도** | 느림 (JVM 시작) | 보통 (JVM + 데몬) | **즉시 실행** (네이티브 바이너리) |
| **설정** | 복잡한 XML | 어려운 Groovy/Kotlin | **단순한 TOML** |
| **범위** | 모든 기능 포함 | 무제한 확장 (DSL) | **핵심 기능 집중** (빌드/의존성) |
| **크기** | ~10MB + JVM | ~100MB + JVM | **~5MB** (단일 실행파일) |

*(참고: Polytunnel은 현재 초기 개발 단계이며, 성숙한 도구들의 고급 기능은 아직 부족할 수 있습니다)*


## 아키텍처

| 크레이트 | 설명 | Description |
|---------|------|-------------|
| `polytunnel` | CLI 바이너리 (`pt` 명령) | CLI binary |
| `polytunnel-core` | 핵심 타입, 설정 파싱, 에러 처리 | Core types, config, error handling |
| `polytunnel-maven` | Maven Central API 클라이언트, POM 파서 | Maven Central API client |
| `polytunnel-resolver` | 의존성 해결 알고리즘 | Dependency resolution algorithm |
| `polytunnel-build` | 빌드 및 테스트 실행 엔진 | Build and test execution engine |

## 설치

```bash
cargo install polytunnel
```

## 빠른 시작

```bash
# 프로젝트 초기화
pt init my-java-app

# 프로젝트 빌드 (컴파일 및 테스트 실행)
pt build

# 테스트만 실행
pt test
```

완전한 작동 예제는 `examples/hello-java`를 참고하세요.

## 설정

`polytunnel.toml`:

```toml
[project]
name = "my-java-app"
java_version = "17"

[build]
# 소스 디렉토리
source_dirs = ["src/main/java"]
test_source_dirs = ["src/test/java"]

# 출력 디렉토리
output_dir = "target/classes"
test_output_dir = "target/test-classes"

# 컴파일러 옵션
compiler_args = ["-encoding", "UTF-8", "-g"]
test_compiler_args = ["-encoding", "UTF-8", "-g"]

# 테스트 프레임워크 (의존성에서 자동 감지)
test_framework = "auto"

[dependencies]
"com.google.guava:guava" = "33.0.0-jre"
"org.junit.jupiter:junit-jupiter" = { version = "5.10.1", scope = "test" }

[[repositories]]
name = "central"
url = "https://repo1.maven.org/maven2/"
```

## 명령어

| 명령어 | 설명 | Description | 상태 |
|--------|------|-------------|------|
| `pt init [name]` | 프로젝트 초기화 | Initialize project | ✅ 작동 |
| `pt build` | 컴파일 및 테스트 | Compile and run tests | ✅ 작동 |
| `pt test [PATTERN]` | 테스트만 실행 | Run tests only | ✅ 작동 |
| `pt add <dep>` | 의존성 추가 | Add dependency | 🚧 Phase 3 |
| `pt remove <dep>` | 의존성 제거 | Remove dependency | 🚧 Phase 3 |
| `pt sync` | 의존성 동기화 | Sync dependencies | 🚧 Phase 3 |
| `pt tree` | 의존성 트리 | Show dependency tree | 🚧 Phase 3 |

### 상태 설명

- ✅ **작동** - 완전히 구현되고 테스트됨
- 🚧 **Phase 3** - 다음 릴리스에서 구현 예정
- ⏳ **계획 중** - 향후 단계에서 구현 예정

## 빌드 명령어

```bash
# 테스트와 함께 전체 빌드
pt build

# 깨끗한 빌드 (기존 아티팩트 제거)
pt build --clean

# 테스트 없이 빌드
pt build --skip-tests

# 상세 출력
pt build -v
```

## 테스트 명령어

```bash
# 모든 테스트 실행
pt test

# 특정 테스트 클래스만 실행
pt test MyTest

# 상세 출력
pt test -v

# 첫 번째 실패에서 중단
pt test --fail-fast
```

## 빌드 기능

### 지원하는 테스트 프레임워크

Polytunnel은 다음 테스트 프레임워크를 자동으로 감지하고 실행합니다:

- **JUnit 5 (Jupiter)** - 주석과 매개변수화된 테스트를 지원하는 현대적인 테스트 프레임워크
- **JUnit 4** - 여전히 광범위하게 사용되는 레거시 테스트 프레임워크
- **TestNG** - 고급 기능을 제공하는 대체 프레임워크

테스트 프레임워크는 `polytunnel.toml`의 의존성에서 자동으로 감지됩니다. 원하는 테스트 프레임워크를 test-scoped 의존성으로 추가하기만 하면 됩니다:

```toml
[dependencies]
"org.junit.jupiter:junit-jupiter" = { version = "5.10.1", scope = "test" }
```

### 빌드 출력

컴파일된 클래스는 다음 위치에 저장됩니다:
- **메인 소스**: `target/classes/`
- **테스트 소스**: `target/test-classes/`

빌드 메타데이터와 다운로드된 의존성은 다음에 캐시됩니다:
- **의존성 캐시**: `.polytunnel/cache/`
- **빌드 캐시**: `.polytunnel/build-cache.json` (증분 빌드용)

### 디렉토리 구조

Polytunnel은 Maven 표준 디렉토리 구조를 따릅니다:

```
project-root/
├── polytunnel.toml
├── src/
│   ├── main/
│   │   └── java/          # 메인 소스 파일
│   └── test/
│       └── java/          # 테스트 소스 파일
├── target/
│   ├── classes/           # 컴파일된 메인 클래스
│   └── test-classes/      # 컴파일된 테스트 클래스
└── .polytunnel/
    ├── cache/             # 다운로드된 JAR
    └── build-cache.json   # 증분 빌드 메타데이터
```

## 예제

`examples/hello-java/`에서 다음을 시연하는 완전한 작동 예제를 확인하세요:

- Java 소스 코드 구조
- Guava 같은 외부 라이브러리 사용
- JUnit 5로 테스트 작성 및 실행
- polytunnel.toml 설정

```bash
# 예제 시도
cd examples/hello-java
../../target/release/pt build
../../target/release/pt test
```

## 개발

```bash
# 빌드
cargo build --workspace

# 테스트
cargo test --workspace

# 코드 검사
cargo clippy --workspace -- -D warnings

# 코드 포매팅
cargo fmt --check
```

## 로드맵

### 완료됨 ✅

- **Phase 1** - 프로젝트 셋업 (Rust 워크스페이스, CLI, 설정 파일)
- **Phase 2a** - Maven Central 연동 (API 클라이언트, POM 파서)
- **Phase 2b** - 빌드 & 테스트 엔진 (javac 컴파일러, 테스트 실행)

### 진행 중 🚧

- **Phase 3** - 핵심 의존성 관리 (`pt add`, `pt remove`, `pt sync`, `pt tree`)

### 완료됨 ✅ (최근)

- **Phase 3.5** - 병렬 의존성 해결 및 동시 다운로드

### 계획 중 ⏳

- **Phase 4** - 고급 기능 (로컬 캐시, Lock 파일, Gradle 지원)

## 라이선스

MIT
