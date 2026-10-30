use eyre::{ContextCompat, eyre};
use isahc::AsyncReadResponseExt;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

const YT_URL: &str = "https://www.youtube.com";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36";
const CONTENT_TYPE: &str = "application/json";

const CONTEXT_DEFAULT: &str = r#"
    {
        "context": {
            "client": {
                "clientName": "WEB",
                "clientVersion": "2.20261026.01.00"
            }
        }
    }
"#;

const YT_BROWSE_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/browse";
const YT_PLAYER_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/player";

mod types;
pub use types::*;

/// The Innertube client used to access YouTube’s API
pub struct Innertube {
    client: isahc::HttpClient,
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

    /// Create a new Innertube instance
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

    /// Get channel metadata with specified `channel_id`
    pub async fn channel_metadata(&self, channel_id: &str) -> eyre::Result<ChannelMetadata> {
        self.req(
            YT_BROWSE_ENDPOINT,
            json!({ "browseId": channel_id }),
            "metadata.channelMetadataRenderer",
        )
        .await
    }

    /// Get video metadata with specified `video_id`
    pub async fn video_metadata(&self, video_id: &str) -> eyre::Result<VideoMetadata> {
        self.req(
            YT_PLAYER_ENDPOINT,
            json!({ "videoId": video_id }),
            "videoDetails",
        )
        .await
    }
}
