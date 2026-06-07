param(
    [switch]$Debug
)

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargo) {
    Write-Error "cargo not found in PATH."
    exit 1
}

if ($Debug) {
    cargo run -p xtask -- run --debug
} else {
    cargo run -p xtask -- run
}
