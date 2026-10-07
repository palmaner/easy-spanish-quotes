param([string]$Compiler = 'x86_64-w64-mingw32-clang', [string]$Linker = 'ld.lld', [string]$Python = 'python')
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
Push-Location $root
try {
    & $Python tools/generate_layout.py
    if ($LASTEXITCODE -ne 0) { throw 'Layout generation failed' }
    & $Compiler -std=c11 -O2 -c -I native/windows-layout build/layout/layout.c -o build/layout/layout.obj
    if ($LASTEXITCODE -ne 0) { throw 'Native compilation failed' }
    & $Linker -flavor link /dll /noentry /nodefaultlib /timestamp:0 /export:KbdLayerDescriptor `
        /out:build/layout/esq-layout.dll build/layout/layout.obj
    if ($LASTEXITCODE -ne 0) { throw 'Native compilation failed' }
    Get-FileHash build/layout/esq-layout.dll -Algorithm SHA256
} finally { Pop-Location }
