# YouTube Desktop — Windows 8.1

A lightweight native Win32 desktop frontend based on the GPL-3.0-or-later `youtube-tui` project. It reuses RustyPipe for YouTube data and avoids Electron, Chromium, WebView2 and GPU-heavy UI frameworks.

## Current implementation

- Native resizable Win32 window and controls.
- Asynchronous RustyPipe search with type/date/duration/sort filters.
- Video, channel and playlist result types.
- Full video details using RustyPipe player + video-details requests.
- Channel metadata, latest videos and playlists.
- Playlist metadata and returned video items.
- Trending feed.
- Selected-result thumbnail download/cache and GDI+ display.
- Search history (20 entries) and watch history (100 entries), stored locally as JSON.
- Locally saved channel subscriptions (not Google-account subscriptions).
- Local Downloads library.
- MPV launch and yt-dlp download integration.
- Browser open fallback.

## External tools

Install these separately and add their executable directories to `PATH`:

- `mpv.exe` for playback.
- `yt-dlp.exe` for downloads.

Downloads are stored in `%APPDATA%\\youtube-desktop\\downloads`. RustyPipe cache, thumbnail cache and local history are stored under `%APPDATA%\\youtube-desktop`.

## Build

Use Rust 1.77.2 with the Visual Studio C++ Build Tools/MSVC workload. Rust 1.78 raised the official Windows target baseline to Windows 10, so this project targets the earlier toolchain to keep Windows 8.1 viable.

```powershell
rustup toolchain install 1.77.2
rustup default 1.77.2
rustup target add i686-pc-windows-msvc
cargo check
cargo build --release
cargo build --release --target i686-pc-windows-msvc
```

The project has not yet been compiled or executed in this environment. The CI workflow is configured to check/build both x64 and x86, but a successful build still does not replace runtime testing on Windows 8.1.

## Known gaps

- Search currently displays the first result batch only; continuation/pagination is not wired.
- Subscriptions are local saved channel shortcuts, not authenticated YouTube subscriptions.
- Embedded audio playback is omitted; external MPV is the low-memory route.
- Custom keybindings and appearance settings are not yet implemented.
- Captions/adaptive stream controls remain outside the implemented RustyPipe adapter scope.

See `FEATURE_STATUS.md` for the detailed checklist and required validation.

## License and attribution

GPL-3.0-or-later. See `LICENSE` and `UPSTREAM_NOTICES.md`. Reuse and distribution must comply with the GPL and preserve attribution and notices.
