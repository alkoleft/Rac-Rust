# RAC Mode: Binary Data Storage

## Purpose

Manage binary data storage and backups for infobases.

## CLI Surface

Primary entrypoint: `rac binary-data-storage <command>`.

Key commands:
- `list`
- `info --storage=<uuid> | --name=<name>`
- `create-full-backup ...`
- `create-diff-backup ...`
- `load-full-backup ...`
- `load-diff-backup ...`
- `clear-unused-space ...`

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
