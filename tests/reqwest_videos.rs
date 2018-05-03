#![cfg(feature = "reqwest-support")]

extern crate reqwest;
extern crate vlive;

use reqwest::Client;
use vlive::ReqwestVLiveRequester;

#[test]
fn test_get_video() {
    let client = Client::new();
    let video = client.get_video(67845).unwrap();

    println!("Found video: {}", video.meta.url);
    assert!(video.meta.url == "http://vlive.tv/video/67845");
}
