# innertube-rs
Metadata parsing library from YouTube. Currently supports `Channel`, `Video`,`Playlist`.

# Getting started
```rust
let innertube = Innertube::new().unwrap();
innertube.chanel("UCBR8-60-B28hp2BmDPdntcQ").metadata().await.unwrap();
innertube.video("dQw4w9WgXcQ").metadata().await.unwrap();
innertube.playlist_metadata("PLFPk5ONFpYvWsnAYJqm6Li4qtg367uUbv").await.unwrap();
```
