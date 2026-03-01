# Unknown/tail/gap audit (schemas/rac)

## Scope
- Schemas: agent, cluster, process, infobase, server
- Problem markers: `unknown_*`, `tail`, `gap_*`

## Matrix (schema -> record -> fields)

### schemas/rac/agent.toml
- record: `AgentAdminRecord`
  - `unknown_flags` (u32_be, v11.0)
  - `auth_tag` (u8, v11.0)
  - `auth_flags` (u8, v11.0)

Evidence / mapping:
- `docs/rac/messages/rac_message_formats_agent.md` shows full record layout:
  - `name`, `descr`, `unknown_flags` (u32_be), `auth_tag` (u8), `auth_flags` (u8), `os_user_len` (u8), `os_user` (str8)
- Server-to-client decode:
  - `artifacts/rac/v16/v16_20260226_053425_agent_admin_list_server_to_client.decode.txt`

Notes:
- Schema updated to include `descr` and `os_user` (auth block decoded); `unknown_tag/unknown_tail` removed.

### schemas/rac/cluster.toml
- record: `ClusterAdminRecord`
  - `unknown_flags` (u32_be, v11.0)
  - `auth_tag` (u8, v11.0)
  - `auth_flags` (u8, v11.0)

- response: `ClusterList` uses `list_u8_tail` (`tail_len_param = "tail_len"`)
- response: `ClusterInfo` uses `record_tail` (`tail_len_param = "tail_len"`)

Evidence / mapping:
- `docs/rac/messages/rac_message_formats_cluster.md` shows record layout:
  - `name`, `descr`, `unknown_flags` (u32_be), `auth_tag` (u8), `auth_flags` (u8), `os_user_len` (u8), `os_user` (str8)
- Server-to-client decode:
  - `artifacts/rac/v16/v16_20260226_053425_cluster_admin_list_server_to_client.decode.txt`

Notes:
- Schema updated to include `descr` and `os_user` (auth block decoded); `unknown_tag/unknown_tail` removed.
- Tail usage (`list_u8_tail`, `record_tail`) needs explicit mapping of tail bytes to fields.

### schemas/rac/process.toml
- record: `ProcessLicense`
  - `_gap_license_0` (u8, skip, v11.0)

- record: `ProcessRecord`
  - `_gap_0` (bytes len=8, skip, v11.0)

Evidence / mapping:
- `docs/rac/messages/rac_message_formats_process.md`:
  - `_gap_0` is 8 bytes after `process` UUID.
  - `_gap_license_0` is a 1-byte flag before `file_name` in license block.

Notes:
- Gap bytes are stable in captures but semantics unknown.

### schemas/rac/infobase.toml
- record: `InfobaseInfoRecord`
  - `unknown_u32_0` (u32_be, v11.0)
  - `unknown_str_0` (str8, v11.0)
  - `unknown_str_1` (str8, v11.0)
  - `unknown_str_2` (str8, v11.0)
  - `unknown_bytes_0` (bytes_fixed len=4, v11.0)
  - `unknown_str_3` (str8, v11.0)
  - `unknown_str_4` (str8, v11.0)
  - `unknown_u32_1` (u32_be, v11.0)
  - `tail` (bytes_fixed len=28, v11.0)

Evidence / mapping:
- `docs/rac/messages/rac_message_formats_infobase.md` lists observed sequence and tail bytes.
- Server-to-client decode:
  - `artifacts/rac/v16/v16_20260226_053425_infobase_info_server_to_client.decode.txt`
  - `artifacts/rac/v16/v16_20260226_053425_infobase_info_response.hex`

Notes:
- Many fields correspond to RAC output but remain unmapped in schema.
- Tail is 7 x u32 (28 bytes) per doc; each likely maps to explicit fields.
- `artifacts/rac/v16/v16_20260226_053425_infobase_info_rac.out` is empty; need a successful `rac infobase info` output to map names/values.

### schemas/rac/server.toml
- record: `ServerRecord`
  - `gap_1` (u32_le, v11.0)
  - `gap_2` (u32_le, v11.0)
  - `gap_3` (u32_le, v11.0)
  - `gap_4` (u32_le, v11.0)
  - `gap_4_pad` (u8, v11.0)
  - `gap_5` (u32_be, v11.0)
  - `gap_6` (u32_be, v11.0)
  - `gap_7` (u8, v11.0)

Evidence / mapping:
- `docs/rac/messages/rac_message_formats_server.md` provides a detailed offset map and candidates:
  - `gap_1` candidate: `memory-limit`
  - `gap_2` candidate: `connections-limit`
  - `gap_3` candidate: `safe-working-processes-memory-limit`
  - `gap_7` likely terminator or trailing flag
- Server-to-client decode:
  - `artifacts/rac/v16/v16_20260226_053425_server_list_after_update_server_to_client.decode.txt`
  - `artifacts/rac/v16/v16_20260226_053425_server_info_server_to_client.decode.txt`

Notes:
- Gaps include numeric fields shown by `rac` but not decoded in schema.

## Control decode inventory (for step 3)
- Agent admin list: `artifacts/rac/v16/v16_20260226_053425_agent_admin_list_response.hex` + `*_server_to_client.decode.txt`
- Cluster admin list: `artifacts/rac/v16/v16_20260226_053425_cluster_admin_list_response.hex` + `*_server_to_client.decode.txt`
- Cluster list: `artifacts/rac/v16/cluster_list_response_*.hex` and `v16_20260226_053425_cluster_list_after_update_response.hex`
- Process list/info: multiple `process_*_response.hex` and `*_server_to_client.decode.txt`
- Infobase info: `artifacts/rac/v16/v16_20260226_053425_infobase_info_response.hex` + `*_server_to_client.decode.txt`
- Server list/info: `artifacts/rac/v16/v16_20260226_053425_server_list_after_update_response.hex` + `*_server_to_client.decode.txt`

## Open items for follow-up
- Agent/Cluster admin records: map `descr`, `auth_tag`, `auth_flags`, `os_user` into schema; replace `unknown_*` fields.
- Infobase info: map unknown strings/bytes to RAC output fields; expand 28-byte tail into 7 explicit u32 fields.
- Server record: map numeric gaps to `rac` output fields (memory-limit, connections-limit, safe-working-processes-memory-limit, etc.).
- Process record: identify `gap_0` and `_gap_license_0` semantics.
