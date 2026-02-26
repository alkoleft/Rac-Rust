#!/usr/bin/env bash
set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ENV_FILE="${RAC_ENV_FILE:-$ROOT/docs/rac/.private/rac_env.txt}"

trim_val() {
  local v="$1"
  v="${v//$'\r'/}"
  v="${v#"${v%%[![:space:]]*}"}"
  v="${v%"${v##*[![:space:]]}"}"
  printf '%s' "$v"
}

if [[ -f "$ENV_FILE" ]]; then
  while IFS= read -r line; do
    [[ -z "$line" || "$line" == \#* ]] && continue
    key="$(trim_val "${line%%=*}")"
    val="$(trim_val "${line#*=}")"
    case "$key" in
      endpoint) RAC_ENDPOINT="${RAC_ENDPOINT:-$val}" ;;
      cluster_uuid) RAC_CLUSTER_UUID="${RAC_CLUSTER_UUID:-$val}" ;;
      cluster_user) RAC_CLUSTER_USER="${RAC_CLUSTER_USER:-$val}" ;;
      cluster_pwd) RAC_CLUSTER_PWD="${RAC_CLUSTER_PWD:-$val}" ;;
    esac
  done < "$ENV_FILE"
fi

ADDR="$(trim_val "${RAC_ENDPOINT:-localhost:1545}")"
CLUSTER_UUID="$(trim_val "${RAC_CLUSTER_UUID:-}")"
CLUSTER_USER="$(trim_val "${RAC_CLUSTER_USER:-}")"
CLUSTER_PWD="$(trim_val "${RAC_CLUSTER_PWD:-}")"
ADMIN_NAME="${RAC_CLUSTER_ADMIN_NAME:-codex_test_admin}"
ADMIN_PWD="${RAC_CLUSTER_ADMIN_PWD:-codex_test_pwd}"
ADMIN_DESCR="${RAC_CLUSTER_ADMIN_DESCR:-codex test admin}"
ADMIN_AUTH="${RAC_CLUSTER_ADMIN_AUTH:-pwd}"

RAC_LITE_BIN="${RAC_LITE_BIN:-$ROOT/target/debug/rac_lite}"

if [[ ! -x "$RAC_LITE_BIN" ]]; then
  echo "Building rac_lite..."
  cargo build -p rac_cli --bin rac_lite
fi

failures=0
run_cmd() {
  local label="$1"
  shift
  echo "==> $label"
  echo "+ $*"
  "$@"
  local status=$?
  if [[ $status -ne 0 ]]; then
    echo "Command failed with exit code $status"
    failures=$((failures + 1))
  fi
  echo
  return 0
}

echo "RAC endpoint: $ADDR"

list_tmp="$(mktemp)"
echo "==> cluster list"
echo "+ $RAC_LITE_BIN cluster list $ADDR"
"$RAC_LITE_BIN" cluster list "$ADDR" 2>&1 | tee "$list_tmp"
list_status=${PIPESTATUS[0]}
if [[ $list_status -ne 0 ]]; then
  echo "Command failed with exit code $list_status"
  failures=$((failures + 1))
fi
echo

if [[ -z "$CLUSTER_UUID" ]]; then
  if command -v rg >/dev/null 2>&1; then
    CLUSTER_UUID="$(rg -o -m1 '[0-9a-fA-F-]{36}' "$list_tmp" || true)"
  else
    CLUSTER_UUID="$(grep -Eo -m1 '[0-9a-fA-F-]{36}' "$list_tmp" || true)"
  fi
fi
rm -f "$list_tmp"

if [[ -z "$CLUSTER_UUID" ]]; then
  echo "cluster_uuid is required (set RAC_CLUSTER_UUID or add cluster_uuid=... to $ENV_FILE)"
  exit 2
fi

if [[ -z "$CLUSTER_USER" || -z "$CLUSTER_PWD" ]]; then
  echo "cluster_user/cluster_pwd is required (set RAC_CLUSTER_USER/RAC_CLUSTER_PWD or update $ENV_FILE)"
  exit 2
fi

run_cmd "cluster info" "$RAC_LITE_BIN" cluster info "$ADDR" --cluster "$CLUSTER_UUID"
run_cmd "cluster admin list" "$RAC_LITE_BIN" cluster admin list "$ADDR" \
  --cluster "$CLUSTER_UUID" --cluster-user "$CLUSTER_USER" --cluster-pwd "$CLUSTER_PWD"
run_cmd "cluster admin register" "$RAC_LITE_BIN" cluster admin register "$ADDR" \
  --cluster "$CLUSTER_UUID" --cluster-user "$CLUSTER_USER" --cluster-pwd "$CLUSTER_PWD" \
  --name "$ADMIN_NAME" --pwd "$ADMIN_PWD" --descr "$ADMIN_DESCR" --auth "$ADMIN_AUTH"

if [[ $failures -ne 0 ]]; then
  echo "Completed with $failures failure(s)."
  exit 1
fi

echo "All cluster commands completed successfully."
