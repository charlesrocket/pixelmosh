use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{clone, Object};
use gtk::{gio, glib};

use libmosh::err::MoshError;

use crate::gui::image::Image;

mod imp;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::new(&[("application", app)])
    }

    fn setup_actions(&self) {
        let action_mosh_image = gio::SimpleAction::new("mosh-file", None);
        action_mosh_image.connect_activate(clone!(@weak self as window => move |_, _| {
            window.mosh();
        }));

        self.add_action(&action_mosh_image);
    }

    fn setup_callbacks(&self) {
        self.set_stack();
    }

    fn skip_placeholder(&self) {
        self.imp().stack.set_visible_child_name("main");
    }

    fn set_stack(&self) {
        self.imp().stack.set_visible_child_name("placeholder");
    }

    fn mosh(&self) {
        self.imp().image.borrow_mut().mosh(&self.imp().options);
        self.imp()
            .picture
            .set_paintable(Some(&self.imp().image.borrow_mut().get_texture()));
    }

    fn set_file(&self, file: &gio::File) -> Result<(), MoshError> {
        self.imp()
            .image
            .borrow_mut()
            .open_file(&file.path().unwrap())?;
        self.imp().image.borrow_mut().mosh(&self.imp().options);
        self.imp()
            .picture
            .set_paintable(Some(&self.imp().image.borrow_mut().get_texture()));
        self.skip_placeholder();
        Ok(())
    }
}
