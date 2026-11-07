# Inspect real Windows Authenticode trust. Never create certificates or change trust stores.
param(
    [string[]]$Files,
    [string]$Report,
    [string]$ExpectedPublisher = '',
    [switch]$RequireTrusted
)
$ErrorActionPreference = 'Stop'

function Test-NirovaSignatureRecord {
    param($Record, [string]$Publisher)
    return ($Publisher.Length -gt 0 -and
        $Record.signature_status -eq 'Valid' -and
        $Record.signer_subject -ceq $Publisher -and
        $Record.code_signing_eku -eq $true -and
        -not [string]::IsNullOrWhiteSpace($Record.timestamp_subject))
}

function Write-NirovaAuthenticodeReport {
    param([string[]]$Paths, [string]$Output, [string]$Publisher = '', [switch]$Enforce)
    if (-not $Paths -or -not $Output) { throw 'Files and Report are required.' }
    if ($Enforce -and [string]::IsNullOrWhiteSpace($Publisher)) {
        throw 'A verified publisher subject must be configured before requiring trusted signatures.'
    }
    $records = @(foreach ($path in $Paths) {
        $file = Get-Item -LiteralPath $path
        if ($file.PSIsContainer) { throw 'Only regular files can be inspected.' }
        $signature = Get-AuthenticodeSignature -LiteralPath $file.FullName
        $signer = $signature.SignerCertificate
        $timestamp = $signature.TimeStamperCertificate
        $eku = @()
        if ($signer) {
            foreach ($extension in $signer.Extensions) {
                if ($extension -is [System.Security.Cryptography.X509Certificates.X509EnhancedKeyUsageExtension]) {
                    $eku += @($extension.EnhancedKeyUsages | ForEach-Object { $_.Value })
                }
            }
        }
        $record = [ordered]@{
            name = $file.Name
            sha256 = (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash
            signature_status = "$($signature.Status)"
            signer_subject = $(if ($signer) { $signer.Subject } else { '' })
            signer_issuer = $(if ($signer) { $signer.Issuer } else { '' })
            signer_thumbprint = $(if ($signer) { $signer.Thumbprint } else { '' })
            code_signing_eku = ($eku -contains '1.3.6.1.5.5.7.3.3')
            timestamp_subject = $(if ($timestamp) { $timestamp.Subject } else { '' })
        }
        $record.trusted = Test-NirovaSignatureRecord $record $Publisher
        $record
    })
    $allTrusted = @($records | Where-Object { -not $_.trusted }).Count -eq 0
    $result = [ordered]@{
        status = $(if ($allTrusted) { 'ALL_TRUSTED' } else { 'UNSIGNED_OR_UNTRUSTED' })
        expected_publisher = $Publisher
        files = $records
    }
    $result | ConvertTo-Json -Depth 6 | Set-Content -Encoding utf8 $Output
    Write-Host "Authenticode: $($result.status). Report: $Output"
    if ($Enforce -and -not $allTrusted) {
        throw 'A file has no valid timestamped Authenticode signature from the configured publisher.'
    }
}

if ($MyInvocation.InvocationName -ne '.') {
    Write-NirovaAuthenticodeReport -Paths $Files -Output $Report -Publisher $ExpectedPublisher -Enforce:$RequireTrusted
}
