#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

CONNECTION_UUID_FROM_LIST="$(list_and_pick_uuid "connection list" "$RAC_LITE_BIN" connection list "$ADDR" --cluster "$CLUSTER_UUID")"
if [[ -z "$CONNECTION_UUID" ]]; then
  CONNECTION_UUID="$CONNECTION_UUID_FROM_LIST"
fi
if [[ -n "$CONNECTION_UUID" ]]; then
  run_cmd "connection info" "$RAC_LITE_BIN" connection info "$ADDR" --cluster "$CLUSTER_UUID" --connection "$CONNECTION_UUID"
else
  echo "connection_uuid not found; skipping connection info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All connection commands completed successfully."
