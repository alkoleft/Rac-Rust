# Prompt: План доисследований RAC v11 (захваты/параметры)

Цель: собрать недостающие захваты для RAC v11 и закрыть gaps/tails/гипотезы в `docs/rac/messages/rac_message_formats_*.md`.
Формат: для каждой команды выполнить указанные варианты запуска `rac` и сохранить захваты в `logs/` + извлечь payloads в `artifacts/rac/`.

## Общие требования к захватам

- Использовать `scripts/rac/capture_rac_command.sh`.
- Для каждого сценария сохранять:
  - `logs/session_*/client_to_server.stream.bin`
  - `logs/session_*/server_to_client.stream.bin`
  - извлеченный payload в `artifacts/rac/<label>.hex`
  - текстовый вывод `rac` в `artifacts/rac/<label>_rac.out`
- Для новых подтверждений поля/смещения: минимум 2 контрастных значения (ноль/неноль, пусто/непусто, разные enum).

Статусы:
- `[ ]` не выполнено
- `[~]` частично (есть базовый захват/частичное описание)
- `[x]` выполнено

## Команды без захватов (выполнить первыми)

### agent
- `rac agent admin register --name <name> --pwd <pwd> --descr <descr> --auth=pwd[,os] --os-user <user> --agent-user <admin> --agent-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
  - Варианты: `--auth=pwd` и `--auth=pwd,os` с `--os-user`.
- `rac agent admin remove --name <name> --agent-user <admin> --agent-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]

### cluster
- `rac cluster admin remove --cluster <id> --name <name> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac cluster insert --host <host> --port <port> --name <name> --expiration-timeout <n> --lifetime-limit <n> --max-memory-size <n> --max-memory-time-limit <n> --security-level <n> --session-fault-tolerance-level <n> --load-balancing-mode performance|memory --errors-count-threshold <n> --kill-problem-processes yes|no --kill-by-memory-with-dump yes|no --agent-user <user> --agent-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
  - Варианты: `load-balancing-mode=performance|memory`.
- `rac cluster update --cluster <id> --name <name> --expiration-timeout <n> --lifetime-limit <n> --max-memory-size <n> --max-memory-time-limit <n> --security-level <n> --session-fault-tolerance-level <n> --load-balancing-mode performance|memory --errors-count-threshold <n> --kill-problem-processes yes|no --kill-by-memory-with-dump yes|no --agent-user <user> --agent-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac cluster remove --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]

### connection
- `rac connection disconnect --cluster <id> --process <id> --connection <id> --infobase-user <user> --infobase-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]

### infobase
- `rac infobase summary update --cluster <id> --infobase <id> --descr <descr> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac infobase create --cluster <id> --create-database --name <name> --dbms <dbms> --db-server <host> --db-name <name> --locale <loc> --db-user <user> --db-pwd <pwd> --descr <descr> --date-offset <offset> --security-level <n> --scheduled-jobs-deny on|off --license-distribution deny|allow --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac infobase update --cluster <id> --infobase <id> --dbms <dbms> --db-server <host> --db-name <name> --db-user <user> --db-pwd <pwd> --descr <descr> --date-offset <offset> --security-level <n> --scheduled-jobs-deny on|off --license-distribution deny|allow --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac infobase drop --cluster <id> --infobase <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]

### server
- `rac server insert --cluster <id> --agent-host <host> --agent-port <port> --port-range <min:max> --name <name> --using main|normal --infobases-limit <n> --memory-limit <n> --connections-limit <n> --cluster-port <port> --dedicate-managers all|none --safe-working-processes-memory-limit <n> --safe-call-memory-limit <n> --critical-total-memory <n> --temporary-allowed-total-memory <n> --temporary-allowed-total-memory-time-limit <n> --service-principal-name <spn> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac server update --cluster <id> --server <id> --port-range <min:max> --using main|normal --infobases-limit <n> --memory-limit <n> --connections-limit <n> --dedicate-managers all|none --safe-working-processes-memory-limit <n> --safe-call-memory-limit <n> --critical-total-memory <n> --temporary-allowed-total-memory <n> --temporary-allowed-total-memory-time-limit <n> --service-principal-name <spn> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac server remove --cluster <id> --server <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]

### session
- `rac session terminate --cluster <id> --session <id> --error-message <msg> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac session interrupt-current-server-call --cluster <id> --session <id> --error-message <msg> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]

