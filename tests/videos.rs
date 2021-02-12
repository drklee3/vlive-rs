use reqwest::Client;
use vlive::VLiveRequester;

#[tokio::test]
async fn test_get_recent_videos() {
    let client = Client::new();
    let videos = client.get_recent_videos(12, 1).await.unwrap();

    println!("Found recent videos: {:#?}", videos);
    assert!(!videos.is_empty());
}

#[tokio::test]
async fn test_get_recent_videos_detail() {
    let client = Client::new();
    let videos = client
        .get_recent_videos(12, 1)
        .await
        .expect("Get recent videos");

    println!("Found recent videos: {:#?}", videos);
    assert!(!videos.is_empty());

    for video in videos {
        let _data = client.get_video(video.video_seq).await.expect("Get video");

        if let Some(url) = video.thumbnail_url {
            assert!(url.ends_with(".png") || url.ends_with(".jpg") || url.ends_with(".jpeg"));
        }
    }
}

#[tokio::test]
async fn test_get_video() {
    let client = Client::new();
    let video = client.get_video(232024).await.unwrap();

    println!("Found video: {:?}", video);
    assert_eq!(
        video.post_detail.get_detail().expect("Has detail").url,
        "https://www.vlive.tv/post/1-20783092"
    );
}

#[tokio::test]
async fn test_get_video_chplus() {
    let client = Client::new();
    let video = client.get_video(233176).await.unwrap();

    println!("Found video: {:?}", video);
    assert_eq!(
        video.post_detail.get_detail().expect("Has detail").url,
        "https://www.vlive.tv/post/0-20890974"
    );
}

#[tokio::test]
async fn test_get_video_streams() {
    let client = Client::new();
    let video = client.get_video_streams(232024).await.unwrap();

    println!("Found video: {}", video.meta.url);
    assert_eq!(video.meta.url, "http://vlive.tv/video/232024");
}

#[tokio::test]
async fn test_get_video_streams_chplus() {
    let client = Client::new();
    let video = client.get_video_streams(233176).await.unwrap();

    println!("Found video: {:?}", video);
    assert_eq!(video.meta.url, "http://vlive.tv/video/233176");
}

// ignore this test as needs a video that's live to pass
/*
#[ignore]
#[tokio::test]
async fn test_get_live_video() {
    let client = Client::new();
    let video = client.get_live_video(70738).await.unwrap();

    println!("Found live video: {:?}", video);
    assert!(video.resolutions.first().is_some());
}
*/
