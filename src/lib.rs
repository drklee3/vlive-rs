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

mod endpoints;
pub mod error;
pub mod model;

pub use error::Error;

use endpoints::APP_ID;
use error::Result;
use model::{
    board_posts::BoardPosts,
    channel,
    grouped_board::{Board, GroupedBoards},
    recent_video::RecentVideo,
    video,
};

macro_rules! api {
    ($e:expr) => {
        concat!("http://api.vfan.vlive.tv/vproxy/channelplus/", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api!($e), $($rest)*)
    };
}

#[async_trait]
pub trait VLiveRequester {
    async fn search_channel(&self, query: &str, num_rows: u64) -> Result<channel::ChannelList>;
    async fn get_channel_info(&self, channel_code: &str) -> Result<channel::ChannelBasicInfo>;

    async fn decode_channel_code(&self, channel_code: &str) -> Result<u64>;
    async fn get_channel_grouped_boards(&self, channel_code: &str) -> Result<GroupedBoards>;
    async fn get_channel_board(&self, channel_code: &str, board_id: u64) -> Result<Board>;
    async fn get_board_posts(&self, channel_code: &str, board_id: u64) -> Result<BoardPosts>;

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

    async fn get_recent_videos(&self, page_size: u64, page_no: u64) -> Result<Vec<RecentVideo>>;
    async fn get_video(&self, video_seq: u64) -> Result<video::VideoState>;
    async fn get_video_streams(&self, video_seq: u64) -> Result<video::Video>;
}

#[async_trait]
impl VLiveRequester for Client {
    /// Search for a channel by name
    async fn search_channel(&self, query: &str, num_rows: u64) -> Result<channel::ChannelList> {
        self.get("http://www.vlive.tv/search/auto/channels")
            .query(&[("query", query), ("maxNumOfRows", &num_rows.to_string())])
            .send()
            .await?
            .json::<channel::ChannelList>()
            .await
            .map_err(From::from)
    }

    /// Get basic information about a channel
    async fn get_channel_info(&self, channel_code: &str) -> Result<channel::ChannelBasicInfo> {
        self.get(&endpoints::channel_info_url(&channel_code))
            .header(
                reqwest::header::REFERER,
                (format!("https://www.vlive.tv/channel/{}", channel_code)),
            )
            .send()
            .await?
            .json::<channel::ChannelBasicInfo>()
            .await
            .map_err(From::from)
    }

    async fn decode_channel_code(&self, channel_code: &str) -> Result<u64> {
        self.get(api!("decodeChannelCode"))
            .query(&[("app_id", APP_ID), ("channelCode", &channel_code)])
            .send()
            .await?
            .json::<channel::DecodeChannelCodeResult>()
            .await
            .map(|d| d.result.channel_seq)
            .map_err(From::from)
    }

    /// Get a channel's boards, grouped into different categories
    async fn get_channel_grouped_boards(&self, channel_code: &str) -> Result<GroupedBoards> {
        self.get(&endpoints::grouped_boards_url(&channel_code))
            .header(
                reqwest::header::REFERER,
                (format!("https://www.vlive.tv/channel/{}", channel_code)),
            )
            .send()
            .await?
            .json::<GroupedBoards>()
            .await
            .map_err(From::from)
    }

    /// Gets a channel's board info. Note that this doesn't include the actual board posts
    /// Channel code is required since the referer requires the channel board URL
    async fn get_channel_board(&self, channel_code: &str, board_id: u64) -> Result<Board> {
        self.get(&endpoints::board_url(board_id))
            .header(
                reqwest::header::REFERER,
                (format!(
                    "https://www.vlive.tv/channel/{}/board/{}",
                    channel_code, board_id
                )),
            )
            .send()
            .await?
            .json::<Board>()
            .await
            .map_err(From::from)
    }

    /// Get the posts in a given board
    async fn get_board_posts(&self, channel_code: &str, board_id: u64) -> Result<BoardPosts> {
        self.get(&endpoints::board_posts_url(board_id))
            .header(
                reqwest::header::REFERER,
                (format!(
                    "https://www.vlive.tv/channel/{}/board/{}",
                    channel_code, board_id
                )),
            )
            .send()
            .await?
            .json::<BoardPosts>()
            .await
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

    /// Fetches new videos from any channel (equivalent to the new section on the homepage)
    async fn get_recent_videos(&self, page_size: u64, page_no: u64) -> Result<Vec<RecentVideo>> {
        self.get("https://www.vlive.tv/home/video/more")
            .query(&[
                ("pageNo", &page_no.to_string()),
                ("pageSize", &page_size.to_string()),
            ])
            .send()
            .await?
            .text()
            .await
            .map_err(From::from)
            .and_then(|text| RecentVideo::from_html(&text))
    }

    async fn get_video(&self, video_seq: u64) -> Result<video::VideoState> {
        let video_url = endpoints::video_url(video_seq);
        let response = self.get(&video_url).send().await?.text().await?;

        find_video(&response)
    }

    /// Get detailed information about a given video
    async fn get_video_streams(&self, video_seq: u64) -> Result<video::Video> {
        let video_url = endpoints::video_url(video_seq);
        let video_state = self.get_video(video_seq).await?;

        let video_id = video_state.post_detail.get_detail().official_video.vod_id.as_ref();
        // let video_id = video_state.post_detail.error.data.official_video.vod_id.as_ref();

        let video_key = self
            .get(&endpoints::inkey_url(video_seq))
            .header(reqwest::header::REFERER, video_url)
            // Requires user agent or error 500
            .header(reqwest::header::USER_AGENT, "vlive-rs")
            .send()
            .await?
            .json::<video::VideoKey>()
            .await?;

        self.get(&endpoints::vod_url(&video_id, &video_key.inkey))
            .send()
            .await?
            .json::<video::Video>()
            .await
            .map_err(From::from)
    }
}

pub fn find_video(s: &str) -> Result<video::VideoState> {
    // basically just scrape the page for video id and key since there's no api endpoint to get this
    // Yes I know regex shouldn't be used for html parsing, but it's kind of just in a JS script in html weird
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"window\.__PRELOADED_STATE__\s*=([^<]*),\s*function"#).unwrap();
    }

    // check regex matches
    let json_str = match RE.captures(s) {
        Some(val) => val.get(1).unwrap().as_str(),
        None => return Err(Error::from("Could not find video JSON state")),
    };

    // let json_val: serde_json::Value = serde_json::from_str(json_str)?;
    // let json_str = serde_json::to_string_pretty(&json_val).unwrap();
    // println!("{}", &json_str);
    let state: video::VideoState = serde_json::from_str(&json_str)?;

    Ok(state)
}
