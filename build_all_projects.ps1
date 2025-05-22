# PowerShell script to compile each Rust project and copy the binary to a bin folder

$binDir = Join-Path $PWD "compiled_commands"
if (Test-Path $binDir) {
    # Remove all files in the compiled_commands folder before copying new binaries
    Get-ChildItem -Path $binDir -File | Remove-Item -Force
} else {
    New-Item -ItemType Directory -Path $binDir | Out-Null
}

Get-ChildItem -Directory | ForEach-Object {
    $cargoToml = Join-Path $_.FullName "Cargo.toml"
    if (Test-Path $cargoToml) {
        Write-Host "Building project in $($_.FullName)..."
        Push-Location $_.FullName
        cargo build --release
        $projectName = (Get-Content $cargoToml | Select-String '^name\s*=\s*"(.*)"').Matches[0].Groups[1].Value
        $exePath = Join-Path $_.FullName "target\release\$projectName.exe"
        if (Test-Path $exePath) {
            Copy-Item $exePath -Destination $binDir -Force
        }
        Pop-Location
    }
}