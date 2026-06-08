param(
    [int]$TimeoutSeconds = 30
)

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargo) {
    Write-Error "cargo not found in PATH."
    exit 1
}

$env:AETHER_BOOT_TIMEOUT_SECS = "$TimeoutSeconds"
$env:AETHER_QEMU_DISPLAY = "none"

cargo +nightly-x86_64-pc-windows-gnu run -p xtask -- boot-check
exit $LASTEXITCODE
