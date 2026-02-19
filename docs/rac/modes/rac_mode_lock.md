# RAC Mode: Lock

## Purpose

List active locks.

## CLI Surface

Primary entrypoint: `rac lock list`.

Key commands:
- `list [--infobase=<uuid>] [--connection=<uuid>] [--session=<uuid>]`

## Protocol Notes

Message format notes: `docs/rac/messages/rac_message_formats_lock.md`.

Related method mapping:
- `docs/rac/documentation/rac_method_map.md`
- `docs/rac/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: yes
- `captures`: yes
- `rpc_mapping`: yes
