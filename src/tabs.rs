extern crate gtk;
extern crate webkit2gtk;

use crate::{adblock_abrw, settings};
use adblock::{lists::FilterSet, Engine};
use gtk::{
    gdk_pixbuf::{InterpType, Pixbuf},
    prelude::*,
};
use std::path::PathBuf;
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
        <title>New tab</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
                margin: 0;
                background-color: #36393F;
                color: #DCDDDE;
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

    let notebook_clone = notebook.clone();
    webview.connect_title_notify(move |webview| {
        let notebook = notebook_clone.clone();
        let webview = webview.clone();

        let title = webview
            .title()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Untitled".to_string());

        println!("Title changed {}", title);

        let current_page = notebook.current_page();

        if let Some(page) = notebook.nth_page(current_page) {
            if let Some(tab) = notebook.tab_label(&page) {
                if let Some(tab_box) = tab.downcast_ref::<gtk::Container>() {
                    for child in tab_box.children() {
                        if let Some(label) = child.downcast_ref::<gtk::Label>() {
                            label.set_label(&title);
                        }
                    }
                }
            }
        }
    });

    // not working
    //let notebook_clone = notebook.clone();
    //webview.connect_favicon_notify(move |webview| {
    //    println!("favicon changed");

    //    let notebook = notebook_clone.clone();
    //    let webview = webview.clone();

    //    let title = webview.favicon();

    //    let current_page = notebook.current_page();

    //    if let Some(page) = notebook.nth_page(current_page) {
    //        if let Some(tab) = notebook.tab_label(&page) {
    //            if let Some(tab_box) = tab.downcast_ref::<gtk::Container>() {
    //                for child in tab_box.children() {
    //                    if let Some(image) = child.downcast_ref::<gtk::Image>() {
    //                        if let Some(favicon) = title.clone() {
    //                            let pixbuf_icon =
    //                                gtk::gdk::pixbuf_get_from_surface(&favicon, 0, 0, 25, 25);

    //                            image.set_from_pixbuf(pixbuf_icon.as_ref());
    //                            image.show();
    //                        }
    //                    }
    //                }
    //            }
    //        }
    //    }
    //});

    //     webview.connect_favicon_notify(move |webview| {});

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let path = PathBuf::from("/usr/share/pixmaps/myicon.png");
    let pixbuf_icon = Pixbuf::from_file(path).expect("Failed to create pixbuf");
    let scaled_pixbuf = &pixbuf_icon.scale_simple(25, 25, InterpType::Bilinear);

    let icon = gtk::Image::from_pixbuf(scaled_pixbuf.as_ref());
    let label = gtk::Label::new(Some(title));

    icon.set_pixel_size(2000);

    // Enable this if you want Safari like tabs
    // label.set_hexpand(true);
    label.set_vexpand(false);

    hbox.pack_start(&icon, false, false, 10);
    hbox.pack_start(&label, false, false, 10);

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

    notebook.set_tab_reorderable(&webview, true);
    notebook.set_tab_detachable(&webview, true);

    webview.show();
    label.show();
    icon.show();
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
