# RAC Implementation Plan

| # | Mode | Command | Description | Message formats | Analyzed | Implemented | Notes |
|---|------|---------|-------------|-----------------|----------|-------------|-------|
| 1 | `server` | `info` | `-` | `-` | yes | - | - |
| 2 | `process` | `list` | ``docs/modes/rac_mode_process.md`` | ``docs/messages/rac_message_formats_process.md`` | yes | - | - |
| 3 | `process` | `info` | `-` | `-` | yes | - | - |
| 4 | `connection` | `list` | ``docs/modes/rac_mode_connection.md`` | ``docs/messages/rac_message_formats_connection.md`` | yes | - | - |
| 5 | `connection` | `info` | `-` | `-` | yes | - | - |
| 6 | `lock` | `list` | ``docs/modes/rac_mode_lock.md`` | ``docs/messages/rac_message_formats_lock.md`` | yes | - | - |
| 7 | `rule` | `apply` | ``docs/modes/rac_mode_rule.md`` | ``docs/messages/rac_message_formats_rule.md`` | yes | - | - |
| 8 | `rule` | `list` | `-` | `-` | yes | - | - |
| 9 | `rule` | `info` | `-` | `-` | yes | - | - |
| 10 | `rule` | `insert` | `-` | `-` | yes | - | - |
| 11 | `rule` | `update` | `-` | `-` | yes | - | - |
| 12 | `rule` | `remove` | `-` | `-` | yes | - | - |
| 13 | `counter` | `list` | ``docs/modes/rac_mode_counter.md`` | ``docs/messages/rac_message_formats_counter.md`` | yes | - | - |
| 14 | `counter` | `info` | `-` | `-` | yes | - | - |
| 15 | `counter` | `update` | `-` | `-` | yes | - | - |
| 16 | `counter` | `values` | `-` | `-` | yes | - | - |
| 17 | `counter` | `accumulated-values` | `-` | `-` | yes | - | - |
| 18 | `counter` | `clear` | `-` | `-` | yes | - | - |
| 19 | `counter` | `remove` | `-` | `-` | yes | - | - |
| 20 | `limit` | `info` | `-` | `-` | yes | - | - |
| 21 | `limit` | `update` | `-` | `-` | yes | - | - |
| 22 | `limit` | `remove` | `-` | `-` | yes | - | - |
| 23 | `service-setting` | `list` | ``docs/modes/rac_mode_service-setting.md`` | ``docs/messages/rac_message_formats_service-setting.md`` | yes | - | - |
| 24 | `service-setting` | `info` | `-` | `-` | yes | - | - |
| 25 | `service-setting` | `insert` | `-` | `-` | yes | - | - |
| 26 | `service-setting` | `update` | `-` | `-` | yes | - | - |
| 27 | `service-setting` | `get-service-data-dirs-for-transfer` | `-` | `-` | yes | - | - |
| 28 | `service-setting` | `remove` | `-` | `-` | yes | - | - |
| 29 | `service-setting` | `apply` | `-` | `-` | yes | - | - |
| 30 | `agent` | `admin list` | ``docs/modes/rac_mode_agent.md`` | `-` | yes | - | - |
