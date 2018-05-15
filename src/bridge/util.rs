use regex::Regex;
use serde_json;
use model::video::VideoStatus;

pub fn find_video(s: &str) -> Option<VideoStatus> {
    // basically just scrape the page for video id and key since there's no api endpoint to get this
    lazy_static! {
        // wtf
        // r#"var oVideoStatus = {(?:[) ,\n\t]*(?:"(?:[a-zA-Z0-9_]*)" ?: ?"?(?:[a-zA-Z0-9_]*)(?:"?,)?))(?:[) ,\n\t]*(?:"(?:[a-zA-Z0-9_]*)" ?: ?"?(?:[a-zA-Z0-9_]*)(?:"?,)?))(?:[) ,\n\t]*(?:"(?:[a-zA-Z0-9_]*)" ?: ?"?(?:[a-zA-Z0-9_]*)(?:"?,)?))(?:[) ,\n\t]*(?:"(?:[a-zA-Z0-9_]*)" ?: ?"?(?:[a-zA-Z0-9_]*)(?:"?,)?))"#
        // r#"vlive\.video\.init\((?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))"#
        // var oVideoStatus = (\{[\n\t"\w :,]*\})
        static ref RE: Regex =
            Regex::new(r#"<script .*>\nvar oVideoStatus = (\{[\{}\[\]/\.?=+\n\t"\w :,]*})\n</script>"#).unwrap();
    }

    // check regex matches
    let caps = match RE.captures(s) {
        Some(val) => val,
        None => return None,
    };

    let json = caps.get(1).map(|m| m.as_str()).unwrap();

    serde_json::from_str::<VideoStatus>(json).ok()
}