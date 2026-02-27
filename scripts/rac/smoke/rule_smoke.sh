#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid
ensure_server_uuid

args=(rule list "$ADDR" --cluster "$CLUSTER_UUID" --server "$SERVER_UUID")
if [[ -n "$CLUSTER_USER" ]]; then
  args+=(--cluster-user "$CLUSTER_USER")
fi
if [[ -n "$CLUSTER_PWD" ]]; then
  args+=(--cluster-pwd "$CLUSTER_PWD")
fi
RULE_UUID_FROM_LIST="$(list_and_pick_uuid "rule list" "$RAC_LITE_BIN" "${args[@]}")"
if [[ -z "$RULE_UUID" ]]; then
  RULE_UUID="$RULE_UUID_FROM_LIST"
fi
if [[ -n "$RULE_UUID" ]]; then
  args=(rule info "$ADDR" --cluster "$CLUSTER_UUID" --server "$SERVER_UUID" --rule "$RULE_UUID")
  if [[ -n "$CLUSTER_USER" ]]; then
    args+=(--cluster-user "$CLUSTER_USER")
  fi
  if [[ -n "$CLUSTER_PWD" ]]; then
    args+=(--cluster-pwd "$CLUSTER_PWD")
  fi
  run_cmd "rule info" "$RAC_LITE_BIN" "${args[@]}"
else
  echo "rule uuid not found; skipping rule info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All rule commands completed successfully."