### service
- `rac service list --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [x], анализ [x], описание [x]

### profile
- `rac profile list --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [x], анализ [ ], описание [~]
- `rac profile update --cluster <id> --name <name> --descr <descr> --config yes|no --priv yes|no --full-privileged-mode yes|no --privileged-mode-roles <list> --crypto yes|no --right-extension yes|no --right-extension-definition-roles <list> --all-modules-extension yes|no --modules-available-for-extension <list> --modules-not-available-for-extension <list> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile remove --cluster <id> --name <name> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl directory list --name <profile> --access list|full --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl directory update --name <profile> --access list|full --alias <url> --descr <descr> --physicalPath <url> --allowedRead yes|no --allowedWrite yes|no --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl directory remove --name <profile> --access list|full --alias <url> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl com list --name <profile> --access list|full --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl com update --name <profile> --access list|full --name <com> --descr <descr> --fileName <url> --id <uuid> --host <url> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl com remove --name <profile> --access list|full --name <com> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl addin list --name <profile> --access list|full --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl addin update --name <profile> --access list|full --name <addin> --descr <descr> --hash <base64> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl addin remove --name <profile> --access list|full --name <addin> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl module list --name <profile> --access list|full --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl module update --name <profile> --access list|full --name <module> --descr <descr> --hash <base64> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl module remove --name <profile> --access list|full --name <module> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl app list --name <profile> --access list|full --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl app update --name <profile> --access list|full --name <app> --descr <descr> --wild <url> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl app remove --name <profile> --access list|full --name <app> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl inet list --name <profile> --access list|full --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl inet update --name <profile> --access list|full --name <inet> --descr <descr> --protocol <proto> --url <url> --port <n> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]
- `rac profile acl inet remove --name <profile> --access list|full --name <inet> --cluster <id> --cluster-user <user> --cluster-pwd <pwd>` — Статус: захват [ ], анализ [ ], описание [ ]

## Команды с gaps/tails/гипотезами (нужны вариативные захваты)

### agent admin list
- Создать минимум 2 администратора с:
  - `auth=pwd` и `auth=pwd,os`
  - непустые `os-user` и `descr`
- Захватить `rac agent admin list` и сопоставить поля. — Статус: захват [~], анализ [ ], описание [~]

### cluster list/info
- Захватить `cluster list` после установки:
  - `--max-memory-size` ненулевой
  - `--max-memory-time-limit` ненулевой
  - `--errors-count-threshold` ненулевой
  - `--allow-access-right-audit-events-recording` (если доступно)
  - `--restart-schedule` (если доступно)
- Сравнить tail в v11/v16, подтвердить неизвестные поля. — Статус: захват [~], анализ [ ], описание [~]

### cluster admin list
- Создать администратора с `auth=pwd,os`, `os-user`, `descr`. — Статус: захват [~], анализ [ ], описание [~]

### cluster admin register
- Варианты `--auth=pwd` и `--auth=pwd,os`.
- Проверить `auth_flags` и хвост `unknown_0`. — Статус: захват [ ], анализ [ ], описание [ ]

### connection list
- Спровоцировать `blocked-by-ls` ненулевое значение (нагрузка/блокировки) и перезахватить. — Статус: захват [~], анализ [ ], описание [~]

### lock list
- Сделать захваты:
  - `rac lock list --infobase <uuid>`
  - `rac lock list --connection <uuid>`
  - `rac lock list --session <uuid>`
- Найти записи с `descr-flag` != `0x01`. — Статус: захват [~], анализ [ ], описание [~]

### server list/info
- Перезахваты при ненулевых:
  - `--memory-limit`
  - `--connections-limit`
  - `--safe-working-processes-memory-limit`
Статус: захват [~], анализ [ ], описание [~]

### rule apply
- Захватить `rac rule apply --partial` и сравнить enum. — Статус: захват [ ], анализ [ ], описание [ ]

### rule list/info/insert/update/remove
- Сделать захваты с различными `rule-type` и `object-type`, `priority` и `application-ext`. — Статус: захват [~], анализ [ ], описание [~]

### counter list/info/update/values/accumulated-values/clear/remove
- Сгенерировать несколько счетчиков с разными enum-значениями.
- Сравнить кодировки `group/filter-type/duration/...`. — Статус: захват [~], анализ [ ], описание [~]

### limit list/info/update/remove
- Изменить значения лимитов (ненулевые для всех u64).
- Захватить `action` разные значения и `error-message` непустой. — Статус: захват [~], анализ [ ], описание [~]

### process list/info
- Захватить варианты с `--licenses` и без, а также на разных серверах. — Статус: захват [~], анализ [ ], описание [~]

## Итог

После сбора захватов:
- Обновить `docs/rac/messages/rac_message_formats_*.md`.
- Зафиксировать новые RPC method IDs (если появятся) в `docs/rac/documentation/rac_cli_method_map.generated.md`.
- Добавить/уточнить offsets и типы в таблицах.

## Статус по репозиторию (локальная проверка артефактов)

Ниже — что уже есть в `artifacts/rac/` и чего нет. Это не заменяет новые захваты, а лишь фиксирует текущее состояние.

Сейчас подтверждены захваты (v11):
- `agent admin list` (см. `artifacts/rac/v11_agent_admin_list_ro_*`).
- `cluster list` (см. `artifacts/rac/v11_cluster_list_ro_*`).
- `cluster info` (см. `artifacts/rac/v11_cluster_info_ro_*`).
- `cluster admin list` (см. `artifacts/rac/v11_cluster_admin_list_ro_*`).
- `connection list` (см. `artifacts/rac/v11_connection_list_ro_*`).
- `lock list` (см. `artifacts/rac/v11_lock_list_*_ro_*`).
- `server list/info` (см. `artifacts/rac/v11_server_list_ro_*`, `artifacts/rac/v11_server_info_ro_*`).
- `process list/info` + `--licenses` (см. `artifacts/rac/v11_process_*_ro_*`).
- `rule list` (см. `artifacts/rac/v11_rule_list_ro_*`).
- `counter list/info/values/accumulated-values` (см. `artifacts/rac/v11_counter_*_ro_*`).
- `limit list/info` (см. `artifacts/rac/v11_limit_*_ro_*`).
- `service list` (см. `artifacts/rac/v11_service_list_ro_*`).
- `profile list` (см. `artifacts/rac/v11_profile_list_ro_*`).
- `session list` (см. `artifacts/rac/v11_session_list_ro_*`).
- `infobase summary list` (см. `artifacts/rac/v11_infobase_summary_list_ro_*`).
- негативные сценарии (см. `artifacts/rac/v11_error_*`).

Требуют новых захватов:
- Все команды из раздела “Команды без захватов” (agent register/remove, cluster admin remove/insert/update/remove, connection disconnect, infobase summary update/create/update/drop, server insert/update/remove, session terminate/interrupt, profile update/remove/acl*).
- `connection info` (нет успешных `v11_` артефактов, есть только `v11_error_*`).
- `session info` (нет успешных `v11_` артефактов).
- `infobase info` (нет успешных `v11_` артефактов).
- `rule apply` (нет `v11_` захвата).
- `rule info/insert/update/remove` (нет `v11_` захватов).
- `counter update/clear/remove` (нет `v11_` захватов).
- `limit update/remove` (нет `v11_` захватов).
- Вариативные захваты из раздела “Команды с gaps/tails/гипотезами”: `agent admin list` с `auth=pwd,os`, `cluster list` с ненулевыми `max-memory-*`/`errors-count-threshold`, `cluster admin list` с `auth=pwd,os`, `connection list` с `blocked-by-ls`, `lock list` с `descr-flag` != `0x01`, `server list/info` с ненулевыми лимитами, `rule apply --partial`, `rule *` с разными enum, `counter *` с разными enum, `limit *` с разными `action`, `process list/info` на разных серверах.
