# 1C Protocol Tooling Checklist

## Pre-Flight

- `cargo --version`
- `jq --version`
- `openssl version`
- One of: `tcpdump --version`, `tshark --version`, `mitmproxy --version`

If a tool is missing, install or adjust PATH before continuing.

## Port/Path Sanity

- `LISTEN_ADDR` free (use `ss -lntp | rg <port>`).
- `./logs/` exists and writable.
- `./artifacts/rac/` exists and writable.

## Control Capture (Minimum)

1. Run one known read-only command via the proxy.
2. Verify streams:
   - `client_to_server.stream.bin`
   - `server_to_client.stream.bin`
3. Decode:
   - `cargo run -p rac_cli --quiet --bin rac_decode -- <stream.bin>`

## Troubleshooting

- Empty `logs/`: proxy not in path or wrong `LISTEN_ADDR`.
- `rac_decode` errors: framing hypothesis wrong or corrupted stream.
- Partial data: command exited early or capture stopped too soon.
- Non-deterministic fields: run same command twice and diff outputs to isolate counters/timestamps.
