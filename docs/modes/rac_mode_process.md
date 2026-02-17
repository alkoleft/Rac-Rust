# RAC Mode: Process

## Purpose

Inspect and control working processes.

## CLI Surface

Primary entrypoint: `rac process <command>`.

Key commands:
- `list [--server=<uuid>]`
- `info --process=<uuid> [--licenses]`
- `turn-off --process=<uuid>`

## Protocol Notes

Message format notes: `docs/messages/rac_message_formats_process.md`.

Related method mapping:
- `docs/documentation/rac_method_map.md`
- `docs/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: present
- `captures`: present
- `rpc_mapping`: partial
