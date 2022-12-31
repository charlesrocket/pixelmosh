use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, Button, CompositeTemplate, Stack};

use std::thread;

use libmosh::ops::{read_file, write_file};
use libmosh::{mosh, MoshOptions};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/pixelmosh/window.ui")]
pub struct Window {
    #[template_child]
    pub button1: TemplateChild<Button>,
    #[template_child]
    pub button2: TemplateChild<Button>,
    #[template_child]
    pub button3: TemplateChild<Button>,
    #[template_child]
    pub button4: TemplateChild<Button>,
    #[template_child]
    pub button5: TemplateChild<Button>,
    #[template_child]
    pub button6: TemplateChild<Button>,
    #[template_child]
    pub button7: TemplateChild<Button>,
    #[template_child]
    pub button8: TemplateChild<Button>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "PixelmoshWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_callbacks();
        obj.setup_actions();
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_moshed() {
        thread::spawn(move || {
            let (mut buf, info) = read_file("src/util/test-rgb.png").unwrap();
            mosh(&info, &mut buf, &MoshOptions::default()).unwrap();
            write_file("test-output.png", &buf, &info).unwrap();
        });
    }

    #[template_callback]
    fn handle_seed() {
        dbg!("fOO");
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
