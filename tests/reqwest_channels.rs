#![cfg(feature = "reqwest-support")]

extern crate reqwest;
extern crate vlive;

use reqwest::Client;
use vlive::ReqwestVLiveRequester;

#[test]
fn test_get_channel_list() {
    let client = Client::new();

    let channels = client.get_channel_list().unwrap().unwrap();
    let channel = channels.find_channel("bts").unwrap();

    println!("Found Channel: {:?}", &channel);
    assert!(channel.code == Some("FE619".into()));
}

#[test]
fn test_get_channel_video_list() {
    let client = Client::new();
    let video_list = client.get_channel_video_list(364, 30, 1).unwrap().unwrap();

    let channel_name = video_list.channel_info.channel_name;
    let video_count = video_list.total_video_count;

    println!("Found Channel: {}, {} videos", 
        channel_name, video_count);
    assert!(channel_name == "BTS+");
}

#[test]
fn test_video_item() {
    let client = Client::new();
    let video_list = client.get_channel_video_list(364, 30, 1).unwrap().unwrap().video_list;

    let last_video = video_list.last().unwrap();

    println!("Found Video: {}, URL: {}, is live: {}", 
        last_video.title,
        last_video.url(),
        last_video.is_live()
    );
    assert!(!last_video.is_live());
}

#[test]
fn test_get_upcoming_video_list() {
    let client = Client::new();
    let upcoming_videos = client.get_upcoming_video_list(6, 30, 1).unwrap().unwrap();

    let video_count = upcoming_videos.video_list
        .map(|x| x.len())
        .unwrap_or(0);

    println!("Found {} upcoming videos", video_count);
    assert!(video_count >= 0);
}

#[test]
fn test_get_video() {
    let client = Client::new();
    let video = client.get_video(68140).unwrap().unwrap();

    println!("Found video: {}", video.meta.url);
    assert!(video.meta.url == "http://vlive.tv/video/68140");
}
