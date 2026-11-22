# innertube-rs
Metadata parsing library from YouTube. Currently supports `Channel`, `Video`,`Playlist`.

# Getting started
```rust
let innertube = Innertube::new().unwrap();
let target = innertube.channel("UCBR8-60-B28hp2BmDPdntcQ");
target.metadata().await?;
target.share_links().await?;
target.title().await?;
```
