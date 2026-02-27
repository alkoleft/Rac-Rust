#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

PROCESS_UUID_FROM_LIST="$(list_and_pick_uuid "process list" "$RAC_LITE_BIN" process list "$ADDR" --cluster "$CLUSTER_UUID")"
if [[ -z "$PROCESS_UUID" ]]; then
  PROCESS_UUID="$PROCESS_UUID_FROM_LIST"
fi
if [[ -n "$PROCESS_UUID" ]]; then
  run_cmd "process info" "$RAC_LITE_BIN" process info "$ADDR" --cluster "$CLUSTER_UUID" --process "$PROCESS_UUID"
else
  echo "process_uuid not found; skipping process info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All process commands completed successfully."
