# Windows: Defender и подпись NIROVA

Предупреждение SmartScreen о неизвестном приложении — не тот же результат, что обнаружение угрозы Microsoft Defender Antivirus. Для диагностики нужны точное имя угрозы из журнала защиты, путь заблокированного файла, версия NIROVA и SHA-256. Нельзя заранее объявлять обнаружение ложным или обещать отсутствие предупреждений.

В сборке Windows записываются CompanyName, ProductName, FileDescription, OriginalFilename и версия NIROVA. Это сведения о файле, а не цифровая подпись и не доказательство отсутствия угроз.

После сборки CI проверяет исполняемый файл и установщик через доступный Microsoft Defender и сохраняет `nirova-defender-report`. Если антивирус отсутствует, выключен или сканирование не удалось, отчёт содержит `NOT_SCANNED`, а не чистый результат. Обнаружение в выходных файлах останавливает build job. Отчёт сохраняет версию движка, сигнатур и SHA-256 проверяемых файлов. Даже чистый результат этого сканирования не гарантирует результат на другом компьютере или с другими сигнатурами.

Если файл ошибочно определяется как вредоносный, его можно отправить Microsoft как Software developer вместе с точным обнаружением. Это отдельное действие владельца; автоматической отправки файлов нет.

- Microsoft: https://www.microsoft.com/en-us/wdsi/filesubmission/
- SmartScreen: https://learn.microsoft.com/windows/apps/package-and-deploy/smartscreen-reputation
- Командные проверки Defender: https://learn.microsoft.com/defender-endpoint/command-line-arguments-microsoft-defender-antivirus

## Две разные подписи

`RELEASE_SIGNING_KEY` — собственный Ed25519-ключ NIROVA для контрольных сумм релиза. Он не заменяет Authenticode-подпись EXE доверенным сертификатом Windows и не устраняет предупреждения SmartScreen автоматически.

Первичная настройка из локального клона репозитория, с установленными Rust и GitHub CLI:

```bash
gh auth login
bash tools/release/setup-nirova-key.sh
```

Скрипт не запрашивает токен в чате. Он генерирует приватный ключ и передаёт его напрямую GitHub CLI для зашифрованного сохранения в repository secret `RELEASE_SIGNING_KEY`. Приватный ключ не выводится на экран и не пишется в файл. Если secret уже существует, скрипт прекращает работу, чтобы не заменить действующий ключ.

После успешной настройки нужно закоммитить и отправить только `crates/horizon-release-sign/release-keys.txt`: этот файл содержит публичный ключ. Затем заново собрать пакеты, чтобы они доверяли именно ключу NIROVA. Старый публичный ключ исходного проекта не позволяет восстановить его приватный ключ.

## Доверенная подпись EXE

**Текущий статус:** сертификат издателя или одобренный профиль сервиса ещё не предоставлены. Метаданные NIROVA и Ed25519-подпись контрольных сумм не являются Authenticode. Наличие готовой интеграции не означает, что уже выпущенные файлы подписаны.

В workflow подготовлена интеграция SignPath с закреплённой версией action. Сервис может использовать доверенный сертификат издателя; бесплатный сертификат SignPath Foundation доступен только после отдельного рассмотрения проекта. Его издателем будет **SignPath Foundation**, а не вымышленная юридическая компания NIROVA. Для собственного имени издателя нужен соответствующий сертификат с проверенной личностью/организацией. Нельзя выдать такой сертификат локальным скриптом.

До выбора сервиса/выпуска сертификата обычные CI-сборки явно остаются unsigned previews. Публичные Windows-релизы требуют включённого и успешно проверенного подписывания. Никакие исключения Defender, отключение Safe Browsing или добавление самоподписанного корня не используются.

После одобрения/подключения сертификата владелец настраивает:

| Настройка GitHub | Тип | Значение |
| --- | --- | --- |
| `SIGNPATH_ENABLED` | Variable | `true`, только когда сертификат и политики готовы |
| `SIGNPATH_ORGANIZATION_ID` | Variable | ID организации в SignPath |
| `SIGNPATH_PROJECT_SLUG` | Variable | Slug проекта, разрешённого для этого репозитория |
| `SIGNPATH_SIGNING_POLICY` | Variable | Политика с production-сертификатом и SHA-256/RFC 3161 timestamp |
| `SIGNPATH_EXPECTED_PUBLISHER` | Variable | Полный Subject DN **фактически выданного** сертификата |
| `SIGNPATH_API_TOKEN` | Secret | Токен submitter для указанного проекта и политики |

В SignPath создаются artifact configurations из `packaging/signpath/launcher-windows.xml`, `installer-windows.xml`, `game-windows.xml` с одноимёнными slug. SignPath GitHub App/Trusted Build System привязывается к этому репозиторию и разрешённым веткам/тегам согласно политике сервиса. Нельзя вместо сертификата production использовать self-signed/test policy.

Порядок: собрать программу → подписать EXE → проверить цепочку Windows, точного издателя, Code Signing EKU и timestamp → упаковать подписанный launcher → построить установщик → подписать установщик → повторно проверить → просканировать финальные файлы Defender → загрузить артефакты → подписать контрольные суммы для автоматического обновления. Rust-клиент и resource preparer подписываются до упаковки игры. Любая ошибка подписывания останавливает соответствующую сборку; silent fallback при включённом signing запрещён.

`*-authenticode.json` содержит реальный `Get-AuthenticodeSignature` status, Subject/Issuer/Thumbprint, timestamp и SHA-256 каждого проверенного файла. Без настройки статус будет `UNSIGNED_OR_UNTRUSTED`, а не «подписано». Встроенный Inno Setup uninstaller отдельной подписи этой интеграцией пока не получает; подпись внешнего Setup.exe не утверждает обратного.

Не передавайте приватные ключи или пароли в чат, коммиты или Library. Ключ CA хранится у провайдера/HSM; токен сервиса — только в GitHub Secrets.

Официальные инструкции:

- https://learn.microsoft.com/windows/apps/package-and-deploy/code-signing-options
- https://learn.microsoft.com/windows/apps/package-and-deploy/smartscreen-reputation
- https://docs.signpath.io/trusted-build-systems/github
- https://signpath.org/terms
- https://signpath.org/apply

## Chrome / Google

Сообщения о малораспространённом файле и о вредоносном содержимом требуют разной диагностики. Нужны точный текст предупреждения, URL файла, версия и SHA-256. Подпись издателя не гарантирует немедленного снятия блокировки Google или SmartScreen. Репутация и результат антивирусной проверки — отдельные сигналы.

Если Google сообщает о проблеме на собственном домене, его проверенный владелец использует Security Issues в Search Console после устранения причины. Владелец репозитория не является владельцем домена github.com, поэтому эта процедура не должна предлагаться как доступ к Search Console GitHub.

- https://support.google.com/webmasters/answer/9044101
- https://developers.google.com/search/docs/monitor-debug/security/malware

## Диагностика уже собранных файлов

Workflow `windows diagnostics` принимает run ID workflow `launcher` и `game`, проверяет репозиторий/имена workflow и успешный Windows job, затем скачивает **именно эти** артефакты. Он не запускает EXE: записывает SHA-256 исходных архивов и четырёх PE-файлов, проверяет Authenticode и сканирует launcher, setup, Rust-клиент и preparer через установленный Defender. Отчёт `nirova-exact-windows-diagnostics` отделяет неподписанный файл от обнаруженной угрозы.
