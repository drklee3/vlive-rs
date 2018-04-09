extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate vlive_rs;

use futures::Future;
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use tokio_core::reactor::{Core, Handle};
use vlive_rs::bridge_hyper::VLiveRequester;

#[inline]
fn client(handle: &Handle) -> Client<HttpConnector, Body> {
    Client::configure()
        .build(handle)
}

#[test]
fn test_get_channel_list() {
    let mut core = Core::new().unwrap();
    let client = client(&core.handle());

    let done = client.get_channel_list().and_then(|resp| {
        let channel = resp.and_then(|x| x.find_channel("bts")).unwrap();

        println!("Found Channel: {:?}", &channel);
        assert!(channel.code == Some("FE619".into()));
        Ok(())
    }).or_else(|err| {
        eprintln!("Error: {}", err);
        assert!(false);

        Err(())
    });

    core.run(done).expect("core err");
}

#[test]
fn test_get_channel_video_list() {
    let mut core = Core::new().unwrap();
    let client = client(&core.handle());

    let done = client.get_channel_video_list(364, 30, 1).and_then(|resp| {
        let channel_video_list = resp.unwrap();
        let channel_name = channel_video_list.channel_info.channel_name;
        let video_count = channel_video_list.total_video_count;

        println!("Found Channel: {}, {} videos", 
            channel_name, video_count);
        assert!(channel_name == "BTS+");
        Ok(())
    }).or_else(|err| {
        eprintln!("Error: {}", err);
        assert!(false);

        Err(())
    });

    core.run(done).expect("core err");
}

#[test]
fn test_video_item() {
    let mut core = Core::new().unwrap();
    let client = client(&core.handle());

    let done = client.get_channel_video_list(364, 30, 1).and_then(|resp| {
        let video_list = resp.unwrap().video_list;
        let last_video = video_list.last().unwrap();

        println!("Found Video: {}, URL: {}, is live: {}", 
            last_video.title,
            last_video.url(),
            last_video.is_live()
        );
        assert!(!last_video.is_live());
        Ok(())
    }).or_else(|err| {
        eprintln!("Error: {}", err);
        assert!(false);

        Err(())
    });

    core.run(done).expect("core err");
}

#[test]
fn test_get_upcoming_video_list() {
    let mut core = Core::new().unwrap();
    let client = client(&core.handle());

    let done = client.get_upcoming_video_list(6, 30, 1).and_then(|resp| {
        let upcoming_videos = resp.unwrap();
        let video_count = upcoming_videos.video_list
            .map(|x| x.len())
            .unwrap_or(0);

        println!("Found {} upcoming videos", video_count);
        assert!(video_count >= 0);
        Ok(())
    }).or_else(|err| {
        eprintln!("Error: {}", err);
        assert!(false);

        Err(())
    });

    core.run(done).expect("core err");
}


