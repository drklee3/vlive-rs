use reqwest::Client;
use vlive::VLiveRequester;

#[tokio::test]
async fn test_search_channel() {
    let client = Client::new();

    let channels = client.search_channel("bts".into(), 10).await.unwrap();
    let channel = channels.0.first().unwrap();

    println!("Found Channel: {:?}", &channel);
    assert!(channel.code == Some("FE619".into()));
}

#[tokio::test]
async fn test_decode_channel_code() {
    let client = Client::new();

    let code = client.decode_channel_code("FE619".into()).await.unwrap();

    assert!(code == 13);
}

#[tokio::test]
async fn test_get_channel_video_list() {
    let client = Client::new();
    let channel = client.get_channel_video_list(364, 30, 1).await.unwrap();

    println!("Found Channel: {}, {} videos", channel.channel_info.channel_name, channel.total_video_count);
    assert!(channel.channel_info.channel_name == "BTS+");
    // Requested enough videos, should definitely have more than 30 videos total
    // so this should be 30 since we requested 30 videos
    assert_eq!(channel.video_list.len(), 30);
}

#[tokio::test]
async fn test_video_item() {
    let client = Client::new();
    let video_list = client
        .get_channel_video_list(364, 30, 1)
        .await
        .unwrap()
        .video_list;

    let last_video = video_list.last().unwrap();

    println!(
        "Found Video: {}, URL: {}, is live: {}",
        last_video.title,
        last_video.url(),
        last_video.is_live()
    );
    assert!(!last_video.is_live());
}

#[tokio::test]
async fn test_get_upcoming_video_list() {
    let client = Client::new();
    let upcoming_videos = client.get_upcoming_video_list(6, 30, 1).await.unwrap();

    let video_count = upcoming_videos.video_list.map(|x| x.len()).unwrap_or(0);

    println!("Found {} upcoming videos", video_count);
    assert!(true);
}
