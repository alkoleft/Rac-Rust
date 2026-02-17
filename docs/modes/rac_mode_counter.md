# RAC Mode: Counter

## Purpose

Manage performance counters and fetch values.

## CLI Surface

Primary entrypoint: `rac counter <command>`.

Key commands:
- `list`
- `info --counter=<name>`
- `update --name=<name> ...`
- `values --counter=<name> ...`
- `accumulated-values --counter=<name> ...`
- `clear --counter=<name> ...`
- `remove --name=<name>`

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
