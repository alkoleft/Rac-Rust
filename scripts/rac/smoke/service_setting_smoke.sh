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

args=(service-setting list "$ADDR" --cluster "$CLUSTER_UUID" --server "$SERVER_UUID")
if [[ -n "$CLUSTER_USER" ]]; then
  args+=(--cluster-user "$CLUSTER_USER")
fi
if [[ -n "$CLUSTER_PWD" ]]; then
  args+=(--cluster-pwd "$CLUSTER_PWD")
fi
SETTING_UUID_FROM_LIST="$(list_and_pick_uuid "service-setting list" "$RAC_LITE_BIN" "${args[@]}")"
if [[ -z "$SETTING_UUID" ]]; then
  SETTING_UUID="$SETTING_UUID_FROM_LIST"
fi
if [[ -n "$SETTING_UUID" ]]; then
  args=(service-setting info "$ADDR" --cluster "$CLUSTER_UUID" --server "$SERVER_UUID" --setting "$SETTING_UUID")
  if [[ -n "$CLUSTER_USER" ]]; then
    args+=(--cluster-user "$CLUSTER_USER")
  fi
  if [[ -n "$CLUSTER_PWD" ]]; then
    args+=(--cluster-pwd "$CLUSTER_PWD")
  fi
  run_cmd "service-setting info" "$RAC_LITE_BIN" "${args[@]}"
else
  echo "service setting uuid not found; skipping service-setting info"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All service-setting commands completed successfully."
