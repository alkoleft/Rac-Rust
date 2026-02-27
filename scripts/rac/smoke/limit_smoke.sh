#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

LIMIT_NAME_FROM_LIST="$(list_and_pick_field "limit list" "name" "$RAC_LITE_BIN" limit list "$ADDR" --cluster "$CLUSTER_UUID")"
if [[ -z "$LIMIT_NAME" ]]; then
  LIMIT_NAME="$LIMIT_NAME_FROM_LIST"
fi
if [[ -n "$LIMIT_NAME" ]]; then
  run_cmd "limit info" "$RAC_LITE_BIN" limit info "$ADDR" --cluster "$CLUSTER_UUID" --limit "$LIMIT_NAME"
else
  echo "limit name not found; skipping limit info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All limit commands completed successfully."
