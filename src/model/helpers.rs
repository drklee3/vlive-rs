use chrono::{offset::FixedOffset, DateTime};
use serde::{de, Deserialize, Deserializer};

pub fn bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s == "Y")
}

pub fn timestamp_from_str<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // All dates on VLive are KST timezone
    let with_timezone = format!("{} +0900", s);
    DateTime::parse_from_str(&with_timezone, "%Y-%m-%d %H:%M:%S %z").map_err(de::Error::custom)
}

pub fn option_timestamp_from_str<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    if let Some(s) = s {
        let with_timezone = format!("{} +0900", s);
        let dt = DateTime::parse_from_str(&with_timezone, "%Y-%m-%d %H:%M:%S %z")
            .map_err(de::Error::custom)?;
        Ok(Some(dt))
    } else {
        Ok(None)
    }
}
