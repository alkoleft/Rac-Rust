# RAC Mode: Infobase

## Purpose

Manage infobases and access settings.

## CLI Surface

Primary entrypoint: `rac infobase <command>`.

Key commands:
- `summary list`
- `list`
- `info --infobase=<uuid>`
- `create/update/remove`

## Protocol Notes

Message format notes: `docs/messages/rac_message_formats_infobase.md`.

Related method mapping:
- `docs/documentation/rac_method_map.md`
- `docs/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: present
- `captures`: pending
- `rpc_mapping`: partial
