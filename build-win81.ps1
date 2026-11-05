$ErrorActionPreference = 'Stop'

Write-Host 'YouTube Desktop - Windows 8.1 build' -ForegroundColor Cyan
$rustc = rustc --version
if ($LASTEXITCODE -ne 0) { throw 'Rust is not installed. Install Rust 1.77.2 first.' }
if ($rustc -notmatch '1\.77\.2') { throw "Expected Rust 1.77.2, found: $rustc" }

rustup target add x86_64-pc-windows-msvc i686-pc-windows-msvc
if ($LASTEXITCODE -ne 0) { throw 'Could not install the Windows MSVC targets.' }

cargo build --release --target x86_64-pc-windows-msvc
if ($LASTEXITCODE -ne 0) { throw 'The x64 build failed.' }

cargo build --release --target i686-pc-windows-msvc
if ($LASTEXITCODE -ne 0) { throw 'The x86 build failed.' }

New-Item -ItemType Directory -Force -Path .\dist | Out-Null
Copy-Item .\target\x86_64-pc-windows-msvc\release\youtube-desktop.exe .\dist\youtube-desktop-x64.exe -Force
Copy-Item .\target\i686-pc-windows-msvc\release\youtube-desktop.exe .\dist\youtube-desktop-x86.exe -Force
Write-Host 'Build complete. Binaries are in .\dist' -ForegroundColor Green
