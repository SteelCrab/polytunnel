# Polytunnel 로드맵

## 마일스톤: v0.1.0 — 초기 릴리즈 (2026-02-18 출시)

[x] 1주차: 초기 동작 명령어 출고 (`init`, `build`, `test`)
[x] 2주차: 기준 릴리즈 패키지 및 공개 준비 진행
  [x] `/package-spec.md`에 비컨테이너 배포 산출물 사양을 문서화한다.
  [x] `/package-spec.md`에 아티팩트 형식, 파일명 규칙, 체크섬 정책, 설치 절차를 정의한다.
  [x] `/package-spec.md`에 기준 플랫폼을 확정한다: linux-x86_64, linux-aarch64, linux-musl, macos-aarch64, windows-x86_64, windows-aarch64, linux-aarch64-musl (7개 플랫폼).
  [x] `/package-spec.md`에 공개 산출물 항목(바이너리, 예시 프로젝트, 호환성 안내, 서명 정책)을 정의한다.
  [x] README/README_KR에 체크섬 검증 기반 설치 절차를 반영한다.
[x] 3주차: CI/빌드/테스트 매트릭스 출시 기준 안정화
  [x] `package-spec.md`에 windows-aarch64 비컨테이너 릴리즈 산출물(`polytunnel-<version>-windows-aarch64.zip`) 타깃 및 파일명 규칙 추가
  [x] `package-spec.md`에 linux-aarch64-musl 비컨테이너 릴리즈 산출물 타깃 및 파일명 규칙 추가
  [x] windows-aarch64 및 linux-aarch64-musl 빌드, 체크섬 생성, 퍼블리시 검증을 포함하도록 릴리즈 CI 매트릭스 확장
  [x] 7개 플랫폼 빌드 매트릭스 전체 안정화 (linux-x86_64, linux-aarch64, linux-musl, linux-aarch64-musl, macos-aarch64, windows-x86_64, windows-aarch64)
[x] 4주차: 기여자 문서 및 릴리즈 자동화 완료
  [x] 개발 환경 설정, 브랜치 명명, 커밋 형식 가이드를 포함한 `CONTRIBUTING.md` 추가
  [x] 초기 v0.1.0 릴리즈 노트를 포함한 `CHANGELOG.md` 추가
  [x] 태그 푸시 시 CI를 통해 GitHub Releases 자동화 (릴리즈 노트, 아티팩트 업로드, SHA256SUMS)
  [x] 릴리즈 아티팩트와 함께 `SHA256SUMS` 체크섬 파일 생성 및 공개

---

## 마일스톤: v0.1.1 — CLI 완성 및 테스트 강화

### 테스트 강화 (대부분 완료)
[x] `build`, `test` 명령의 에러 플로우 전체 감사
[x] Java 도구 미설치 시 스킵/안내 처리 보강
[x] Maven 클라이언트 정상/비정상 경로 통합 테스트 확장
[x] 비-2xx 응답 매핑 및 오류 계층 정합성 테스트
[x] `build`/`vscode` 오류 매핑 정비 및 사용자 메시지 정리
[x] 공통 실패 케이스 메시지 품질 개선
[x] 제한 환경에서 CI 안정성 확보를 위한 비결정성 완화
[x] 커버리지 확인 — 라인 커버리지 91.14% 달성

### CLI 완성
[ ] `pt sync` 명령 구현: 선언된 의존성 전체 해석 및 다운로드
    *(현재 `fix/cli-sync-tree-test` 브랜치에서 진행 중)*
[ ] `pt tree` 명령 구현: 해석된 의존성 트리 출력
    *(현재 `fix/cli-sync-tree-test` 브랜치에서 진행 중)*
[ ] `pt add <coordinate>` 명령 구현 + 단위/통합 테스트
[ ] `pt add` 시 중복 의존성 및 잘못된 좌표 입력 검증 강화
[ ] `pt remove <coordinate>` 명령 구현 + 테스트
[ ] 부분 실패 대비 파일 기반 롤백 동작 보강
[ ] `pt run <main-class>` 명령 구현으로 실행 엔트리 포인트 지원
[ ] 신규 명령 전체의 사용법 및 종료 코드 문서 정비

---

## 마일스톤: v0.2.0 — Lock 파일, 캐시 및 증분 빌드

### Lock 파일
[ ] `pt sync` 실행 시 `polytunnel.lock` 생성 및 검증
[ ] Lock 파일에 해석된 좌표, 버전, 아티팩트 체크섬 기록
[ ] `pt sync --frozen`: Lock 파일 없거나 오래된 경우 실패 (CI 안전 모드)

