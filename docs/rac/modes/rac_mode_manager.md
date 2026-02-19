# RAC Mode: Manager

## Purpose

Inspect cluster manager instances.

## CLI Surface

Primary entrypoint: `rac manager <command>`.

Key commands:
- `list`
- `info --manager=<uuid>`

## Protocol Notes

Message format notes: `docs/rac/messages/rac_message_formats_manager.md`.

Related method mapping:
- `docs/rac/documentation/rac_method_map.md`
- `docs/rac/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: present
- `captures`: present
- `rpc_mapping`: partial
