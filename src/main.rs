extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

use adblock::{
    blocker::BlockerResult,
    lists::{FilterSet, ParseOptions},
    request::Request,
    Engine,
};
use gtk::{glib::Propagation, prelude::*};
use reqwest::blocking;
use std::env;
use std::error::Error;
use url::Url;
use webkit2gtk::{SettingsExt, URIRequestExt, WebViewExt};

fn fetch_block_list(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Fetching block lists");
    let response = blocking::get(url)?;
    let block_list = response
        .text()?
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    Ok(block_list)
}

fn on_resource_load_started(
    _webview: &webkit2gtk::WebView,
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
                        eprintln!("Failed to parse domain from URL: {}", url_string);
                        return; // Exit early if there's no valid domain
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

                                        // BLOCK ADS HERE
                                        // i dont know how
                                    }
                                }
                            }
                            BlockerResult { matched: false, .. } => {
                                println!("Request did not match any block.");
                            }
                        }
                    }
                    Err(err) => eprintln!("Error creating request: {}", err),
                }
            }
            Err(err) => eprintln!("Error parsing URL {}: {}", url_string, err),
        }
    }
}

fn add_webview_tab(
    notebook: &gtk::Notebook,
    url: &str,
    title: &str,
    search_entry: &gtk::Entry,
    filter_set: &FilterSet,
) {
    let webview = webkit2gtk::WebView::new();

    let engine = Engine::from_filter_set(filter_set.clone(), true);

    webview.connect_resource_load_started(move |webview, resource, request| {
        on_resource_load_started(webview, resource, request, &engine);
    });

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    webview.load_uri(url);

    let search_entry_clone = search_entry.clone();
    webview.connect_notify_local(Some("uri"), move |webview, _| {
        if let Some(uri) = webview.uri() {
            search_entry_clone.set_text(&uri);
        }
    });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    // Create the tab label
    let label = gtk::Label::new(Some(title));
    label.set_hexpand(true);
    label.set_vexpand(false);

    hbox.pack_start(&label, true, true, 10);

    let close_button = gtk::Button::new();

    close_button.set_size_request(25, 25);

    let close_label = gtk::Label::new(Some("X"));
    close_button.set_relief(gtk::ReliefStyle::None);
    close_button.add(&close_label);

    hbox.pack_start(&close_button, false, false, 0);

    let new_tab_index = notebook.append_page(&webview, Some(&hbox));

    webview.show();
    label.show();
    close_label.show();
    close_button.show();
    hbox.show();

    notebook.set_current_page(Some(new_tab_index));

    let notebook_clone = notebook.clone();
    close_button.connect_clicked(move |_| {
        notebook_clone.remove_page(Some(new_tab_index));
    });
}

