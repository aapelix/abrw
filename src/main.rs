extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

mod adblock_abrw;
mod connections;
mod settings;
mod tabs;

use adblock::lists::{FilterSet, ParseOptions};
use gtk::{glib::Propagation, prelude::*};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use webkit2gtk::WebViewExt;

fn main() {
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

    let initial_rules = vec![
        String::from("-advertisement-icon."),
        String::from("-advertisement-management/"),
        String::from("-advertisement."),
        String::from("-advertisement/script."),

        String::from("youtube.com##+js(set,yt.config_.openPopupConfig.supportedPopups.adBlockMessageViewModel, false)"),
        String::from("youtube.com##+js(set, Object.prototype.adBlocksFound, 0)"),
        String::from("youtube.com##+js(set, ytplayer.config.args.raw_player_response.adPlacements, [])"),
        String::from("youtube.com##+js(set, Object.prototype.hasAllowedInstreamAd, true)"),
    ];

    let urls = vec![
        "https://ublockorigin.github.io/uAssets/thirdparties/easylist-cookies.txt",
        "https://ublockorigin.github.io/uAssets/filters/annoyances-cookies.txt",
        "https://ublockorigin.github.io/uAssets/thirdparties/easylist-newsletters.txt",
        "https://ublockorigin.github.io/uAssets/filters/annoyances-others.txt",
        "https://ublockorigin.github.io/uAssets/thirdparties/easylist-social.txt",
        "https://ublockorigin.github.io/uAssets/thirdparties/easylist-chat.txt",
        "https://ublockorigin.github.io/uAssets/thirdparties/easylist-annoyances.txt",
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts",
    ];

    let rules = Arc::new(Mutex::new(initial_rules));

    // Enable parallel-safe mutable access to `filter_set` using Arc and Mutex
    let filter_set = Arc::new(Mutex::new(FilterSet::new(true)));

    // Process each URL in parallel
    urls.par_iter().for_each(|url| {
        if let Ok(fetched_rules) = adblock_abrw::fetch_block_list(url) {
            // Lock the Mutex to safely access and modify `rules`
            let mut rules_guard = rules.lock().unwrap();
            rules_guard.extend(fetched_rules);

            // Lock the Mutex to safely access and modify `filter_set`
            let mut filter_set_guard = filter_set.lock().unwrap();
            filter_set_guard.add_filters(&*rules_guard, ParseOptions::default());
        }
    });

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

    let js_toggle = gtk::Switch::new();
    js_toggle.set_state(true);
    search_box.pack_start(&js_toggle, false, false, 0);

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

    tabs::add_webview_tab(
        &notebook,
        "https://start.duckduckgo.com/",
        "New tab",
        &search_entry,
        &filter_set.clone(),
    );

    search_entry.connect_activate({
        let notebook = notebook.clone();

        move |search_entry| {
            let current_page = notebook.current_page();
            let url = search_entry.text();
            if let Some(widget) = notebook.nth_page(current_page) {
                if let Some(webview) = widget.downcast_ref::<webkit2gtk::WebView>() {
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

    options.connect_clicked(move |_| {
        settings::show_settings_window();
    });

    connections::js_toggle_activate(&js_toggle, &notebook);
    connections::back_button_clicked(&notebook, &back_button);
    connections::forward_button_clicked(&notebook, &forward_button);
    connections::refresh_button_clicked(&notebook, &refresh_button);
    connections::notebook_switch_page(&notebook, &search_entry);
    connections::new_tab_button_clicked(&new_tab_button, &notebook, &search_entry, &filter_set);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Propagation::Stop
    });

    search_entry.set_is_focus(true);

    gtk::main();
}
