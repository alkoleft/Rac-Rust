# RAC Mode: Rule

## Purpose

Manage assignment rules for working servers.

## CLI Surface

Primary entrypoint: `rac rule <command>`.

Key commands:
- `apply [--full|--partial]`
- `list --server=<uuid>`
- `info --server=<uuid> --rule=<uuid>`
- `insert/update/remove`

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
