# A release cannot silently fall back to unsigned files when signing is requested.
param([switch]$RequireSigning)
$ErrorActionPreference = 'Stop'
$enabled = $env:SIGNPATH_ENABLED -eq 'true'
if (-not $enabled) {
    if ($RequireSigning) { throw 'Public Windows releases require trusted code signing. Configure the certificate/service first.' }
    Write-Host 'Code signing is not configured: these are unsigned preview artifacts.'
    exit 0
}
$required = @('SIGNPATH_API_TOKEN', 'SIGNPATH_ORGANIZATION_ID', 'SIGNPATH_PROJECT_SLUG', 'SIGNPATH_SIGNING_POLICY', 'SIGNPATH_EXPECTED_PUBLISHER')
foreach ($name in $required) {
    if ([string]::IsNullOrWhiteSpace([Environment]::GetEnvironmentVariable($name))) {
        throw "Missing code-signing setting: $name"
    }
}
Write-Host 'Code signing configuration is present. Files must still pass real Windows trust verification.'
