# Polytunnel 로드맵

이 로드맵은 워크스페이스 버전 **0.1.0** 기준으로 12주 간 3개 마일스톤과 1주차 단위 실행 항목으로 구성했습니다.

## 과거 커밋/브랜치 기준 완료 작업
- [x] `feat/build-progress-bar` — 진행률 표시와 병렬 다운로드 오케스트레이션 적용.
- [x] `test/resolver`, `test/polytunnel-maven` — resolver/전이 의존성/ Maven 클라이언트 테스트 강화.
- [x] `test/commands-coverage` — CLI 모듈의 에러 경로와 커버리지 미싱 구간 정리.
- [x] `ci/codecov` — CI 커버리지 파이프라인 및 임계값 튜닝.
- [x] `docs/add-roadmap` — 초기 로드맵 초안 및 기초 계획 문서화.
- [x] `docs/update-roadmap-parallel` — 구현 완료된 병렬 처리 항목 기준으로 로드맵 업데이트.
- [x] `docs/simplify-readme` — README 가독성 개선 및 명령어 목록 정비.
- [x] `feat/error-reporting` — 중앙 집중 오류 리포팅 플로우 추가.
- [x] `feat/windows-arm64-support` — Windows ARM64 플랫폼 지원 반영.
- [x] `feat/refactor-resolve` — 의존성 해석 흐름 리팩터링.
- [x] `fix/transitive-dependency-resolution` — 전이 의존성 해석 수정.
- [x] `fix/ci-workflow-improvements` — CI 신뢰성 개선.
- [x] `chore/license-apache-2.0` — 라이선스 변경 및 문서 정합성 정비.
- [x] `chore/setup-precommit` — pre-commit 툴 체인 구성.
- [x] `ci/codecov` + `test-coverage` — 테스트 계측/커버리지 워크플로우 현대화.

## 마일스톤: v0.1.0 (진행중)
- [x] 1주차: 초기 동작 명령어 출고 (`init`, `build`, `test`)
- [ ] 2주차: 기준 릴리즈 패키지 및 공개 준비 진행
  - 비컨테이너 배포 경로(아티팩트 형식, 파일명 규칙, 체크섬/서명) 기준 확정.
  - 저장소 공개 항목(바이너리, 예시 프로젝트, 호환성 안내) 구성 결정.
  - README/문서는 현재 CLI 배포 흐름 우선 정리하고 컨테이너 항목은 v0.2.0에서 완료.
- [ ] 3주차: CI/빌드/테스트 매트릭스 출시 기준 안정화
- [ ] 4주차: README 릴리즈 노트/문서 정비

### 구현 예시
- CLI 정의 위치: `crates/polytunnel/src/cli.rs`의 서브커맨드/옵션 정의.
- 실행 분기 위치: `crates/polytunnel/src/main.rs`에서 `commands::*`로 실행 라우팅.
- MVP 핵심 흐름:
  - `pt init`: 템플릿 기반 프로젝트 설정 생성 + Java 버전 확인.
  - `pt build`: 의존성 해결 → 컴파일 → 테스트 실행 경로.
  - `pt test`: 패턴/`--fail-fast`를 사용하는 테스트 전용 실행.
- UX:
  - 명령별 도움말 정돈.
  - Java 툴 누락 시 바로잡을 수 있는 액션 제안 메시지 출력.

## 마일스톤: v0.1.1 (1~4주차) — 신뢰성 및 테스트 강화
- [ ] 1주차: `build`, `test` 명령의 에러 플로우 전체 감사
- [ ] 1주차: Java 도구 미설치 시 스킵/안내 처리 보강
- [ ] 2주차: Maven 클라이언트 정상/비정상 경로 통합 테스트 확장
- [ ] 2주차: 비-2xx 응답 매핑 및 오류 계층 정합성 테스트
- [ ] 3주차: `build`/`vscode` 오류 매핑 정비 및 사용자 메시지 정리
- [ ] 3주차: 공통 실패 케이스 메시지 품질 개선
- [ ] 4주차: 제한 환경에서 CI 안정성 확보를 위한 비결정성 완화
- [ ] 4주차: 패치 범위 커버리지 확인 및 누락 구간 보완
- [ ] 의존성: 기존 CI 워크플로우, `polytunnel-build`, `polytunnel-maven`

### 구현 예시
- 에러 경로 보강 패턴:
  - 명령 모듈 단위의 에러 전파 단위 테스트.
  - 임시 워크스페이스 + 모의 스크립트 기반 통합 테스트로 실제 실패 경로 검증.
- 테스트 전략:
  - `crates/polytunnel/tests/cli_coverage_tests.rs`: 커버리지 중심 실패 경로.
  - `crates/polytunnel/tests/cli_coverage_real.rs`: Java 툴 존재 여부 가드(`java_tools_available()`).
