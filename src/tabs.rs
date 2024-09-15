extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

use crate::{adblock_abrw, settings};
use adblock::{lists::FilterSet, Engine};
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
use webkit2gtk::{SettingsExt, WebViewExt};

pub fn add_webview_tab(
    notebook: &gtk::Notebook,
    _url: &str,
    title: &str,
    search_entry: &gtk::Entry,
    filter_set: &Arc<Mutex<FilterSet>>,
) {
    let webview = webkit2gtk::WebView::new();
    let engine = Engine::from_filter_set(filter_set.lock().unwrap().clone(), true);

    webview.connect_resource_load_started(move |webview, resource, request| {
        adblock_abrw::on_resource_load_started(webview, resource, request, &engine);
    });

    let web_view_settings = WebViewExt::settings(&webview).unwrap();
    let web_view_settings_json = settings::Settings::load();

    // SETTINGS
    web_view_settings.set_enable_developer_extras(true);
    web_view_settings.set_enable_smooth_scrolling(true);

    web_view_settings.set_enable_javascript(web_view_settings_json.enable_javascript);
    web_view_settings.set_enable_webgl(web_view_settings_json.enable_webgl);
    web_view_settings.set_enable_page_cache(web_view_settings_json.page_cache);
    web_view_settings.set_media_playback_requires_user_gesture(
        web_view_settings_json.media_playback_requires_user_gesture,
    );
    web_view_settings.set_user_agent(Some("aapelix/abrw"));

    let html_data = r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>abrw</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
                margin: 0;
                background-color: #0f0f0f;
            }
            .container {
                text-align: center;
            }
            .search-bar {
                width: 100%;
                max-width: 600px;
                padding: 10px;
                border: 1px solid #ccc;
                border-radius: 4px;
                font-size: 16px;
            }
            .search-bar:focus {
                outline: none;
                border-color: #007bff;
            }
            h1 {
                color: white;
                text-align: center;
            }
            a {
                color: white;
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>aapelix/abrw</h1>
            <a href="https://github.com/aapelix/abrw">Source code</a>
        </div>
    </body>
    </html>
"#;

    webview.load_html(&html_data, None);

    let search_entry_clone = search_entry.clone();
    webview.connect_notify_local(Some("uri"), move |webview, _| {
        if let Some(uri) = webview.uri() {
            search_entry_clone.set_text(&uri);
        }
    });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    // Create the tab label
    let label = gtk::Label::new(Some(title));

    // Enable this if you want Safari like tabs
    // label.set_hexpand(true);
    label.set_vexpand(false);

    hbox.pack_start(&label, true, true, 10);

    let close_button = gtk::Button::new();

    close_button.set_size_request(25, 25);

    let close_label = gtk::Label::new(Some("x"));

    close_button.add(&close_label);

    let css_provider = gtk::CssProvider::new();
    css_provider
        .load_from_data(
            b"
        button {
            background: transparent;
            border: none;
            border-radius: 5px;
        }

        button:hover {
            background: #313131;
        }
    ",
        )
        .expect("Failed to load css");

    let style_context = close_button.style_context();
    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

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

    search_entry.set_is_focus(true);
}
