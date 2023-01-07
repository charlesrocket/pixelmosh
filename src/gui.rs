use adw::gio;
use adw::prelude::*;
use window::Window;

mod image;
mod window;

const APP_ID: &str = "org.hellbyte.pixelmosh";

pub fn start() {
    gio::resources_register_include!("pixelmosh.gresource").expect("Failed to register resources.");
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    let style_manager = adw::StyleManager::default();
    let window = Window::new(app);

    style_manager.set_color_scheme(adw::ColorScheme::ForceDark);
    window.present();
}
