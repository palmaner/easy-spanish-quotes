# Read-only experiment evidence. Run inside the disposable Windows test VM.
param([switch]$SingleUserVm)

$ErrorActionPreference = 'Stop'
if (-not $SingleUserVm) {
    throw 'Baseline collection requires -SingleUserVm inside the disposable test VM.'
}

$baselineIdentity = [Security.Principal.WindowsIdentity]::GetCurrent()
$baselinePrincipal = [Security.Principal.WindowsPrincipal]::new($baselineIdentity)
$baselineOs = Get-CimInstance Win32_OperatingSystem
$baselineSecureBoot = $null
$baselineTpm = $null
$baselineLimitations = @()
try {
    $baselineSecureBoot = Confirm-SecureBootUEFI
} catch {
    $baselineLimitations += 'Secure Boot state could not be queried with this token.'
}
try {
    $baselineTpm = Get-Tpm | Select-Object TpmPresent, TpmReady, TpmEnabled, TpmActivated
} catch {
    $baselineLimitations += 'TPM state could not be queried with this token.'
}
$baselineDefaultInput = Get-WinDefaultInputMethodOverride
$baselineLanguages = @(Get-WinUserLanguageList | Select-Object LanguageTag, InputMethodTips)
$baselinePreload = Get-ItemProperty -LiteralPath 'HKCU:\Keyboard Layout\Preload' -ErrorAction SilentlyContinue
$baselinePreloadValues = @()
if ($null -ne $baselinePreload) {
    $baselinePreloadValues = @($baselinePreload.PSObject.Properties |
        Where-Object { $_.Name -match '^[0-9]+$' } |
        Sort-Object { [int]$_.Name } |
        ForEach-Object { [ordered]@{ position = $_.Name; value = $_.Value } })
}
$baselineOriginal = Get-ItemProperty -LiteralPath 'HKLM:\SYSTEM\CurrentControlSet\Control\Keyboard Layouts\0000040A' -ErrorAction SilentlyContinue

[ordered]@{
    os = $baselineOs.Caption
    version = $baselineOs.Version
    build = $baselineOs.BuildNumber
    owner_sid = $baselineIdentity.User.Value
    # A filtered administrator token can contain this SID while IsInRole is
    # false. Report token membership and active elevation separately.
    administrators_sid_in_token = @($baselineIdentity.Groups.Value) -contains 'S-1-5-32-544'
    elevated = $baselinePrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    languages = $baselineLanguages
    default_input_override = if ($null -eq $baselineDefaultInput) { $null } else { $baselineDefaultInput.InputMethodTip }
    preload = $baselinePreloadValues
    original_spanish_registered = $null -ne $baselineOriginal
    original_spanish_layout_file = if ($null -eq $baselineOriginal) { $null } else { $baselineOriginal.'Layout File' }
    secure_boot = $baselineSecureBoot
    tpm = $baselineTpm
    limitations = $baselineLimitations
    calling_thread_layout = 'Collect separately with easy-spanish-quotes status.'
} | ConvertTo-Json -Depth 6
