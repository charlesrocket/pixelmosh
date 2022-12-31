use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{clone, Object};
use gtk::{gio, glib};

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
        let action_new_image = gio::SimpleAction::new("new-image", None);
        action_new_image.connect_activate(clone!(@weak self as window => move |_, _| {
            window.skip_placeholder();
        }));

        self.add_action(&action_new_image);
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
}
