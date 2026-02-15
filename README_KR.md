# Polytunnel

[Roadmap](ROADMAP.md) | [로드맵](ROADMAP_KR.md)

Rust로 작성된 빠른 Java 의존성 관리 도구 (uv/ruff 스타일).

## 기능

- **속도**: Rust 기반의 빠른 속도
- **병렬 처리**: 동시 의존성 해결 및 다운로드
- **단순함**: 직관적인 CLI와 설정
- **빌드**: javac 직접 컴파일 지원
- **테스트**: JUnit 5/4, TestNG 자동 감지
- **크로스 플랫폼**: Windows, macOS, Linux 지원

## 비교

| 특징 | Maven | Gradle | Polytunnel |
|-----|-------|--------|------------|
| 속도 | 느림 | 보통 | **즉시 실행** |
| 설정 | 복잡한 XML | 어려운 Groovy/Kotlin | **단순한 TOML** |
| 범위 | 모든 기능 | 무제한 확장 | **핵심 기능 집중** |
| 크기 | ~10MB | ~100MB | **~5MB** |

## 아키텍처

| 크레이트 | 설명 |
|---------|------|
| `polytunnel` | CLI 바이너리 |
| `polytunnel-core` | 핵심 타입 및 설정 |
| `polytunnel-maven` | Maven Central API 클라이언트 |
| `polytunnel-resolver` | 의존성 해결 알고리즘 |
| `polytunnel-build` | 빌드 및 테스트 실행 엔진 |

## 설치

```bash
cargo install polytunnel
```

## 빠른 시작

```bash
# 프로젝트 초기화
pt init my-java-app

# 빌드 (컴파일 및 테스트)
pt build

# 테스트만 실행
pt test
```

예제는 `examples/hello-java`를 참고하세요.

## 설정

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

## 명령어

| 명령어 | 설명 | 상태 |
|--------|------|------|
| `pt init` | 프로젝트 초기화 | 작동 |
| `pt build` | 컴파일 및 테스트 | 작동 |
| `pt test` | 테스트만 실행 | 작동 |
| `pt run` | 애플리케이션 실행 엔트리포인트 실행 | 계획 |
| `pt add` | 의존성 추가 | 계획 |
| `pt remove` | 의존성 제거 | 계획 |
| `pt sync` | 의존성 동기화 | 계획 |
| `pt tree` | 의존성 트리 | 계획 |

## 빌드 및 테스트

```bash
# 빌드
pt build              # 전체 빌드
pt build --clean      # 클린 빌드
pt build --skip-tests # 테스트 제외
pt build -v           # 상세 출력

# 테스트
pt test           # 모든 테스트
pt test MyClass   # 특정 클래스 테스트
pt test -v        # 상세 출력
pt test --fail-fast
```

## 디렉토리 구조

표준 Maven 구조를 따릅니다:

```
project-root/
├── polytunnel.toml
├── src/main/java/
├── src/test/java/
└── target/
```

## 개발

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

## 라이선스

Apache-2.0
