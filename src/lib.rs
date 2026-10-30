use std::io::Read;

use eyre::ContextCompat;
use serde::de::DeserializeOwned;
use serde_json::json;

pub const YT_URL: &str = "https://www.youtube.com";
pub const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36";
pub const CONTENT_TYPE: &str = "application/json";
pub const CLIENT_NAME: &str = "WEB";
pub const CLIENT_VERSION: &str = "2.20261026.01.00";

pub const YT_BROWSE_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/browse";
pub const YT_PLAYER_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/player";

pub mod tools;
pub mod types;
use types::*;

pub struct Innertube {
    pub client: isahc::HttpClient,
}

impl Innertube {
    fn request<D: DeserializeOwned>(&self, url: &str, json: serde_json::Value) -> eyre::Result<D> {
        let mut buf = String::new();
        self.client
            .post(url, json.to_string())?
            .into_body()
            .read_to_string(&mut buf)?;

        Ok(serde_json::from_str(&buf)?)
    }
    pub fn new() -> eyre::Result<Self> {
        let client = isahc::HttpClient::builder()
            .cookies()
            .default_headers(&[("User-Agent", USER_AGENT), ("Content-Type", CONTENT_TYPE)])
            .build()
            .unwrap();

        // fetches cookies retrieved from YT. they're not required, but maybe useful to mimic standard requests
        client.get(YT_URL)?;

        Ok(Innertube { client })
    }

    pub fn channel_metadata(&self, channel_id: &str) -> eyre::Result<ChannelMetadata> {
        let raw = self.request::<serde_json::Value>(
            YT_BROWSE_ENDPOINT,
            json!({
                "browseId": channel_id,
                "context": {
                    "client": {
                        "clientName": CLIENT_NAME,
                        "clientVersion": CLIENT_VERSION,
                    },
                },
            }),
        )?;
        Ok(serde_json::from_value(
            raw.get("metadata")
                .context("key `metadata` was not found")?
                .get("channelMetadataRenderer")
                .context("key `channelMetadataRenderer` was not found")?
                .to_owned(),
        )?)
    }

    pub fn video_metadata(&self, video_id: &str) -> eyre::Result<VideoMetadata> {
        let raw = self.request::<serde_json::Value>(
            YT_PLAYER_ENDPOINT,
            json!({
                "videoId": video_id,
                "context": {
                    "client": {
                        "clientName": CLIENT_NAME,
                        "clientVersion": CLIENT_VERSION,
                    },
                },
            }),
        )?;

        Ok(serde_json::from_value(
            raw.get("videoDetails")
                .context("key `videoDetails` was not found")?
                .to_owned(),
        )?)
    }
}
