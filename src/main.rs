use glib::clone;
use gtk4::{Application, ApplicationWindow, prelude::*};
use webkit6::{WebView, prelude::WebViewExt};

fn main() {
    let app = Application::new(Some("org.qb.webkit6-test.test"), Default::default());
    app.connect_activate(move |app| {
        let vbox = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(10)
            .build();

        let webview = WebView::builder().vexpand(true).visible(true).build();

        let uri_entry = gtk4::Entry::builder()
            .placeholder_text("Input URI here...")
            .hexpand(true)
            .build();
        let button = gtk4::Button::builder().label("Go").build();

        button.connect_clicked(clone!(
            #[weak]
            webview,
            #[weak]
            uri_entry,
            move |_| {
                let input_text = uri_entry.text();
                let input_str = input_text.as_str();
                // TODO: Format input string
                println!("Loading uri: {input_str}");
                webview.load_uri(input_str);
            }
        ));

        let navbar = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(10)
            .margin_top(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        navbar.append(&uri_entry);
        navbar.append(&button);

        vbox.append(&navbar);
        vbox.append(&webview);

        let window = ApplicationWindow::builder()
            .application(app)
            .child(&vbox)
            .build();

        let settings = WebViewExt::settings(&webview).unwrap();
        settings.set_enable_developer_extras(true);

        let inspector = webview.inspector().unwrap();
        inspector.show();

        window.present();
    });
    app.run();
}
