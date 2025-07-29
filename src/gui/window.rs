use adw::{prelude::*, subclass::prelude::*};
use glib::{Object, clone};
use gtk::{EntryIconPosition::Secondary, License, gio, glib};

use libmosh::err::MoshError;

use std::sync::Arc;

use crate::gui::base::Base;

mod imp;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[derive(PartialEq)]
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
                window.set_ansi(button.is_active());
            }
        ));

        self.imp().seed.connect_activate(clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.mosh(Mode::Seed);
            }
        ));

        self.imp().seed.connect_icon_release(clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                window.mosh(Mode::Seed);
            }
        ));

        self.imp().seed.connect_changed(clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                let seed = &window.imp().seed;
                if seed.buffer().text().to_string().is_empty() {
                    seed.set_icon_sensitive(Secondary, false);
                } else {
                    seed.set_icon_sensitive(Secondary, true);
                }
            }
        ));

        self.set_stack();
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

    fn set_ansi(&self, value: bool) {
        self.imp().base.lock().unwrap().set_ansi(value);
    }

    fn parse_seed(&self) {
        let buffer = &self.imp().seed.buffer();
        let seed = buffer.text().to_string();
        let filtered_seed: String = seed.chars().filter(|c| c.is_numeric()).collect();

        if filtered_seed.parse::<u64>().is_err()
            || filtered_seed.is_empty()
            || filtered_seed.len() > u64::MAX.try_into().unwrap()
        {
            self.imp().base.lock().unwrap().new_seed();
            self.update_seed();
        } else {
            self.imp()
                .base
                .lock()
                .unwrap()
                .set_seed(filtered_seed.parse::<u64>().unwrap());
        }
    }

    fn update_seed(&self) {
        let seed = self.imp().base.lock().unwrap().get_seed();
        self.imp().seed.buffer().set_text(seed.to_string());
    }

    fn update_picture(&self) {
        let mut base = self.imp().base.lock().unwrap();
        let texture = base.get_texture();
        self.imp().picture.set_paintable(Some(&texture));
    }

    fn busy(&self, is_busy: bool) {
        let window = self.imp();
        let seed = &window.seed;

        if is_busy {
            window.spinner.set_visible(true);
            window.btn_rewind.set_sensitive(false);
            window.btn_mosh.set_sensitive(false);
            window.btn_open.set_sensitive(false);
            window.btn_save.set_sensitive(false);
            window.btn_menu.set_sensitive(false);
        } else {
            window.spinner.set_visible(false);
            window.btn_mosh.set_sensitive(true);
            window.btn_open.set_sensitive(true);
            window.btn_save.set_sensitive(true);
            window.btn_menu.set_sensitive(true);

            if window.base.lock().unwrap().settings.is_some() {
                window.btn_rewind.set_sensitive(true);
            }

            if seed.buffer().text().to_string().is_empty() {
                seed.set_icon_sensitive(Secondary, false);
            } else {
                seed.set_icon_sensitive(Secondary, true);
            }
        }
    }

    fn mosh(&self, mode: Mode) {
        self.busy(true);

        let (sender, receiver) = async_channel::bounded(1);
        let (sender_e, receiver_e) = async_channel::bounded(1);

        if mode == Mode::Seed {
            self.parse_seed();
        }

        let base = Arc::clone(&self.imp().base);
        gio::spawn_blocking(move || {
            let mut thread_base = base.lock().unwrap();

            match mode {
                Mode::Normal => {
                    thread_base.save_settings();
                    thread_base.new_seed();
                    match thread_base.mosh_file() {
                        Ok(()) => sender.send_blocking(true).unwrap(),
                        Err(error) => sender_e.send_blocking(error).unwrap(),
                    }
                }
                Mode::Rewind => {
                    thread_base.load_settings();
                    match thread_base.mosh_file() {
                        Ok(()) => sender.send_blocking(true).unwrap(),
                        Err(error) => sender_e.send_blocking(error).unwrap(),
                    }
                }
                Mode::Seed => match thread_base.mosh_file() {
                    Ok(()) => sender.send_blocking(true).unwrap(),
                    Err(error) => sender_e.send_blocking(error).unwrap(),
                },
            }
        });

        glib::spawn_future_local(clone!(
            #[weak(rename_to = window_clone)]
            self,
            async move {
                while let Ok(show_image) = receiver.recv().await {
                    if show_image {
                        window_clone.busy(false);
                        window_clone.update_seed();
                        window_clone.update_picture();
                    }
                }
            }
        ));

        glib::spawn_future_local(clone!(
            #[weak(rename_to = window_clone)]
            self,
            async move {
                while let Ok(error) = receiver_e.recv().await {
                    window_clone.busy(false);
                    window_clone.show_message(&format!("Failed: {error}"), 10);
                }
            }
        ));
    }

    fn load_file(&self, file: &gio::File) {
        self.skip_placeholder();
        self.imp().btn_ansi.set_active(false);
        self.busy(true);
        let (sender, receiver) = async_channel::bounded(1);
        let base_arc = Arc::clone(&self.imp().base);
        let base_clone = base_arc.clone();
        let file_copy = file.clone();

        gio::spawn_blocking(move || {
            let mut base = base_clone.lock().unwrap();
            base.save_settings();

            if base.open_file(&file_copy.path().unwrap()).is_ok() {
                sender.send_blocking(true).unwrap();
            } else {
                sender.send_blocking(false).unwrap();
            }
        });

        glib::spawn_future_local(clone!(
            #[weak(rename_to = window_clone)]
            self,
            async move {
                while let Ok(image_loaded) = receiver.recv().await {
                    let base_clone_w = base_arc.clone();
                    if image_loaded {
                        let main_base = base_clone_w.lock().unwrap();
                        let data = &main_base.core.data;
                        let settings = &main_base.settings.clone().unwrap();

                        window_clone
                            .imp()
                            .picture
                            .set_paintable(Some(&Base::generate_texture(data, settings)));
                    } else {
                        window_clone.set_instructions();
                    }

                    window_clone.busy(false);
                }
            }
        ));
    }

    fn save_file(&self, file: &gio::File) -> Result<(), MoshError> {
        self.imp()
            .base
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
