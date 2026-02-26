#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 3 ]]; then
  echo "Usage: $0 <session_dir> <method_id_hex> <label>"
  echo "Example: $0 logs/session_... 0x42 session_list_response"
  exit 2
fi

SESSION_DIR="$1"
METHOD_ID="$2"
LABEL="$3"

RAC_DECODE="${RAC_DECODE:-cargo run -p rac_cli --quiet --bin rac_decode --}"
OUT_DIR="${OUT_DIR:-./artifacts}"

stream="${SESSION_DIR%/}/server_to_client.stream.bin"
mkdir -p "$OUT_DIR"

payload_hex="$($RAC_DECODE "$stream" | awk -v mid="$METHOD_ID" '
  /payload_hex=/ {last=$0; sub(".*payload_hex=", "", last)}
  $0 ~ ("rpc_method_id=" mid) {
    if (length(last) > 0) {
      payload=last
    }
  }
  END {
    if (length(payload) > 0) {
      print payload
    }
  }
')"

if [[ -z "$payload_hex" ]]; then
  echo "error: payload for method $METHOD_ID not found in $stream" >&2
  exit 1
fi

out_file="$OUT_DIR/${LABEL}.hex"
printf "%s\n" "$payload_hex" >"$out_file"
echo "written=$out_file"
