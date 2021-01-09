use super::grouped_board::BoardType;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardPosts {
    pub paging: Paging,
    pub data: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub attachments: Attachments,
    pub url: String,
    pub title: String,
    pub created_at: i64,
    pub available_actions: Vec<String>,
    pub comment_count: i64,
    pub written_in: String,
    pub emotion_count: i64,
    pub post_id: String,
    pub is_comment_enabled: bool,
    pub is_hidden_from_star: bool,
    pub post_version: String,
    pub thumbnail: Thumbnail,
    pub plain_body: String,
    pub content_type: String,
    pub shared_posts: Vec<Option<serde_json::Value>>,
    pub author: Author,
    pub channel: Channel,
    pub board: Board,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachments {
    pub video_count: i64,
    pub photo: HashMap<String, Photo>,
    pub photo_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub url: String,
    pub width: i64,
    pub created_at: i64,
    pub height: i64,
    pub photo_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub member_id: String,
    pub channel_code: String,
    pub joined: bool,
    pub nickname: String,
    pub profile_image_url: String,
    pub official_profile_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub board_id: i64,
    pub title: String,
    pub board_type: BoardType,
    pub pay_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub channel_code: String,
    pub channel_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    #[serde(rename = "type")]
    pub thumbnail_type: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    pub next_params: NextParams,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextParams {
    pub limit: String,
    pub after: String,
}
