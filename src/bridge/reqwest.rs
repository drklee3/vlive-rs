use reqwest::Client;
use model::channel;
use model::video;
use serde_json;
use Error;
use bridge::util;
use error::Result;
use ::APP_ID;
use ::BASE_URL;

pub trait VLiveRequester {
    fn get_channel_list(&self)
        -> Result<Option<channel::ChannelList>>;
    
    fn decode_channel_code<T: AsRef<str>>(&self, channel_code: T)
        -> Result<Option<u64>>;
    
    fn get_channel_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Result<Option<channel::ChannelVideoList>>;
    
    fn get_upcoming_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Result<Option<channel::ChannelUpcomingVideoList>>;
    
    fn get_video(&self, video_seq: u32)
        -> Result<Option<video::Video>>;
}

impl VLiveRequester for Client {
    fn get_channel_list(&self) -> Result<Option<channel::ChannelList>> {
        let uri = "http://www.vlive.tv/search/auto/channels";

        let response = self.get(uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }
    fn decode_channel_code<T: AsRef<str>>(&self, channel_code: T) -> Result<Option<u64>> {
        let uri = format!("{}decodeChannelCode?app_id={}&channelCode={}", BASE_URL, APP_ID, channel_code.as_ref());

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }
    fn get_channel_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Result<Option<channel::ChannelVideoList>> {
        let uri = format!("{}getChannelVideoList?app_id={}&channelSeq={}&maxNumOfRows={}&pageNo={}",
            BASE_URL, APP_ID, channel_seq, max_rows, page_no,
        );
        let response = self.get(&uri).send()?;

        serde_json::from_reader(response)
            .map_err(From::from)
            .map(|resp: channel::ChannelVideoListResult| {
                Some(resp.result)
            })
    }
    fn get_upcoming_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Result<Option<channel::ChannelUpcomingVideoList>> {
        let uri = format!("{}getUpcomingVideoList?app_id={}&channelSeq={}&maxNumOfRows={}&pageNo={}",
            BASE_URL, APP_ID, channel_seq, max_rows, page_no,
        );

        let response = self.get(&uri).send()?;
        serde_json::from_reader(response)
            .map_err(From::from)
            .map(|resp: channel::ChannelUpcomingVideoListResult| {
                Some(resp.result)
            })
    }
    fn get_video(&self, video_seq: u32)
        -> Result<Option<video::Video>> {
        
        let uri = format!("http://www.vlive.tv/video/{}", video_seq);

        let response = self.get(&uri).send()?.text()?;

        let (video_id, key) = match util::find_video_id_key(&response) {
            Some((video_id, key)) => (video_id, key),
            None => return Err(Error::from("Could not find video ID and key")),
        };

        let uri = format!("http://global.apis.naver.com/rmcnmv/rmcnmv/vod_play_videoInfo.json?videoId={}&key={}",
            video_id, key);
        
        let response = self.get(&uri).send()?;
        serde_json::from_reader(response)
            .map_err(From::from)
    }
}