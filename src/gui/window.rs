use adw::{prelude::*, subclass::prelude::*};
use glib::{Object, clone};
use gtk::{EntryIconPosition::Secondary, License, gio, glib};

use libmosh::err::MoshError;

use std::sync::Arc;

use crate::gui::image::Image;

mod imp;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

enum Mode {
    Normal,
    Rewind,
    Seed,
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder::<Window>()
            .property("application", app)
            .build()
    }

    #[cfg(debug_assertions)]
    fn setup_debug(&self) {
        self.add_css_class("devel");
    }

    fn setup_actions(&self) {
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                let dialog = Window::about_dialog();
                dialog.present(Some(&window));
            }
        ));

        let action_minimize = gio::SimpleAction::new("minimize", None);
        action_minimize.connect_activate(clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                window.minimize();
            }
        ));

        let action_maximize = gio::SimpleAction::new("maximize", None);
        action_maximize.connect_activate(clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                if window.is_maximized() {
                    window.unmaximize();
                } else {
                    window.maximize();
                }
            }
        ));

        let action_close = gio::SimpleAction::new("close", None);
        action_close.connect_activate(clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                window.close();
            }
        ));

        let action_style_manager = gio::SimpleAction::new("toggle-color-scheme", None);
        action_style_manager.connect_activate(clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                window.toggle_color_scheme();
            }
        ));

        self.add_action(&action_about);
        self.add_action(&action_minimize);
        self.add_action(&action_maximize);
        self.add_action(&action_close);
        self.add_action(&action_style_manager);
    }

    fn setup_callbacks(&self) {
        self.imp().btn_ansi.connect_toggled(clone!(
            #[weak(rename_to = window)]
            self,
            move |button| {
                window.toggle_ansi(button.is_active());
            }
        ));

        self.imp().seed.connect_icon_release(clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                match window.mosh(Mode::Seed) {
                    Ok(()) => {}
                    Err(error) => {
                        window.show_message(&format!("Failed: {error}"), 0);
                    }
                };
            }
        ));

        self.imp().seed.connect_changed(clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.set_seed_button();
            }
        ));

        self.set_stack();
    }

    fn setup_buttons(&self) {
        self.set_seed_button();
        self.set_rewind_button();
    }

    fn skip_placeholder(&self) {
        self.imp().stack.set_visible_child_name("main");
    }

    fn set_stack(&self) {
        self.imp().stack.set_visible_child_name("placeholder");
    }

    fn set_instructions(&self) {
        self.imp().stack.set_visible_child_name("instructions");
    }

    fn set_color_type(&self, label: &str) {
        self.imp().color_type.set_label(label);
    }

    fn toggle_ansi(&self, value: bool) {
        self.imp().image.lock().unwrap().set_ansi(value);
    }

    fn set_seed_button(&self) {
        let seed = &self.imp().seed;

        if seed.buffer().text().to_string().is_empty() {
            seed.set_icon_sensitive(Secondary, false);
        } else {
            seed.set_icon_sensitive(Secondary, true);
        }
    }

    fn set_rewind_button(&self) {
        if self.imp().image.lock().unwrap().settings.is_none() {
            self.imp().btn_rewind.set_sensitive(false);
        } else {
            self.imp().btn_rewind.set_sensitive(true);
        }
    }

    fn mosh(&self, mode: Mode) -> Result<(), MoshError> {
        self.imp().spinner.set_visible(true);
        let (sender, receiver) = async_channel::bounded(1);
        let buffer = &self.imp().seed.buffer();
        let seed = buffer.text().to_string();
        let image = Arc::clone(&self.imp().image);

        if seed.parse::<u64>().is_err() {
            image.lock().unwrap().new_seed();
            self.imp()
                .seed
                .buffer()
                .set_text(image.lock().unwrap().get_seed().to_string());
        } else {
            image.lock().unwrap().set_seed(seed.parse::<u64>().unwrap());
        }

        if image.lock().unwrap().is_present {
            let image_spawn_clone = image.clone();
            gio::spawn_blocking(move || {
                let mut thread_image = image_spawn_clone.lock().unwrap();

                match mode {
                    Mode::Normal => {
                        thread_image.save_settings();
                        thread_image.new_seed();
                        thread_image.mosh_file();
                    }
                    Mode::Rewind => {
                        thread_image.load_settings();
                        thread_image.mosh_file();
                    }
                    Mode::Seed => {
                        thread_image.mosh_file();
                        thread_image.new_seed();
                    }
                };

                sender.send_blocking(true).unwrap();
            });

            self.imp()
                .seed
                .buffer()
                .set_text(image.clone().lock().unwrap().get_seed().to_string());

            glib::spawn_future_local(clone!(
                #[weak(rename_to = image_clone)]
                self,
                async move {
                    while let Ok(show_image) = receiver.recv().await {
                        if show_image {
                            image_clone.imp().spinner.set_visible(false);
                            image_clone
                                .imp()
                                .seed
                                .buffer()
                                .set_text(image.lock().unwrap().get_seed().to_string());
                            image_clone
                                .imp()
                                .picture
                                .set_paintable(Some(&image.lock().unwrap().get_texture()));
                        }
                    }
                }
            ));
        }

        Ok(())
    }

    fn load_file(&self, file: &gio::File) {
        let cont = Arc::clone(&self.imp().image);
        let mut image = cont.lock().unwrap();

        image.new_seed();

        if image.open_file(&file.path().unwrap()).is_ok() {
            image.mosh_file();
            self.imp().picture.set_paintable(Some(&image.get_texture()));
            self.skip_placeholder();
        } else {
            self.set_instructions();
        }
    }

    fn save_file(&self, file: &gio::File) -> Result<(), MoshError> {
        self.imp()
            .image
            .lock()
            .unwrap()
            .save_file(&file.path().unwrap())?;

        Ok(())
    }

    fn toggle_color_scheme(&self) {
        if self.imp().style_manager.is_dark() {
            self.imp()
                .style_manager
                .set_color_scheme(adw::ColorScheme::ForceLight);
        } else {
            self.imp()
                .style_manager
                .set_color_scheme(adw::ColorScheme::ForceDark);
        }
    }

    pub fn add_toast(&self, toast: adw::Toast) {
        self.imp().toast_overlay.add_toast(toast);
    }

    pub fn show_message(&self, message: &str, timeout: u32) {
        let toast = adw::Toast::new(message);
        toast.set_timeout(timeout);
        self.add_toast(toast);
    }

    fn about_dialog() -> adw::AboutDialog {
        let about_dialog = adw::AboutDialog::builder()
            .application_name("PIXELMOSH")
            .version(env!("CARGO_PKG_VERSION"))
            .license_type(License::MitX11)
            .website(env!("CARGO_PKG_REPOSITORY"))
            .comments(env!("CARGO_PKG_DESCRIPTION"))
            .build();

        about_dialog.add_link(
            "Release Notes",
            "https://github.com/charlesrocket/pixelmosh/blob/trunk/CHANGELOG.md",
        );

        about_dialog
    }
}
