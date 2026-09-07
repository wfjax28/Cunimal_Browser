use gtk::{
    prelude::*, Application, ApplicationWindow, Box as GtkBox, Button, Entry, HeaderBar, Label, Orientation,
};
use std::cell::RefCell;
use std::rc::Rc;
use webkit6::{prelude::*, WebView};

#[derive(Clone, Debug)]
struct TabMeta {
    title: String,
    url: String,
}

struct BrowserState {
    tabs: Vec<TabMeta>,
    active_tab: usize,
    current_webview: Option<WebView>,
    web_container: GtkBox,
    url_entry: Entry,
    tab_bar: GtkBox,
}

impl BrowserState {
    fn new(web_container: GtkBox, url_entry: Entry, tab_bar: GtkBox) -> Self {
        Self {
            tabs: vec![TabMeta {
                title: String::from("New Tab"),
                url: String::from("https://example.com"),
            }],
            active_tab: 0,
            current_webview: None,
            web_container,
            url_entry,
            tab_bar,
        }
    }

    fn normalize_url(input: &str) -> String {
        let trimmed = input.trim();
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            trimmed.to_string()
        } else {
            format!("https://{}", trimmed)
        }
    }

    fn render_tab_bar(state_rc: &Rc<RefCell<Self>>) {
        let tab_bar = state_rc.borrow().tab_bar.clone();

        while let Some(child) = tab_bar.first_child() {
            tab_bar.remove(&child);
        }

        let tabs = state_rc.borrow().tabs.clone();
        let active = state_rc.borrow().active_tab;

        for (index, tab) in tabs.iter().enumerate() {
            let label = if index == active {
                format!("● {}", tab.title)
            } else {
                format!("○ {}", tab.title)
            };

            let button = Button::with_label(&label);
            let state_clone = Rc::clone(state_rc);

            button.connect_clicked(move |_| {
                BrowserState::switch_tab(&state_clone, index);
            });

            tab_bar.append(&button);
        }
    }

    fn mount_webview(state_rc: &Rc<RefCell<Self>>, url: &str) {
        {
            let mut state = state_rc.borrow_mut();

            if let Some(old_webview) = state.current_webview.take() {
                state.web_container.remove(&old_webview);
            }
        }

        let webview = WebView::new();
        webview.load_uri(url);

        {
            let state_clone = Rc::clone(state_rc);
            webview.connect_notify_local(Some("uri"), move |wv, _| {
                if let Some(uri) = wv.uri() {
                    let mut state = state_clone.borrow_mut();
                    let active = state.active_tab;
                    state.tabs[active].url = uri.to_string();
                    state.url_entry.set_text(&uri);
                }
            });
        }

        {
            let state_clone = Rc::clone(state_rc);
            webview.connect_notify_local(Some("title"), move |wv, _| {
                let title = wv.title().unwrap_or_else(|| "Untitled".into());
                let mut state = state_clone.borrow_mut();
                let active = state.active_tab;
                state.tabs[active].title = title.to_string();
                drop(state);
                BrowserState::render_tab_bar(&state_clone);
            });
        }

        {
            let mut state = state_rc.borrow_mut();
            state.url_entry.set_text(url);
            state.web_container.append(&webview);
            state.current_webview = Some(webview);
        }
    }

    fn switch_tab(state_rc: &Rc<RefCell<Self>>, index: usize) {
        {
            let mut state = state_rc.borrow_mut();
            state.active_tab = index;
        }

        let url = {
            let state = state_rc.borrow();
            state.tabs[index].url.clone()
        };

        BrowserState::mount_webview(state_rc, &url);
        BrowserState::render_tab_bar(state_rc);
    }

    fn navigate_active_tab(state_rc: &Rc<RefCell<Self>>, input: &str) {
        let url = Self::normalize_url(input);

        {
            let mut state = state_rc.borrow_mut();
            let active = state.active_tab;
            state.tabs[active].url = url.clone();
        }

        BrowserState::mount_webview(state_rc, &url);
        BrowserState::render_tab_bar(state_rc);
    }

    fn new_tab(state_rc: &Rc<RefCell<Self>>) {
        let new_index = {
            let mut state = state_rc.borrow_mut();
            state.tabs.push(TabMeta {
                title: "New Tab".to_string(),
                url: "https://example.com".to_string(),
            });
            state.tabs.len() - 1
        };

        BrowserState::switch_tab(state_rc, new_index);
    }

    fn close_active_tab(state_rc: &Rc<RefCell<Self>>) {
        let next_index = {
            let mut state = state_rc.borrow_mut();

            if state.tabs.len() == 1 {
                state.tabs[0] = TabMeta {
                    title: "New Tab".to_string(),
                    url: "https://example.com".to_string(),
                };
                0
            } else {
                let active = state.active_tab;
                state.tabs.remove(active);

                if active >= state.tabs.len() {
                    state.tabs.len() - 1
                } else {
                    active
                }
            }
        };

        BrowserState::switch_tab(state_rc, next_index);
    }
}

pub struct WebviewApp;

impl WebviewApp {
    pub fn run() {
        let app = Application::builder()
            .application_id("com.cunimal_browser.app")
            .build();

        app.connect_activate(|app| {
            let root = GtkBox::new(Orientation::Vertical, 0);

            let header = HeaderBar::new();
            header.set_title_widget(Some(&Label::new(Some("Cunimal Browser"))));

            let controls = GtkBox::new(Orientation::Horizontal, 6);

            let new_tab_button = Button::with_label("+");
            let close_tab_button = Button::with_label("×");
            let url_entry = Entry::new();
            url_entry.set_hexpand(true);
            url_entry.set_placeholder_text(Some("URLを入力してください"));

            controls.append(&new_tab_button);
            controls.append(&close_tab_button);
            controls.append(&url_entry);

            let tab_bar = GtkBox::new(Orientation::Horizontal, 4);
            let web_container = GtkBox::new(Orientation::Vertical, 0);
            web_container.set_vexpand(true);
            web_container.set_hexpand(true);

            root.append(&controls);
            root.append(&tab_bar);
            root.append(&web_container);

            let window = ApplicationWindow::builder()
                .application(app)
                .title("Cunimal browser for linux (0.0.1)")
                .default_width(1100)
                .default_height(760)
                .child(&root)
                .build();

            let state = Rc::new(RefCell::new(BrowserState::new(
                web_container,
                url_entry.clone(),
                tab_bar,
            )));

            {
                let state_clone = Rc::clone(&state);
                url_entry.connect_activate(move |entry| {
                    let text = entry.text().to_string();
                    BrowserState::navigate_active_tab(&state_clone, &text);
                });
            }

            {
                let state_clone = Rc::clone(&state);
                new_tab_button.connect_clicked(move |_| {
                    BrowserState::new_tab(&state_clone);
                });
            }

            {
                let state_clone = Rc::clone(&state);
                close_tab_button.connect_clicked(move |_| {
                    BrowserState::close_active_tab(&state_clone);
                });
            }

            BrowserState::mount_webview(&state, "https://example.com");
            BrowserState::render_tab_bar(&state);

            window.present();
        });

        app.run();
    }
}

pub fn run() {
    WebviewApp::run();
}
