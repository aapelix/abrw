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
    url: &str,
    title: &str,
    search_entry: &gtk::Entry,
    filter_set: &Arc<Mutex<FilterSet>>,
) {
    let filter_set_guard = filter_set.lock().unwrap();

    let webview = webkit2gtk::WebView::new();

    let engine = Engine::from_filter_set(filter_set_guard.clone(), true);

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

    // Enable this if you want Safari like tabs
    // label.set_hexpand(true);
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

    search_entry.set_is_focus(true);
}
