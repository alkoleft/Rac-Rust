# RAC v11 Capture Tasks (Progress)

Фаза 1: read-only команды (без изменений на сервере).

Статусы:
- [ ] В очереди
- [~] В работе
- [x] Готово

## Read-only: базовые list/info

- [x] agent admin list
- [x] cluster list
- [x] cluster info
- [x] cluster admin list
- [x] connection list
- [x] lock list (без фильтра)
- [x] server list
- [x] server info
- [x] process list
- [x] process list --licenses
- [x] process info
- [x] process info --licenses
- [x] rule list
- [ ] rule info
- [x] counter list
- [x] counter info
- [x] counter values
- [x] counter accumulated-values
- [x] limit list
- [x] limit info
- [x] service list
- [x] profile list

## Read-only: для выборочных захватов

- [x] session list (нужен для lock list --session)
- [x] infobase summary list (нужен для lock list --infobase)

## Read-only: фильтры (если есть IDs)

- [x] lock list --session <id> (payload без session)
- [x] lock list --connection <id>
- [x] lock list --infobase <id>

## Read-only: ошибки (негативные сценарии)

- [x] session list --cluster-user badpwd
- [x] cluster info --cluster <несуществующий uuid>
- [x] session info --session <несуществующий uuid>
- [x] session info --session <некорректный uuid>
- [x] connection info --connection <несуществующий uuid>
- [x] infobase info --infobase <несуществующий uuid>

## Write: команды с изменениями (нет захватов)

- [ ] agent admin register
- [ ] agent admin remove
- [ ] cluster admin remove
- [ ] cluster insert
- [ ] cluster update
- [ ] cluster remove
- [ ] connection disconnect
- [ ] infobase summary update
- [ ] infobase create
- [ ] infobase update
- [ ] infobase drop
- [ ] server insert
- [ ] server update
- [ ] server remove
- [ ] session terminate
- [ ] session interrupt-current-server-call
- [ ] rule insert
- [ ] rule update
- [ ] rule remove
- [ ] counter update
- [ ] counter clear
- [ ] counter remove
- [ ] limit update
- [ ] limit remove
- [ ] profile update
- [ ] profile remove
- [ ] profile acl directory list
- [ ] profile acl directory update
- [ ] profile acl directory remove
- [ ] profile acl com list
- [ ] profile acl com update
- [ ] profile acl com remove
- [ ] profile acl addin list
- [ ] profile acl addin update
- [ ] profile acl addin remove
- [ ] profile acl module list
- [ ] profile acl module update
- [ ] profile acl module remove
- [ ] profile acl app list
- [ ] profile acl app update
- [ ] profile acl app remove
- [ ] profile acl inet list
- [ ] profile acl inet update
- [ ] profile acl inet remove

## Вариативные захваты (gaps/tails)

- [ ] agent admin list с auth=pwd,os и непустыми os-user/descr
- [ ] cluster list с ненулевыми max-memory-*, errors-count-threshold, allow-access-right-audit-events-recording, restart-schedule
- [ ] cluster admin list с auth=pwd,os и непустыми os-user/descr
- [ ] cluster admin register с auth=pwd и auth=pwd,os (проверить auth_flags и unknown tail)
- [ ] connection list с ненулевым blocked-by-ls
- [ ] lock list с descr-flag != 0x01
- [ ] server list/info с ненулевыми memory-limit, connections-limit, safe-working-processes-memory-limit
- [ ] rule list/info с разными rule-type/object-type/priority/application-ext
- [ ] rule apply --partial (сравнить enum)
- [ ] counter list/info/update/values/accumulated-values/clear/remove с разными enum значениями
- [ ] limit list/info/update/remove с ненулевыми u64, разными action, непустым error-message
- [ ] process list/info с --licenses и без, на разных серверах

## Документация

- [x] Обновить `docs/rac/messages/rac_message_formats_*.md` по новым захватам
- [x] Обновить `docs/rac/documentation/rac_cli_method_map.generated.md` при новых RPC id
- [x] Обновить `docs/rac/modes/rac_modes_registry.md` при изменениях
