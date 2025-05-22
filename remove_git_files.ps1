# PowerShell script to delete all .git folders and .gitignore files in Week_One

Get-ChildItem -Path . -Recurse -Force -Directory -Filter ".git" | ForEach-Object {
    Remove-Item -Path $_.FullName -Recurse -Force
}

Get-ChildItem -Path . -Recurse -Force -File -Filter ".gitignore" | ForEach-Object {
    Remove-Item -Path $_.FullName -Force
}