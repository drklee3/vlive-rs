use regex::Regex;

pub fn find_video_id_key(s: &str) -> Option<(String, String)> {
    // basically just scrape the page for video id and key since there's no api endpoint to get this
    lazy_static! {
        // wtf
        static ref RE: Regex =
            Regex::new(r#"vlive\.video\.init\((?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))(?:[) ,\n\t]*(?:"([a-zA-Z0-9_]*)"))"#)
                .unwrap();
    }

    // check regex matches
    let caps = match RE.captures(s) {
        Some(val) => val,
        None => return None,
    };

    let video_id = caps.get(6).map(|m| m.as_str()).unwrap();
    let key = caps.get(7).map(|m| m.as_str()).unwrap();

    Some((video_id.into(), key.into()))
}