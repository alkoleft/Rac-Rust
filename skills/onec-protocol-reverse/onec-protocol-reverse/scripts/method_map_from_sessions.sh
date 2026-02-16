#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <session_dir...>"
  echo "Example: $0 logs/session_123 logs/session_456"
  exit 2
fi

for session in "$@"; do
  c2s="$session/client_to_server.stream.bin"
  s2c="$session/server_to_client.stream.bin"
  echo "=== session: $session ==="
  if [[ -f "$c2s" ]]; then
    echo "--- c2s methods ---"
    cargo run --quiet --bin rac_decode -- "$c2s" | rg 'rpc_method_id=' || true
  fi
  if [[ -f "$s2c" ]]; then
    echo "--- s2c methods ---"
    cargo run --quiet --bin rac_decode -- "$s2c" | rg 'rpc_method_id=' || true
  fi
  echo
done
