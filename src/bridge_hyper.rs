use futures::{Future, Stream, future};
use hyper::client::{Client, Connect};
use hyper::{Error as HyperError, Uri};
use serde_json;
use std::str::FromStr;
use std::str;
use ::model::channel;
use ::Error;

const APP_ID: &str = "8c6cc7b45d2568fb668be6e05b6e5a3b";
const BASE_URL: &str = "http://api.vfan.vlive.tv/vproxy/channelplus/";

pub trait VLiveRequester {
    fn get_channel_list(&self)
        -> Box<Future<Item = Option<channel::ChannelList>, Error = Error>>;
    
    fn decode_channel_code<T: AsRef<str>>(&self, channel_code: T)
        -> Box<Future<Item = Option<u64>, Error = Error>>;
    
    fn get_channel_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Box<Future<Item = Option<channel::ChannelVideoList>, Error = Error>>;
    
    // fn get_upcoming_video_list(&self, channel_seq: u32, max_rows: u32)
    //     -> Box<Future<Item = Option<channel::ChannelUpcomingVideoList>, Error = Error>>;
}

impl<B, C: Connect> VLiveRequester for Client<C, B>
    where B: Stream<Error = HyperError> + 'static, B::Item: AsRef<[u8]> {
    fn get_channel_list(&self)
        -> Box<Future<Item = Option<channel::ChannelList>, Error = Error>> {
        Box::new(get_channel_list(self))
    }

    fn decode_channel_code<T: AsRef<str>>(&self, channel_code: T)
        -> Box<Future<Item = Option<u64>, Error = Error>> {
        Box::new(decode_channel_code(self, channel_code))
    }
    
    fn get_channel_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Box<Future<Item = Option<channel::ChannelVideoList>, Error = Error>> {
        Box::new(get_channel_video_list(self, channel_seq, max_rows, page_no))
    }
    
    // fn get_upcoming_video_list(&self, channel_seq: u32, max_rows: u32)
    //     -> Box<Future<Item = Option<channel::ChannelUpcomingVideoList>, Error = Error>> {
    //     Box::new(get_upcoming_video_list(self, channel_seq, max_rows))
    // }
}


pub fn get_channel_list<B, C> (client: &Client<C, B>)
    -> Box<Future<Item = Option<channel::ChannelList>, Error = Error>>
	    where C: Connect,
	          B: Stream<Error = HyperError> + 'static,
	          B::Item: AsRef<[u8]> {
    
    let url = "http://www.vlive.tv/search/auto/channels";
    let uri = match Uri::from_str(&url) {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(Error::Uri(why))),
    };

    Box::new(client.get(uri)
        .and_then(|res| res.body().concat2())
        .map_err(From::from)
        .and_then(|body| serde_json::from_slice::<channel::ChannelList>(&body).map_err(From::from))
        .map(|resp| if !resp.0.is_empty() {
            Some(resp)
        } else {
            None
        }))
}


pub fn decode_channel_code<B, C, T> (client: &Client<C, B>, channel_code: T)
    -> Box<Future<Item = Option<u64>, Error = Error>>
        where T: AsRef<str>,
              C: Connect,
              B: Stream<Error = HyperError> + 'static,
              B::Item: AsRef<[u8]> {
    
    let url = format!("{}decodeChannelCode?app_id={}&channelCode={}", BASE_URL, APP_ID, channel_code.as_ref());
    let uri = match Uri::from_str(&url) {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(Error::Uri(why))),
    };

    Box::new(client.get(uri)
        .and_then(|res| res.body().concat2())
        .map_err(From::from)
        .and_then(|body| 
            serde_json::from_slice::<serde_json::Value>(&body)
                .map_err(From::from)
                .map(|d| d.pointer("/result/channelSeq").unwrap().as_u64().unwrap())
        )
        .map(|resp| Some(resp))
    )
}

pub fn get_channel_video_list<B, C> (client: &Client<C, B>, channel_seq: u32, max_rows: u32, page_no: u32)
    -> Box<Future<Item = Option<channel::ChannelVideoList>, Error = Error>>
        where C: Connect,
              B: Stream<Error = HyperError> + 'static,
              B::Item: AsRef<[u8]> {
    let url = format!("{}getChannelVideoList?app_id={}&channelSeq={}&maxNumOfRows={}&pageNo={}",
        BASE_URL, APP_ID, channel_seq, max_rows, page_no,
    );
    let uri = match Uri::from_str(&url) {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(Error::Uri(why))),
    };

    Box::new(client.get(uri)
        .and_then(|res| res.body().concat2())
        .map_err(From::from)
        .and_then(|body| {
            // let string = str::from_utf8(&body).unwrap();
            // println!("{}", string);
            serde_json::from_slice::<channel::ChannelVideoListResult>(&body).map_err(From::from)
        })
        .map(|resp| Some(resp.result))
    )
}
pub fn get_upcoming_video_list<B, C> (client: &Client<C, B>, channel_seq: u32, max_rows: u32) {

}