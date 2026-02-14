# v8-proxy

TCP proxy tunnel for reverse engineering protocol traffic.

## Features

- Listens on local TCP port and forwards traffic to target server.
- Handles one client session per run.
- Logs full raw streams:
  - `client_to_server.stream.bin`
  - `server_to_client.stream.bin`
- Writes chunk timeline:
  - `events.log` (`event_id`, timestamp, direction, exchange, size, hex preview)
- Generates readable hex+ascii dumps:
  - `client_to_server.stream.hex.txt`
  - `server_to_client.stream.hex.txt`
- Splits traffic into "exchanges":
  - `exchanges/exchange_000001/request.bin`
  - `exchanges/exchange_000001/response.bin`
- Generates readable exchange dumps:
  - `exchanges/exchange_000001/request.hex.txt`
  - `exchanges/exchange_000001/response.hex.txt`
- Keeps `--try-inflate` CLI flag reserved for next iteration (currently no-op in offline std-only build).

## Important note

TCP has no request boundaries.  
This prototype treats each newly received `client -> server` chunk as a new logical request, and appends subsequent `server -> client` data to that request's response until next client chunk appears.

## Workspace

This repository is a Cargo workspace:

- Root crate `v8_protocols`: TCP proxy.
- Subproject `apps/rac_protocol`: RAC tooling (`rac_lite`, `rac_decode`).

## Build

```bash
cargo build --release -p v8_protocols
cargo build --release -p rac_protocol
```

## RAC Decoder Helper

To inspect RAC stream framing from logged `*.stream.bin`:

```bash
cargo run -p rac_protocol --bin rac_decode -- logs/<session>/client_to_server.stream.bin
cargo run -p rac_protocol --bin rac_decode -- logs/<session>/server_to_client.stream.bin
```

To capture one `rac` command through the proxy:

```bash
scripts/capture_rac_command.sh <name> <rac args...>
# example:
scripts/capture_rac_command.sh cluster_list cluster list
```

Protocol notes:

- `docs/rac_protocol_notes.md`
- `docs/rac_method_map.md`

## rac_lite Prototype

Minimal protocol client without `rac` (work in progress):

```bash
cargo run -p rac_protocol --bin rac_lite -- agent-version 127.0.0.1:1545
cargo run -p rac_protocol --bin rac_lite -- cluster-list 127.0.0.1:1545
```

## Run

```bash
cargo run --release -p v8_protocols -- 
  --listen 127.0.0.1:15410 \
  --target 127.0.0.1:1541 \
  --log-dir ./logs \
  --try-inflate=true
```

Then point your 1C client to `127.0.0.1:15410`.

## Output layout

Example session directory:

```text
logs/session_1739560000_12345_127_0_0_1_55314/
  session.txt
  events.log
  client_to_server.stream.bin
  client_to_server.stream.hex.txt
  server_to_client.stream.bin
  server_to_client.stream.hex.txt
  exchanges/
    exchange_000001/
      request.bin
      request.hex.txt
      response.bin
      response.hex.txt
```
