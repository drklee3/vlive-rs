#![cfg(feature = "hyper-support")]

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate vlive;

use futures::Future;
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use tokio_core::reactor::{Core, Handle};
use vlive::HyperVLiveRequester;

#[inline]
fn client(handle: &Handle) -> Client<HttpConnector, Body> {
    Client::configure()
        .build(handle)
}

/*
#[test]
fn test_get_video() {
    let mut core = Core::new().unwrap();
    let client = client(&core.handle());

    let done = client.get_video(66976).and_then(|resp| {
        let (video_id, key) = resp.unwrap();
        println!("video_id: {},\n key: {}", video_id, key);

        assert!(video_id.len() > 3);
        Ok(())
    }).or_else(|err| {
        eprintln!("Error: {}", err);
        assert!(false);

        Err(())
    });

    core.run(done).expect("core err");
}
*/