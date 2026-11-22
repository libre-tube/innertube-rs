use std::{fmt::Debug, marker::PhantomData, str::FromStr, sync::OnceLock};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use isahc::AsyncReadResponseExt;
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
const YT_SHARE_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/share/get_share_panel";

mod paths;
mod tools;
mod types;
pub use types::*;

type ProtobufRaw = &'static [u8];

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
    async fn req(&self, url: &str, mut payload: serde_json::Value) -> InnertubeResult<Value> {
        let mut json = serde_json::from_str::<serde_json::Value>(CONTEXT_DEFAULT).unwrap();
        tools::append(&mut payload, &mut json);

        let mut resp = self.client.post_async(url, json.to_string()).await?;
        let text = resp.text().await?;

        if resp.status().is_success() {
            Ok(serde_json::from_str(&text)?)
        } else {
            Err(InnertubeError::Http {
                status_code: resp.status(),
                context: text,
            })
        }
    }

    async fn req_share_links(
        &self,
        protobuf_raw: &[u8],
        value: &str,
    ) -> InnertubeResult<ShareLinks> {
        let protobuf_with_value = protobuf_raw.iter().fold(Vec::new(), |mut acc, ch| {
            acc.append(&mut {
                match *ch {
                    b'V' => value.as_bytes().to_vec(),
                    b'?' => vec![value.chars().count() as u8],
                    ch => vec![ch],
                }
            });
            acc
        });

        let json = self.req(
                YT_SHARE_ENDPOINT,
                json!({ "serializedSharedEntity": urlencoding::encode(&STANDARD.encode(protobuf_with_value)) }),
            ).await?;
        Ok(ShareLinks::new(
            tools::json(paths::SHARE_TARGETS_ARRAY, &json)?
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|share_target| {
                    tools::json(paths::SHARE_TARGET_URL, share_target)
                        .ok()
                        .and_then(|url_| url_.as_str().and_then(|s| url::Url::from_str(s).ok()))
                })
                .collect(),
        ))
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

    pub fn channel<S: ToString>(&self, channel_id: S) -> InnertubeReq<'_, ChannelMetadata> {
        let channel_id = channel_id.to_string();

        InnertubeReq {
            it: self,
            id: channel_id.clone(),
            metadata_endpoint: YT_BROWSE_ENDPOINT,
            metadata_payload: json!({ "browseId": channel_id }),
            metadata_parser: Box::new(|value| tools::json(paths::CHANNEL_METADATA, &value)),
            share_links_payload: b"\x1a?V",
            share_links_cached: OnceLock::new(),
            _marker: PhantomData,
        }
    }

    pub fn video<S: ToString>(&self, video_id: S) -> InnertubeReq<'_, VideoMetadata> {
        let video_id = video_id.to_string();

        InnertubeReq {
            it: self,
            id: video_id.to_string(),
            metadata_endpoint: YT_PLAYER_ENDPOINT,
            metadata_payload: json!({ "videoId": video_id }),
            metadata_parser: Box::new(|value| tools::json(paths::VIDEO_METADATA, &value)),
            share_links_payload: b"\n?V\xa0\x01\x01",
            share_links_cached: OnceLock::new(),
            _marker: PhantomData,
        }
    }

    pub fn playlist<S: ToString>(&self, playlist_id: S) -> InnertubeReq<'_, PlaylistMetadata> {
        let playlist_id = playlist_id.to_string();

        InnertubeReq {
            it: self,
            id: playlist_id.to_string(),
            metadata_endpoint: YT_BROWSE_ENDPOINT,
            metadata_payload: json!({ "browseId": String::from("VL")+&playlist_id }),
            metadata_parser: Box::new(|value| {
                let mut playlist_metadata = tools::json(paths::PLAYLIST_METADATA, &value)?;
                let thumbnail = tools::json(paths::PLAYLIST_THUMBNAIL, &value)?;
                let additional_info = tools::json(paths::PLAYLIST_ADDITIONAL_METADATA, &value)?;

                let mut additional_info = additional_info.as_array().unwrap().iter().map(|value| {
                    tools::json("text.content", value)
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string()
                });

                // e.g. ["Playlist", "102 videos", "100,102 views"] -> [102_usize, 100102_usize]
                let [video_count, view_count] = ["video", "view"]
                    .iter()
                    .filter_map(|pat| {
                        additional_info.find(|x| x.contains(pat)).and_then(|raw| {
                            raw.split_once(" ")
                                .and_then(|split| split.0.replace(',', "").parse::<usize>().ok())
                        })
                    })
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap();

                tools::append(
                    &mut json!({"videoCount": video_count, "viewCount": view_count}),
                    &mut playlist_metadata,
                );
                tools::append(
                    &mut json!({"thumbnail": {"thumbnails": thumbnail}}),
                    &mut playlist_metadata,
                );

                Ok(playlist_metadata)
            }),
            share_links_payload: b"\x12?V",
            share_links_cached: OnceLock::new(),
            _marker: PhantomData,
        }
    }
}

pub struct InnertubeReq<'it, T: DeserializeOwned> {
    it: &'it Innertube,
    id: String,
    metadata_endpoint: &'static str,
    metadata_payload: Value,
    metadata_parser: Box<dyn Fn(Value) -> InnertubeResult<Value> + Sync>,
    share_links_payload: ProtobufRaw,
    share_links_cached: OnceLock<ShareLinks>,
    _marker: PhantomData<T>,
}

unsafe impl<'it, T: DeserializeOwned> Send for InnertubeReq<'it, T> {}

impl<'it, T: DeserializeOwned> InnertubeReq<'it, T> {
    async fn req_share_links(&self) -> InnertubeResult<&ShareLinks> {
        Ok(if let Some(share_links) = self.share_links_cached.get() {
            share_links
        } else {
            let resp = self
                .it
                .req_share_links(self.share_links_payload, &self.id)
                .await?;
            self.share_links_cached.get_or_init(|| resp)
        })
    }

    pub async fn metadata(&self) -> InnertubeResult<T> {
        let resp = self
            .it
            .req(self.metadata_endpoint, self.metadata_payload.clone())
            .await?;
        let parser = &self.metadata_parser;
        Ok(serde_json::from_value(parser(resp)?)?)
    }

    pub async fn share_links(&self) -> InnertubeResult<ShareLinks> {
        Ok(self.req_share_links().await?.to_owned())
    }

    pub async fn id(&self) -> String {
        self.id.to_string()
    }

    pub async fn title(&self) -> InnertubeResult<Option<String>> {
        Ok(self
            .req_share_links()
            .await?
            .find("www.linkedin.com")
            .and_then(|lt| {
                lt.query_pairs()
                    .find(|(key, _)| key == "title")
                    .map(|(_, val)| val.to_string())
            }))
    }
}
