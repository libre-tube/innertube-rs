# innertube-rs
Metadata parsing library from YouTube. Currently supports `Channel`, `Video`,`Playlist`.

# Getting started
```rust
let innertube = Innertube::new().unwrap();
innertube.channel_metadata("UCBR8-60-B28hp2BmDPdntcQ").await.unwrap();
innertube.video_metadata("dQw4w9WgXcQ").await.unwrap();
innertube.playlist_metadata("PLFPk5ONFpYvWsnAYJqm6Li4qtg367uUbv").await.unwrap();
```
