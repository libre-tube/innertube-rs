use std::sync::LazyLock;

use innertube::Innertube;

const INNERTUBE: LazyLock<Innertube> = LazyLock::new(|| Innertube::new().unwrap());

#[tokio::test]
async fn channel_title() {
    assert_eq!(
        &INNERTUBE
            .channel("UCBR8-60-B28hp2BmDPdntcQ")
            .title()
            .await
            .unwrap()
            .unwrap(),
        "YouTube"
    );
}

#[tokio::test]
async fn video_title() {
    assert_eq!(
        INNERTUBE
            .video("dQw4w9WgXcQ")
            .title()
            .await
            .unwrap()
            .unwrap(),
        "Rick Astley - Never Gonna Give You Up (Official Video) (4K Remaster)"
    );
}

#[tokio::test]
async fn playlist_title() {
    assert_eq!(
        &INNERTUBE
            .playlist("PLTTUULw3bjATzc86JscVaBHy-O_d_1Ibh")
            .title()
            .await
            .unwrap()
            .unwrap(),
        "Installing Arch Linux every day until I find a girlfriend"
    );
}

#[tokio::test]
async fn channel_metadata() {
    let metadata = INNERTUBE
        .channel("UCBR8-60-B28hp2BmDPdntcQ")
        .metadata()
        .await
        .unwrap()
        .to_owned();
    assert_eq!(&metadata.title, "YouTube");
    assert_eq!(&metadata.description, "YouTube's official YouTube channel");
}

#[tokio::test]
async fn video_metadata() {
    let metadata = INNERTUBE
        .video("dQw4w9WgXcQ")
        .metadata()
        .await
        .unwrap()
        .to_owned();
    assert_eq!(
        &metadata.title,
        "Rick Astley - Never Gonna Give You Up (Official Video) (4K Remaster)"
    );
    assert!(
        metadata
            .short_description
            .starts_with("The official video for “Never Gonna Give You Up” by Rick Astley.")
    );
    assert_eq!(metadata.length_seconds, 213);
    assert_eq!(&metadata.author, "Rick Astley");
    assert_eq!(
        metadata.thumbnail.max_resolution().url,
        "https://i.ytimg.com/vi_webp/dQw4w9WgXcQ/maxresdefault.webp".to_string()
    );
    assert!(
        metadata
            .thumbnail
            .min_resolution()
            .url
            .starts_with("https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg")
    );
}

#[tokio::test]
async fn playlist_metadata() {
    let metadata = INNERTUBE
        .playlist("PLFPk5ONFpYvWsnAYJqm6Li4qtg367uUbv")
        .metadata()
        .await
        .unwrap()
        .to_owned();
    assert_eq!(
        &metadata.title,
        "Rickroll Playlist(Never Gonna Give you Up)"
    );
    assert_eq!(
        metadata.description,
        Some("Rick Astley \"Never Gonna give you Up\" Remixes".to_string())
    );
    assert_eq!(metadata.video_count, 571);
    assert!(
        metadata
            .thumbnail
            .max_resolution()
            .url
            .starts_with("https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg")
    );
    assert!(
        metadata
            .thumbnail
            .min_resolution()
            .url
            .starts_with("https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg")
    );
}

#[tokio::test]
async fn video_metadata_multiple() {
    let it = INNERTUBE;

    for video_id in [
        "Ge_qSvD8-1o",
        "usXhiWE2Uc0",
        "989-7xsRLR4",
        "5MIsMbFjvkw",
        "aoag03mSuXQ",
    ] {
        let target = it.video(video_id);
        target.title().await.unwrap().unwrap();
        target.metadata().await.unwrap();
    }
}

#[tokio::test]
async fn channel_metadata_multiple() {
    let it = INNERTUBE;

    for channel_id in [
        "UCNCktfoFAXtXnMlhjyc9SPA",
        "UCJ6td3C9QlPO9O_J5dF4ZzA",
        "UCaFoZFhV1LgbFIB3-6zdWVg",
        "UCEbYhDd6c6vngsF5PQpFVWg",
        "UCHnyfMqiRRG1u-2MsSQLbXA",
    ] {
        let target = it.channel(channel_id);
        target.title().await.unwrap().unwrap();
        target.metadata().await.unwrap();
    }
}
#[tokio::test]
async fn playlist_metadata_multiple() {
    let it = INNERTUBE;

    for playlist_id in [
        "PLX0oFSIlwhPY",
        "PLUeQUfurHN6ZdGRI7tmCF-Oxgs1ptlU3U",
        "PLB27A26BF930034D9",
        "PLTTUULw3bjATzc86JscVaBHy-O_d_1Ibh",
        "PLu94B0pepV1O78Yi_wlOm3-H-cJxROWVf",
    ] {
        let target = it.playlist(playlist_id);
        target.title().await.unwrap().unwrap();
        target.metadata().await.unwrap();
    }
}
