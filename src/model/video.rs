#[derive(Serialize, Deserialize, Debug)]
pub enum VideoType {
    VOD,
    LIVE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cover {
    #[serde(rename = "type")]
    type_: String,    // type_: "single",
    source: String,  // source: "http://video.phinf.naver.net/20180201_87/1517478329771PWSCY_JPEG/a2b4ecf8-0734-11e8-89b9-0000000049b9_07.jpg"
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    usable: bool,
    count: u32,
    only_inner_services: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,         // "muploader_j",
    name: String,       // "muploader_j",
    url: String,        // "null"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiListItem {
    name: String,
    source: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoMeta {
    master_video_id: String,  // "0DC15652502D637372BA3E18CECAAE499F65",
    content_id: String,      // "null",
    service_id: u32,         // 2024,
    count: u32,             // 180808,
    interface_lang: String,  // "en_US",
    url: String,            // "",
    home_url: String,        // "null",
    subject: String,        // "",
    cover: Cover,
    share: Share,
    user: User,
    api_list: Vec<ApiListItem>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodingOption {
    id: String,         // "144P_256_100_64"
    name: String,       // "144P"
    profile: String,    // "BASE"
    width: u32,         // 250
    height: u32,        // 144
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bitrate {
    video: u32,
    audio: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoItem {
    id: String,             // "E49EC1F9611925347CFD30D5494F10929821",
    #[serde(rename = "useP2P")]
    use_p2p: bool,           // false,
    duration: f32,          // 620.538,
    preview_duration: u32,   // 30,
    size: u64,              // 12880198,
    #[serde(rename = "type")]
    type_: String,           // "avc1",
    encoding_option: EncodingOption,
    bitrate: Bitrate,
    p2p_meta_url: String,     // "",
    p2p_url: String,         // "",
    source: String,         // "http://globalv.p.naverrmc.edgesuite.net/global/read/global_v_2018_02_01_4/a36b6bbd-0734-11e8-89b9-0000000049b9.mp4?__gda__=1517618848_e4d87e6279f0e75cd59d483e6523a0e9"
    
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Videos {
    #[serde(rename = "type")]
    type_: String,       // "video",
    has_preview: String, // "true",
    list: Vec<VideoItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    #[serde(rename = "type")]
    type_: String,       // "param",
    name: String,       // "__gda__",
    value: String       // "1517618848_c6b0e8d115c8e780999621c9b8b0dfe7"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    #[serde(rename = "type")]
    type_: String,   // "HLS",
    key: Key,
    source: String, // "http://globalv.p.naverrmc.edgesuite.net/global/read/global_v_2018_02_01_4/hls/f4740c94-0734-11e8-8062-0000000041ed.m3u8"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Caption {
    language: String,   // "en",
    country: String,    // "US",
    locale: String,     // "en_US",
    label: String,      // "English",
    source: String,     // "http://caption.rmcnmv.naver.net/globalv/global_meta/read/global_v_2018_02_01_3/09c42cb8-0741-11e8-8582-3ca82a214e91-1517483655295_en_US_cp.vtt?__gda__=1517618848_dc2c8b243b42c43632d58757677a189a"
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Captions {
    caption_lang: String,
    list: Vec<Caption>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    time: u64,
    source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnails {
    list: Vec<Thumbnail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    meta: VideoMeta,
    videos: Videos,
    streams: Vec<Stream>,
    captions: Captions,
    thumbnails: Thumbnails,
}