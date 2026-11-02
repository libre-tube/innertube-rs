use std::io::Read;

// use eyre::{ContextCompat, eyre};
use isahc::AsyncReadResponseExt;
use regex::regex;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

pub use isahc::http::StatusCode;

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

const BUFFER_LENGTH: usize = 1024 * 256;

mod types;
pub use types::*;

#[derive(thiserror::Error, Debug)]
pub enum InnertubeError {
    #[error("http error ({status_code}): {context}")]
    Http {
        status_code: StatusCode,
        context: String,
    },
    #[error("key `{0}` was not found")]
    KeyNotFound(String),
    #[error("isahc error: {0}")]
    Isahc(#[from] isahc::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to deserialize json: {0}")]
    Json(#[from] serde_json::Error),
}

type InnertubeResult<T> = Result<T, InnertubeError>;

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
    ) -> InnertubeResult<D> {
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
                        value
                            .get(key)
                            .ok_or(InnertubeError::KeyNotFound(key.to_string()))
                    })?
                    .to_owned(),
            )?)
        } else {
            Err(InnertubeError::Http {
                status_code: resp.status(),
                context: text,
            })
        }
    }

    pub fn fetch_browse_id(&self, url: &str) -> InnertubeResult<Option<String>> {
        let mut buf = [0; BUFFER_LENGTH];
        let mut text = String::new();
        let mut resp = isahc::get(url)?.into_body();
        while resp.read_exact(&mut buf).is_ok() {
            text.push_str(&String::from_utf8_lossy(&buf));

            if let Some(match_) = regex!(r#""browseId":+"(?:[^"]|"")*""#).find(&text) {
                let (_, browse_id) = match_.as_str().split_once(":").unwrap();
                return Ok(Some(
                    browse_id
                        .trim()
                        .trim_start_matches('"')
                        .trim_end_matches('"')
                        .to_string(),
                ));
            }
        }

        Ok(None)
    }

    /// Create a new Innertube instance
    pub fn new() -> InnertubeResult<Self> {
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
    pub async fn channel_metadata(&self, channel_id: &str) -> InnertubeResult<ChannelMetadata> {
        self.req(
            YT_BROWSE_ENDPOINT,
            json!({ "browseId": channel_id }),
            "metadata.channelMetadataRenderer",
        )
        .await
    }

    /// Get video metadata with specified `video_id`
    pub async fn video_metadata(&self, video_id: &str) -> InnertubeResult<VideoMetadata> {
        self.req(
            YT_PLAYER_ENDPOINT,
            json!({ "videoId": video_id }),
            "videoDetails",
        )
        .await
    }
}
