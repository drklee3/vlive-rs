#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VideoType {
    VOD,
    LIVE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cover {
    #[serde(rename = "type")]
    pub type_: String,    // type_: "single",
    pub source: String,  // source: "http://video.phinf.naver.net/20180201_87/1517478329771PWSCY_JPEG/a2b4ecf8-0734-11e8-89b9-0000000049b9_07.jpg"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub usable: bool,
    pub count: u32,
    pub only_inner_services: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,         // "muploader_j",
    pub name: String,       // "muploader_j",
    pub url: String,        // "null"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiListItem {
    pub name: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncodingOption {
    /// Resolution name and dimension code
    pub id: String,         // "144P_256_100_64"
    /// Resolution name
    pub name: String,       // "144P"
    /// Encoding profile, usually `BASE` for low resolution videos or `HIGH`
    pub profile: String,    // "BASE"
    /// Width of video resolution
    pub width: u32,         // 250
    /// Height of video resolution
    pub height: u32,        // 144
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bitrate {
    /// Video bitrate in kbps
    pub video: f64,
    /// Audio bitrate in kbps
    pub audio: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoItem {
    /// ID of video
    pub id: String,             // "E49EC1F9611925347CFD30D5494F10929821",
    #[serde(rename = "useP2P")]
    pub use_p2p: bool,           // false,
    pub duration: f64,          // 620.538,
    pub preview_duration: Option<u32>,   // 30,
    /// Size of video in bytes
    pub size: u64,              // 12880198,
    /// Encoding codec name
    #[serde(rename = "type")]
    pub type_: String,           // "avc1",
    /// Encoding options of video
    pub encoding_option: EncodingOption,
    /// Video and audio bitrate
    pub bitrate: Bitrate,
    pub p2p_meta_url: String,     // "",
    pub p2p_url: String,         // "",
    /// URL to the mp4 file.  This expires after an unknown amount of time.
    pub source: String,         // "http://globalv.p.naverrmc.edgesuite.net/global/read/global_v_2018_02_01_4/a36b6bbd-0734-11e8-89b9-0000000049b9.mp4?__gda__=1517618848_e4d87e6279f0e75cd59d483e6523a0e9"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Videos {
    #[serde(rename = "type")]
    pub type_: String,       // "video",
    pub has_preview: String, // "true",
    pub list: Vec<VideoItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key {
    #[serde(rename = "type")]
    pub type_: String,       // "param",
    pub name: String,       // "__gda__",
    /// Key value of video, required to get video data.
    pub value: String       // "1517618848_c6b0e8d115c8e780999621c9b8b0dfe7"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stream {
    /// Type of stream, usually `HLS`
    #[serde(rename = "type")]
    pub type_: String,   // "HLS",
    pub key: Key,
    /// m3u8 URL for streaming
    pub source: String, // "http://globalv.p.naverrmc.edgesuite.net/global/read/global_v_2018_02_01_4/hls/f4740c94-0734-11e8-8062-0000000041ed.m3u8"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Caption {
    /// Language of caption, such as `en`
    pub language: String,   // "en",
    /// Country of language, such as `US`
    pub country: String,    // "US",
    /// Locale code
    pub locale: String,     // "en_US",
    /// Label of caption used in the UI, such as `English`
    pub label: String,      // "English",
    /// Full URL of .vtt caption file
    pub source: String,     // "http://caption.rmcnmv.naver.net/globalv/global_meta/read/global_v_2018_02_01_3/09c42cb8-0741-11e8-8582-3ca82a214e91-1517483655295_en_US_cp.vtt?__gda__=1517618848_dc2c8b243b42c43632d58757677a189a"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Captions {
    /// Default / selected caption language(?)
    pub caption_lang: String,
    /// List of all available captions
    pub list: Vec<Caption>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thumbnail {
    /// Time during the video for this thumbnail in seconds
    pub time: f64,
    /// URL to the .jpg thumbnail file
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thumbnails {
    /// List of available thumbnails
    pub list: Vec<Thumbnail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    /// Video information
    pub meta: VideoMeta,
    /// .mp4 videos
    pub videos: Videos,
    /// .m3u8 streams
    pub streams: Vec<Stream>,
    /// .vtt captions
    pub captions: Option<Captions>,
    /// .jpg thumbnails
    pub thumbnails: Thumbnails,
}
