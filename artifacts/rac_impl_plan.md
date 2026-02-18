# RAC Implementation Plan

| # | Mode | Command | Description | Message formats | Analyzed | Implemented | Notes |
|---|------|---------|-------------|-----------------|----------|-------------|-------|
| 1 | `process` | `info` | `-` | `-` | yes | - | - |
| 2 | `connection` | `list` | ``docs/modes/rac_mode_connection.md`` | ``docs/messages/rac_message_formats_connection.md`` | yes | - | - |
| 3 | `connection` | `info` | `-` | `-` | yes | - | - |
| 4 | `lock` | `list` | ``docs/modes/rac_mode_lock.md`` | ``docs/messages/rac_message_formats_lock.md`` | yes | - | - |
| 5 | `rule` | `apply` | ``docs/modes/rac_mode_rule.md`` | ``docs/messages/rac_message_formats_rule.md`` | yes | - | - |
| 6 | `rule` | `list` | `-` | `-` | yes | - | - |
| 7 | `rule` | `info` | `-` | `-` | yes | - | - |
| 8 | `rule` | `insert` | `-` | `-` | yes | - | - |
| 9 | `rule` | `update` | `-` | `-` | yes | - | - |
| 10 | `rule` | `remove` | `-` | `-` | yes | - | - |
| 11 | `counter` | `list` | ``docs/modes/rac_mode_counter.md`` | ``docs/messages/rac_message_formats_counter.md`` | yes | - | - |
| 12 | `counter` | `info` | `-` | `-` | yes | - | - |
| 13 | `counter` | `update` | `-` | `-` | yes | - | - |
| 14 | `counter` | `values` | `-` | `-` | yes | - | - |
| 15 | `counter` | `accumulated-values` | `-` | `-` | yes | - | - |
| 16 | `counter` | `clear` | `-` | `-` | yes | - | - |
| 17 | `counter` | `remove` | `-` | `-` | yes | - | - |
| 18 | `limit` | `info` | `-` | `-` | yes | - | - |
| 19 | `limit` | `update` | `-` | `-` | yes | - | - |
| 20 | `limit` | `remove` | `-` | `-` | yes | - | - |
| 21 | `service-setting` | `list` | ``docs/modes/rac_mode_service-setting.md`` | ``docs/messages/rac_message_formats_service-setting.md`` | yes | - | - |
| 22 | `service-setting` | `info` | `-` | `-` | yes | - | - |
| 23 | `service-setting` | `insert` | `-` | `-` | yes | - | - |
| 24 | `service-setting` | `update` | `-` | `-` | yes | - | - |
| 25 | `service-setting` | `get-service-data-dirs-for-transfer` | `-` | `-` | yes | - | - |
| 26 | `service-setting` | `remove` | `-` | `-` | yes | - | - |
| 27 | `service-setting` | `apply` | `-` | `-` | yes | - | - |
| 28 | `agent` | `admin list` | ``docs/modes/rac_mode_agent.md`` | `-` | yes | - | - |
