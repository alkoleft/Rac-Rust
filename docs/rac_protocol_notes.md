# RAC Protocol Notes (Draft)

Captured session:

- command: `./rac cluster list localhost:1565`
- proxy listen: `127.0.0.1:1565`
- target: `127.0.0.1:1545`
- log root: `logs/session_1771098941_320377_127_0_0_1_42586`

## Transport framing

Observed frame shape in both directions:

- `opcode: u8`
- `len: varuint` (LEB128-like)
- `payload: [len]`

Example:

- `0b 1f <31-byte payload>`
- `0e 74 <116-byte payload>`
- `0e 84 01 <132-byte payload>` (length > 127)

## Sequence for `cluster list`

1. Init frame:
   - c2s init packet (special, not `opcode/len`): `1c 53 57 50 ... "connect.timeout" ...`
   - s2c: `02 01 80`
2. Service negotiation:
   - c2s opcode `0x0b`, payload has `v8.service.Admin.Cluster`, `16.0`
   - s2c opcode `0x0c`, echoes service/version and adds success marker
3. Command call:
   - c2s opcode `0x0e`, payload `01 00 00 01 0b`
   - s2c opcode `0x0e`, payload contains one cluster record
4. End:
   - c2s opcode `0x0d`, payload `01`

## Extracted fields from response

From frame `s2c opcode=0x0e len=0x74`:

- cluster UUID:
  - `1619820a-d36f-4d8a-a716-1516b1dea077`
- host: `alko-home`
- UTF-8 text: `Локальный кластер`

Remaining numeric fields are not mapped yet.

## Helper decoder

Use the local parser:

```bash
cargo run --bin rac_decode -- logs/session_1771098941_320377_127_0_0_1_42586/client_to_server.stream.bin
cargo run --bin rac_decode -- logs/session_1771098941_320377_127_0_0_1_42586/server_to_client.stream.bin
```

The parser prints frame offsets, opcode/length, payload hex, text candidates, and UUID candidates.

## Capture helper

Use:

```bash
scripts/capture_rac_command.sh <name> <rac args...>
```

Example:

```bash
scripts/capture_rac_command.sh cluster_list cluster list
scripts/capture_rac_command.sh cluster_info cluster info --cluster 1619820a-d36f-4d8a-a716-1516b1dea077
```

Artifacts:

- `/tmp/rac_<name>.out`
- `/tmp/rac_<name>.err`
- `/tmp/v8_capture_<name>.log`
- `logs/session_<...>/...`
- `artifacts/<label>.hex` (extracted response payload examples)

Example extraction:

```bash
scripts/extract_rac_response_example.sh logs/session_1771098941_320377_127_0_0_1_42586 0x42 session_list_response
```

## Next reference

Method map extracted from multiple commands is in `docs/rac_method_map.md`.

For this capture, expected client-side output:

```text
init_packet len=32
frame opcode=0x0b len=31   # service/version negotiation
frame opcode=0x0e len=5    # cluster list call
frame opcode=0x0d len=1    # close/end
```
