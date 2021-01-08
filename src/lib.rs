/// # vlive-rs
///
/// Unofficial Rust crate for VLive API
///
/// VLive does not have a public API so some actions may not be be ideal,
/// such as having to scape a video page in order to retrieve video data.
/// Some functions will make multiple API requests in order to fetch required data.
///
use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;
use serde_json;

pub mod error;
pub mod model;

pub use error::Error;

use error::Result;
use model::channel;
use model::recent_video::RecentVideo;
use model::video;
use model::video::VideoStatus;

macro_rules! api {
    ($e:expr) => {
        concat!("http://api.vfan.vlive.tv/vproxy/channelplus/", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api!($e), $($rest)*)
    };
}

pub const APP_ID: &str = "8c6cc7b45d2568fb668be6e05b6e5a3b";

#[async_trait]
pub trait VLiveRequester {
    async fn search_channel(&self, query: String, num_rows: u64) -> Result<channel::ChannelList>;

    async fn decode_channel_code(&self, channel_code: String) -> Result<u64>;

    async fn get_channel_video_list(
        &self,
        channel_seq: u32,
        max_rows: u32,
        page_no: u32,
    ) -> Result<channel::ChannelVideoList>;

    async fn get_upcoming_video_list(
        &self,
        channel_seq: u32,
        max_rows: u32,
        page_no: u32,
    ) -> Result<channel::ChannelUpcomingVideoList>;

    async fn get_recent_videos(&self) -> Result<Vec<RecentVideo>>;
    async fn get_video(&self, video_seq: u32) -> Result<video::Video>;

    async fn get_live_video(&self, video_seq: u32) -> Result<video::LiveStreamInfo>;
}

#[async_trait]
impl VLiveRequester for Client {
    async fn search_channel(&self, query: String, num_rows: u64) -> Result<channel::ChannelList> {
        self.get("http://www.vlive.tv/search/auto/channels")
            .query(&[("query", &query), ("maxNumOfRows", &num_rows.to_string())])
            .send()
            .await?
            .json::<channel::ChannelList>()
            .await
            .map_err(From::from)
    }

    async fn decode_channel_code(&self, channel_code: String) -> Result<u64> {
        self.get(api!("decodeChannelCode"))
            .query(&[("app_id", APP_ID), ("channelCode", &channel_code)])
            .send()
            .await?
            .json::<channel::DecodeChannelCodeResult>()
            .await
            .map(|d| d.result.channel_seq)
            .map_err(From::from)
    }

    async fn get_channel_video_list(
        &self,
        channel_seq: u32,
        max_rows: u32,
        page_no: u32,
    ) -> Result<channel::ChannelVideoList> {
        self.get(api!("getChannelVideoList"))
            .query(&[
                ("app_id", APP_ID),
                ("channelSeq", &channel_seq.to_string()),
                ("maxNumOfRows", &max_rows.to_string()),
                ("pageNo", &page_no.to_string()),
            ])
            .send()
            .await?
            .json::<channel::ChannelVideoListResult>()
            .await
            .map(|r| r.result)
            .map_err(From::from)
    }

    async fn get_upcoming_video_list(
        &self,
        channel_seq: u32,
        max_rows: u32,
        page_no: u32,
    ) -> Result<channel::ChannelUpcomingVideoList> {
        self.get(api!("getUpcomingVideoList"))
            .query(&[
                ("app_id", APP_ID),
                ("channelSeq", &channel_seq.to_string()),
                ("maxNumOfRows", &max_rows.to_string()),
                ("pageNo", &page_no.to_string()),
            ])
            .send()
            .await?
            .json::<channel::ChannelUpcomingVideoListResult>()
            .await
            .map(|r| r.result)
            .map_err(From::from)
    }

    async fn get_recent_videos(&self) -> Result<Vec<RecentVideo>> {
        self.get("https://www.vlive.tv/home/video/more?pageNo=1&pageSize=12&viewType=recent")
            .send()
            .await?
            .text()
            .await
            .map_err(From::from)
            .and_then(|text| RecentVideo::from_html(&text))
    }

    async fn get_video(&self, video_seq: u32) -> Result<video::Video> {
        let response = self
            .get("http://www.vlive.tv/video/init/view")
            .query(&[("videoSeq", &video_seq.to_string())])
            .header(
                reqwest::header::REFERER,
                (format!("http://www.vlive.tv/video/{}", video_seq)),
            )
            .send()
            .await?
            .text()
            .await?;

        let (video_id, key) = match find_video(&response) {
            Some(val) => {
                if !val.has_vid_key() {
                    return Err(Error::from("No video ID or Key"));
                }

                // safe to unwrap here
                (val.vid.unwrap(), val.inkey.unwrap())
            }
            None => return Err(Error::from("Could not find video ID and key")),
        };

        let uri = format!(
            "http://global.apis.naver.com/rmcnmv/rmcnmv/vod_play_videoInfo.json?videoId={}&key={}",
            video_id, key
        );
        self.get(&uri)
            .send()
            .await?
            .json::<video::Video>()
            .await
            .map_err(From::from)
    }

    async fn get_live_video(&self, video_seq: u32) -> Result<video::LiveStreamInfo> {
        let response = &self
            .get("http://www.vlive.tv/video/init/view")
            .query(&[("videoSeq", &video_seq.to_string())])
            .header(
                reqwest::header::REFERER,
                format!("http://www.vlive.tv/video/{}", video_seq),
            )
            .send()
            .await?
            .text()
            .await?;

        let video_status = match find_video(&response) {
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

pub fn find_video(s: &str) -> Option<VideoStatus> {
    // basically just scrape the page for video id and key since there's no api endpoint to get this
    lazy_static! {
        // wtf
        // r#"var oVideoStatus = {(?:[) ,\n\t]*(?:"(?:[a-zA-Z0-9_]*)" ?: ?"?(?:[a-zA-Z0-9_]*)(?:"?,)?))(?:[) ,\n\t]*(?:"(?:[a-zA-Z0-9_]*)" ?: ?"?(?:[a-zA-Z0-9_]*)(?:"?,)?))(?:[) ,\n\t]*(?:"(?:[a-zA-Z0-9_]*)" ?: ?"?(?:[a-zA-Z0-9_]*)(?:"?,)?))(?:[) ,\n\t]*(?:"(?:[a-zA-Z0-9_]*)" ?: ?"?(?:[a-zA-Z0-9_]*)(?:"?,)?))"#
        // r#"vlive\.video\.init\((?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))"#
        // var oVideoStatus = (\{[\n\t"\w :,]*\})
        static ref RE: Regex =
            Regex::new(r#"<script .*>\nvar oVideoStatus = (\{[\\\{}\[\]/\.?=+\n\t"\w :,]*})\n</script>"#).unwrap();
    }

    // check regex matches
    let caps = match RE.captures(s) {
        Some(val) => val,
        None => return None,
    };

    let json = caps.get(1).map(|m| m.as_str()).unwrap();

    serde_json::from_str::<VideoStatus>(json).ok()
}
