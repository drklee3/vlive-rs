#![cfg(feature = "reqwest-support")]

extern crate reqwest;
extern crate vlive;

use reqwest::Client;
use vlive::ReqwestVLiveRequester;

#[test]
fn test_get_video() {
    let client = Client::new();
    let video = client.get_video(68140).unwrap().unwrap();

    println!("Found video: {}", video.meta.url);
    assert!(video.meta.url == "http://vlive.tv/video/68140");
}
