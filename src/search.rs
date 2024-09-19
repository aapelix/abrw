use reqwest::blocking::get;

use serde_json::Value;

use std::io;

pub fn fetch_suggestions(query: &str) -> Result<Value, io::Error> {
    let url = format!("https://ac.duckduckgo.com/ac/?q={}", query);

    let response = get(&url)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .text()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let json: Value =
        serde_json::from_str(&response).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(json)
}
