# Phase 6 — browsing, metadata, thumbnails and local library

## Added in this source update

- Video detail requests use RustyPipe `player` and `video_details` concurrently.
- Channel pages load channel information, channel videos and channel playlists.
- Playlist pages load playlist metadata and the videos returned by RustyPipe.
- Trending feed uses the RustyPipe trending endpoint.
- Thumbnail URLs are downloaded asynchronously with Windows URLMon, cached below `%APPDATA%\\youtube-desktop\\thumbnails`, then decoded with GDI+ and displayed in the details pane.
- Search history is persisted as JSON (20 entries).
- Watch history is persisted as JSON (100 entries).
- Local channel subscriptions can be added/removed and revisited.
- Downloads library scans the app's Downloads directory.
- MPV and yt-dlp are launched as external processes to avoid bundling heavy media dependencies.
- Enter submits the search query.

## Intentional limitations

- The upstream adapter does not provide a wired continuation token/pagination workflow here.
- Local subscriptions do not sign into or synchronize with a Google account.
- Embedded audio playback is omitted in favor of MPV.
- Custom keybindings and theme settings remain future work.

## Validation status

Source archive generation and static inspection only. This environment has no Rust compiler and cannot run Windows binaries. x86/x64 compilation and actual Windows 8.1 runtime tests remain mandatory before calling this a working release.
