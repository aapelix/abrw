use regex::Regex;

pub fn is_url(text: &str) -> bool {
    let url_regex = Regex::new(r"^(https?://[^\s/$.?#].[^\s]*)$").unwrap();
    url_regex.is_match(text)
}
