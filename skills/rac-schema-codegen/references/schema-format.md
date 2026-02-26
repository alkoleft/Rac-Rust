# RAC Schema TOML Format (Canonical)

Use this format when editing `schemas/rac/*.toml`. The canonical example is
`schemas/rac/cluster.toml`.

## Core Sections
- `[record.*]`: sequential field layouts for response records.
- `[rpc.*]`: RPC definitions. Always inline request fields here.
- `[response.*]`: response body definitions.

## Inline RPC/Request (Required)
Define request fields directly in the RPC block using `fields = [...]`. Never use `[request.*]` blocks.

```toml
[rpc.ClusterAdminRemove]
response = "AckResponse"
derive = ["Debug", "Clone"]
fields = [
  { name = "cluster", type = "uuid" },
  { name = "name", type = "str8" },
]
version = "11.0"
method_req = 0x07
requires_cluster_context = false
requires_infobase_context = false
```

## Response Definitions
Responses are declared under `[response.*]` and refer to a record layout:

```toml
[response.ClusterAdminList]
body = { type = "list_u8", item = "ClusterAdminRecord", field = "admins", struct = true }
```

Generate a response struct only when `body.struct = true`.

## Record Layouts
Records must be sequential and decoded via `RecordCursor`:

```toml
[record.ClusterAdminRecord]
derive = ["Debug", "Serialize", "Clone"]
fields = [
  { name = "name", type = "str8", version = "11.0" },
  { name = "unknown_tag", type = "u8", version = "11.0" },
  { name = "unknown_flags", type = "u32_be", version = "11.0" },
  { name = "unknown_tail", type = "bytes_fixed", len = 3, version = "11.0" },
]
```

## Notes
- Keep schemas codegen-friendly: avoid `super::` references and minimize manual types.
- Response bodies may set `field = "<name>"` to control the generated field name.
- Always set minimal `version` on RPCs and fields.
