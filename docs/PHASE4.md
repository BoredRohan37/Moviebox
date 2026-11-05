# Phase 4 implementation notes

## Added

- Native Win32 combo-box controls for item type, upload date, duration and sort order.
- Search controls map directly to RustyPipe's `SearchFilter` builder (`item_type_opt`, `date_opt`, `length_opt`, `sort_opt`).
- Search-result normalization now retains the best available thumbnail URL for videos, channels and playlists.
- Video result subtitle includes channel and available view-count/duration metadata.
- Result details expose item kind, ID, canonical URL and thumbnail URL.
- A monotonically increasing search generation prevents a slow older request from overwriting the latest search results.
- Updated feature tracker and build documentation to distinguish implemented code from verified behavior.

## Not included yet

- Thumbnail download, decoding, rendering or disk-cache eviction.
- Full video/channel/playlist detail requests.
- Pagination / continuation tokens.
- Search history persistence.
- Player and downloader process integration.

## Validation

Source-level review was performed against the upstream adapter's RustyPipe API usage. This environment has no `rustc` or `cargo`, so compilation and Windows runtime behavior are **not verified**. CI is configured for x86 and x64 MSVC targets using Rust 1.77.2; its result must be checked before treating the build as successful.
