#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

SESSION_UUID_FROM_LIST="$(list_and_pick_uuid "session list" "$RAC_LITE_BIN" session list "$ADDR" --cluster "$CLUSTER_UUID")"
if [[ -z "$SESSION_UUID" ]]; then
  SESSION_UUID="$SESSION_UUID_FROM_LIST"
fi
if [[ -n "$SESSION_UUID" ]]; then
  run_cmd "session info" "$RAC_LITE_BIN" session info "$ADDR" --cluster "$CLUSTER_UUID" --session "$SESSION_UUID"
else
  echo "session_uuid not found; skipping session info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All session commands completed successfully."
