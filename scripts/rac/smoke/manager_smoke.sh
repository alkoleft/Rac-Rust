#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

MANAGER_UUID_FROM_LIST="$(list_and_pick_uuid "manager list" "$RAC_LITE_BIN" manager list "$ADDR" --cluster "$CLUSTER_UUID")"
if [[ -z "$MANAGER_UUID" ]]; then
  MANAGER_UUID="$MANAGER_UUID_FROM_LIST"
fi
if [[ -n "$MANAGER_UUID" ]]; then
  run_cmd "manager info" "$RAC_LITE_BIN" manager info "$ADDR" --cluster "$CLUSTER_UUID" --manager "$MANAGER_UUID"
else
  echo "manager_uuid not found; skipping manager info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All manager commands completed successfully."
