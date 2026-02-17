# v8_protocols

Эксперимент по реверс‑инжинирингу протоколов 1С с помощью AI.  
Цель: собирать трафик, раскладывать его на обмены, строить карты методов и постепенно заменять официальный `rac` на собственные утилиты.

## Что внутри

- `v8_protocols` — TCP‑прокси для перехвата и логирования сессий.
- `apps/rac_protocol` — инструменты для RAC: декодер фрейминга и минимальный клиент.
- `docs/` — заметки по протоколу и карта методов.

## Быстрый старт

Сборка:

```bash
cargo build --release -p v8_protocols
cargo build --release -p rac_protocol
```

Запуск прокси:

```bash
cargo run --release -p v8_protocols -- \
  --listen 127.0.0.1:15410 \
  --target 127.0.0.1:1541 \
  --log-dir ./logs \
  --try-inflate=true
```

Декодирование потока RAC:

```bash
cargo run -p rac_protocol --bin rac_decode -- logs/<session>/client_to_server.stream.bin
cargo run -p rac_protocol --bin rac_decode -- logs/<session>/server_to_client.stream.bin
```

Мини‑клиент без `rac` (WIP):

```bash
cargo run -p rac_protocol --bin rac_lite -- agent-version 127.0.0.1:1545
cargo run -p rac_protocol --bin rac_lite -- cluster-list 127.0.0.1:1545
```

## Как использовать в исследовании

1. Прокси ставится между клиентом 1С и сервером.
2. Трафик сохраняется в `logs/`, раскладывается на пары request/response.
3. Дальше — анализ фрейминга, соответствия методов, построение карты команд в `docs/`.

## Статус

Проект экспериментальный, многое WIP. Форматы и утилиты будут меняться по мере накопления наблюдений.

## Полезные файлы

- `docs/documentation/rac_protocol_notes.md`
- `docs/documentation/rac_method_map.md`