fn main() {
    env::set_var("WEBKIT_FORCE_ACCELERATED_COMPOSITING", "1");
    env::set_var("WEBKIT_FORCE_HARDWARE_ACCELERATION", "1");

    gtk::init().unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Abrw");
    window.set_default_size(800, 600);
    window.set_decorated(false);

    window.connect_button_press_event(|window, event| {
        if event.button() == 1 {
            window.begin_move_drag(
                event.button().try_into().unwrap(),
                event.root().0 as i32,
                event.root().1 as i32,
                event.time(),
            );
        }
        Propagation::Stop
    });

    let mut rules = vec![
        String::from("-advertisement-icon."),
        String::from("-advertisement-management/"),
        String::from("-advertisement."),
        String::from("-advertisement/script."),
    ];

    let debug_info = true;
    let mut filter_set = FilterSet::new(debug_info);

    let github_url = "https://ublockorigin.github.io/uAssets/thirdparties/easylist-cookies.txt";
    if let Ok(fetched_rules) = fetch_block_list(github_url) {
        rules.extend(fetched_rules);
        filter_set.add_filters(&rules, ParseOptions::default());
    }

    let github_url = "https://ublockorigin.github.io/uAssets/filters/annoyances-cookies.txt";
    if let Ok(fetched_rules) = fetch_block_list(github_url) {
        rules.extend(fetched_rules);
        filter_set.add_filters(&rules, ParseOptions::default());
    }

    let github_url = "https://ublockorigin.github.io/uAssets/thirdparties/easylist-newsletters.txt";
    if let Ok(fetched_rules) = fetch_block_list(github_url) {
        rules.extend(fetched_rules);
        filter_set.add_filters(&rules, ParseOptions::default());
    }

    let github_url = "https://ublockorigin.github.io/uAssets/filters/annoyances-others.txt";
    if let Ok(fetched_rules) = fetch_block_list(github_url) {
        rules.extend(fetched_rules);
        filter_set.add_filters(&rules, ParseOptions::default());
    }

    let github_url = "https://ublockorigin.github.io/uAssets/thirdparties/easylist-social.txt";
    if let Ok(fetched_rules) = fetch_block_list(github_url) {
        rules.extend(fetched_rules);
        filter_set.add_filters(&rules, ParseOptions::default());
    }

    let github_url = "https://ublockorigin.github.io/uAssets/thirdparties/easylist-chat.txt";
    if let Ok(fetched_rules) = fetch_block_list(github_url) {
        rules.extend(fetched_rules);
        filter_set.add_filters(&rules, ParseOptions::default());
    }

    let github_url = "https://ublockorigin.github.io/uAssets/thirdparties/easylist-annoyances.txt";
    if let Ok(fetched_rules) = fetch_block_list(github_url) {
        rules.extend(fetched_rules);
        filter_set.add_filters(&rules, ParseOptions::default());
    }

    let github_url = "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts";
    if let Ok(fetched_rules) = fetch_block_list(github_url) {
        rules.extend(fetched_rules);
        filter_set.add_filters(&rules, ParseOptions::default());
    }

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let notebook = gtk::Notebook::new();

    let css_providerr = gtk::CssProvider::new();
    css_providerr
        .load_from_data(
            b"
        notebook header.top tabs {
            background: #212121;
        }

        notebook header.top tabs tab {
            background: transparent;
            border: none;
            border-radius: 7px;
            margin: 4px;
            padding: 10px;
            transition-duration: 300ms;
        }

        notebook header.top tabs tab:checked {
            background: #303030;
        }

        notebook header.top tabs tab:hover {
            background: #3a3a3a;
        }
        ",
        )
        .expect("Error loading css");

    let notebook_style = notebook.style_context();
    notebook_style.add_provider(&css_providerr, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let back_button = gtk::Button::with_label("<");
    let forward_button = gtk::Button::with_label(">");
    let refresh_button = gtk::Button::with_label("↻");
    let new_tab_button = gtk::Button::with_label("+");

    back_button.set_size_request(30, 30);
    forward_button.set_size_request(30, 30);
    refresh_button.set_size_request(30, 30);
    new_tab_button.set_size_request(30, 30);

    hbox.pack_start(&back_button, false, false, 5);
    hbox.pack_start(&forward_button, false, false, 5);
    hbox.pack_start(&refresh_button, false, false, 5);
    hbox.pack_start(&new_tab_button, false, false, 5);

    let css_provider = gtk::CssProvider::new();
    css_provider
        .load_from_data(
            b"
            button {
                background: transparent;
                border: none;
                border-radius: 7px;
                box-shadow: none;
                -gtk-icon-shadow: none;
                text-shadow: none;
                transition-duration: 300ms;
            }
            button:hover {
                background: #3a3a3a;
            }
        ",
        )
        .expect("Failed to load CSS");

    let back_button_style = back_button.style_context();
    let forward_button_style = forward_button.style_context();
    let refresh_button_style = refresh_button.style_context();
    let new_tab_button_style = refresh_button.style_context();

    back_button_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    forward_button_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    refresh_button_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    new_tab_button_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let search_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.pack_start(&search_box, true, true, 0);
    let search_entry = gtk::Entry::new();
    search_entry.set_width_request(700);

    let css_provider = gtk::CssProvider::new();
    css_provider
        .load_from_data(
            b"
        .search-entry {
        border-radius: 7px;
        background: #424242;
        padding-right: 5px;
        padding-left: 5px;
        }

        .box {
            background: #212121;
        }
    ",
        )
        .expect("Failed to load css");

    let style_context = hbox.style_context();
    let style_context_2 = search_entry.style_context();
    let style_context_3 = vbox.style_context();
    style_context.add_class("box");
    style_context_3.add_class("box");
    style_context_2.add_class("search-entry");
    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    style_context_2.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    style_context_3.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let options = gtk::Button::with_label("⋮");
    let minimize = gtk::Button::with_label("_");
    let maximize = gtk::Button::with_label("[ ]");
    let close = gtk::Button::with_label("X");

    hbox.pack_end(&close, false, false, 5);
    hbox.pack_end(&maximize, false, false, 5);
    hbox.pack_end(&minimize, false, false, 5);
    hbox.pack_end(&options, false, false, 5);

    options.set_size_request(25, 25);
    minimize.set_size_request(30, 30);
    maximize.set_size_request(30, 30);
    close.set_size_request(30, 30);

    let css_provider = gtk::CssProvider::new();
    css_provider
        .load_from_data(
            b"
            button {
                background: #2a2a2a;
                border-radius: 7px;
                border: none;
                box-shadow: none;
                -gtk-icon-shadow: none;
                text-shadow: none;
            }
            .options_button {
                background: transparent
            }
            button:hover {
                background: rgba(0, 0, 0, 0.5);
            }
            .close_button:hover {
                background: #fc3737;
            }

        ",
        )
        .expect("Failed to load CSS");

    let options_style = options.style_context();
    let minimize_style = minimize.style_context();
    let maximize_style = maximize.style_context();
    let close_style = close.style_context();

    options_style.add_class("options_button");
    close_style.add_class("close_button");

    options_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    minimize_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    maximize_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    close_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    search_box.pack_start(&search_entry, false, false, 0);
    search_box.set_halign(gtk::Align::Center);

    vbox.pack_start(&hbox, false, false, 15);
    vbox.pack_start(&notebook, true, true, 5);

    add_webview_tab(
        &notebook,
        "https://start.duckduckgo.com/",
        "New tab",
        &search_entry,
        &filter_set,
    );

    search_entry.connect_activate({
        let notebook = notebook.clone();
        let entry = search_entry.clone();

        move |_| {
            let current_page = notebook.current_page();
            if let Some(widget) = notebook.nth_page(current_page) {
                if let Some(webview) = widget.downcast_ref::<webkit2gtk::WebView>() {
                    let url = entry.text();

                    if url.is_empty() {
                        return;
                    }

                    if !url.starts_with("http://") && !url.starts_with("https://") {
                        if url.contains('.') && !url.contains(' ') {
                            webview.load_uri(&format!("https://{}", url));
                        } else {
                            let search_query = url.replace(" ", "+");
                            webview
                                .load_uri(&format!("https://duckduckgo.com/?q={}", search_query));
                        }
                    } else {
                        webview.load_uri(&url);
                    }
                }
            }
        }
    });

    window.add(&vbox);
    window.show_all();

    let window_clone = window.clone();
    minimize.connect_clicked(move |_| {
        window_clone.iconify();
    });

    let window_clone = window.clone();
    maximize.connect_clicked(move |_| {
        if window_clone.is_maximized() {
            window_clone.unmaximize();
        } else {
            window_clone.maximize();
        }
    });

    let window_clone = window.clone();
    close.connect_clicked(move |_| {
        window_clone.close(); // Close the window
    });

    back_button.connect_clicked({
        let notebook = notebook.clone();

        move |_| {
            let current_page = notebook.current_page();
            if let Some(widget) = notebook.nth_page(current_page) {
                if let Some(webview) = widget.downcast_ref::<webkit2gtk::WebView>() {
                    if webview.can_go_back() {
                        webview.go_back();
                    }
                }
            }
        }
    });

    forward_button.connect_clicked({
        let notebook = notebook.clone();

        move |_| {
            let current_page = notebook.current_page();
            if let Some(widget) = notebook.nth_page(current_page) {
                if let Some(webview) = widget.downcast_ref::<webkit2gtk::WebView>() {
                    if webview.can_go_forward() {
                        webview.go_forward();
                    }
                }
            }
        }
    });

    refresh_button.connect_clicked({
        let notebook = notebook.clone();

        move |_| {
            let current_page = notebook.current_page();
            if let Some(widget) = notebook.nth_page(current_page) {
                if let Some(webview) = widget.downcast_ref::<webkit2gtk::WebView>() {
                    webview.reload()
                }
            }
        }
    });

    new_tab_button.connect_clicked({
        let notebook = notebook.clone();
        let search_entry = search_entry.clone();
        let filter_set = filter_set.clone();

        move |_| {
            add_webview_tab(
                &notebook,
                "https://start.duckduckgo.com/",
                "New tab",
                &search_entry,
                &filter_set,
            )
        }
    });

    notebook.connect_switch_page({
        let search_entry = search_entry.clone();

        move |notebook, _, page_num| {
            if let Some(widget) = notebook.nth_page(Some(page_num)) {
                if let Some(webview) = widget.downcast_ref::<webkit2gtk::WebView>() {
                    if let Some(uri) = webview.uri() {
                        search_entry.set_text(&uri);
                    }
                }
            }
        }
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Propagation::Stop
    });

    gtk::main();
}
