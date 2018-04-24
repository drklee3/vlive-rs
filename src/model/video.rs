#[derive(Serialize, Deserialize, Debug)]
pub enum VideoType {
    VOD,
    LIVE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cover {
    #[serde(rename = "type")]
    pub type_: String,    // type_: "single",
    pub source: String,  // source: "http://video.phinf.naver.net/20180201_87/1517478329771PWSCY_JPEG/a2b4ecf8-0734-11e8-89b9-0000000049b9_07.jpg"
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub usable: bool,
    pub count: u32,
    pub only_inner_services: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,         // "muploader_j",
    pub name: String,       // "muploader_j",
    pub url: String,        // "null"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiListItem {
    pub name: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoMeta {
    pub master_video_id: String,  // "0DC15652502D637372BA3E18CECAAE499F65",
    pub content_id: String,      // "null",
    pub service_id: u32,         // 2024,
    pub count: u32,             // 180808,
    pub interface_lang: String,  // "en_US",
    pub url: String,            // "",
    pub home_url: String,        // "null",
    pub subject: String,        // "",
    pub cover: Cover,
    pub share: Share,
    pub user: User,
    pub api_list: Vec<ApiListItem>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodingOption {
    pub id: String,         // "144P_256_100_64"
    pub name: String,       // "144P"
    pub profile: String,    // "BASE"
    pub width: u32,         // 250
    pub height: u32,        // 144
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bitrate {
    pub video: f64,
    pub audio: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoItem {
    pub id: String,             // "E49EC1F9611925347CFD30D5494F10929821",
    #[serde(rename = "useP2P")]
    pub use_p2p: bool,           // false,
    pub duration: f64,          // 620.538,
    pub preview_duration: Option<u32>,   // 30,
    pub size: u64,              // 12880198,
    #[serde(rename = "type")]
    pub type_: String,           // "avc1",
    pub encoding_option: EncodingOption,
    pub bitrate: Bitrate,
    pub p2p_meta_url: String,     // "",
    pub p2p_url: String,         // "",
    pub source: String,         // "http://globalv.p.naverrmc.edgesuite.net/global/read/global_v_2018_02_01_4/a36b6bbd-0734-11e8-89b9-0000000049b9.mp4?__gda__=1517618848_e4d87e6279f0e75cd59d483e6523a0e9"
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Videos {
    #[serde(rename = "type")]
    pub type_: String,       // "video",
    pub has_preview: String, // "true",
    pub list: Vec<VideoItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    #[serde(rename = "type")]
    pub type_: String,       // "param",
    pub name: String,       // "__gda__",
    pub value: String       // "1517618848_c6b0e8d115c8e780999621c9b8b0dfe7"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    #[serde(rename = "type")]
    pub type_: String,   // "HLS",
    pub key: Key,
    pub source: String, // "http://globalv.p.naverrmc.edgesuite.net/global/read/global_v_2018_02_01_4/hls/f4740c94-0734-11e8-8062-0000000041ed.m3u8"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Caption {
    pub language: String,   // "en",
    pub country: String,    // "US",
    pub locale: String,     // "en_US",
    pub label: String,      // "English",
    pub source: String,     // "http://caption.rmcnmv.naver.net/globalv/global_meta/read/global_v_2018_02_01_3/09c42cb8-0741-11e8-8582-3ca82a214e91-1517483655295_en_US_cp.vtt?__gda__=1517618848_dc2c8b243b42c43632d58757677a189a"
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Captions {
    pub caption_lang: String,
    pub list: Vec<Caption>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    pub time: f64,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnails {
    pub list: Vec<Thumbnail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub meta: VideoMeta,
    pub videos: Videos,
    pub streams: Vec<Stream>,
    pub captions: Captions,
    pub thumbnails: Thumbnails,
}
