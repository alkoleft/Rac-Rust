#!/usr/bin/env bash
set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
ENV_FILE="${RAC_ENV_FILE:-$ROOT/scripts/rac/smoke/.env}"

trim_val() {
  local v="$1"
  v="${v//$'\r'/}"
  v="${v#"${v%%[![:space:]]*}"}"
  v="${v%"${v##*[![:space:]]}"}"
  printf '%s' "$v"
}

load_env() {
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
        server_uuid) RAC_SERVER_UUID="${RAC_SERVER_UUID:-$val}" ;;
        manager_uuid) RAC_MANAGER_UUID="${RAC_MANAGER_UUID:-$val}" ;;
        process_uuid) RAC_PROCESS_UUID="${RAC_PROCESS_UUID:-$val}" ;;
        infobase_uuid) RAC_INFOBASE_UUID="${RAC_INFOBASE_UUID:-$val}" ;;
        connection_uuid) RAC_CONNECTION_UUID="${RAC_CONNECTION_UUID:-$val}" ;;
        session_uuid) RAC_SESSION_UUID="${RAC_SESSION_UUID:-$val}" ;;
        rule_uuid) RAC_RULE_UUID="${RAC_RULE_UUID:-$val}" ;;
        setting_uuid) RAC_SETTING_UUID="${RAC_SETTING_UUID:-$val}" ;;
        counter_name) RAC_COUNTER_NAME="${RAC_COUNTER_NAME:-$val}" ;;
        limit_name) RAC_LIMIT_NAME="${RAC_LIMIT_NAME:-$val}" ;;
        agent_user) RAC_AGENT_USER="${RAC_AGENT_USER:-$val}" ;;
        agent_pwd) RAC_AGENT_PWD="${RAC_AGENT_PWD:-$val}" ;;
      esac
    done < "$ENV_FILE"
  fi

  ADDR="$(trim_val "${RAC_ENDPOINT:-localhost:1545}")"
  CLUSTER_UUID="$(trim_val "${RAC_CLUSTER_UUID:-}")"
  CLUSTER_USER="$(trim_val "${RAC_CLUSTER_USER:-}")"
  CLUSTER_PWD="$(trim_val "${RAC_CLUSTER_PWD:-}")"
  SERVER_UUID="$(trim_val "${RAC_SERVER_UUID:-}")"
  MANAGER_UUID="$(trim_val "${RAC_MANAGER_UUID:-}")"
  PROCESS_UUID="$(trim_val "${RAC_PROCESS_UUID:-}")"
  INFOBASE_UUID="$(trim_val "${RAC_INFOBASE_UUID:-}")"
  CONNECTION_UUID="$(trim_val "${RAC_CONNECTION_UUID:-}")"
  SESSION_UUID="$(trim_val "${RAC_SESSION_UUID:-}")"
  RULE_UUID="$(trim_val "${RAC_RULE_UUID:-}")"
  SETTING_UUID="$(trim_val "${RAC_SETTING_UUID:-}")"
  COUNTER_NAME="$(trim_val "${RAC_COUNTER_NAME:-}")"
  LIMIT_NAME="$(trim_val "${RAC_LIMIT_NAME:-}")"
  AGENT_USER="$(trim_val "${RAC_AGENT_USER:-}")"
  AGENT_PWD="$(trim_val "${RAC_AGENT_PWD:-}")"
}

RAC_LITE_BIN="${RAC_LITE_BIN:-$ROOT/target/debug/rac_lite}"

ensure_rac_lite() {
  if [[ ! -x "$RAC_LITE_BIN" ]]; then
    echo "Building rac_lite..."
    cargo build -p rac_cli --bin rac_lite
  fi
}

SMOKE_FAILURES=0
run_cmd() {
  local label="$1"
  shift
  echo "==> $label"
  echo "+ $*"
  "$@"
  local status=$?
  if [[ $status -ne 0 ]]; then
    echo "Command failed with exit code $status"
    SMOKE_FAILURES=$((SMOKE_FAILURES + 1))
  fi
  echo
  return 0
}

first_uuid_from_file() {
  local file="$1"
  if command -v rg >/dev/null 2>&1; then
    rg -o -m1 '[0-9a-fA-F-]{36}' "$file" || true
  else
    grep -Eo -m1 '[0-9a-fA-F-]{36}' "$file" || true
  fi
}

first_field_value() {
  local key="$1"
  local file="$2"
  local line=""
  if command -v rg >/dev/null 2>&1; then
    line="$(rg -m1 "^${key}[[:space:]]*:" "$file" || true)"
  else
    line="$(grep -E -m1 "^${key}[[:space:]]*:" "$file" || true)"
  fi
  if [[ -z "$line" ]]; then
    return 0
  fi
  echo "$line" | sed -E 's/^[^:]+:[[:space:]]*//'
}

list_and_pick_uuid() {
  local label="$1"
  shift
  local tmp
  tmp="$(mktemp)"
  echo "==> $label"
  echo "+ $*"
  "$@" 2>&1 | tee "$tmp"
  local status=${PIPESTATUS[0]}
  if [[ $status -ne 0 ]]; then
    echo "Command failed with exit code $status"
    SMOKE_FAILURES=$((SMOKE_FAILURES + 1))
  fi
  local uuid
  uuid="$(first_uuid_from_file "$tmp")"
  rm -f "$tmp"
  echo "$uuid"
  echo
}

