$cargo = Get-Command cargo -ErrorAction SilentlyContinue
$rustc = Get-Command rustc -ErrorAction SilentlyContinue
$qemu = Get-Command qemu-system-x86_64 -ErrorAction SilentlyContinue

Write-Host "AetherOS environment check"
Write-Host "--------------------------"
Write-Host ("cargo: " + $(if ($cargo) { $cargo.Source } else { "not found" }))
Write-Host ("rustc: " + $(if ($rustc) { $rustc.Source } else { "not found" }))
Write-Host ("qemu-system-x86_64: " + $(if ($qemu) { $qemu.Source } else { "not found" }))
Write-Host ""
Write-Host "Expected components:"
Write-Host "- Rust nightly"
Write-Host "- rust-src"
Write-Host "- llvm-tools-preview"
Write-Host "- QEMU"
Write-Host "- QEMU edk2-x86_64 firmware"
Write-Host "- Internet access for the first Limine bundle download"
