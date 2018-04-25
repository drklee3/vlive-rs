/// # vlive-rs
/// 
/// Unofficial Rust crate for VLive API
/// 
/// VLive does not have a public API so some actions may not be be ideal,
/// such as having to scape a video page in order to retrieve video data.
/// Some functions will make multiple API requests in order to fetch required data.
/// 

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "reqwest-support")]
extern crate reqwest;
#[cfg(feature = "hyper-support")]
extern crate hyper;
#[cfg(feature = "hyper-support")]
extern crate futures;

mod error;
pub mod model;
pub mod bridge;

pub use error::Error;

pub const APP_ID: &str = "8c6cc7b45d2568fb668be6e05b6e5a3b";
pub const BASE_URL: &str = "http://api.vfan.vlive.tv/vproxy/channelplus/";

// re-export traits
#[cfg(feature = "hyper-support")]
pub use bridge::hyper::VLiveRequester as HyperVLiveRequester;
#[cfg(feature = "reqwest-support")]
pub use bridge::reqwest::VLiveRequester as ReqwestVLiveRequester;
