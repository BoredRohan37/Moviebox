# Feature parity tracker — Phase 6

Status is deliberately conservative. “Implemented in source” does not mean Windows runtime-tested.

| Capability | Status | Implementation / limitation |
|---|---|---|
| YouTube search | Implemented in source | RustyPipe 0.11.4 asynchronous search |
| Search filters | Implemented in source | Type, upload date, duration and sort |
| Video details | Implemented in source | Player + video-details requests; description, channel, views and duration |
| Channel browsing | Implemented in source | Channel metadata, latest videos and channel playlists |
| Playlist browsing | Implemented in source | Playlist metadata and returned video items |
| Trending | Implemented in source | RustyPipe trending endpoint |
| Thumbnail cache/rendering | Implemented in source | URLMon download + GDI+ bitmap display; selected-item thumbnail |
| Search history | Implemented in source | Last 20 distinct queries, local JSON |
| Watch history | Implemented in source | Last 100 launched/opened videos, local JSON |
| Subscriptions | Partial | Locally saved channel list; no Google account OAuth or account sync |
| Offline library | Implemented in source | Lists files in the app's Downloads directory |
| External video player | Implemented in source | Launches `mpv.exe` from PATH; browser open remains available |
| Download integration | Implemented in source | Launches `yt-dlp.exe` from PATH into app Downloads folder |
| Embedded audio player | Not implemented | Deliberately omitted to keep memory and dependency footprint low; use MPV |
| Mouse navigation | Implemented in source | Native Win32 buttons and list controls |
| Keyboard navigation | Partial | Enter submits search; standard native control navigation |
| Custom keybindings | Not implemented | No settings UI yet |
| Configurable appearance | Not implemented | Uses classic system controls and system colors |
| Search pagination | Upstream/API limitation | This port currently consumes the first result batch returned by RustyPipe; no continuation workflow has been wired |
| Captions / adaptive stream controls | Upstream limitation | Not implemented by the original RustyPipe adapter used by youtube-tui |

## Required validation

- Build x86 and x64 with Rust 1.77.2/MSVC.
- Run both binaries on a real Windows 8.1 machine.
- Exercise search, filters, details, thumbnails, channel/playlist navigation, trending and local history.
- Verify MPV and yt-dlp with user-installed binaries.
- Verify GDI+ and URLMon thumbnail handling with HTTPS thumbnail URLs.

No feature should be described as release-ready until those checks pass.
