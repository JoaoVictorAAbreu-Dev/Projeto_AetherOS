param(
    [switch]$Debug
)

$qemu = Get-Command qemu-system-x86_64 -ErrorAction SilentlyContinue
if (-not $qemu) {
    Write-Error "qemu-system-x86_64 not found in PATH."
    exit 1
}

Write-Host "AetherOS QEMU launcher"
Write-Host "This project still needs a bootable disk or ISO artifact wired into the script."
Write-Host "Suggested next integration:"
Write-Host "- build kernel artifact"
Write-Host "- assemble Limine image"
Write-Host "- launch QEMU with serial stdio"

if ($Debug) {
    Write-Host "Debug mode requested. Recommended QEMU flags: -s -S"
}
