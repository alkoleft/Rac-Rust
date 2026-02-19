#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <capture_name> <rac args...>"
  echo "Example: $0 cluster_list cluster list"
  exit 2
fi

NAME="$1"
shift

RAC_BIN="${RAC_BIN:-/opt/1cv8/x86_64/8.5.1.1150/rac}"
LISTEN_ADDR="${LISTEN_ADDR:-127.0.0.1:1565}"
TARGET_ADDR="${TARGET_ADDR:-127.0.0.1:1545}"
LOG_DIR="${LOG_DIR:-./logs}"
RAC_TIMEOUT_SEC="${RAC_TIMEOUT_SEC:-15}"
PROXY_BIN="${PROXY_BIN:-./target/release/v8_proxy}"

PROXY_LOG="/tmp/v8_capture_${NAME}.log"
RAC_OUT="/tmp/rac_${NAME}.out"
RAC_ERR="/tmp/rac_${NAME}.err"

before_file="$(mktemp)"
after_file="$(mktemp)"
proxy_pid=""

remove_temp_files() {
  rm -f "$before_file" "$after_file"
}

finish() {
  cleanup
  remove_temp_files
}

ls -1 "$LOG_DIR" 2>/dev/null | sort >"$before_file" || true

"$PROXY_BIN" \
  --listen "$LISTEN_ADDR" \
  --target "$TARGET_ADDR" \
  --log-dir "$LOG_DIR" >"$PROXY_LOG" 2>&1 &
proxy_pid=$!

cleanup() {
  if kill -0 "$proxy_pid" 2>/dev/null; then
    kill "$proxy_pid" 2>/dev/null || true
    wait "$proxy_pid" 2>/dev/null || true
  fi
}
trap finish EXIT

sleep 0.25

if ! kill -0 "$proxy_pid" 2>/dev/null; then
  echo "proxy failed to start; see $PROXY_LOG" >&2
  if [[ -f "$PROXY_LOG" ]]; then
    tail -n 20 "$PROXY_LOG" >&2 || true
  fi
  exit 1
fi

set +e
timeout "$RAC_TIMEOUT_SEC" "$RAC_BIN" "$@" "$LISTEN_ADDR" >"$RAC_OUT" 2>"$RAC_ERR"
rac_exit=$?
set -e

cleanup

ls -1 "$LOG_DIR" 2>/dev/null | sort >"$after_file" || true
session_dir="$(comm -13 "$before_file" "$after_file" | tail -n 1)"
trap - EXIT

echo "capture_name=$NAME"
echo "session_dir=$session_dir"
echo "rac_exit=$rac_exit"
echo "proxy_log=$PROXY_LOG"
echo "rac_out=$RAC_OUT"
echo "rac_err=$RAC_ERR"
