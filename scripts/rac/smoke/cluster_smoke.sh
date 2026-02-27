#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"

CLUSTER_UUID_FROM_LIST="$(list_and_pick_uuid "cluster list" "$RAC_LITE_BIN" cluster list "$ADDR")"
if [[ -z "$CLUSTER_UUID" ]]; then
  CLUSTER_UUID="$CLUSTER_UUID_FROM_LIST"
fi
if [[ -z "$CLUSTER_UUID" ]]; then
  echo "cluster_uuid is required (set RAC_CLUSTER_UUID or add cluster_uuid=... to $ENV_FILE)"
  exit 2
fi

run_cmd "cluster info" "$RAC_LITE_BIN" cluster info "$ADDR" --cluster "$CLUSTER_UUID"

args=(cluster admin list "$ADDR" --cluster "$CLUSTER_UUID")
if [[ -n "$CLUSTER_USER" ]]; then
  args+=(--cluster-user "$CLUSTER_USER")
fi
if [[ -n "$CLUSTER_PWD" ]]; then
  args+=(--cluster-pwd "$CLUSTER_PWD")
fi
run_cmd "cluster admin list" "$RAC_LITE_BIN" "${args[@]}"

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All cluster commands completed successfully."
