#!/usr/bin/env bash
set -euo pipefail

RAC_BIN="${1:-/opt/1cv8/x86_64/8.5.1.1150/rac}"

if [[ ! -f "$RAC_BIN" ]]; then
  echo "rac binary not found: $RAC_BIN" >&2
  exit 1
fi

echo "== file =="
file "$RAC_BIN"
ls -lh "$RAC_BIN"

echo
echo "== build id and debuglink =="
readelf -n "$RAC_BIN" | sed -n '1,120p'
readelf -x .gnu_debuglink "$RAC_BIN" 2>/dev/null || true

echo
echo "== linked libs =="
ldd "$RAC_BIN" || true

echo
echo "== modes from resources =="
strings -a -n 4 "$RAC_BIN" | rg '^IDS_MODE_DESCR_' | sort -u

echo
echo "== commands from resources =="
strings -a -n 4 "$RAC_BIN" | rg '^IDS_CMD_DESCR_' | sort -u

echo
echo "== options count from resources =="
strings -a -n 4 "$RAC_BIN" | rg '^IDS_OPT_DESCR_' | wc -l

echo
echo "== help-supported modes =="
"$RAC_BIN" --help | sed -n '/Supported modes:/,$p'
