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
        println!("Response: {:?}", resp);
        assert!(resp.is_some());

        Ok(())
    }).or_else(|err| {
        eprintln!("Error: {}", err);
        assert!(false);

        Err(())
    });

    core.run(done).expect("core err");
}

