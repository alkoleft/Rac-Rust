#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

run_cmd "profile list" "$RAC_LITE_BIN" profile list "$ADDR" --cluster "$CLUSTER_UUID"

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All profile commands completed successfully."
