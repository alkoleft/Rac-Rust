#!/usr/bin/env bash
set -euo pipefail

# Build a CLI->RPC method map by capturing RAC commands through proxy and
# parsing method ids from rac_decode output.

ROOT_DIR="${ROOT_DIR:-$(pwd)}"
CAPTURE_SCRIPT="${CAPTURE_SCRIPT:-$ROOT_DIR/skills/onec-protocol-reverse/scripts/capture_rac_command.sh}"
RAC_DECODE="${RAC_DECODE:-cargo run --quiet --bin rac_decode --}"
OUT_MD="${1:-$ROOT_DIR/docs/documentation/rac_cli_method_map.generated.md}"

if [[ ! -x "$CAPTURE_SCRIPT" ]]; then
  echo "capture script not found or not executable: $CAPTURE_SCRIPT" >&2
  exit 1
fi

extract_first_uuid() {
  local file="$1"
  rg -o '[0-9a-fA-F-]{36}' "$file" | head -n 1 || true
}

extract_methods() {
  local stream="$1"
  if [[ ! -f "$stream" ]]; then
    echo ""
    return
  fi
  local out=""
  out="$(eval "$RAC_DECODE \"$stream\"" | rg -a 'rpc_method_id=' | sed -E 's/.*rpc_method_id=0x([0-9a-f]+).*/0x\1/' | paste -sd ',' - || true)"
  echo "$out"
}

run_capture() {
  local name="$1"
  shift
  local result
  result="$("$CAPTURE_SCRIPT" "$name" "$@" || true)"
  local session
  session="$(printf '%s\n' "$result" | sed -n 's/^session_dir=//p')"
  local exit_code
  exit_code="$(printf '%s\n' "$result" | sed -n 's/^rac_exit=//p')"
  echo "$result" >&2
  printf '%s|%s\n' "${session:-}" "${exit_code:-}"
}

mkdir -p "$(dirname "$OUT_MD")"

# 1) Baseline capture to get cluster UUID.
baseline_name="map_cluster_list"
baseline_info="$(run_capture "$baseline_name" cluster list)"
baseline_session="${baseline_info%%|*}"
baseline_exit="${baseline_info##*|}"
cluster_out="/tmp/rac_${baseline_name}.out"

if [[ "$baseline_exit" != "0" || -z "${baseline_session}" ]]; then
  echo "failed baseline capture cluster list (exit=$baseline_exit, session=$baseline_session)" >&2
  exit 1
fi

cluster_uuid="$(extract_first_uuid "$cluster_out")"
if [[ -z "$cluster_uuid" ]]; then
  echo "failed to extract cluster uuid from $cluster_out" >&2
  exit 1
fi

# Best-effort object UUIDs for info commands.
run_capture map_manager_list manager list --cluster "$cluster_uuid" >/dev/null || true
manager_uuid="$(extract_first_uuid /tmp/rac_map_manager_list.out)"
run_capture map_server_list server list --cluster "$cluster_uuid" >/dev/null || true
server_uuid="$(extract_first_uuid /tmp/rac_map_server_list.out)"
run_capture map_process_list process list --cluster "$cluster_uuid" >/dev/null || true
process_uuid="$(extract_first_uuid /tmp/rac_map_process_list.out)"
run_capture map_connection_list connection list --cluster "$cluster_uuid" >/dev/null || true
connection_uuid="$(extract_first_uuid /tmp/rac_map_connection_list.out)"
run_capture map_session_list session list --cluster "$cluster_uuid" >/dev/null || true
session_uuid="$(extract_first_uuid /tmp/rac_map_session_list.out)"
run_capture map_infobase_summary_list infobase summary list --cluster "$cluster_uuid" >/dev/null || true
infobase_uuid_from_summary="$(extract_first_uuid /tmp/rac_map_infobase_summary_list.out)"

session_uuid="${SESSION_UUID:-$session_uuid}"

counter_name="${COUNTER_NAME:-}"
limit_name="${LIMIT_NAME:-}"
service_setting_uuid="${SERVICE_SETTING_UUID:-}"
service_setting_server_uuid="${SERVICE_SETTING_SERVER_UUID:-}"
infobase_uuid="${INFOBASE_UUID:-${infobase_uuid_from_summary:-}}"
infobase_user="${INFOBASE_USER:-}"
infobase_pwd="${INFOBASE_PWD:-}"
binary_storage_name="${BINARY_STORAGE_NAME:-}"
binary_storage_uuid="${BINARY_STORAGE_UUID:-}"

