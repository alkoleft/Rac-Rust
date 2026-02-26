# RAC v16 Format Parse Tasklist (gaps/tails/no capture)

Источник: `docs/rac/messages/rac_message_formats_*.md` и `docs/rac/messages/rac_v11_capture_plan_prompt.md`.

**Capture Requirements (v11)**
- [ ] Использовать `scripts/rac/capture_rac_command.sh` и сохранять: `logs/session_*/client_to_server.stream.bin`, `logs/session_*/server_to_client.stream.bin`, payload в `artifacts/rac/<label>.hex`, вывод `rac` в `artifacts/rac/<label>_rac.out`.
- [ ] Для подтверждения полей/смещений делать минимум 2 контрастных варианта значений (ноль/неноль, пусто/непусто, разные enum).

**Agent**
- [ ] `rac agent admin list`: захват с `auth=pwd,os`, `os-user` и `descr` непустыми; сопоставить `unknown_tag/unknown_flags/unknown_tail` с полями `auth/os-user/descr` и проверить несколько записей.
- [ ] `rac agent version`: получить захват запроса/ответа и подтвердить порядок/формат ответа.
- [ ] `rac agent admin register`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac agent admin remove`: получить захват запроса/ответа, восстановить порядок полей.

**Cluster**
- [ ] `rac cluster list`: захват с ненулевыми `max-memory-size`, `max-memory-time-limit`, `errors-count-threshold`, с `allow-access-right-audit-events-recording` и непустым `restart-schedule`; подтвердить смещения и расшифровать `tail[32]` (v16).
- [ ] `rac cluster info`: подтвердить те же неизвестные поля/хвост, что и в `cluster list`.
- [ ] `rac cluster admin list`: захват с `auth=pwd,os`, `os-user`, `descr`; идентифицировать `unknown_1/unknown_2` и порядок строк.
- [ ] `rac cluster admin register`: захват с `--auth=pwd,os` и `--auth=os`; подтвердить `auth_flags` и смысл `unknown_0` в хвосте запроса.
- [ ] `rac cluster admin remove`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac cluster insert`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac cluster update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac cluster remove`: получить захват запроса/ответа, восстановить порядок полей.

**Connection**
- [ ] `rac connection list`: захват с ненулевым `blocked-by-ls` для подтверждения поля `blocked_by_ls`.
- [ ] `rac connection list`: захват с `--process` для определения порядка полей запроса.
- [ ] `rac connection list`: захват с `--infobase` для определения порядка полей запроса.
- [ ] `rac connection list`: захват с `--infobase-user` для определения порядка полей запроса.
- [ ] `rac connection list`: захват с `--infobase-pwd` для определения порядка полей запроса.
- [ ] `rac connection info`: захват запроса/ответа для определения порядка полей запроса.
- [ ] `rac connection disconnect`: получить захват запроса/ответа, восстановить порядок полей.

**Infobase**
- [ ] `rac infobase summary list`: захват с разными значениями, чтобы идентифицировать `u8 tag` в записи.
- [ ] `rac infobase info`: получить v11-захват payload и сопоставить неизвестные строки/байты и `tail[28]` с полями.
- [ ] `rac infobase summary info`: получить захват и подтвердить формат ответа.
- [ ] `rac infobase summary update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac infobase create`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac infobase update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac infobase drop`: получить захват запроса/ответа, восстановить порядок полей.

**Lock**
- [ ] `rac lock list`: захват с `descr-flag != 0x01` для подтверждения значений и смысла.
- [ ] `rac lock list`: захват с ненулевым `object` UUID для подтверждения хвоста записи.
- [ ] `rac lock list --session`: захват, чтобы проверить наличие дополнительных полей в запросе.

**Process**
- [ ] `rac process list/info`: определить `gap_0` (8 байт) и неизвестный хвост после license block; нужен захват с вариативными значениями (разные серверы/нагрузка).
- [ ] `rac process list/info --licenses`: определить `gap_license_0` и подтвердить хвостовые поля `use`/`reserve`/`memory-excess-time`.

**Server**
- [ ] `rac server list/info`: определить назначения `gap_1..gap_6` (ожидаемые `memory-limit`, `connections-limit`, `safe-working-processes-memory-limit`), подтвердить порядок `port-range` и поведение `gap_7` с непустым `restart-schedule`.
- [ ] `rac server insert`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac server update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac server remove`: получить захват запроса/ответа, восстановить порядок полей.

**Session**
- [ ] `rac session info`: получить v11-захват payload для подтверждения порядка/наличия полей.
- [ ] `rac session list/info --licenses`: захват запроса, чтобы зафиксировать наличие и порядок `licenses` флага.
- [ ] `rac session terminate`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac session interrupt-current-server-call`: получить захват запроса/ответа, восстановить порядок полей.

**Profile**
- [ ] `rac profile list`: захват с непустым списком для подтверждения формата записи.
- [ ] `rac profile update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile remove`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl directory list`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl directory update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl directory remove`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl com list`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl com update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl com remove`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl addin list`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl addin update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl addin remove`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl module list`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl module update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl module remove`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl app list`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl app update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl app remove`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl inet list`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl inet update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac profile acl inet remove`: получить захват запроса/ответа, восстановить порядок полей.

**Counter**
- [ ] `rac counter list/info/values/accumulated-values`: захваты с разными enum (`group`, `filter-type`, `duration`, и др.), подтвердить кодировки.
- [ ] `rac counter update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac counter clear`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac counter remove`: получить захват запроса/ответа, восстановить порядок полей.

**Limit**
- [ ] `rac limit list/info`: захваты с разными `action` и ненулевыми лимитами, подтвердить enum и поля.
- [ ] `rac limit update`: получить захват запроса/ответа, восстановить порядок полей.
- [ ] `rac limit remove`: получить захват запроса/ответа, восстановить порядок полей.

**Rule**
- [ ] `rac rule apply --partial`: захват для подтверждения enum `apply_mode`.
- [ ] `rac rule list`: захват с непустыми `infobase-name`, `application-ext` и `priority>0`; проверить `object-type` и `rule-type`.
- [ ] `rac rule info`: захват с `rule-type=always/never` и `priority>0` для подтверждения enum и порядка.
- [ ] `rac rule insert`: захват с непустыми `infobase-name`, `application-ext`, `priority>0` для подтверждения смещений.
- [ ] `rac rule update`: захват с непустыми полями для подтверждения смещений и наличия предварительного `rule info`.
- [ ] `rac rule remove`: захват ошибки (неуспешный запрос) для описания формата ошибок.
