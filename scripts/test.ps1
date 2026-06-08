$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargo) {
    Write-Error "cargo not found in PATH."
    exit 1
}

Write-Host "Running AetherOS workspace checks through xtask..."
cargo +nightly-x86_64-pc-windows-gnu run -p xtask -- test
exit $LASTEXITCODE
