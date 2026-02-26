# RAC v16 Capture Tasks (Readonly)

Статусы:
- Захват: `[x]` выполнен, `[ ]` нет. Для пустого вывода — пометка `пустой`.
- Анализ: `нет`, `успех` (нет tail), `gap` (есть хвост/пробелы).
- Описание: `нет`, `частично`, `полностью`.

| Команда | Захват | Анализ | Описание |
| --- | --- | --- | --- |
| `agent admin list` | [x] `artifacts/rac/v16/help/agent_admin_list.out` | нет | нет |
| `agent version` | [x] `artifacts/rac/v16/help/agent_version.out` | нет | нет |
| `cluster list` | [x] `artifacts/rac/v16/help/cluster_list.out` | gap | частично |
| `cluster info` | [x] `artifacts/rac/v16/help/cluster_info.out` | gap | частично |
| `cluster admin list` | [x] `artifacts/rac/v16/help/cluster_admin_list.out` | gap | частично |
| `manager list` | [x] `artifacts/rac/v16/help/manager_list.out` | успех | полностью |
| `manager info` | [x] `artifacts/rac/v16/help/manager_info.out` | успех | полностью |
| `server list` | [x] `artifacts/rac/v16/help/server_list.out` | нет | нет |
| `server info` | [x] `artifacts/rac/v16/help/server_info.out` | нет | нет |
| `process list` | [x] `artifacts/rac/v16/help/process_list.out` | нет | нет |
| `process info` | [x] `artifacts/rac/v16/help/process_info.out` | нет | нет |
| `service list` | [x] `artifacts/rac/v16/help/service_list.out` | нет | нет |
| `infobase summary list` | [x] `artifacts/rac/v16/help/infobase_summary_list.out` | нет | нет |
| `infobase summary info` | [x] `artifacts/rac/v16/help/infobase_summary_info.out` | нет | нет |
| `infobase info` | [x] `artifacts/rac/v16/help/infobase_info.out` | нет | нет |
| `connection list` | [x] `artifacts/rac/v16/help/connection_list.out` | нет | нет |
| `connection info` | [x] `artifacts/rac/v16/help/connection_info.out` | нет | нет |
| `session list` | [x] `artifacts/rac/v16/help/session_list.out` | нет | нет |
| `lock list` | [x] `artifacts/rac/v16/help/lock_list.out` | нет | нет |
| `rule list` | [x] пустой: `artifacts/rac/v16/help/rule_list.out` | gap | частично |
| `profile list` | [x] пустой: `artifacts/rac/v16/help/profile_list.out` | нет | нет |
| `counter list` | [x] `artifacts/rac/v16/help/counter_list.out` | gap | частично |
| `counter info` | [x] `artifacts/rac/v16/help/counter_info.out` | gap | частично |
| `counter values` | [x] `artifacts/rac/v16/help/counter_values.out` | gap | частично |
| `counter accumulated-values` | [x] `artifacts/rac/v16/help/counter_accumulated_values.out` | gap | частично |
| `limit list` | [x] `artifacts/rac/v16/help/limit_list.out` | gap | частично |
| `limit info` | [x] `artifacts/rac/v16/help/limit_info.out` | gap | частично |
| `service-setting list` | [x] пустой: `artifacts/rac/v16/help/service_setting_list.out` | gap | частично |
| `service-setting get-service-data-dirs-for-transfer` | [x] `artifacts/rac/v16/help/service_setting_get_service_data_dirs_for_transfer.out` | gap | частично |
| `binary-data-storage list` | [x] ошибка прав: `artifacts/rac/v16/help/binary_data_storage_list.out` | нет | нет |
