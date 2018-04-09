use chrono::{
    DateTime,
    offset::FixedOffset,
};
use model::helpers::*;

#[derive(Deserialize, Clone, Debug)]
pub enum ChannelType {
    BASIC,
    PREMIUM,
}

#[derive(Deserialize, Debug)]
pub struct ChannelList(pub Vec<ChannelListItem>);

impl ChannelList {
    /// Searches for a channel by name.  This is case insensitive.
    pub fn find_channel<T: AsRef<str>>(&self, search: T) -> Option<ChannelListItem> {
        self.0
            .iter()
            .find(|chan|{
                chan.name.to_lowercase() == search
                        .as_ref()
                        .to_string()
                        .to_lowercase()
            })
            .cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChannelListItem {
    pub name: String,           // "BTS",
	pub icon: String,           // "http://v.phinf.naver.net/20180406_39/1522940433294kxJHw_PNG/profile13_15775.png?type=round58_58",
	
    #[serde(rename = "type")]
    pub channel_type: ChannelType,      // "BASIC",
	pub code: Option<String>,           // "FE619"
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelInfo {
    pub channel_seq: u32,
    pub basic_channel_seq: u32,
    pub channel_code: String,
    pub channel_plus_type: ChannelType,
    pub channel_name: String,
    pub representative_color: String,
    pub channel_profile_image: String,
    pub background_color: String,
    pub channel_cover_image: String,
    pub fan_count: u32,
    pub comment: String,
    pub prohibited_word_like: String,
    pub prohibited_word_exact: String,
    pub sns_share_img: String,

    #[serde(deserialize_with = "bool_from_str")]
    pub banner_show_yn: bool,             //  "N"
    pub qrcode: String,

    #[serde(deserialize_with = "bool_from_str")]
    pub upcoming_show_yn: bool,           //  "N"
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoListItem {
    pub video_seq: u32,                      //  57788
    pub video_type: String,                  //  "VOD"
    pub title: String,                       //  "[1년 전 오늘의 TWICE] 지효야 1년 …’s birthday a year ago)"
    pub play_count: u32,                     //  46169
    pub like_count: u32,                     //  1387373
    pub comment_count: u32,                  //  6179
    pub thumbnail: String,                   //  "http://v.phinf.naver.net…6_20.%BD%BA%C6%BF002.jpg"
    pub pick_sort_order: u32,                //  0
    pub screen_orientation: String,          //  "HORIZONTAL"

    #[serde(deserialize_with = "timestamp_from_str")]
    pub will_start_at: DateTime<FixedOffset>,               //  "2018-02-01 20:39:00"

    #[serde(deserialize_with = "timestamp_from_str")]
    pub will_end_at: DateTime<FixedOffset>,                 //  "2099-12-31 23:59:59"
    
    #[serde(default)]
    #[serde(deserialize_with = "option_timestamp_from_str")]
    pub created_at: Option<DateTime<FixedOffset>>,          //  "2018-04-06 13:35:09"
    pub upcoming_yn: String,                 //  "N"

    #[serde(deserialize_with = "bool_from_str")]
    pub special_live_yn: bool,               //  "N"

    #[serde(deserialize_with = "bool_from_str")]
    pub live_thumb_yn: bool,                 //  "N"
    pub product_id: String,                  //  ""
    pub package_product_id: String,          //  ""
    pub product_type: String,                //  "NONE"
    pub play_time: u32,                      //  199

    #[serde(deserialize_with = "bool_from_str")]
    pub channel_plus_public_yn: bool,        //  "N"
    pub expose_status: String,               //  "EXPOSED"
    
    #[serde(deserialize_with = "timestamp_from_str")]
    pub on_air_start_at: DateTime<FixedOffset>,             //  "2018-02-01 20:44:00"
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelUpcomingVideoList {
    /// Count of upcoming videos.
    pub total_video_count: u32,
    /// Upcoming video list, this will be None when there are no upcoming videos.
    pub video_list: Option<Vec<VideoListItem>>,
}

#[derive(Deserialize, Debug)]
pub struct ChannelUpcomingVideoListResult {
    pub result: ChannelUpcomingVideoList,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideoList {
    pub channel_info: ChannelInfo,
    pub total_video_count: u32,           //  724
    pub video_list: Vec<VideoListItem>,
}

#[derive(Deserialize, Debug)]
pub struct ChannelVideoListResult {
    pub result: ChannelVideoList,
}
