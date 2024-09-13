use adblock::{blocker::BlockerResult, request::Request, Engine};
use reqwest::blocking;
use std::error::Error;
use url::Url;
use webkit2gtk::{URIRequestExt, WebViewExt};

pub fn fetch_block_list(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Fetching block lists");
    let response = blocking::get(url)?;
    let block_list = response
        .text()?
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    Ok(block_list)
}

pub fn on_resource_load_started(
    webview: &webkit2gtk::WebView,
    _resource: &webkit2gtk::WebResource,
    request: &webkit2gtk::URIRequest,
    engine: &Engine,
) {
    if let Some(url_string) = request.uri() {
        match Url::parse(&url_string) {
            Ok(url) => {
                let domain = match url.host_str() {
                    Some(domain) => domain,
                    None => {
                        return;
                    }
                };

                let request2 = Request::new(&url.to_string(), domain, "");
                match request2 {
                    Ok(req) => {
                        let result = engine.check_network_request(&req);
                        match result {
                            BlockerResult {
                                matched: true,
                                important,
                                redirect,
                                rewritten_url,
                                exception,
                                filter,
                            } => {
                                if important {
                                    println!(
                                        "Request matched an important rule and should be blocked."
                                    );
                                } else {
                                    println!("Request matched a non-important rule.");
                                    if let Some(redirect_url) = redirect {
                                        println!("Redirecting to: {}", redirect_url);
                                    } else if let Some(rewritten_url) = rewritten_url {
                                        println!("Rewritten URL: {}", rewritten_url);
                                    } else if let Some(exception) = exception {
                                        println!("Request is an exception: {}", exception);
                                    } else if let Some(filter) = filter {
                                        println!("Request matched filter: {}", filter);

                                        webview.stop_loading()
                                    }
                                }
                            }
                            BlockerResult { matched: false, .. } => {}
                        }
                    }
                    Err(err) => eprintln!("Error creating request: {}", err),
                }
            }
            Err(err) => eprintln!("Error parsing URL {}: {}", url_string, err),
        }
    }
}