list_and_pick_field() {
  local label="$1"
  local key="$2"
  shift 2
  local tmp
  tmp="$(mktemp)"
  echo "==> $label"
  echo "+ $*"
  "$@" 2>&1 | tee "$tmp"
  local status=${PIPESTATUS[0]}
  if [[ $status -ne 0 ]]; then
    echo "Command failed with exit code $status"
    SMOKE_FAILURES=$((SMOKE_FAILURES + 1))
  fi
  local value
  value="$(first_field_value "$key" "$tmp")"
  rm -f "$tmp"
  echo "$value"
  echo
}

ensure_cluster_uuid() {
  if [[ -z "$CLUSTER_UUID" ]]; then
    CLUSTER_UUID="$(list_and_pick_uuid "cluster list" "$RAC_LITE_BIN" cluster list "$ADDR")"
  else
    run_cmd "cluster list" "$RAC_LITE_BIN" cluster list "$ADDR"
  fi
  if [[ -z "$CLUSTER_UUID" ]]; then
    echo "cluster_uuid is required (set RAC_CLUSTER_UUID or add cluster_uuid=... to $ENV_FILE)"
    exit 2
  fi
}

ensure_server_uuid() {
  if [[ -z "$SERVER_UUID" ]]; then
    SERVER_UUID="$(list_and_pick_uuid "server list" "$RAC_LITE_BIN" server list "$ADDR" --cluster "$CLUSTER_UUID")"
  fi
  if [[ -z "$SERVER_UUID" ]]; then
    echo "server_uuid is required (set RAC_SERVER_UUID or add server_uuid=... to $ENV_FILE)"
    exit 2
  fi
}

ensure_manager_uuid() {
  if [[ -z "$MANAGER_UUID" ]]; then
    MANAGER_UUID="$(list_and_pick_uuid "manager list" "$RAC_LITE_BIN" manager list "$ADDR" --cluster "$CLUSTER_UUID")"
  fi
  if [[ -z "$MANAGER_UUID" ]]; then
    echo "manager_uuid is required (set RAC_MANAGER_UUID or add manager_uuid=... to $ENV_FILE)"
    exit 2
  fi
}

ensure_process_uuid() {
  if [[ -z "$PROCESS_UUID" ]]; then
    PROCESS_UUID="$(list_and_pick_uuid "process list" "$RAC_LITE_BIN" process list "$ADDR" --cluster "$CLUSTER_UUID")"
  fi
  if [[ -z "$PROCESS_UUID" ]]; then
    echo "process_uuid is required (set RAC_PROCESS_UUID or add process_uuid=... to $ENV_FILE)"
    exit 2
  fi
}

ensure_infobase_uuid() {
  if [[ -z "$INFOBASE_UUID" ]]; then
    INFOBASE_UUID="$(list_and_pick_uuid "infobase summary list" "$RAC_LITE_BIN" infobase summary list "$ADDR" --cluster "$CLUSTER_UUID")"
  fi
  if [[ -z "$INFOBASE_UUID" ]]; then
    echo "infobase_uuid is required (set RAC_INFOBASE_UUID or add infobase_uuid=... to $ENV_FILE)"
    exit 2
  fi
}

ensure_connection_uuid() {
  if [[ -z "$CONNECTION_UUID" ]]; then
    CONNECTION_UUID="$(list_and_pick_uuid "connection list" "$RAC_LITE_BIN" connection list "$ADDR" --cluster "$CLUSTER_UUID")"
  fi
  if [[ -z "$CONNECTION_UUID" ]]; then
    echo "connection_uuid is required (set RAC_CONNECTION_UUID or add connection_uuid=... to $ENV_FILE)"
    exit 2
  fi
}

ensure_session_uuid() {
  if [[ -z "$SESSION_UUID" ]]; then
    SESSION_UUID="$(list_and_pick_uuid "session list" "$RAC_LITE_BIN" session list "$ADDR" --cluster "$CLUSTER_UUID")"
  fi
  if [[ -z "$SESSION_UUID" ]]; then
    echo "session_uuid is required (set RAC_SESSION_UUID or add session_uuid=... to $ENV_FILE)"
    exit 2
  fi
}

ensure_rule_uuid() {
  if [[ -z "$RULE_UUID" ]]; then
    RULE_UUID="$(list_and_pick_uuid "rule list" "$RAC_LITE_BIN" rule list "$ADDR" --cluster "$CLUSTER_UUID" --server "$SERVER_UUID")"
  fi
}

ensure_setting_uuid() {
  if [[ -z "$SETTING_UUID" ]]; then
    SETTING_UUID="$(list_and_pick_uuid "service-setting list" "$RAC_LITE_BIN" service-setting list "$ADDR" --cluster "$CLUSTER_UUID" --server "$SERVER_UUID")"
  fi
}

ensure_counter_name() {
  if [[ -z "$COUNTER_NAME" ]]; then
    COUNTER_NAME="$(list_and_pick_field "counter list" "name" "$RAC_LITE_BIN" counter list "$ADDR" --cluster "$CLUSTER_UUID")"
  fi
}

ensure_limit_name() {
  if [[ -z "$LIMIT_NAME" ]]; then
    LIMIT_NAME="$(list_and_pick_field "limit list" "name" "$RAC_LITE_BIN" limit list "$ADDR" --cluster "$CLUSTER_UUID")"
  fi
}