declare -a SPECS=(
  "cluster_list|cluster list"
  "cluster_info|cluster info --cluster $cluster_uuid"
  "agent_version|agent version"
  "manager_list|manager list --cluster $cluster_uuid"
  "server_list|server list --cluster $cluster_uuid"
  "process_list|process list --cluster $cluster_uuid"
  "infobase_summary_list|infobase summary list --cluster $cluster_uuid"
  "connection_list|connection list --cluster $cluster_uuid"
  "session_list|session list --cluster $cluster_uuid"
  "lock_list|lock list --cluster $cluster_uuid"
  "rule_list|rule list --cluster $cluster_uuid"
  "profile_list|profile list --cluster $cluster_uuid"
  "counter_list|counter list --cluster $cluster_uuid"
  "limit_list|limit list --cluster $cluster_uuid"
)

if [[ -n "$manager_uuid" ]]; then
  SPECS+=("manager_info|manager info --cluster $cluster_uuid --manager $manager_uuid")
fi
if [[ -n "$server_uuid" ]]; then
  SPECS+=("server_info|server info --cluster $cluster_uuid --server $server_uuid")
fi
if [[ -n "$process_uuid" ]]; then
  SPECS+=("process_info|process info --cluster $cluster_uuid --process $process_uuid")
fi
if [[ -n "$connection_uuid" ]]; then
  SPECS+=("connection_info|connection info --cluster $cluster_uuid --connection $connection_uuid")
fi
if [[ -n "$session_uuid" ]]; then
  SPECS+=("session_info|session info --cluster $cluster_uuid --session $session_uuid")
fi
infobase_cred_args=""
if [[ -n "$infobase_user" ]]; then
  infobase_cred_args="--infobase-user $infobase_user --infobase-pwd $infobase_pwd"
fi
if [[ -n "$infobase_uuid" ]]; then
  SPECS+=("infobase_info|infobase info --cluster $cluster_uuid --infobase $infobase_uuid $infobase_cred_args")
  SPECS+=("infobase_summary_info|infobase summary info --cluster $cluster_uuid --infobase $infobase_uuid")
fi
if [[ -n "$counter_name" ]]; then
  SPECS+=("counter_info|counter info --cluster $cluster_uuid --counter $counter_name")
fi
if [[ -n "$limit_name" ]]; then
  SPECS+=("limit_info|limit info --cluster $cluster_uuid --limit $limit_name")
fi
if [[ -n "$service_setting_uuid" && -n "$service_setting_server_uuid" ]]; then
  SPECS+=("service_setting_info|service-setting info --cluster $cluster_uuid --server $service_setting_server_uuid --setting $service_setting_uuid")
fi
if [[ -n "$infobase_uuid" ]]; then
  if [[ -n "$binary_storage_uuid" ]]; then
    SPECS+=("binary_storage_info|binary-data-storage info --cluster $cluster_uuid --infobase $infobase_uuid --storage $binary_storage_uuid $infobase_cred_args")
  elif [[ -n "$binary_storage_name" ]]; then
    SPECS+=("binary_storage_info|binary-data-storage info --cluster $cluster_uuid --infobase $infobase_uuid --name $binary_storage_name $infobase_cred_args")
  fi
fi

{
  echo "# RAC CLI -> Method Map (Generated)"
  echo
  echo "Generated: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo
  echo "- Cluster UUID: \`$cluster_uuid\`"
  [[ -n "$manager_uuid" ]] && echo "- Manager UUID: \`$manager_uuid\`"
  [[ -n "$server_uuid" ]] && echo "- Server UUID: \`$server_uuid\`"
  [[ -n "$process_uuid" ]] && echo "- Process UUID: \`$process_uuid\`"
  echo
  echo "| Command | rac_exit | Session | c2s method IDs | s2c method IDs |"
  echo "|---|---:|---|---|---|"
} >"$OUT_MD"

for spec in "${SPECS[@]}"; do
  name="${spec%%|*}"
  cmd="${spec#*|}"
  info="$(run_capture "map_${name}" $cmd)"
  session="${info%%|*}"
  exit_code="${info##*|}"
  c2s=""
  s2c=""
  if [[ -n "$session" && -d "$ROOT_DIR/logs/$session" ]]; then
    c2s="$(extract_methods "$ROOT_DIR/logs/$session/client_to_server.stream.bin")"
    s2c="$(extract_methods "$ROOT_DIR/logs/$session/server_to_client.stream.bin")"
  fi
  printf '| `%s` | %s | `%s` | `%s` | `%s` |\n' \
    "$cmd" "${exit_code:-}" "${session:-}" "${c2s:-}" "${s2c:-}" >>"$OUT_MD"
done

echo "written: $OUT_MD"
