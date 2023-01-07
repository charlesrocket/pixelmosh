use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate, ResponseType, SpinButton, Stack};

use std::cell::RefCell;
use std::thread;

use libmosh::ops::{read_file, write_file};
use libmosh::{mosh, MoshOptions};

use crate::gui::window::Image;

#[derive(CompositeTemplate)]
#[template(resource = "/pixelmosh/window.ui")]
pub struct Window {
    #[template_child]
    pub btn_min_rate: TemplateChild<SpinButton>,
    #[template_child]
    pub btn_max_rate: TemplateChild<SpinButton>,
    #[template_child]
    pub btn_pixelation: TemplateChild<SpinButton>,
    #[template_child]
    pub btn_line_shift: TemplateChild<SpinButton>,
    #[template_child]
    pub btn_reverse: TemplateChild<SpinButton>,
    #[template_child]
    pub btn_flip: TemplateChild<SpinButton>,
    #[template_child]
    pub btn_channel_swap: TemplateChild<SpinButton>,
    #[template_child]
    pub btn_channel_shift: TemplateChild<SpinButton>,
    pub dialog: gtk::FileChooserNative,
    pub image: RefCell<Image>,
    pub options: MoshOptions,
    #[template_child]
    pub picture: TemplateChild<gtk::Picture>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "PixelmoshWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn new() -> Self {
        let png_filter = gtk::FileFilter::new();
        let dialog = gtk::FileChooserNative::builder()
            .title("Open File")
            .action(gtk::FileChooserAction::Open)
            .accept_label("Open")
            .cancel_label("Cancel")
            .modal(true)
            .build();

        png_filter.add_mime_type("image/png");
        png_filter.set_name(Some("PNG image"));
        dialog.add_filter(&png_filter);

        Self {
            btn_min_rate: TemplateChild::default(),
            btn_max_rate: TemplateChild::default(),
            btn_pixelation: TemplateChild::default(),
            btn_line_shift: TemplateChild::default(),
            btn_reverse: TemplateChild::default(),
            btn_flip: TemplateChild::default(),
            btn_channel_swap: TemplateChild::default(),
            btn_channel_shift: TemplateChild::default(),
            dialog,
            image: RefCell::new(Image::default()),
            options: MoshOptions::default(),
            picture: TemplateChild::default(),
            stack: TemplateChild::default(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
        klass.install_action_async(
            "win.open-file",
            None,
            |win, _action_name, _action_target| async move {
                let dialog = &win.imp().dialog;
                dialog.set_transient_for(Some(&win));
                if dialog.run_future().await == ResponseType::Accept {
                    if let Err(error) = win.set_file(&dialog.file().unwrap()) {
                        println!("Error loading the image: {error}");
                    }
                }
            },
        );
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
