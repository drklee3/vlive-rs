use reqwest::Client;
use vlive::VLiveRequester;

// Not updated to new vlive site yet
#[ignore]
#[tokio::test]
async fn test_get_video() {
    let client = Client::new();
    let video = client.get_video(67845).await.unwrap();

    println!("Found video: {}", video.meta.url);
    assert!(video.meta.url == "http://vlive.tv/video/67845");
}

// ignore this test as needs a video that's live to pass
#[ignore]
#[tokio::test]
async fn test_get_live_video() {
    let client = Client::new();
    let video = client.get_live_video(70738).await.unwrap();

    println!("Found live video: {:?}", video);
    assert!(video.resolutions.first().is_some());
}
