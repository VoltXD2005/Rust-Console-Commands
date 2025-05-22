# PowerShell script to organize Rust files into separate Cargo projects

Get-ChildItem -Filter *.rs | ForEach-Object {
    $baseName = $_.BaseName
    $folder = Join-Path -Path $PWD -ChildPath $baseName
    if (-not (Test-Path $folder)) {
        cargo init --bin $folder
    }
    $dest = Join-Path -Path $folder -ChildPath "src\main.rs"
    Move-Item -Path $_.FullName -Destination $dest -Force
}