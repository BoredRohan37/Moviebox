# Build and Windows 8.1 validation checklist

## Build prerequisites

- Windows 8.1 x64 or x86.
- Rust 1.77.2 (selected by `rust-toolchain.toml`).
- Visual Studio Build Tools with the MSVC C++ toolchain and Windows SDK.
- Working internet connection for Cargo dependency downloads and YouTube requests.

## Build

Run from the project directory in PowerShell:

```powershell
.\build-win81.ps1
```

The script produces:

- `dist\youtube-desktop-x64.exe`
- `dist\youtube-desktop-x86.exe`

## Runtime test plan

1. Launch the x64 binary on Windows 8.1 x64.
2. Search for a common public video; verify results and all four filter controls.
3. Select a video; verify the thumbnail, title, channel, views, duration and description.
4. Select a channel; verify channel details, videos and playlists.
5. Select a playlist; verify playlist details and returned video entries.
6. Use Trending; verify a result list loads.
7. Close/reopen; verify search history and watch history persist.
8. Subscribe to a channel, reopen Subscriptions, select it, then unsubscribe.
9. Install MPV and yt-dlp separately, add them to PATH, and test playback/download.
10. Download a video, open Downloads, and play the local file.
11. Repeat on x86 Windows if an x86 installation is available.

## Important distinction

A successful Cargo build only confirms compilation. Windows 8.1 compatibility, live YouTube behavior, thumbnail decoding, MPV and yt-dlp must all be tested on the actual operating system before release.
