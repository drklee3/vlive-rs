use scraper::{Html, Selector};
use serde::{de, Deserialize, Deserializer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::result::Result as StdResult;

use super::channel::ChannelType;
use super::video::VideoType;
use crate::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct RecentVideo {
    pub title: String,

    #[serde(deserialize_with = "u64_from_str")]
    pub video_seq: u64,

    #[serde(rename = "type")]
    pub kind: VideoType,
    pub channel_name: String,

    #[serde(deserialize_with = "u64_from_str")]
    pub channel_seq: u64,
    pub channel_url: String,
    pub channel_type: ChannelType,
    pub thumbnail_url: Option<String>,

    /// None if this is live
    #[serde(default, deserialize_with = "u64_from_duration_str")]
    pub duration_secs: Option<u64>,

    /// May be missing for some videos
    #[serde(default, deserialize_with = "some_u64_from_str")]
    pub plays: Option<u64>,

    /// May be missing for some videos
    #[serde(default, deserialize_with = "some_u64_from_str")]
    pub likes: Option<u64>,
}

impl RecentVideo {
    pub fn from_html(html: &str) -> Result<Vec<Self>> {
        let fragment = Html::parse_fragment(html);
        let selector = Selector::parse("li").unwrap();

        let mut videos = Vec::new();

        for video_element in fragment.select(&selector) {
            let mut video_attrs = HashMap::new();

            // Basic metadata from thumbnail
            if let Some(e) = video_element
                .select(&Selector::parse("a.thumb_area").unwrap())
                .nth(0)
            {
                for (attr_name, attr_value) in e.value().attrs() {
                    match attr_name {
                        "data-seq" => video_attrs.insert("video_seq", Cow::from(attr_value)),
                        "data-ga-type" => video_attrs.insert("type", Cow::from(attr_value)),
                        "data-ga-name" => video_attrs.insert("title", Cow::from(attr_value)),
                        "data-ga-cseq" => video_attrs.insert("channel_seq", Cow::from(attr_value)),
                        "data-ga-cname" => {
                            video_attrs.insert("channel_name", Cow::from(attr_value))
                        }
                        "data-ga-ctype" => {
                            video_attrs.insert("channel_type", Cow::from(attr_value))
                        }
                        _ => continue,
                    };
                }
            }

            // Video duration
            if let Some(d) = video_element
                .select(&Selector::parse("span.time").unwrap())
                .nth(0)
                .and_then(|e| e.last_child())
                .map(|c| c.value())
                .and_then(|v| v.as_text())
                .map(|t| t.text.to_string())
            {
                video_attrs.insert("duration_secs", Cow::from(d));
            }

            // Video thumbnail url
            if let Some(url) = video_element
                .select(&Selector::parse("img").unwrap())
                .nth(0)
                .map(|c| c.value())
                .and_then(|e| e.attr("src"))
            {
                video_attrs.insert("thumbnail_url", Cow::from(url));
            }

            // When posted
            if let Some(age) = video_element
                .select(&Selector::parse("div.video_date > span.date").unwrap())
                .nth(0)
                .map(|c| c.inner_html())
            {
                video_attrs.insert("posted_age", Cow::from(age));
            }

            // Channel URL
            if let Some(url) = video_element
                .select(&Selector::parse("div.video_date > a.name").unwrap())
                .nth(0)
                .map(|c| c.value())
                .and_then(|e| e.attr("href"))
            {
                video_attrs.insert("channel_url", Cow::from(url));
            }

            // Plays
            if let Some(plays) = video_element
                .select(&Selector::parse("div.video_info > span.play > span").unwrap())
                .nth(0)
                .map(|c| c.inner_html())
            {
                video_attrs.insert("plays", Cow::from(plays));
            }

            // Likes
            if let Some(likes) = video_element
                .select(&Selector::parse("div.video_info > span.like > span").unwrap())
                .nth(0)
                .map(|c| c.inner_html())
            {
                video_attrs.insert("likes", Cow::from(likes));
            }

            let val = serde_json::to_value(&video_attrs)?;
            let video: RecentVideo = serde_json::from_value(val)?;

            videos.push(video);
        }

        Ok(videos)
    }
}

pub fn u64_from_str<'de, D>(deserializer: D) -> StdResult<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.replace(",", "").parse::<u64>().map_err(de::Error::custom)
}

pub fn some_u64_from_str<'de, D>(deserializer: D) -> StdResult<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    u64_from_str(deserializer).map(|num| Some(num))
}

pub fn u64_from_duration_str<'de, D>(deserializer: D) -> StdResult<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;

    if let Some(s) = s {
        let mut seconds = 0;

        for (i, chunk) in s.split(':').rev().enumerate() {
            let seconds_multi = match i {
                0 => 1,
                1 => 60,
                // Hours
                2 => 60 * 60,
                // Days, shouldn't be another one after this
                _ => 60 * 60 * 24,
            };

            seconds += chunk.parse::<u64>().map_err(de::Error::custom)? * seconds_multi;
        }

        Ok(Some(seconds))
    } else {
        Ok(None)
    }
}
