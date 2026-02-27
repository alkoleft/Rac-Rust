#!/usr/bin/env bash
set -u

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=_common.sh
source "$DIR/_common.sh"

load_env
ensure_rac_lite

echo "RAC endpoint: $ADDR"

run_cmd "agent version" "$RAC_LITE_BIN" agent version "$ADDR"

args=(agent admin list "$ADDR")
if [[ -n "$AGENT_USER" ]]; then
  args+=(--agent-user "$AGENT_USER")
fi
if [[ -n "$AGENT_PWD" ]]; then
  args+=(--agent-pwd "$AGENT_PWD")
fi
run_cmd "agent admin list" "$RAC_LITE_BIN" "${args[@]}"

if [[ $SMOKE_FAILURES -ne 0 ]]; then
  echo "Completed with $SMOKE_FAILURES failure(s)."
  exit 1
fi

echo "All agent commands completed successfully."
