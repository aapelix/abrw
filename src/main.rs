extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

use gtk::{glib::Propagation, prelude::*};
use webkit2gtk::WebViewExt;

fn add_webview_tab(notebook: &gtk::Notebook, url: &str, title: &str) {
    let webview = webkit2gtk::WebView::new();
    webview.load_html(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Abrw Home</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                    background-color: #0a0a0a;
                    color: white;
                }
                h1 {
                    font-size: 4em;
                    font-weight: bold;
                    margin: 0;
                }
                .search-container {
                    margin-top: 20px;
                }
                input[type="text"] {
                    width: 400px;
                    padding: 10px;
                    font-size: 1.2em;
                    border: 2px solid #333;
                    border-radius: 5px;
                    background-color: #121212;
                    color: white;
                }
                input[type="submit"] {
                    padding: 10px 20px;
                    font-size: 1em;
                    border: none;
                    background-color: #121212;
                    color: white;
                    border-radius: 5px;
                    margin-left: 10px;
                    cursor: pointer;
                }
                input[type="submit"]:hover {
                    background-color: #555;
                }
            </style>
        </head>
        <body>
            <h1>Abrw</h1>
            <div class="search-container">
                <form onsubmit="searchDuckDuckGo(event)">
                    <input type="text" id="searchQuery" placeholder="Search DuckDuckGo or enter a address">
                </form>
            </div>

            <script>
                function searchDuckDuckGo(event) {
                    event.preventDefault();
                    const query = document.getElementById('searchQuery').value.trim();
                    if (query.startsWith("http://") || query.startsWith("https://") || query.includes(".")) {
                        window.location.href = query;
                    } else {
                        window.location.href = `https://duckduckgo.com/?q=${encodeURIComponent(query)}`;
                    }
                }
            </script>
        </body>
        </html>
        "#, None);

    // Create a box to hold the label and close button
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    // Create the tab label
    let label = gtk::Label::new(Some(title));

    // Add padding around the label to make the tab wider
    hbox.pack_start(&label, true, true, 10); // Increased padding (last argument)

    // Create the close button
    let close_button = gtk::Button::new();
    let close_label = gtk::Label::new(Some("X"));
    close_button.set_relief(gtk::ReliefStyle::None);
    close_button.add(&close_label);

    // Add the label and button to the box
    hbox.pack_start(&close_button, false, false, 0);

    // Append the new page to the notebook, with the custom tab label
    let new_tab_index = notebook.append_page(&webview, Some(&hbox));

    // Show all widgets in the tab
    webview.show();
    label.show();
    close_label.show();
    close_button.show();
    hbox.show();

    // Set the current page to the new tab
    notebook.set_current_page(Some(new_tab_index));

    // Connect the close button to remove the tab when clicked
    let notebook_clone = notebook.clone();
    close_button.connect_clicked(move |_| {
        notebook_clone.remove_page(Some(new_tab_index));
    });
}

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
            background: #161616;
            border-left: none;
            border-right: none;
            border-radius: 5px;
            margin: 2px;
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

    back_button.set_size_request(40, 40);
    forward_button.set_size_request(40, 40);
    refresh_button.set_size_request(40, 40);
    new_tab_button.set_size_request(40, 40);

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
                box-shadow: none;
                -gtk-icon-shadow: none;
                text-shadow: none;
            }
            button:hover {
                background: rgba(0, 0, 0, 0.5);
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
        border-radius: 25px;
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

    let minimize = gtk::Button::with_label("_");
    let maximize = gtk::Button::with_label("[ ]");
    let close = gtk::Button::with_label("X");

    hbox.pack_end(&close, false, false, 5);
    hbox.pack_end(&maximize, false, false, 5);
    hbox.pack_end(&minimize, false, false, 5);

    minimize.set_size_request(10, 10);
    maximize.set_size_request(10, 10);
    close.set_size_request(10, 10);

    let css_provider = gtk::CssProvider::new();
    css_provider
        .load_from_data(
            b"
            button {
                background: transparent;
                border: none;
                box-shadow: none;
                -gtk-icon-shadow: none;
                text-shadow: none;
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

    let minimize_style = minimize.style_context();
    let maximize_style = maximize.style_context();
    let close_style = close.style_context();

    close_style.add_class("close_button");

    minimize_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    maximize_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    close_style.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    search_box.pack_start(&search_entry, false, false, 0);
    search_box.set_halign(gtk::Align::Center); // Center the hbox within the vbox

    vbox.pack_start(&hbox, false, false, 15);
    vbox.pack_start(&notebook, true, true, 10);

    add_webview_tab(&notebook, "https://start.duckduckgo.com/", "New tab");

    // Connect search button to change URL
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

                    // Check if the input starts with "http://" or "https://"
                    if !url.starts_with("http://") && !url.starts_with("https://") {
                        // Check if the input looks like a URL (basic check)
                        if url.contains('.') && !url.contains(' ') {
                            // Load as a URL
                            webview.load_uri(&format!("https://{}", url));
                        } else {
                            // Perform a search instead (using a search engine)
                            let search_query = url.replace(" ", "+");
                            webview
                                .load_uri(&format!("https://duckduckgo.com/?q={}", search_query));
                        }
                    } else {
                        // If it's a valid URL, load it
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
        window_clone.iconify(); // Minimize the window
    });

    let window_clone = window.clone();
    maximize.connect_clicked(move |_| {
        if window_clone.is_maximized() {
            window_clone.unmaximize(); // Restore the window
        } else {
            window_clone.maximize(); // Maximize the window
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

        move |_| add_webview_tab(&notebook, "https://start.duckduckgo.com/", "New tab")
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Propagation::Stop
    });

    gtk::main();
}
