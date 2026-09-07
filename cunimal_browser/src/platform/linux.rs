use gtk::{prelude::*, Application, ApplicationWindow};
use webkit6::{prelude::*, WebView};

pub struct WebviewApp;

impl WebviewApp {
  pub fn run() {
    let app = Application::builder()
      .application_id("com.cunimal_browser.app")
      .build();

    app.connect_activate(|app| {
      let webview = WebView::new();
      webview.load_uri("https://google.com");

      let window = ApplicationWindow::builder()
        .application(app)
        .title("Cunimal browser for linux (0.0.1)")
        .default_width(1000)
        .default_height(700)
        .child(&webview)
        .build();

      window.present();
    });

    app.run();
  }
}

pub fn run(){
    WebviewApp::run();
}