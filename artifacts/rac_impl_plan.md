# RAC Implementation Plan

| # | Mode | Command | Description | Message formats | Analyzed | Implemented | Notes |
|---|------|---------|-------------|-----------------|----------|-------------|-------|
| 1 | `cluster` | `admin list` | ``docs/modes/rac_mode_cluster.md`` | ``docs/messages/rac_message_formats_cluster.md`` | yes | yes | - |
| 2 | `cluster` | `admin register` | `-` | `-` | yes | - | - |
| 3 | `manager` | `list` | ``docs/modes/rac_mode_manager.md`` | `-` | yes | yes | - |
| 4 | `manager` | `info` | `-` | `-` | yes | - | - |
| 5 | `server` | `list` | ``docs/modes/rac_mode_server.md`` | ``docs/messages/rac_message_formats_server.md`` | yes | - | - |
| 6 | `server` | `info` | `-` | `-` | yes | - | - |
| 7 | `process` | `list` | ``docs/modes/rac_mode_process.md`` | ``docs/messages/rac_message_formats_process.md`` | yes | - | - |
| 8 | `process` | `info` | `-` | `-` | yes | - | - |
| 9 | `connection` | `list` | ``docs/modes/rac_mode_connection.md`` | ``docs/messages/rac_message_formats_connection.md`` | yes | - | - |
| 10 | `connection` | `info` | `-` | `-` | yes | - | - |
| 11 | `lock` | `list` | ``docs/modes/rac_mode_lock.md`` | ``docs/messages/rac_message_formats_lock.md`` | yes | - | - |
| 12 | `rule` | `apply` | ``docs/modes/rac_mode_rule.md`` | ``docs/messages/rac_message_formats_rule.md`` | yes | - | - |
| 13 | `rule` | `list` | `-` | `-` | yes | - | - |
| 14 | `rule` | `info` | `-` | `-` | yes | - | - |
| 15 | `rule` | `insert` | `-` | `-` | yes | - | - |
| 16 | `rule` | `update` | `-` | `-` | yes | - | - |
| 17 | `rule` | `remove` | `-` | `-` | yes | - | - |
| 18 | `counter` | `list` | ``docs/modes/rac_mode_counter.md`` | ``docs/messages/rac_message_formats_counter.md`` | yes | - | - |
| 19 | `counter` | `info` | `-` | `-` | yes | - | - |
| 20 | `counter` | `update` | `-` | `-` | yes | - | - |
| 21 | `counter` | `values` | `-` | `-` | yes | - | - |
| 22 | `counter` | `accumulated-values` | `-` | `-` | yes | - | - |
| 23 | `counter` | `clear` | `-` | `-` | yes | - | - |
| 24 | `counter` | `remove` | `-` | `-` | yes | - | - |
| 25 | `limit` | `list` | ``docs/modes/rac_mode_limit.md`` | `-` | yes | yes | - |
| 26 | `limit` | `info` | `-` | `-` | yes | - | - |
| 27 | `limit` | `update` | `-` | `-` | yes | - | - |
| 28 | `limit` | `remove` | `-` | `-` | yes | - | - |
| 29 | `service-setting` | `list` | ``docs/modes/rac_mode_service-setting.md`` | ``docs/messages/rac_message_formats_service-setting.md`` | yes | - | - |
| 30 | `service-setting` | `info` | `-` | `-` | yes | - | - |
| 31 | `service-setting` | `insert` | `-` | `-` | yes | - | - |
| 32 | `service-setting` | `update` | `-` | `-` | yes | - | - |
| 33 | `service-setting` | `get-service-data-dirs-for-transfer` | `-` | `-` | yes | - | - |
| 34 | `service-setting` | `remove` | `-` | `-` | yes | - | - |
| 35 | `service-setting` | `apply` | `-` | `-` | yes | - | - |
| 36 | `agent` | `admin list` | ``docs/modes/rac_mode_agent.md`` | `-` | yes | - | - |
