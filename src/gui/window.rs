use adw::{prelude::*, subclass::prelude::*};
use glib::{clone, Object};
use gtk::{gdk::gdk_pixbuf, gio, glib, EntryIconPosition::Secondary, License, Picture};
use png::ColorType;

use libmosh::err::MoshError;

use crate::gui::image::Image;

mod imp;

pub const LOGO: &[u8; 67398] = include_bytes!("../resources/app_logo.png");

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder::<Window>()
            .property("application", app)
            .build()
    }

    fn setup_actions(&self) {
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as window => move |_, _| {
            window.show_about_dialog();
        }));

        let action_minimize = gio::SimpleAction::new("minimize", None);
        action_minimize.connect_activate(clone!(@weak self as window => move |_, _| {
            window.minimize();
        }));

        let action_maximize = gio::SimpleAction::new("maximize", None);
        action_maximize.connect_activate(clone!(@weak self as window => move |_, _| {
            if window.is_maximized() {
                window.unmaximize();
            } else {
                window.maximize();
            }
        }));

        let action_close = gio::SimpleAction::new("close", None);
        action_close.connect_activate(clone!(@weak self as window => move |_, _| {
            window.close();
        }));

        let action_style_manager = gio::SimpleAction::new("toggle-color-scheme", None);
        action_style_manager.connect_activate(clone!(@weak self as window => move |_, _| {
            window.toggle_color_scheme();
        }));

        self.add_action(&action_about);
        self.add_action(&action_minimize);
        self.add_action(&action_maximize);
        self.add_action(&action_close);
        self.add_action(&action_style_manager);
    }

    fn setup_callbacks(&self) {
        self.imp()
            .seed
            .connect_icon_release(clone!(@weak self as window => move |_,_| {
                window.mosh_with_seed();
            }));

        self.imp()
            .seed
            .connect_changed(clone!(@weak self as window => move |_| {
                window.set_seed_button();
            }));

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

    fn set_seed_button(&self) {
        if self.imp().seed.buffer().text().to_string().is_empty() {
            self.imp().seed.set_icon_sensitive(Secondary, false)
        } else {
            self.imp().seed.set_icon_sensitive(Secondary, true);
        }
    }

    fn mosh_with_seed(&self) {
        let buffer = self.imp().seed.buffer();
        let seed = buffer.text().to_string();

        if seed.parse::<u64>().is_err() {
            self.imp().image.borrow_mut().new_seed();
            self.imp()
                .seed
                .buffer()
                .set_text(self.imp().image.borrow_mut().get_seed().to_string());

            self.imp().image.borrow_mut().mosh();
        } else {
            self.imp()
                .image
                .borrow_mut()
                .set_seed(seed.parse::<u64>().unwrap());

            self.imp().image.borrow_mut().mosh();

            buffer.set_text(self.imp().image.borrow_mut().get_seed().to_string());
        }

        self.imp().image.borrow_mut().new_seed();
        self.imp()
            .picture
            .set_paintable(Some(&self.imp().image.borrow_mut().get_texture()));
    }

    fn mosh(&self) {
        if self.imp().image.borrow_mut().is_present {
            self.imp().image.borrow_mut().new_seed();
            self.imp().image.borrow_mut().mosh();
            self.imp()
                .seed
                .buffer()
                .set_text(self.imp().image.borrow_mut().get_seed().to_string());

            self.imp()
                .picture
                .set_paintable(Some(&self.imp().image.borrow_mut().get_texture()));
        }
    }

    fn load_file(&self, file: &gio::File) {
        self.imp().image.borrow_mut().new_seed();

        if self
            .imp()
            .image
            .borrow_mut()
            .open_file(&file.path().unwrap())
            .is_ok()
        {
            if self.imp().image.borrow_mut().core.data.color_type != ColorType::Indexed {
                self.imp().image.borrow_mut().mosh();
                self.imp()
                    .picture
                    .set_paintable(Some(&self.imp().image.borrow_mut().get_texture()));

                self.skip_placeholder();
            } else {
                self.set_instructions();
            }
        } else {
            self.set_instructions();
        }
    }

    fn save_file(&self, file: &gio::File) -> Result<(), MoshError> {
        self.imp()
            .image
            .borrow_mut()
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

    fn show_about_dialog(&self) {
        gtk::AboutDialog::builder()
            .program_name("PIXELMOSH")
            .version(env!("CARGO_PKG_VERSION"))
            .license_type(License::MitX11)
            .website(env!("CARGO_PKG_REPOSITORY"))
            .comments(env!("CARGO_PKG_DESCRIPTION"))
            .logo(
                &Picture::for_pixbuf(
                    &gdk_pixbuf::Pixbuf::from_read(std::io::BufReader::new(&LOGO[..])).unwrap(),
                )
                .paintable()
                .unwrap(),
            )
            .build()
            .present();
    }
}
