# RAC Mode: Service Setting

## Purpose

Manage per-service settings for a server.

## CLI Surface

Primary entrypoint: `rac service-setting <command>`.

Key commands:
- `list`
- `info --setting=<uuid>`
- `insert --service-name=<name> ...`
- `update --setting=<uuid> ...`
- `get-service-data-dirs-for-transfer ...`
- `remove --setting=<uuid>`
- `apply`

## Protocol Notes

Message format notes:
- `docs/messages/rac_message_formats_service-setting.md`

Related method mapping:
- `docs/documentation/rac_method_map.md`
- `docs/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: done
- `captures`: done
- `rpc_mapping`: done
