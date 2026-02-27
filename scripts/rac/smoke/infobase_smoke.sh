#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

INFOBASE_UUID_FROM_LIST="$(list_and_pick_uuid "infobase summary-list" "$RAC_LITE_BIN" infobase summary-list "$ADDR" --cluster "$CLUSTER_UUID")"
if [[ -z "$INFOBASE_UUID" ]]; then
  INFOBASE_UUID="$INFOBASE_UUID_FROM_LIST"
fi
if [[ -n "$INFOBASE_UUID" ]]; then
  run_cmd "infobase summary-info" "$RAC_LITE_BIN" infobase summary-info "$ADDR" --cluster "$CLUSTER_UUID" --infobase "$INFOBASE_UUID"
  run_cmd "infobase info" "$RAC_LITE_BIN" infobase info "$ADDR" --cluster "$CLUSTER_UUID" --infobase "$INFOBASE_UUID"
else
  echo "infobase_uuid not found; skipping infobase info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All infobase commands completed successfully."
