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

Message format notes:
- `docs/messages/rac_message_formats_server.md`

Related method mapping:
- `docs/documentation/rac_method_map.md`
- `docs/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: done
- `captures`: list/info captured
- `rpc_mapping`: partial
