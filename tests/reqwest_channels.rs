#![cfg(feature = "reqwest-support")]

extern crate reqwest;
extern crate vlive;

use reqwest::Client;
use vlive::ReqwestVLiveRequester;

#[test]
fn test_get_channel_list() {
    let client = Client::new();

    let channels = client.get_channel_list().unwrap();
    let channel = channels.find_channel("bts").unwrap();

    println!("Found Channel: {:?}", &channel);
    assert!(channel.code == Some("FE619".into()));
}

#[test]
fn test_find_channels() {
    let client = Client::new();

    let channels = client.get_channel_list().unwrap();
    let found = channels.find_channels("bts");

    println!("Found Channels: {:?}", &found);
    assert!(found.len() == 2);
}

#[test]
fn test_find_channel() {
    let client = Client::new();

    let channels = client.get_channel_list().unwrap();
    let found = channels.find_channel("bts").unwrap();

    println!("Found Channel: {:?}", &found);
    assert!(found.name == "BTS");
}

#[test]
fn test_find_partial_channel() {
    let client = Client::new();

    let channels = client.get_channel_list().unwrap();
    let found = channels.find_partial_channel("good day").unwrap();

    println!("Found Channel: {:?}", &found);
    assert!(found.name == "GOOD DAY(굿데이)");
}

#[test]
fn test_decode_channel_code() {
    let client = Client::new();

    let code = client.decode_channel_code("FE619").unwrap();

    assert!(code == 13);
}

#[test]
fn test_get_channel_video_list() {
    let client = Client::new();
    let video_list = client.get_channel_video_list(364, 30, 1).unwrap();

    let channel_name = video_list.channel_info.channel_name;
    let video_count = video_list.total_video_count;

    println!("Found Channel: {}, {} videos", 
        channel_name, video_count);
    assert!(channel_name == "BTS+");
}

#[test]
fn test_video_item() {
    let client = Client::new();
    let video_list = client.get_channel_video_list(364, 30, 1).unwrap().video_list;

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
    let upcoming_videos = client.get_upcoming_video_list(6, 30, 1).unwrap();

    let video_count = upcoming_videos.video_list
        .map(|x| x.len())
        .unwrap_or(0);

    println!("Found {} upcoming videos", video_count);
    assert!(video_count >= 0);
}
