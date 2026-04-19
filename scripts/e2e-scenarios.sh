#!/usr/bin/env bash
# e2e-scenarios.sh — pt CLI end-to-end 시나리오 (사람이 직접 실행 가능)
# Usage: bash scripts/e2e-scenarios.sh [smoke|dependency|lifecycle|all]
#
# 환경변수:
#   PT_BIN  — 사용할 pt 바이너리 경로 (기본: $REPO_ROOT/target/debug/pt)
#
# 각 시나리오는 mktemp -d 로 격리된 디렉토리에서 실행된다.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PT_BIN="${PT_BIN:-$REPO_ROOT/target/debug/pt}"
SELECT="${1:-all}"

# --- Color helpers (test-examples.sh 스타일) ---
if [ -t 1 ] && command -v tput >/dev/null 2>&1 && tput colors >/dev/null 2>&1; then
    GREEN=$(tput setaf 2); RED=$(tput setaf 1)
    YELLOW=$(tput setaf 3); BOLD=$(tput bold); RESET=$(tput sgr0)
else
    GREEN=''; RED=''; YELLOW=''; BOLD=''; RESET=''
fi

info() { printf "%s%s%s\n" "$YELLOW" "$1" "$RESET"; }
ok()   { printf "  %s✓%s %s\n" "$GREEN" "$RESET" "$1"; }
oops() { printf "  %s✗%s %s\n" "$RED" "$RESET" "$1" >&2; }

LAST_OUT=""
# step <description> <cmd> [args...]
# 명령어 실행 후 성공 시 ✓, 실패 시 ✗ + 출력 덤프 + exit.
step() {
    local desc="$1"; shift
    if LAST_OUT="$("$@" 2>&1)"; then
        ok "$desc"
    else
        oops "$desc"
        printf "%s\n" "$LAST_OUT" >&2
        exit 1
    fi
}
# expect <needle>
# 직전 step 의 출력에서 needle 이 나오는지 검증.
expect() {
    local needle="$1"
    if ! printf '%s' "$LAST_OUT" | grep -q -- "$needle"; then
        oops "expected '$needle' in last output:"
        printf "%s\n" "$LAST_OUT" >&2
        exit 1
    fi
}

# --- pt 바이너리 확인 ---
if [[ ! -x "$PT_BIN" ]]; then
    info "Building pt binary (not found at $PT_BIN)..."
    cargo build --manifest-path "$REPO_ROOT/Cargo.toml" -p polytunnel
fi

if [[ ! -x "$PT_BIN" ]]; then
    oops "pt binary still missing after build: $PT_BIN"
    exit 1
fi

# --- 시나리오들 ---

scenario_smoke() {
    info "=== smoke: pt init → build → run ==="
    local d; d="$(mktemp -d)"
    (
        cd "$d"
        step "pt init demo"       "$PT_BIN" init demo
        expect "Created"

        mkdir -p src/main/java/com/example
        cat > src/main/java/com/example/Hello.java <<'JAVA'
package com.example;

public class Hello {
    public static void main(String[] args) {
        System.out.println("hello from e2e");
    }
}
JAVA

        step "pt build --skip-tests" "$PT_BIN" build --skip-tests
        expect "BUILD SUCCESSFUL"

        step "pt run com.example.Hello" "$PT_BIN" run com.example.Hello
        expect "hello from e2e"
    )
    rm -rf "$d"
}

scenario_dependency() {
    info "=== dependency: pt init → add → sync → tree (Maven Central 접근 필요) ==="
    local d; d="$(mktemp -d)"
    (
        cd "$d"
        step "pt init demo" "$PT_BIN" init demo

        step "pt add guava" "$PT_BIN" add "com.google.guava:guava:33.0.0-jre"
        expect "Added"

        step "pt sync"      "$PT_BIN" sync
        expect "Synced"

        step "pt tree"      "$PT_BIN" tree
        expect "guava"
    )
    rm -rf "$d"
}

scenario_lifecycle() {
    info "=== lifecycle: pt init → add → remove (네트워크 불필요) ==="
    local d; d="$(mktemp -d)"
    (
        cd "$d"
        step "pt init demo" "$PT_BIN" init demo

        step "pt add junit (scope=test)" "$PT_BIN" add "junit:junit:4.13.2" --scope test
        expect "Added"

        step "pt remove junit" "$PT_BIN" remove "junit:junit"
        expect "Removed"
    )
    rm -rf "$d"
}

# --- 디스패치 ---
case "$SELECT" in
    smoke)      scenario_smoke ;;
    dependency) scenario_dependency ;;
    lifecycle)  scenario_lifecycle ;;
    all)        scenario_smoke; scenario_dependency; scenario_lifecycle ;;
    *)
        oops "Unknown scenario: $SELECT"
        printf "Usage: %s [smoke|dependency|lifecycle|all]\n" "$0" >&2
        exit 2
        ;;
esac

printf "\n%s%sAll selected scenarios passed%s\n" "$GREEN" "$BOLD" "$RESET"
