use eyre::{ContextCompat, eyre};
use isahc::AsyncReadResponseExt;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

pub const YT_URL: &str = "https://www.youtube.com";
pub const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36";
pub const CONTENT_TYPE: &str = "application/json";

pub const CONTEXT_DEFAULT: &str = r#"
    {
        "context": {
            "client": {
                "clientName": "WEB",
                "clientVersion": "2.20261026.01.00"
            }
        }
    }
"#;

pub const YT_BROWSE_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/browse";
pub const YT_PLAYER_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/player";

pub mod types;
use types::*;

pub struct Innertube {
    pub client: isahc::HttpClient,
}

impl Innertube {
    async fn req<D: DeserializeOwned>(
        &self,
        url: &str,
        mut payload: serde_json::Value,
        get: &'static str,
    ) -> eyre::Result<D> {
        let mut json = serde_json::from_str::<serde_json::Value>(CONTEXT_DEFAULT).unwrap();
        json.as_object_mut()
            .unwrap()
            .append(payload.as_object_mut().unwrap());

        let mut resp = self.client.post_async(url, json.to_string()).await?;
        let text = resp.text().await?;

        if resp.status().is_success() {
            let buf_parsed: Value = serde_json::from_str(&text)?;
            Ok(serde_json::from_value(
                get.split(".")
                    .try_fold(&buf_parsed, |value, key| {
                        value.get(key).context(eyre!("key `{key}` was not found"))
                    })?
                    .to_owned(),
            )?)
        } else {
            Err(eyre!("http error {}: {text}", resp.status()))
        }
    }

    pub fn new() -> eyre::Result<Self> {
        let client = isahc::HttpClient::builder()
            .cookies()
            .default_header("User-Agent", USER_AGENT)
            .default_header("Content-Type", CONTENT_TYPE)
            .build()
            .unwrap();

        // Fetches cookies retrieved from YT. They're not required, but maybe useful to mimic standard requests
        client.get(YT_URL)?;

        Ok(Innertube { client })
    }

    pub async fn channel_metadata(&self, channel_id: &str) -> eyre::Result<ChannelMetadata> {
        self.req(
            YT_BROWSE_ENDPOINT,
            json!({ "browseId": channel_id }),
            "metadata.channelMetadataRenderer",
        )
        .await
    }

    pub async fn video_metadata(&self, video_id: &str) -> eyre::Result<VideoMetadata> {
        self.req(
            YT_PLAYER_ENDPOINT,
            json!({ "videoId": video_id }),
            "videoDetails",
        )
        .await
    }
}
