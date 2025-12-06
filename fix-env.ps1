$mingw = Join-Path $PSScriptRoot "mingw64"
$bin = Join-Path $mingw "bin"
$env:PATH = "$bin;$env:PATH"
$env:CC = Join-Path $bin "gcc.exe"
$env:CXX = Join-Path $bin "g++.exe"
$env:AR = Join-Path $bin "ar.exe"
$env:LIBRARY_PATH = Join-Path $mingw "lib"

Write-Host "Set CC to $env:CC"
cargo check
