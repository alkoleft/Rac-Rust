#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <cluster_uuid> <session_uuid> [count]"
  echo "Example: $0 1619820a-d36f-4d8a-a716-1516b1dea077 25510e27-f24a-4586-9ac9-9f7837c0dea1 40"
  exit 2
fi

CLUSTER_UUID="$1"
SESSION_UUID="$2"
COUNT="${3:-40}"

INTERVAL_SEC="${INTERVAL_SEC:-10}"
CAPTURE_PREFIX="${CAPTURE_PREFIX:-session_info_load}"
RAC_TIMEOUT_SEC="${RAC_TIMEOUT_SEC:-60}"
LISTEN_HOST="${LISTEN_HOST:-127.0.0.1}"
LISTEN_PORT_BASE="${LISTEN_PORT_BASE:-1565}"
LISTEN_PORT_MAX="${LISTEN_PORT_MAX:-1595}"

if ! [[ "$COUNT" =~ ^[0-9]+$ ]] || [[ "$COUNT" -eq 0 ]]; then
  echo "count must be a positive integer"
  exit 2
fi

if ! [[ "$INTERVAL_SEC" =~ ^[0-9]+$ ]]; then
  echo "INTERVAL_SEC must be a non-negative integer"
  exit 2
fi

if ! [[ "$LISTEN_PORT_BASE" =~ ^[0-9]+$ ]] || ! [[ "$LISTEN_PORT_MAX" =~ ^[0-9]+$ ]]; then
  echo "LISTEN_PORT_BASE and LISTEN_PORT_MAX must be integers"
  exit 2
fi

if [[ "$LISTEN_PORT_BASE" -gt "$LISTEN_PORT_MAX" ]]; then
  echo "LISTEN_PORT_BASE must be <= LISTEN_PORT_MAX"
  exit 2
fi

is_port_open() {
  local host="$1"
  local port="$2"
  local addr="${host}:${port}"
  if ss -ltn 2>/dev/null | awk '{print $4}' | grep -Fq "$addr"; then
    return 0
  fi
  if ss -ltn 2>/dev/null | awk '{print $4}' | grep -Fq "*:${port}"; then
    return 0
  fi
  if ss -ltn 2>/dev/null | awk '{print $4}' | grep -Fq "0.0.0.0:${port}"; then
    return 0
  fi
  return 1
}

pick_listen_addr() {
  local port
  for ((port = LISTEN_PORT_BASE; port <= LISTEN_PORT_MAX; port++)); do
    if ! is_port_open "$LISTEN_HOST" "$port"; then
      echo "${LISTEN_HOST}:${port}"
      return 0
    fi
  done
  return 1
}

echo "cluster=$CLUSTER_UUID"
echo "session=$SESSION_UUID"
echo "count=$COUNT"
echo "interval_sec=$INTERVAL_SEC"
echo "capture_prefix=$CAPTURE_PREFIX"
echo "rac_timeout_sec=$RAC_TIMEOUT_SEC"
echo "listen_host=$LISTEN_HOST"
echo "listen_port_range=${LISTEN_PORT_BASE}-${LISTEN_PORT_MAX}"

width=${#COUNT}
if [[ "$width" -lt 2 ]]; then
  width=2
fi

for i in $(seq 1 "$COUNT"); do
  idx="$(printf "%0${width}d" "$i")"
  name="${CAPTURE_PREFIX}_${idx}"
  listen_addr="$(pick_listen_addr)" || {
    echo "no free listen port in range ${LISTEN_PORT_BASE}-${LISTEN_PORT_MAX}" >&2
    exit 1
  }
  echo
  echo "[$i/$COUNT] capture=$name listen_addr=$listen_addr"

  LISTEN_ADDR="$listen_addr" RAC_TIMEOUT_SEC="$RAC_TIMEOUT_SEC" \
    scripts/rac/capture_rac_command.sh "$name" \
      session info --cluster="$CLUSTER_UUID" --session="$SESSION_UUID"

  if [[ "$i" -lt "$COUNT" ]] && [[ "$INTERVAL_SEC" -gt 0 ]]; then
    sleep $INTERVAL_SEC
  fi
done
