#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"
ensure_cluster_uuid

if [[ -z "$COUNTER_NAME" ]]; then
  COUNTER_NAME="codex-smoke-counter-$(date +%Y%m%d%H%M%S)-$$"
fi

create_args=(counter update "$ADDR" --cluster "$CLUSTER_UUID" --name "$COUNTER_NAME")
if [[ -n "$CLUSTER_USER" ]]; then
  create_args+=(--cluster-user "$CLUSTER_USER")
fi
if [[ -n "$CLUSTER_PWD" ]]; then
  create_args+=(--cluster-pwd "$CLUSTER_PWD")
fi
create_args+=(
  --collection-time 60
  --group users
  --filter-type all
  --filter ""
  --duration analyze
  --cpu-time not-analyze
  --duration-dbms not-analyze
  --service not-analyze
  --memory not-analyze
  --read not-analyze
  --write not-analyze
  --dbms-bytes not-analyze
  --call not-analyze
  --number-of-active-sessions not-analyze
  --number-of-sessions not-analyze
  --descr "codex smoke counter"
)
before_failures=$SMOKE_FAILURES
run_cmd "counter update" "$RAC_LITE_BIN" "${create_args[@]}"

run_cmd "counter list" "$RAC_LITE_BIN" counter list "$ADDR" --cluster "$CLUSTER_UUID"
if [[ $SMOKE_FAILURES -eq $before_failures ]]; then
  run_cmd "counter info" "$RAC_LITE_BIN" counter info "$ADDR" --cluster "$CLUSTER_UUID" --counter "$COUNTER_NAME"
else
  echo "counter update failed; skipping counter info"
  echo
fi

remove_args=(counter remove "$ADDR" --cluster "$CLUSTER_UUID" --name "$COUNTER_NAME")
if [[ -n "$CLUSTER_USER" ]]; then
  remove_args+=(--cluster-user "$CLUSTER_USER")
fi
if [[ -n "$CLUSTER_PWD" ]]; then
  remove_args+=(--cluster-pwd "$CLUSTER_PWD")
fi
if [[ $SMOKE_FAILURES -eq $before_failures ]]; then
  run_cmd "counter remove" "$RAC_LITE_BIN" "${remove_args[@]}"
else
  echo "counter update failed; skipping counter remove"
  echo
fi

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All counter commands completed successfully."
