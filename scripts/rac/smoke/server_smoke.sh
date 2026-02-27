#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

SERVER_UUID_FROM_LIST="$(list_and_pick_uuid "server list" "$RAC_LITE_BIN" server list "$ADDR" --cluster "$CLUSTER_UUID")"
if [[ -z "$SERVER_UUID" ]]; then
  SERVER_UUID="$SERVER_UUID_FROM_LIST"
fi
if [[ -n "$SERVER_UUID" ]]; then
  run_cmd "server info" "$RAC_LITE_BIN" server info "$ADDR" --cluster "$CLUSTER_UUID" --server "$SERVER_UUID"
else
  echo "server_uuid not found; skipping server info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All server commands completed successfully."
