use adw::{gio, prelude::*};
use gtk::{gdk::Display, CssProvider};

use window::Window;

mod image;
mod window;

const APP_ID: &str = "org.hellbyte.pixelmosh";

pub fn start() {
    gio::resources_register_include!("pixelmosh.gresource").expect("Failed to register resources.");
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.set_accels_for_action("win.open-file", &["<Primary>o"]);
    app.set_accels_for_action("win.mosh-file", &["<Primary><Shift>p"]);
    app.set_accels_for_action("win.save-file", &["<Primary>s"]);
    app.set_accels_for_action("win.minimize", &["m"]);
    app.set_accels_for_action("win.maximize", &["f"]);
    app.set_accels_for_action("win.toggle-color-scheme", &["c"]);
    app.set_accels_for_action("win.close", &["<Primary>q"]);

    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);
    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("resources/style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
