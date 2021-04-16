use crate::model::helpers::*;
use chrono::{offset::FixedOffset, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelWrapper {
    pub channel: Channel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub channel_code: String,
    pub channel_name: String,
    /// Hex color code prefixed with #
    pub representative_color: Option<String>,
    /// Hex color code prefixed with #
    pub background_color: Option<String>,
    pub channel_profile_image: Option<String>,
    pub channel_cover_image: Option<String>,
    pub channel_description: Option<String>,
    pub sns_share_img: Option<String>,
    pub qr_code: Option<String>,
    pub open_at: Option<i64>,
    pub show_upcoming: Option<bool>,
    pub use_member_level: Option<bool>,
    pub member_count: Option<i64>,
    pub post_count_of_star: Option<i64>,
    pub video_count_of_star: Option<i64>,
    pub video_play_count_of_star: Option<i64>,
    pub video_like_count_of_star: Option<i64>,
    pub video_comment_count_of_star: Option<i64>,
}

impl Channel {
    pub fn url(&self) -> String {
        format!("https://www.vlive.tv/channel/{}", self.channel_code)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartialChannel {
    pub channel_code: String,
    pub channel_name: String,
}

/// Type of channel, basic or CHANNEL+
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ChannelType {
    /// Regular channels
    BASIC,
    /// CHANNEL+ channels
    PREMIUM,
}

/// List of all channels on VLive
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelList(pub Vec<ChannelListItem>);

/// Channel Item return in list of all available channels
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelListItem {
    /// Name of channel
    pub name: String, // "BTS",
    /// Icon URl for channel
    pub icon: String, // "http://v.phinf.naver.net/20180406_39/1522940433294kxJHw_PNG/profile13_15775.png?type=round58_58",

    /// Type of channel
    #[serde(rename = "type")]
    pub channel_type: ChannelType, // "BASIC",
    /// Channel code used in URLs
    pub code: Option<String>, // "FE619"
}

// https://www.vlive.tv/globalv-web/vam-web/vhs/store/v1.0/channels/{}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelBasicInfo {
    pub profile_img: String,
    pub name: String,
    pub latest_updated_at: i64,
    pub channel_code: String,
}

/// Information on a channel
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelInfo {
    /// Channel ID, used in API queries.
    pub channel_seq: u32,
    /// Channel ID of basic channel if this is a CHANNEL+ channel.
    pub basic_channel_seq: u32,
    /// Channel code, used in URLs.
    pub channel_code: String,
    /// Type of channel.
    pub channel_plus_type: ChannelType,
    /// Name of channel.
    pub channel_name: String,
    /// Color of channel.
    pub representative_color: String,
    /// Channel profile image
    pub channel_profile_image: String,
    /// Channel background color
    pub background_color: String,
    /// Channel color image
    pub channel_cover_image: String,
    /// Number of followers
    pub fan_count: u32,
    /// Description of page, usually just "Welcome to {name} Channel!"
    pub comment: String,
    /// Usually empty
    pub prohibited_word_like: String,
    /// Usually empty
    pub prohibited_word_exact: String,
    /// Image used for sharing
    pub sns_share_img: String,

    #[serde(deserialize_with = "bool_from_str")]
    pub banner_show_yn: bool, //  "N"
    /// QR code that gives a link to this channel
    pub qrcode: String,

    /// If there are upcoming videos
    #[serde(deserialize_with = "bool_from_str")]
    pub upcoming_show_yn: bool, //  "N"
}

impl ChannelInfo {
    /// Gets the URL to this video.
    pub fn url(&self) -> String {
        format!("http://channels.vlive.tv/{}", self.channel_code)
    }
}
/// Information on a video
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoListItem {
    /// The ID of the video, used in the URL.
    pub video_seq: u32, //  57788
    /// Type of video, either VOD or LIVE
    pub video_type: String, //  "VOD"
    /// Title of the video.
    pub title: String, //  "[1년 전 오늘의 TWICE] 지효야 1년 …’s birthday a year ago)"
    /// Number of plays this video has.
    pub play_count: u32, //  46169
    /// Number of likes or hearts this video has.
    pub like_count: u32, //  1387373
    /// Number of comments this video has.
    pub comment_count: u32, //  6179
    /// The thumbnail URL for this video.
    pub thumbnail: String, //  "http://v.phinf.naver.net…6_20.%BD%BA%C6%BF002.jpg"
    pub pick_sort_order: u32, //  0
    /// Screen orientation of this video, either `HORIZONTAL` or `VERTICAL`
    pub screen_orientation: String, //  "HORIZONTAL"

    /// Upload date of this video(?)  This video may not have been visible at this point.
    #[serde(deserialize_with = "timestamp_from_str")]
    pub will_start_at: DateTime<FixedOffset>, //  "2018-02-01 20:39:00"

    /// End time of this video, usually sometime in 2099 so this isn't really useful.
    #[serde(deserialize_with = "timestamp_from_str")]
    pub will_end_at: DateTime<FixedOffset>, //  "2099-12-31 23:59:59"

    #[serde(default)]
    #[serde(deserialize_with = "option_timestamp_from_str")]
    pub created_at: Option<DateTime<FixedOffset>>, //  "2018-04-06 13:35:09"
    pub upcoming_yn: String, //  "N"

    /// If this is a "special" live video.
    #[serde(deserialize_with = "bool_from_str")]
    pub special_live_yn: bool, //  "N"

    /// If this video has a live thumbnail.
    #[serde(deserialize_with = "bool_from_str")]
    pub live_thumb_yn: bool, //  "N"
    /// VLIVE+ product ID if this is a VLIVE+ video, otherwise it's empty.
    pub product_id: String, //  ""
    /// VLIVE+ package ID if this is a VLIVE+ video, otherwise it's empty.
    pub package_product_id: String, //  ""
    /// If this is a VLIVE+ product video.
    pub product_type: String, //  "NONE" / "PAID"
    /// Duration of the video in seconds.
    pub play_time: u32, //  199

    /// If this is a basic or CHANNEL+ video
    #[serde(deserialize_with = "bool_from_str")]
    pub channel_plus_public_yn: bool, //  "N"
    pub expose_status: String, //  "EXPOSED"

    /// Date when this video was available.
    #[serde(deserialize_with = "timestamp_from_str")]
    pub on_air_start_at: DateTime<FixedOffset>, //  "2018-02-01 20:44:00"
}

impl VideoListItem {
    /// Gets the URL to this video.
    pub fn url(&self) -> String {
        format!("http://www.vlive.tv/video/{}", self.video_seq)
    }

    /// Checks if this video is currently live.
    pub fn is_live(&self) -> bool {
        self.video_type == "LIVE"
    }
}

/// Upcoming videos for a channel
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelUpcomingVideoList {
    /// Total count of upcoming videos.
    pub total_video_count: u32,
    /// Upcoming video list, this will be None when there are no upcoming videos.
    pub video_list: Option<Vec<VideoListItem>>,
}

/// Wrapper for upcoming video list.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ChannelUpcomingVideoListResult {
    pub result: ChannelUpcomingVideoList,
}

/// A list of a channel's videos
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideoList {
    /// Information about this channel
    pub channel_info: ChannelInfo,
    /// Total videos this channel has.
    pub total_video_count: u32, //  724
    /// List of videos
    pub video_list: Vec<VideoListItem>,
}

/// Wrapper for channel video list.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ChannelVideoListResult {
    pub result: ChannelVideoList,
}

/// Decoded channel code to channel seq
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodeChannelCode {
    pub channel_seq: u64,
    pub channel_code: String,
}

/// Wrapper for decoded channel code
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct DecodeChannelCodeResult {
    pub result: DecodeChannelCode,
}
