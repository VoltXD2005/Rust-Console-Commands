# PowerShell script to compile each Rust project in the week_one folder

Get-ChildItem -Directory | ForEach-Object {
    $cargoToml = Join-Path $_.FullName "Cargo.toml"
    if (Test-Path $cargoToml) {
        Write-Host "Building project in $($_.FullName)..."
        Push-Location $_.FullName
        cargo clean
        Pop-Location
    }
}