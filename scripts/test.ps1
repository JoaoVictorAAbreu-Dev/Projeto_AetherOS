$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargo) {
    Write-Error "cargo not found in PATH."
    exit 1
}

Write-Host "Running AetherOS workspace checks..."
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

cargo check --workspace
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

cargo test --workspace
exit $LASTEXITCODE
