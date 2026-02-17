# RAC Mode: Server

## Purpose

Manage working servers registered in the cluster.

## CLI Surface

Primary entrypoint: `rac server <command>`.

Key commands:
- `list`
- `info --server=<uuid>`
- `insert`
- `update`
- `remove --server=<uuid>`

## Protocol Notes

Message format notes: not yet captured.

Related method mapping:
- `docs/documentation/rac_method_map.md`
- `docs/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: missing
- `captures`: pending
- `rpc_mapping`: partial
