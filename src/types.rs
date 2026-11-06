use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// YouTube channel metadata object
pub struct ChannelMetadata {
    pub title: String,
    pub description: String,
    pub rss_url: String,
    pub external_id: String,
    pub keywords: String,
    pub owner_urls: Vec<String>,
    pub avatar: Thumbnails,
    pub channel_url: String,
    pub is_family_safe: bool,
    pub available_countries: Option<Countries>,
    pub android_deep_link: String,
    pub android_appindexing_link: String,
    pub ios_appindexing_link: String,
    pub vanity_channel_url: String,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// YouTube video metadata object
pub struct VideoMetadata {
    pub video_id: String,
    pub title: String,
    #[serde_as(as = "DisplayFromStr")]
    pub length_seconds: usize,
    #[serde_as(as = "DisplayFromStr")]
    pub view_count: usize,
    pub author: String,
    pub channel_id: String,
    pub short_description: String,
    pub is_crawlable: bool,
    pub is_owner_viewing: bool,
    pub is_private: bool,
    pub is_unplugged_corpus: bool,
    pub is_live_content: bool,
    pub is_tvfilm_video: bool,
    pub thumbnail: Thumbnails,
    pub available_countries: Option<Countries>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// YouTube playlist metadata object
pub struct PlaylistMetadata {
    pub title: String,
    pub videos_count: usize,
    pub views_count: usize,
    pub android_appindexing_link: String,
    pub ios_appindexing_link: String,
}

#[derive(Deserialize, Debug, Clone)]
/// Used in deserialization of video thumbnail or avatar
pub struct Thumbnail {
    pub url: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Deserialize, Debug, Clone)]
/// Stores video thumbnail / avatar variants in different resolutions
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}

/// List of countries in `ISO 639-1` format
pub type Countries = Vec<String>;
