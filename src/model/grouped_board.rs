use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BoardType {
    Notice,
    Star,
    Common,
    VlivePlus,
}

pub type GroupedBoards = Vec<GroupedBoard>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupedBoard {
    pub group_title: String,
    pub boards: Vec<Board>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub board_id: i64,
    pub title: String,
    pub board_type: BoardType,
    pub use_star_filter: bool,
    pub pay_required: bool,
    pub expose: bool,
    pub open_type: String,
    pub last_updated_at: i64,
    pub channel_code: String,
}
