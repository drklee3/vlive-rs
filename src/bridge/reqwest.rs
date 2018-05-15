use reqwest::Client;
use reqwest::header::Referer;
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
        -> Result<channel::ChannelList>;
    
    fn decode_channel_code<T: AsRef<str>>(&self, channel_code: T)
        -> Result<u64>;
    
    fn get_channel_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Result<channel::ChannelVideoList>;
    
    fn get_upcoming_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Result<channel::ChannelUpcomingVideoList>;
    
    fn get_video(&self, video_seq: u32)
        -> Result<video::Video>;
    
    fn get_live_video(&self, video_seq: u32)
        -> Result<video::LiveStreamInfo>;
}

impl VLiveRequester for Client {
    fn get_channel_list(&self) -> Result<channel::ChannelList> {
        let uri = "http://www.vlive.tv/search/auto/channels";
        let response = self.get(uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn decode_channel_code<T: AsRef<str>>(&self, channel_code: T) -> Result<u64> {
        let uri = format!("{}decodeChannelCode?app_id={}&channelCode={}", BASE_URL, APP_ID, channel_code.as_ref());
        let response = self.get(&uri).send()?;

        serde_json::from_reader(response)
            .map_err(From::from)
            .and_then(|d: serde_json::Value| d
                .pointer("/result/channelSeq")
                .and_then(|d| d.as_u64())
                .ok_or(Error::from("Invalid channelSeq"))
            )
    }

    fn get_channel_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Result<channel::ChannelVideoList> {
        let uri = format!("{}getChannelVideoList?app_id={}&channelSeq={}&maxNumOfRows={}&pageNo={}",
            BASE_URL, APP_ID, channel_seq, max_rows, page_no,
        );
        let response = self.get(&uri).send()?;

        serde_json::from_reader(response)
            .map_err(From::from)
            .map(|resp: channel::ChannelVideoListResult| {
                resp.result
            })
    }

    fn get_upcoming_video_list(&self, channel_seq: u32, max_rows: u32, page_no: u32)
        -> Result<channel::ChannelUpcomingVideoList> {
        let uri = format!("{}getUpcomingVideoList?app_id={}&channelSeq={}&maxNumOfRows={}&pageNo={}",
            BASE_URL, APP_ID, channel_seq, max_rows, page_no,
        );
        let response = self.get(&uri).send()?;

        serde_json::from_reader(response)
            .map_err(From::from)
            .map(|resp: channel::ChannelUpcomingVideoListResult| {
                resp.result
            })
    }

    fn get_video(&self, video_seq: u32) -> Result<video::Video> {
        let uri = format!("http://www.vlive.tv/video/init/view?videoSeq={}", video_seq);
        let response = self
            .get(&uri)
            .header(Referer::new(format!("http://www.vlive.tv/video/{}", video_seq)))
            .send()?.text()?;

        let (video_id, key) = match util::find_video(&response) {
            Some(val) => {
                if !val.has_vid_key() {
                    return Err(Error::from("No video ID or Key"));
                }

                // safe to unwrap here
                (val.vid.unwrap(), val.inkey.unwrap())
            },
            None => return Err(Error::from("Could not find video ID and key")),
        };

        let uri = format!("http://global.apis.naver.com/rmcnmv/rmcnmv/vod_play_videoInfo.json?videoId={}&key={}",
            video_id, key);
        let response = self.get(&uri).send()?;

        serde_json::from_reader(response)
            .map_err(From::from)
    }

    fn get_live_video(&self, video_seq: u32) -> Result<video::LiveStreamInfo> {
        let uri = format!("http://www.vlive.tv/video/init/view?videoSeq={}", video_seq);
        let response = self
            .get(&uri)
            .header(Referer::new(format!("http://www.vlive.tv/video/{}", video_seq)))
            .send()?.text()?;

        let video_status = match util::find_video(&response) {
            Some(val) => val,
            None => return Err(Error::from("Could not find video ID and key")),
        };

        if video_status.live_stream_info.is_none() {
            return Err(Error::from("Video is not live or is invalid"));
        }

        serde_json::from_str::<video::LiveStreamInfo>(&video_status.live_stream_info.unwrap())
            .map_err(From::from)
    }
}