### 캐시
[ ] `pt cache clean` 및 `pt cache stats` 서브명령 구현
[ ] 읽기 시 캐시 아티팩트 SHA-256 무결성 검증
[ ] 캐시 만료 정책 및 크기 보고

### 증분 빌드
[ ] 소스 파일 해시 및 클래스패스 지문 기반 `BuildCache` 구현
[ ] `BuildCache`를 오케스트레이터에 연결: 변경되지 않은 모듈의 `javac` 호출 건너뜀
[ ] 전체 재빌드 강제를 위한 `--no-cache` 플래그 추가

### 안정성
[ ] 병렬 다운로드 동시성 제어 (설정 가능한 워커 수)
[ ] 일시적 네트워크 실패에 대한 지수 백오프 재시도 로직
[ ] 개선된 진단: 구조화된 오류 메시지와 디버그 힌트 (`pt --debug`)

---

## 마일스톤: v0.3.0 — Gradle 호환성 및 어노테이션 프로세서

### Gradle 지원
[ ] `polytunnel-gradle` crate 구현: Gradle Plugin Portal HTTP 클라이언트 및 POM 해석
[ ] `build.gradle` / `build.gradle.kts` 의존성 블록 파싱 (공통 부분집합)
[ ] `pt migrate` 구현: `build.gradle` → `polytunnel.toml` 변환

### 어노테이션 프로세서
[ ] Lombok 어노테이션 프로세서 지원 (소스 유지, `delombok` 통합)
[ ] MapStruct 코드 생성 지원
[ ] Spring Boot BOM 임포트 및 `bootJar` / `bootRun` 동등 기능

---

## 마일스톤: v0.4.0 — 플러그인 시스템 및 고급 IDE/컨테이너 지원

### 플러그인 시스템
[ ] WASM 플러그인 런타임 (wasmtime): `Plugin` trait 및 샌드박스 실행 모델 정의
[ ] 플러그인 레지스트리: `polytunnel.toml`의 `[plugins]` 테이블에서 플러그인 로드
[ ] 빌트인 플러그인: SpotBugs, Checkstyle, PMD, JaCoCo

### IDE 통합
[ ] VS Code 확장: 실시간 의존성 해석 피드백 및 오류 렌즈
[ ] IntelliJ 플러그인: `polytunnel.toml`을 프로젝트 모델로 임포트

### 컨테이너 패키지
[ ] Docker 패키지 아키텍처 설계 (Dockerfile, 베이스 이미지, non-root 실행, TZ/JAVA_HOME/캐시 경로 정책)
[ ] CI 기반 다중 아키텍처 이미지 빌드/푸시 (`linux/amd64`, `linux/arm64`) 및 태그 전략
[ ] 컨테이너 런타임 Smoke 테스트 (`docker run --rm <image> --help`, 버전 출력, 샘플 빌드)
[ ] README 및 README_KR에 `docker run` 사용법 및 마운트 예시 반영

---

## 마일스톤: v1.0.0 — 프로덕션 준비

### 정확성 및 완성도
[ ] JUnit 4 및 TestNG 테스트 실행 (단순 감지가 아닌 전체 러너 통합)
[ ] 버전 충돌 감지: 다이아몬드 의존성 충돌 경고; `--strict` 모드로 충돌 시 실패
[ ] 오프라인 모드: 캐시에서만 해석, 아티팩트 없을 시 즉시 실패

### 성능
[ ] Maven 및 Gradle 대비 콜드/워밍 빌드 시간 벤치마크 스위트 공개
[ ] 메모리 및 할당 프로파일링; 대용량 의존성 그래프에서 피크 RSS 감소

### 문서화
[ ] 완전한 사용자 가이드 (시작하기, 명령어 레퍼런스, 설정 레퍼런스)
[ ] 마이그레이션 가이드: Maven → Polytunnel, Gradle → Polytunnel
[ ] 기여자 가이드: 아키텍처 심층 분석, 새 명령어 추가, 플러그인 개발

### 출시 준비
[ ] 엔드-투-엔드 출시 준비 체크리스트 최종 승인
[ ] `CONTRIBUTING.md`에 PR 리뷰 플레이북 및 머지 기준 문서화
[ ] 전체 CHANGELOG 및 마이그레이션 노트와 함께 GitHub Release로 v1.0.0 공개
