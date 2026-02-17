# RAC Modes Registry

Tracks per-mode documentation coverage. "Processed" means a mode has a dedicated description file in `docs/modes/`.

| Mode                  | Description file                             | Message formats file                              | Command                              | Analyzed | Implemented | Notes |
|-----------------------|----------------------------------------------|---------------------------------------------------|--------------------------------------|----------|-------------|-------|
| `cluster`             | `docs/modes/rac_mode_cluster.md`             | `docs/messages/rac_message_formats_cluster.md`    | `admin list`                         | yes      | -           | -     |
|                       |                                              |                                                   | `admin register`                     | yes      | -           | -     |
|                       |                                              |                                                   | `admin remove`                       | -        | -           | -     |
|                       |                                              |                                                   | `list`                               | yes      | yes         | -     |
|                       |                                              |                                                   | `info`                               | yes      | yes         | -     |
|                       |                                              |                                                   | `insert`                             | -        | -           | -     |
|                       |                                              |                                                   | `update`                             | -        | -           | -     |
|                       |                                              |                                                   | `remove`                             | -        | -           | -     |
| `manager`             | `docs/modes/rac_mode_manager.md`             | -                                                 | `list`                               | yes      |             | -     |
|                       |                                              |                                                   | `info`                               | yes      |             | -     |
| `server`              | `docs/modes/rac_mode_server.md`              | -                                                 | `list`                               |          |             | -     |
|                       |                                              |                                                   | `info`                               |          |             | -     |
|                       |                                              |                                                   | `insert`                             | -        | -           | -     |
|                       |                                              |                                                   | `update`                             | -        | -           | -     |
|                       |                                              |                                                   | `remove`                             | -        | -           | -     |
| `process`             | `docs/modes/rac_mode_process.md`             | `docs/messages/rac_message_formats_process.md`    | `list`                               | yes      |             | -     |
|                       |                                              |                                                   | `info`                               | yes      |             | -     |
|                       |                                              |                                                   | `turn-off`                           | -        | -           | -     |
| `connection`          | `docs/modes/rac_mode_connection.md`          | `docs/messages/rac_message_formats_connection.md` | `list`                               | yes      |             | -     |
|                       |                                              |                                                   | `info`                               | yes      |             | -     |
|                       |                                              |                                                   | `disconnect`                         | -        | -           | -     |
| `session`             | `docs/modes/rac_mode_session.md`             | `docs/messages/rac_message_formats_session.md`    | `list`                               | yes      | yes         | -     |
|                       |                                              |                                                   | `info`                               | yes      | yes         | -     |
|                       |                                              |                                                   | `terminate`                          | -        | -           | -     |
|                       |                                              |                                                   | `interrupt-current-server-call`      | -        | -           | -     |
| `lock`                | `docs/modes/rac_mode_lock.md`                | -                                                 | `list`                               |          |             | -     |
| `rule`                | `docs/modes/rac_mode_rule.md`                | -                                                 | `apply`                              | -        | -           | -     |
|                       |                                              |                                                   | `list`                               | -        | -           | -     |
|                       |                                              |                                                   | `info`                               | -        | -           | -     |
|                       |                                              |                                                   | `insert`                             | -        | -           | -     |
|                       |                                              |                                                   | `update`                             | -        | -           | -     |
|                       |                                              |                                                   | `remove`                             | -        | -           | -     |
| `profile`             | `docs/modes/rac_mode_profile.md`             | -                                                 | `list`                               |          |             | -     |
|                       |                                              |                                                   | `update`                             | -        | -           | -     |
|                       |                                              |                                                   | `remove`                             | -        | -           | -     |
|                       |                                              |                                                   | `acl directory`                      | -        | -           | -     |
|                       |                                              |                                                   | `acl com`                            | -        | -           | -     |
|                       |                                              |                                                   | `acl addin`                          | -        | -           | -     |
| `counter`             | `docs/modes/rac_mode_counter.md`             | -                                                 | `list`                               |          |             | -     |
|                       |                                              |                                                   | `info`                               | -        | -           | -     |
|                       |                                              |                                                   | `update`                             | -        | -           | -     |
|                       |                                              |                                                   | `values`                             | -        | -           | -     |
|                       |                                              |                                                   | `accumulated-values`                 | -        | -           | -     |
|                       |                                              |                                                   | `clear`                              | -        | -           | -     |
|                       |                                              |                                                   | `remove`                             | -        | -           | -     |
| `limit`               | `docs/modes/rac_mode_limit.md`               | -                                                 | `list`                               |          |             | -     |
|                       |                                              |                                                   | `info`                               | -        | -           | -     |
|                       |                                              |                                                   | `update`                             | -        | -           | -     |
|                       |                                              |                                                   | `remove`                             | -        | -           | -     |
| `service-setting`     | `docs/modes/rac_mode_service-setting.md`     | -                                                 | `list`                               | -        | -           | -     |
|                       |                                              |                                                   | `info`                               | -        | -           | -     |
|                       |                                              |                                                   | `insert`                             | -        | -           | -     |
|                       |                                              |                                                   | `update`                             | -        | -           | -     |
|                       |                                              |                                                   | `get-service-data-dirs-for-transfer` | -        | -           | -     |
|                       |                                              |                                                   | `remove`                             | -        | -           | -     |
|                       |                                              |                                                   | `apply`                              | -        | -           | -     |
| `binary-data-storage` | `docs/modes/rac_mode_binary-data-storage.md` | -                                                 | `list`                               | -        | -           | -     |
|                       |                                              |                                                   | `info`                               | -        | -           | -     |
|                       |                                              |                                                   | `create-full-backup`                 | -        | -           | -     |
|                       |                                              |                                                   | `create-diff-backup`                 | -        | -           | -     |
|                       |                                              |                                                   | `load-full-backup`                   | -        | -           | -     |
|                       |                                              |                                                   | `load-diff-backup`                   | -        | -           | -     |
|                       |                                              |                                                   | `clear-unused-space`                 | -        | -           | -     |
| `agent`               | `docs/modes/rac_mode_agent.md`               | -                                                 | `admin list`                         | -        | -           | -     |
|                       |                                              |                                                   | `admin register`                     | -        | -           | -     |
|                       |                                              |                                                   | `admin remove`                       | -        | -           | -     |
|                       |                                              |                                                   | `version`                            |          |             | -     |
| `service`             | `docs/modes/rac_mode_service.md`             | -                                                 | `list`                               | -        | -           | -     |
| `infobase`            | `docs/modes/rac_mode_infobase.md`            | `docs/messages/rac_message_formats_infobase.md`   | `summary list`                       | yes      | yes         | -     |
|                       |                                              |                                                   | `list`                               | -        | -           | -     |
|                       |                                              |                                                   | `info`                               | yes      | yes         | -     |
|                       |                                              |                                                   | `create`                             | -        | -           | -     |
|                       |                                              |                                                   | `update`                             | -        | -           | -     |
|                       |                                              |                                                   | `remove`                             | -        | -           | -     |
