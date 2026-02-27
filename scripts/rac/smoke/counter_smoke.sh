#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

COUNTER_NAME_FROM_LIST="$(list_and_pick_field "counter list" "name" "$RAC_LITE_BIN" counter list "$ADDR" --cluster "$CLUSTER_UUID")"
if [[ -z "$COUNTER_NAME" ]]; then
  COUNTER_NAME="$COUNTER_NAME_FROM_LIST"
fi
if [[ -n "$COUNTER_NAME" ]]; then
  run_cmd "counter info" "$RAC_LITE_BIN" counter info "$ADDR" --cluster "$CLUSTER_UUID" --counter "$COUNTER_NAME"
else
  echo "counter name not found; skipping counter info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All counter commands completed successfully."
