# youtube-tui source audit and Windows desktop port plan

## Source snapshot

- Archive root: `youtube-tui-master`
- Upstream version in `Cargo.toml`: `0.9.5`
- Rust edition: 2021
- Archive commit identifier: `800b65178f3c6cc2b2afdfc28b3e04e0fc212769`
- License: GPL-3.0-or-later
- Data client in the lockfile: `rustypipe 0.11.4`

## Architectural finding

The original is not a separable headless backend. `src/main.rs`, `src/init.rs`, and `src/run.rs` establish a Ratatui/Crossterm terminal event loop. The page system, task queue, rendering, framework state and many interactions use `tui-additions`, `ratatui`, `crossterm`, and `typemap` directly.

The reusable part is primarily the upstream `rustypipe` client and the original provider adapter/conversion logic in `src/global/structs/providers/rustypipe.rs`. The adapter itself also depends on original application models and configuration types, so it must be refactored rather than copied unchanged.

## Feature inventory from source and manual

| Capability | Evidence in source/docs | Port status |
|---|---|---|
| Trending and popular pages | `Page`, `MainMenuPage`, universal provider modules | Planned |
| Search | Search provider trait, RustyPipe adapter, search UI/filter modules | Planned |
| Search filters: date, sort, duration, item type | `config/search.rs`, `items/searchfilters.rs`, provider adapter | Planned |
| Search history and suggestions | config/global structs and universal suggestions module | Planned; verify exact UX |
| Video details | `video.rs`, `singleitem.rs`, video common models | Planned |
| Channel details and channel tabs | `channeldisplay.rs`, `channellist.rs`, exposed channel modules | Planned |
| Playlist details and browsing | playlist models and universal/channel modules | Planned |
| Subscriptions and feed sync | `subscriptions.rs`, `videolist.rs`, `channellist.rs` | Planned |
| Watch history | `history.rs`, main menu history page | Planned |
| Offline library/bookmarks | `library.rs`, `find_library.rs`, command handlers | Planned |
| Comments | exposed video/channel comments modules | Planned; provider coverage must be tested |
| Captions | exposed captions module and video model | Incomplete upstream; do not claim parity until implemented |
| Video playback | external command integration and optional libmpv | Planned via MPV; embedded playback is a separate compatibility decision |
| Audio playback | optional libmpv wrapper and audio controls | Planned; verify MPV ABI and runtime requirements |
| Downloads | configurable `youtube-downloader` command, normally yt-dlp | Planned external integration |
| Keyboard navigation | keybinding/config modules and command bindings | Planned; desktop shortcuts will be mapped |
| Mouse navigation | TUI mouse event handling | Planned; native GUI mouse behavior replaces TUI-specific rules |
| Vim-like command mode | commands and command remapping modules | Planned; expose as command palette/optional command bar |
| Configurable appearance | appearance config and YAML config loader | Planned; translate to desktop settings |
| Custom launch commands | command config and environment expansion | Planned; security and quoting review required |
| Image rendering | sixel/halfblock terminal image support | Not ported literally; use normal Windows image controls/decoded thumbnails |
| Invidious provider | optional Cargo feature and provider adapter | Optional later; upstream docs currently de-emphasize it |
| MPV property/command bridge | MPV command handlers | Planned only if embedded libmpv is retained |

## Known upstream incompleteness

The inspected RustyPipe adapter has explicit placeholders/TODOs for fields including adaptive formats, format streams, captions, recommended videos and dislike counts. Some data cannot be treated as supported simply because a model field exists. The desktop port must track these separately and test provider responses.

The original README also states that the application launches external programs such as MPV and yt-dlp. Those tools are integrations, not bundled functionality.

## Proposed desktop architecture

1. **Win32 shell:** window lifecycle, controls, keyboard/mouse input, GDI drawing. Avoid browser runtimes and GPU-centric frameworks.
2. **Application state:** typed page/navigation state, selected item, loading/error states and user settings. No `typemap` or terminal framework.
3. **YouTube service:** isolated RustyPipe calls, response normalization and cancellation/timeouts.
4. **Persistence:** versioned JSON for history, library, subscriptions and settings; atomic writes and migration support.
5. **Media integration:** discover/configure MPV and yt-dlp; launch without blocking the UI; capture exit status and errors.
6. **Image/cache service:** bounded thumbnail cache with size limits and background decoding.

## Windows 8.1 constraints

- Avoid WebView2, Electron, Chromium and Direct3D-only rendering paths.
- Prefer Win32 controls and GDI for the initial UI; test image scaling and font rendering on GMA 3100.
- Keep network work off the UI thread.
- Do not assume a current Rust toolchain or dependency set runs on Windows 8.1 just because it compiles on a newer Windows host.
- Pin dependency versions and validate the final executable on Windows 8.1 x64. Check runtime DLL dependencies and TLS/certificate behavior.
- The initial target is x64. The Pentium E4400 supports x86-64, but the actual installed Windows architecture still needs confirmation before choosing the only release target.

## Milestones and acceptance gates

1. **Shell:** create, resize, close, DPI-safe window; no crash on empty config.
2. **Core service:** search query, result normalization, errors and cancellation; test with saved response fixtures.
3. **Browse UI:** search/results, detail pages, channel and playlist navigation.
4. **User data:** history, library, subscriptions and atomic persistence across restart.
5. **Media:** MPV playback and yt-dlp download launch, including missing-tool/error cases.
6. **Parity pass:** comments, captions, command system, keybindings, appearance and remaining upstream functions.
7. **Compatibility:** run on Windows 8.1 x64, low-memory profiling, long-session stability and clean portable packaging.

A feature is complete only after its UI action, service behavior, persistence (where applicable), error path and Windows 8.1 runtime behavior have been tested.
