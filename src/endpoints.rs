pub const APP_ID: &str = "8c6cc7b45d2568fb668be6e05b6e5a3b";

// Not optimal way of doing this, but endpoints are weird with parts in middle, not just query params

pub fn grouped_boards_url(channel_code: &str) -> String {
    format!(
        "https://www.vlive.tv/globalv-web/vam-web/board/v1.0/channel-{}/groupedBoards?appId={}&fields=boardId,title,boardType,openType,allowedViewers,includedCountries,excludedCountries,useStarFilter,payRequired,expose,channelCode,lastUpdatedAt",
        channel_code, APP_ID
    )
}

pub fn board_url(board_id: u64) -> String {
    format!(
        "https://www.vlive.tv/globalv-web/vam-web/board/v1.0/board-{}?appId={}&fields=boardId,title,boardType,openType,allowedViewers,includedCountries,excludedCountries,useStarFilter,payRequired,expose,channelCode,lastUpdatedAt&gcc=US&locale=en_US",
        board_id, APP_ID
    )
}

pub fn board_posts_url(board_id: u64) -> String {
    format!(
        "https://www.vlive.tv/globalv-web/vam-web/post/v1.0/board-{}/posts?appId={}&fields=attachments,author,availableActions,board%7BboardId,title,boardType,payRequired,includedCountries,excludedCountries%7D,channel%7BchannelName,channelCode%7D,commentCount,contentType,createdAt,emotionCount,excludedCountries,includedCountries,isCommentEnabled,isHiddenFromStar,lastModifierMember,notice,officialVideo,plainBody,postId,postVersion,reservation,starReactions,targetMember,thumbnail,title,url,viewerEmotionId,writtenIn,sharedPosts,originPost&sortType=LATEST&gcc=US&locale=en_US",
        board_id, APP_ID
    )
}

pub fn channel_url(channel_code: &str) -> String {
    format!("https://www.vlive.tv/channel/{}", channel_code)
}

pub fn video_url(video_seq: u64) -> String {
    format!("https://www.vlive.tv/video/{}", video_seq)
}

pub fn inkey_url(video_seq: u64) -> String {
    format!(
        "https://www.vlive.tv/globalv-web/vam-web/video/v1.0/vod/{}/inkey?appId={}",
        video_seq, APP_ID
    )
}

pub fn vod_url(video_id: &str, key: &str) -> String {
    format!(
        "https://apis.naver.com/rmcnmv/rmcnmv/vod/play/v2.0/{}?key={}",
        video_id, key
    )
}
