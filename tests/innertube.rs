use std::sync::LazyLock;

use innertube::Innertube;

const INNERTUBE: LazyLock<Innertube> = LazyLock::new(|| Innertube::new().unwrap());

#[tokio::test]
async fn channel_metadata() {
    let metadata = INNERTUBE
        .channel_metadata("UCBR8-60-B28hp2BmDPdntcQ")
        .await
        .unwrap();
    assert_eq!(&metadata.title, "YouTube");
    assert_eq!(&metadata.description, "YouTube's official YouTube channel");
}

#[tokio::test]
async fn video_metadata() {
    let metadata = INNERTUBE.video_metadata("dQw4w9WgXcQ").await.unwrap();
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
}

#[tokio::test]
async fn playlist_metadata() {
    let metadata = INNERTUBE
        .playlist_metadata("PLFPk5ONFpYvWsnAYJqm6Li4qtg367uUbv")
        .await
        .unwrap();
    assert_eq!(
        &metadata.title,
        "Rickroll Playlist(Never Gonna Give you Up)"
    );
    assert_eq!(metadata.videos_count, 571);
}

#[tokio::test]
async fn video_metadata_multiple() {
    for video_id in [
        "Ge_qSvD8-1o",
        "usXhiWE2Uc0",
        "989-7xsRLR4",
        "5MIsMbFjvkw",
        "aoag03mSuXQ",
    ] {
        INNERTUBE.video_metadata(video_id).await.unwrap();
    }
}

#[tokio::test]
async fn channel_metadata_multiple() {
    for channel_id in [
        "UCNCktfoFAXtXnMlhjyc9SPA",
        "UCJ6td3C9QlPO9O_J5dF4ZzA",
        "UCaFoZFhV1LgbFIB3-6zdWVg",
        "UCEbYhDd6c6vngsF5PQpFVWg",
        "UCHnyfMqiRRG1u-2MsSQLbXA",
    ] {
        INNERTUBE.channel_metadata(channel_id).await.unwrap();
    }
}

#[tokio::test]
async fn playlist_metadata_multiple() {
    for playlist_id in [
        "PLX0oFSIlwhPY",
        "PLUeQUfurHN6ZdGRI7tmCF-Oxgs1ptlU3U",
        "PLB27A26BF930034D9",
        "PLTTUULw3bjATzc86JscVaBHy-O_d_1Ibh",
        "PLu94B0pepV1O78Yi_wlOm3-H-cJxROWVf",
    ] {
        INNERTUBE.playlist_metadata(playlist_id).await.unwrap();
    }
}
