use std::fmt::Debug;

// use eyre::{ContextCompat, eyre};
use isahc::AsyncReadResponseExt;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

pub use isahc::http::StatusCode;

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

const YT_CONSENT_URL: &str = "https://consent.youtube.com/save?continue=https://www.youtube.com/&gl=US&m=0&pc=yt&x=5&src=2&hl=en&bl=0&cm=2&set_eom=false&set_apyt=true&set_ytc=true";
const YT_BROWSE_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/browse";
const YT_PLAYER_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/player";

mod tools;
mod types;
pub use types::*;

#[derive(thiserror::Error, Debug)]
pub enum InnertubeError {
    #[error("http error ({status_code}): {context}")]
    Http {
        status_code: StatusCode,
        context: String,
    },
    #[error("`{0:?} not found in JSON")]
    NotFoundInJSON(Vec<String>),
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
    async fn req<T: TryFrom<Vec<Value>>>(
        &self,
        url: &str,
        mut payload: serde_json::Value,
        get: &[&'static str],
    ) -> InnertubeResult<T> {
        let mut json = serde_json::from_str::<serde_json::Value>(CONTEXT_DEFAULT).unwrap();
        tools::append(&mut payload, &mut json);

        let mut resp = self.client.post_async(url, json.to_string()).await?;
        let text = resp.text().await?;

        if resp.status().is_success() {
            let buf_parsed: Value = serde_json::from_str(&text)?;
            let (successes, failures): (Vec<Value>, Vec<String>) = get
                .iter()
                .map(|path| tools::json(path, &buf_parsed).map_err(|_| path.to_string()))
                .partition_result();
            if failures.is_empty() {
                Ok(successes.try_into().unwrap_or_else(|_| panic!()))
            } else {
                Err(InnertubeError::NotFoundInJSON(failures))
            }
        } else {
            Err(InnertubeError::Http {
                status_code: resp.status(),
                context: text,
            })
        }
    }

    async fn req_single<D: DeserializeOwned>(
        &self,
        url: &str,
        payload: serde_json::Value,
        get: &'static str,
    ) -> InnertubeResult<D> {
        let [value] = self.req::<[Value; 1]>(url, payload, &[get]).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Create a new Innertube instance
    pub fn new() -> InnertubeResult<Self> {
        let client = isahc::HttpClient::builder()
            .cookies()
            .default_header("User-Agent", USER_AGENT)
            .default_header("Content-Type", CONTENT_TYPE)
            .build()
            .unwrap();

        client.post(YT_CONSENT_URL, "")?;

        Ok(Innertube { client })
    }

    /// Get channel metadata with specified `channel_id`
    pub async fn channel_metadata(&self, channel_id: &str) -> InnertubeResult<ChannelMetadata> {
        self.req_single(
            YT_BROWSE_ENDPOINT,
            json!({ "browseId": channel_id }),
            "metadata.channelMetadataRenderer",
        )
        .await
    }

    /// Get video metadata with specified `video_id`
    pub async fn video_metadata(&self, video_id: &str) -> InnertubeResult<VideoMetadata> {
        self.req_single(
            YT_PLAYER_ENDPOINT,
            json!({ "videoId": video_id }),
            "videoDetails",
        )
        .await
    }

    /// Get video metadata with specified `playlist_id`
    pub async fn playlist_metadata(&self, playlist_id: &str) -> InnertubeResult<PlaylistMetadata> {
        let [mut playlist_metadata, additional_info] = self
            .req(
                YT_BROWSE_ENDPOINT,
                json!({ "browseId": String::from("VL")+playlist_id}),
                &[
                    "metadata.playlistMetadataRenderer",
                    "header.pageHeaderRenderer.content.pageHeaderViewModel.metadata.contentMetadataViewModel.metadataRows.1.metadataParts",
                ],
            )
            .await?;
        let mut additional_info = additional_info.as_array().unwrap().iter().map(|value| {
            tools::json("text.content", value)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        });

        let [videos_count, views_count] = ["video", "view"]
            .iter()
            .map(|pat| {
                additional_info.find(|x| x.contains(pat)).and_then(|x| {
                    x.split_once(" ")
                        .and_then(|x| x.0.replace(',', "").parse::<usize>().ok())
                })
            })
            .flatten()
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        tools::append(
            &mut json!({"videosCount": videos_count, "viewsCount": views_count}),
            &mut playlist_metadata,
        );

        Ok(serde_json::from_value(playlist_metadata.to_owned())?)
    }
}
