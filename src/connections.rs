extern crate gtk;

use crate::{settings, tabs};
use adblock::FilterSet;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
use webkit2gtk::WebViewExt;

pub fn notebook_switch_page(notebook: &gtk::Notebook, search_entry: &gtk::Entry) {
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
}

pub fn new_tab_button_clicked(
    new_tab_button: &gtk::Button,
    notebook: &gtk::Notebook,
    search_entry: &gtk::Entry,
    filter_set: &Arc<Mutex<FilterSet>>,
) {
    new_tab_button.connect_clicked({
        let notebook = notebook.clone();
        let search_entry = search_entry.clone();
        let filter_set = filter_set.clone();

        move |_| {
            tabs::add_webview_tab(
                &notebook,
                "https://start.duckduckgo.com/",
                "New tab",
                &search_entry,
                &filter_set,
            )
        }
    });
}

pub fn refresh_button_clicked(notebook: &gtk::Notebook, refresh_button: &gtk::Button) {
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
}

pub fn forward_button_clicked(notebook: &gtk::Notebook, forward_button: &gtk::Button) {
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
}

pub fn back_button_clicked(notebook: &gtk::Notebook, back_button: &gtk::Button) {
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
}

pub fn js_toggle_activate(js_toggle: &gtk::Switch, notebook: &gtk::Notebook) {
    js_toggle.connect_active_notify({
        let notebook = notebook.clone();

        move |js_toggle| {
            let current_page = notebook.current_page();
            if let Some(widget) = notebook.nth_page(current_page) {
                if let Some(webview) = widget.downcast_ref::<webkit2gtk::WebView>() {
                    let webview = webview.clone();

                    settings::toggle_javascript_when_running(webview, js_toggle.is_active());
                }
            }
        }
    });
}
