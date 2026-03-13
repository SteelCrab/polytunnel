#!/usr/bin/env bash
# test-examples.sh — Build and test all example projects
# Usage: ./scripts/test-examples.sh [--skip-build] [--skip-tests]

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PT_BIN="$REPO_ROOT/target/debug/pt"
LOG_DIR="$REPO_ROOT/logs/examples"

SKIP_BUILD=false
SKIP_TESTS=false

for arg in "$@"; do
  case "$arg" in
    --skip-build) SKIP_BUILD=true ;;
    --skip-tests) SKIP_TESTS=true ;;
    *) echo "Unknown option: $arg" >&2; exit 1 ;;
  esac
done

# --- Color helpers ---
if [ -t 1 ] && command -v tput &>/dev/null && tput colors &>/dev/null; then
  GREEN=$(tput setaf 2)
  RED=$(tput setaf 1)
  YELLOW=$(tput setaf 3)
  BOLD=$(tput bold)
  RESET=$(tput sgr0)
else
  GREEN='\033[0;32m'
  RED='\033[0;31m'
  YELLOW='\033[0;33m'
  BOLD='\033[1m'
  RESET='\033[0m'
fi

pass() { echo -e "${GREEN}${BOLD}  PASS${RESET}  $1"; }
fail() { echo -e "${RED}${BOLD}  FAIL${RESET}  $1"; }
info() { echo -e "${YELLOW}$1${RESET}"; }

# --- Build pt binary ---
if [ "$SKIP_BUILD" = false ]; then
  info "Building pt binary..."
  cargo build --workspace --manifest-path "$REPO_ROOT/Cargo.toml" 2>&1
  echo ""
fi

if [ ! -x "$PT_BIN" ]; then
  echo "Error: $PT_BIN not found. Run without --skip-build first." >&2
  exit 1
fi

# --- Discover example projects ---
mkdir -p "$LOG_DIR"

EXAMPLES=()
while IFS= read -r line; do
  EXAMPLES+=("$line")
done < <(
  find "$REPO_ROOT/examples" -name "polytunnel.toml" -maxdepth 2 |
  sed 's|/polytunnel.toml||' |
  sort
)

if [ ${#EXAMPLES[@]} -eq 0 ]; then
  echo "No example projects found in $REPO_ROOT/examples" >&2
  exit 1
fi

echo "${BOLD}Running ${#EXAMPLES[@]} example(s)...${RESET}"
echo ""

PASSED=()
FAILED=()

# --- Run each example ---
for example_dir in "${EXAMPLES[@]}"; do
  name="$(basename "$example_dir")"
  log_file="$LOG_DIR/${name}.log"

  if [ "$SKIP_TESTS" = true ]; then
    cmd=("$PT_BIN" "build" "--clean" "-v" "--skip-tests")
  else
    cmd=("$PT_BIN" "build" "--clean" "-v")
  fi

  if (cd "$example_dir" && "${cmd[@]}" >"$log_file" 2>&1); then
    pass "$name"
    PASSED+=("$name")
  else
    fail "$name  (log: $log_file)"
    FAILED+=("$name")
  fi
done

# --- Summary ---
echo ""
echo "${BOLD}Results: ${GREEN}${#PASSED[@]} passed${RESET}${BOLD}, ${RED}${#FAILED[@]} failed${RESET}"

if [ ${#FAILED[@]} -gt 0 ]; then
  echo ""
  echo "${RED}${BOLD}Failed projects:${RESET}"
  for name in "${FAILED[@]}"; do
    echo "  - $name  →  $LOG_DIR/${name}.log"
  done
  exit 1
fi
