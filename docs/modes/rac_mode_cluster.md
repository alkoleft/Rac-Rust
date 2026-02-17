# RAC Mode: Cluster

## Purpose

Manage cluster definitions and cluster administrators.

## CLI Surface

Primary entrypoint: `rac cluster <command>`.

Key commands:
- `admin list/register/remove`
- `list`
- `info --cluster=<uuid>`
- `insert`
- `update`
- `remove --cluster=<uuid>`

## Protocol Notes

Message format notes: `docs/messages/rac_message_formats_cluster.md`.

Related method mapping:
- `docs/documentation/rac_method_map.md`
- `docs/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: present
- `captures`: pending
- `rpc_mapping`: partial
