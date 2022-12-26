use gtk::prelude::*;
use gtk::{gio, Application};
use window::Window;

mod window;

const APP_ID: &str = "org.hellbyte.PIXELMOSH";

pub fn start() {
    gio::resources_register_include!("pixelmosh.gresource").expect("Failed to register resources.");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}
fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
