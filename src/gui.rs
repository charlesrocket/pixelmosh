use adw::gio;
use adw::prelude::*;
use gtk::gdk::Display;
use gtk::{CssProvider, StyleContext};

use window::Window;

mod image;
mod window;

const APP_ID: &str = "org.hellbyte.pixelmosh";

pub fn start() {
    gio::resources_register_include!("pixelmosh.gresource").expect("Failed to register resources.");
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    let style_manager = adw::StyleManager::default();
    let window = Window::new(app);

    style_manager.set_color_scheme(adw::ColorScheme::PreferDark);
    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("../src/resources/style.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
