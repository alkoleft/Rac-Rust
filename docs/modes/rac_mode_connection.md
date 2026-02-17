# RAC Mode: Connection

## Purpose

Inspect and terminate client connections.

## CLI Surface

Primary entrypoint: `rac connection <command>`.

Key commands:
- `list [--process=<uuid>] [--infobase=<uuid>]`
- `info --connection=<uuid>`
- `disconnect --process=<uuid> --connection=<uuid>`

## Protocol Notes

Message format notes: `docs/messages/rac_message_formats_connection.md`.

Related method mapping:
- `docs/documentation/rac_method_map.md`
- `docs/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: present
- `captures`: pending
- `rpc_mapping`: partial
