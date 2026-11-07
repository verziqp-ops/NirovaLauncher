$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
. "$PSScriptRoot/authenticode-report.ps1"
$publisher = 'CN=Verified Example Publisher, O=Verified Example Publisher, C=US'
$valid = @{ signature_status = 'Valid'; signer_subject = $publisher; code_signing_eku = $true; timestamp_subject = 'CN=Timestamp authority' }
if (-not (Test-NirovaSignatureRecord $valid $publisher)) { throw 'Valid record rejected.' }
foreach ($change in @(
    @{ signature_status = 'NotSigned' }, @{ signature_status = 'NotTrusted' },
    @{ signature_status = 'HashMismatch' }, @{ signer_subject = 'CN=Someone Else' },
    @{ code_signing_eku = $false }, @{ timestamp_subject = '' }
)) {
    $record = $valid.Clone()
    foreach ($key in $change.Keys) { $record[$key] = $change[$key] }
    if (Test-NirovaSignatureRecord $record $publisher) { throw "Untrusted record accepted: $($change.Keys)" }
}
if (Test-NirovaSignatureRecord $valid '') { throw 'An unspecified publisher was accepted.' }
$work = Join-Path ([IO.Path]::GetTempPath()) ([guid]::NewGuid().ToString())
New-Item -ItemType Directory $work | Out-Null
try {
    $file = Join-Path $work 'unsigned.exe'
    $report = Join-Path $work 'report.json'
    Set-Content $file 'This is an unsigned test fixture, not a program.'
    if ($IsWindows) {
        Write-NirovaAuthenticodeReport -Paths @($file) -Output $report -Publisher $publisher
        $result = Get-Content -Raw $report | ConvertFrom-Json
        if ($result.status -ne 'UNSIGNED_OR_UNTRUSTED' -or $result.files[0].trusted) { throw 'Unsigned fixture claimed to be trusted.' }
        $refused = $false
        try { Write-NirovaAuthenticodeReport -Paths @($file) -Output $report -Publisher $publisher -Enforce } catch { $refused = $true }
        if (-not $refused) { throw 'Unsigned publication was allowed.' }
    }
    $env:SIGNPATH_ENABLED = 'false'
    & pwsh -NoProfile -File "$PSScriptRoot/check-signing-config.ps1"
    if ($LASTEXITCODE -ne 0) { throw 'Unsigned previews should remain available.' }
    & pwsh -NoProfile -File "$PSScriptRoot/check-signing-config.ps1" -RequireSigning 2>$null
    if ($LASTEXITCODE -eq 0) { throw 'An unsigned public release was allowed.' }
    $env:SIGNPATH_ENABLED = 'true'
    $env:SIGNPATH_API_TOKEN = ''
    & pwsh -NoProfile -File "$PSScriptRoot/check-signing-config.ps1" 2>$null
    if ($LASTEXITCODE -eq 0) { throw 'Incomplete signing configuration was accepted.' }
    if ($IsWindows) { Write-Host 'Actual unsigned-file reporting and publication rejection passed.' }
    Write-Host 'Authenticode policy and fail-closed configuration checks passed.'
} finally {
    Remove-Item -LiteralPath $work -Recurse -Force
}

$global:LASTEXITCODE = 0