- 하드닝 예시:
  - Java 환경 검사 실패 시 공통 래퍼 에러가 아니라 커맨드별 에러 타입 반환.

## 마일스톤: v0.1.2 (5~8주차) — CLI 완성
- [ ] 5주차: `pt add` 구현 + 테스트
- [ ] 5주차: 중복/잘못된 좌표 입력 검증 강화
- [ ] 6주차: `pt remove` 구현 + 테스트
- [ ] 6주차: 부분 실패 대비 롤백 동작 보강
- [ ] 7주차: `pt sync` 구현 + 정상/오류 동작 확인
- [ ] 7주차: 동기화 불완전 상태 검증 테스트
- [ ] 8주차: `pt tree` 구현 및 출력 포맷 정립
- [ ] 8주차: `pt run` 구현으로 실행 엔트리 포인트 실행 지원
- [ ] 8주차: `pt <-> gradlew` 마이그레이션 시나리오 도입(호환 모드, 명령/옵션 매핑 명시)
- [ ] 8주차: 사용법/종료 코드 문서 정비
- [ ] 의존성: Resolver + Maven client 계약, CLI 파싱 구조

### 구현 예시
- `pt add`:
  - 좌표 파싱 후 `polytunnel.toml`의 dependency 항목 반영.
- `pt remove`:
  - 의존성 항목 삭제 후 불필요한 섹션 정리.
- `pt sync`:
  - 현재 의존성 그래프 기준으로 재해결 후 메타데이터 갱신.
- `pt tree`:
  - 트리 형태 출력에서 scope(`compile`/`test`/`runtime`) 레벨 시각화.
- `pt run`:
  - entry point 후보 추정 후 JVM으로 컴파일 산출물을 실행.
- `pt` <-> `gradlew` 마이그레이션:
  - 대표 명령(`build`, `test`, `run`)에 대한 매핑을 명시하고 호환성 매트릭스 작성.
  - `gradlew` -> `pt`, `pt` -> `gradlew` 양방향 전환 체크리스트를 문서와 스크립트로 제공.
- 품질 기준:
  - 각 명령에 정/역방향(성공/실패) 테스트 추가 및 종료코드 검증.

## 마일스톤: v0.2.0 (9~12주차) — 워크플로우 개선
- [ ] 9주차: 공통 실패 케이스 진단 메시지 및 로그 정비
- [ ] 9주차: 개발자용 디버깅 힌트 강화
- [ ] 9주차: 컨테이너 패키지 아키텍처 설계(`Dockerfile`, 베이스 이미지, non-root 실행, TZ/JAVA_HOME/캐시 경로 정책)
- [ ] 10주차: `CHANGELOG` 골격 및 릴리스 노트 템플릿 정비
- [ ] 10주차: 업그레이드 가이드 초안 작성
- [ ] 10주차: CI 기반 다중 아키텍처 이미지 빌드/푸시(`linux/amd64`, `linux/arm64`) 및 태그 전략 정립
- [ ] 11주차: 재현성 검사 및 캐시 무효화 진단 강화
- [ ] 11주차: 신규 기여자 환경에서 온보딩 검증
- [ ] 11주차: 컨테이너 런타임 Smoke 테스트(`docker run --rm <image> --help`, 버전 출력, 샘플 빌드) 추가
- [ ] 12주차: README_KR/README에 `docker run` 사용법 및 마운트 예시 반영
- [ ] 12주차: 문서 동기화 최종 점검(README/ROADMAP/기능 문서)
- [ ] 12주차: 출시 준비 체크리스트 최종 승인
- [ ] 기존 기여자 문서/CI 파이프라인 연계

### 구현 예시
- 로컬 디버깅:
  - `--verbose`에서 해석/다운로드/컴파일 경로와 환경 변수 로그 출력.
- 컨테이너 패키징:
  - 멀티스테이지 또는 경량 런타임 기반 Dockerfile로 크기와 재현성 균형 확보.
  - 컨테이너는 non-root로 실행하고, `./workspace:/workspace` 볼륨 마운트 패턴을 문서화.
  - CI에서 이미지 빌드/푸시 실패, 기동 오류를 빠르게 검출하는 Smoke test를 고정 배포.
- 운영 문서:
  - 상위 5개 실패 시나리오와 복구 절차를 포함한 트러블슈팅 문서.
- 릴리즈 점검:
  - 재현성 스크립트: `git clone` → `cargo build` → `pt init` → `pt build`.

## 이번 주기 제외 항목
- IDE 플러그인 완전 통합
- 현재 빌드/테스트 플로우를 넘어선 대규모 캐시 아키텍처
- 엔터프라이즈 레벨 정책 엔진
