use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

// "metadata": {
//     "channelMetadataRenderer": {
//         "title": "Asd Dsa",
//         "description": "",
//         "rssUrl": "https://www.youtube.com/feeds/videos.xml?channel_id=UCCOXbb-KWEx0OZTMt67iJaw",
//         "externalId": "UCCOXbb-KWEx0OZTMt67iJaw",
//         "keywords": "",
//         "ownerUrls": [
//             "http://www.youtube.com/@AsdDsa-j1v8r"
//         ],
//         "avatar": {
//             "thumbnails": [
//                 {
//                     "url": "https://yt3.googleusercontent.com/ytc/AIdro_nATaj_XyZ449yTpyY9GNkkpedOEjet8B9ru95ikqwS3Mf-MXvcS1UlV0t6a5VEu6AwMQ=s900-c-k-c0x00ffffff-no-rj",
//                     "width": 900,
//                     "height": 900
//                 }
//             ]
//         },
//         "channelUrl": "https://www.youtube.com/channel/UCCOXbb-KWEx0OZTMt67iJaw",
//         "isFamilySafe": true,
//         "availableCountryCodes": [
// //   "androidDeepLink": "android-app://com.google.android.youtube/http/www.youtube.com/channel/UCCOXbb-KWEx0OZTMt67iJaw",
//         "androidAppindexingLink": "android-app://com.google.android.youtube/http/www.youtube.com/channel/UCCOXbb-KWEx0OZTMt67iJaw",
//         "iosAppindexingLink": "ios-app://544007664/vnd.youtube/www.youtube.com/channel/UCCOXbb-KWEx0OZTMt67iJaw",
//         "vanityChannelUrl": "http://www.youtube.com/@AsdDsa-j1v8r"

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMetadata {
    pub title: String,
    pub description: String,
    pub rss_url: String,
    pub external_id: String,
    pub keywords: String,
    pub owner_urls: Vec<String>,
    // avatar: TODO
    pub channel_url: String,
    pub is_family_safe: bool,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
    //thumbnail: TODO,
}
