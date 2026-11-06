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
