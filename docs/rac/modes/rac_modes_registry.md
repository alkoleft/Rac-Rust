# RAC Modes Registry

Tracks per-mode documentation coverage. "Processed" means a mode has a dedicated description file in `docs/rac/modes/`.

| Mode | Description file | Message formats file | Command | Analyzed | Implemented | Notes |
| ----------------------- | ---------------------------------------------- | --------------------------------------------------- | -------------------------------------- | ---------- | ------------- | ------- |
| `cluster` | `docs/rac/modes/rac_mode_cluster.md` | `docs/rac/messages/rac_message_formats_cluster.md` | `admin list` | yes | yes | - |
|  |  |  | `admin register` | yes | yes | - |
|  |  |  | `admin remove` | - | - | - |
|  |  |  | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `insert` | - | - | - |
|  |  |  | `update` | yes | - | req `0x5b`, resp ack |
|  |  |  | `remove` | - | - | - |
| `manager` | `docs/rac/modes/rac_mode_manager.md` | - | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
| `server` | `docs/rac/modes/rac_mode_server.md` | `docs/rac/messages/rac_message_formats_server.md` | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `insert` | - | - | - |
|  |  |  | `update` | - | - | - |
|  |  |  | `remove` | - | - | - |
| `process` | `docs/rac/modes/rac_mode_process.md` | `docs/rac/messages/rac_message_formats_process.md` | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `turn-off` | - | - | - |
| `connection` | `docs/rac/modes/rac_mode_connection.md` | `docs/rac/messages/rac_message_formats_connection.md` | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `disconnect` | - | - | - |
| `session` | `docs/rac/modes/rac_mode_session.md` | `docs/rac/messages/rac_message_formats_session.md` | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `terminate` | - | - | - |
|  |  |  | `interrupt-current-server-call` | - | - | - |
| `lock` | `docs/rac/modes/rac_mode_lock.md` | `docs/rac/messages/rac_message_formats_lock.md` | `list` | yes | yes | - |
| `rule` | `docs/rac/modes/rac_mode_rule.md` | `docs/rac/messages/rac_message_formats_rule.md` | `apply` | yes | yes | - |
|  |  |  | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `insert` | yes | yes | - |
|  |  |  | `update` | yes | yes | - |
|  |  |  | `remove` | yes | yes | - |
| `profile` | `docs/rac/modes/rac_mode_profile.md` | `docs/rac/messages/rac_message_formats_profile.md` | `list` | yes | - | req `0x59`, resp `0x5a` (auth `0x09`) |
|  |  |  | `update` | - | - | - |
|  |  |  | `remove` | - | - | - |
|  |  |  | `acl directory` | - | - | - |
|  |  |  | `acl com` | - | - | - |
|  |  |  | `acl addin` | - | - | - |
| `counter` | `docs/rac/modes/rac_mode_counter.md` | `docs/rac/messages/rac_message_formats_counter.md` | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `update` | yes | yes | - |
|  |  |  | `values` | yes | yes | - |
|  |  |  | `accumulated-values` | yes | yes | - |
|  |  |  | `clear` | yes | yes | - |
|  |  |  | `remove` | yes | yes | - |
| `limit` | `docs/rac/modes/rac_mode_limit.md` | - | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `update` | yes | yes | - |
|  |  |  | `remove` | yes | yes | - |
| `service-setting` | `docs/rac/modes/rac_mode_service-setting.md` | `docs/rac/messages/rac_message_formats_service-setting.md` | `list` | yes | yes | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `insert` | yes | yes | - |
|  |  |  | `update` | yes | yes | - |
|  |  |  | `get-service-data-dirs-for-transfer` | yes | yes | - |
|  |  |  | `remove` | yes | yes | - |
|  |  |  | `apply` | yes | yes | - |
| `binary-data-storage` | `docs/rac/modes/rac_mode_binary-data-storage.md` | - | `list` | - | - | - |
|  |  |  | `info` | - | - | - |
|  |  |  | `create-full-backup` | - | - | - |
|  |  |  | `create-diff-backup` | - | - | - |
|  |  |  | `load-full-backup` | - | - | - |
|  |  |  | `load-diff-backup` | - | - | - |
|  |  |  | `clear-unused-space` | - | - | - |
| `agent` | `docs/rac/modes/rac_mode_agent.md` | - | `admin list` | yes | yes | - |
|  |  |  | `admin register` | - | - | - |
|  |  |  | `admin remove` | - | - | - |
|  |  |  | `version` |  |  | - |
| `service` | `docs/rac/modes/rac_mode_service.md` | `docs/rac/messages/rac_message_formats_service.md` | `list` | yes | - | - |
| `infobase` | `docs/rac/modes/rac_mode_infobase.md` | `docs/rac/messages/rac_message_formats_infobase.md` | `summary list` | yes | yes | - |
|  |  |  | `summary info` | yes | - | - |
|  |  |  | `summary update` | yes | - | - |
|  |  |  | `info` | yes | yes | - |
|  |  |  | `create` | yes | - | - |
|  |  |  | `update` | yes | - | - |
|  |  |  | `drop` | yes | - | - |
