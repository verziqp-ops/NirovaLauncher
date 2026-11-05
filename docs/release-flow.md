# Релизы NIROVA

Этот документ описывает фактическую схему публикации. Наличие файла в GitHub Releases само по себе не означает, что сборка проверена.

## Пакеты лаунчера

Workflow `.github/workflows/launcher.yml` собирает:

| Платформа | Артефакты |
| --- | --- |
| Windows x64 | `NIROVA-Setup.exe` и `nirova-launcher-windows-x64.zip` |
| Linux x86_64 | `nirova-launcher-linux-x86_64.tar.gz` |
| macOS Apple Silicon | `nirova-launcher-macos-arm64.zip` и `nirova-launcher-macos-arm64.dmg` |
| macOS Intel | `nirova-launcher-macos-x64.zip` и `nirova-launcher-macos-x64.dmg` |

DMG и ZIP создаются из app bundle `NIROVA.app`. Сейчас скрипт подписывает приложение ad-hoc, если не передан Developer ID. Это не равно полноценной Developer ID подписи и notarization; для публичного macOS релиза нужны Apple Developer ID secrets и notarization profile либо отдельная инструкция по Gatekeeper.

## Публикация

1. Обновить версию в `launcher/Cargo.toml`.
2. Проверить lockfile и запустить тесты лаунчера.
3. Убедиться, что PR workflow завершился успешно на всех build jobs.
4. Слить проверенный код в `main`.
5. Создать тег `launcher@X.Y.Z` на точном commit в `main`.
6. Workflow соберёт платформенные архивы, подпишет `SHA256SUMS` и `SHA256SUMS.sig`, проверит подпись тем же механизмом, который использует лаунчер, и опубликует релиз.
7. Проверить состав релиза: каждый ожидаемый файл, контрольные суммы, подпись и release notes.

Секрет `RELEASE_SIGNING_KEY` обязателен для подписанного релиза. Без него публикация должна завершиться ошибкой, а не считаться успешной.

## Разделение каналов

В репозитории одновременно есть game releases с тегами `vX.Y.Z` и launcher releases с тегами `launcher@X.Y.Z`. Нельзя полагаться на общий GitHub alias `releases/latest` для выбора нужного продукта: установщик и self-updater должны выбирать релиз именно с префиксом `launcher@`, а загрузчик игры — игровой тег/канал. Перед следующим публичным релизом нужно проверить все URL выбора последнего релиза.

## Чеклист перед публикацией

- [ ] CI tests зелёные.
- [ ] Windows x64 installer и updater ZIP присутствуют.
- [ ] Linux x86_64 archive присутствует.
- [ ] macOS arm64 ZIP и DMG присутствуют.
- [ ] macOS x64 ZIP и DMG присутствуют.
- [ ] SHA256SUMS содержит все распространяемые бинарные пакеты.
- [ ] SHA256SUMS.sig проходит локальную проверку.
- [ ] Release notes объясняют установку, системные требования и известные ограничения.
- [ ] Ссылки установки ведут на launcher release, а не на game release.
- [ ] Обновление поверх предыдущей версии сохраняет настройки, профили и игровые данные.
- [ ] Для macOS ясно указано состояние Developer ID/notarization.
- [ ] Публичная загрузка не содержит пустых или частичных черновых релизов.

## Ограничение проверки

Сборка на GitHub Actions проверяет компиляцию и тесты. Она не заменяет проверку установки и первого запуска на реальных Windows, Linux и macOS системах. До такой проверки релиз нельзя называть полностью подтверждённым.

## Brand migration boundary

The launcher package and public binary are named `nirova-launcher`. Some internal Rust crate names, source directories, environment variables, game executable names and legacy archive aliases still contain `horizon`. They are compatibility identifiers used across the game engine, updater, user data and release verification; changing them with a blind text replacement could break builds, saves or existing installations. They should be renamed in small dependency-aware changes, with `cargo metadata`, lockfile, installer, update and migration tests. These identifiers are not the product name shown in the launcher.
