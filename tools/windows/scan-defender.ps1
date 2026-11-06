# Scan build outputs with the installed Microsoft Defender engine. Never add exclusions.
param([Parameter(Mandatory=$true)][string]$Folder, [Parameter(Mandatory=$true)][string]$Report)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path $Folder).Path
$result = [ordered]@{ status = 'NOT_SCANNED'; folder = $root; files = @(); reason = '' }
$result.files = @(Get-ChildItem $root -File | ForEach-Object {
    @{ name = $_.Name; sha256 = (Get-FileHash $_.FullName -Algorithm SHA256).Hash }
})
try {
    $status = Get-MpComputerStatus
    $result.engine = $status.AMEngineVersion
    $result.signature = $status.AntivirusSignatureVersion
    if (-not $status.AntivirusEnabled -or -not $status.AMServiceEnabled) {
        $result.reason = 'Defender Antivirus is not active on this runner; no clean verdict is claimed.'
    } else {
        try { Update-MpSignature } catch { $result.signature_update_error = $_.Exception.Message }
        $result.signature = (Get-MpComputerStatus).AntivirusSignatureVersion
        $scanner = Get-ChildItem "$env:ProgramData\Microsoft\Windows Defender\Platform\*\MpCmdRun.exe" -ErrorAction SilentlyContinue |
            Sort-Object LastWriteTime -Descending | Select-Object -First 1 -ExpandProperty FullName
        if (-not $scanner) { $scanner = Join-Path $env:ProgramFiles 'Windows Defender\MpCmdRun.exe' }
        if (-not (Test-Path $scanner)) { throw 'Defender command-line scanner is unavailable.' }
        $result.scan_log = @(& $scanner -Scan -ScanType 3 -File $root 2>&1 | ForEach-Object { "$_" })
        $result.scan_exit_code = $LASTEXITCODE
        $detections = @(Get-MpThreatDetection | Where-Object {
            $matchesRoot = $false
            foreach ($resource in $_.Resources) {
                if ($resource.IndexOf($root, [System.StringComparison]::OrdinalIgnoreCase) -ge 0) { $matchesRoot = $true }
            }
            $matchesRoot
        })
        $result.detections = $detections
        $result.threats = @(Get-MpThreat)
        if ($detections.Count -gt 0) {
            $result.status = 'THREAT_DETECTED'
        } elseif ($result.scan_exit_code -eq 0) {
            $result.status = 'SCAN_COMPLETED_NO_DETECTIONS'
        } else {
            $result.reason = "Defender scan did not complete successfully (exit $($result.scan_exit_code)); no clean verdict is claimed."
        }
    }
} catch {
    $result.reason = $_.Exception.Message
}
$result | ConvertTo-Json -Depth 8 | Set-Content -Encoding UTF8 $Report
Write-Host "Defender result: $($result.status). See defender-scan.json for engine and signature versions."
if ($result.status -eq 'THREAT_DETECTED') { throw 'Defender detected a build output. Publication is blocked; review the report.' }
if ($result.status -eq 'NOT_SCANNED') { Write-Warning $result.reason }
