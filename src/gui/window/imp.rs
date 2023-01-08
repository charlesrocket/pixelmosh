use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate, ResponseType, SpinButton, Stack};

use std::cell::RefCell;

use crate::gui::image::Options;
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
    pub dialog_open: gtk::FileChooserNative,
    pub dialog_save: gtk::FileChooserNative,
    pub image: RefCell<Image>,
    pub options: RefCell<Options>,
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
        let dialog_open = gtk::FileChooserNative::builder()
            .title("Open file")
            .action(gtk::FileChooserAction::Open)
            .accept_label("Open")
            .cancel_label("Cancel")
            .modal(true)
            .build();

        let dialog_save = gtk::FileChooserNative::builder()
            .title("Save file")
            .action(gtk::FileChooserAction::Save)
            .accept_label("Save")
            .cancel_label("Cancel")
            .modal(true)
            .build();

        png_filter.add_mime_type("image/png");
        png_filter.set_name(Some("PNG image"));
        dialog_open.add_filter(&png_filter);

        Self {
            btn_min_rate: TemplateChild::default(),
            btn_max_rate: TemplateChild::default(),
            btn_pixelation: TemplateChild::default(),
            btn_line_shift: TemplateChild::default(),
            btn_reverse: TemplateChild::default(),
            btn_flip: TemplateChild::default(),
            btn_channel_swap: TemplateChild::default(),
            btn_channel_shift: TemplateChild::default(),
            dialog_open,
            dialog_save,
            image: RefCell::new(Image::default()),
            options: RefCell::new(Options::default()),
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
                let dialog = &win.imp().dialog_open;
                dialog.set_transient_for(Some(&win));
                if dialog.run_future().await == ResponseType::Accept {
                    if let Err(error) = win.load_file(&dialog.file().unwrap()) {
                        println!("Error loading the image: {error}");
                    }
                }
            },
        );

        klass.install_action_async(
            "win.save-file",
            None,
            |win, _action_name, _action_target| async move {
                let dialog = &win.imp().dialog_save;
                dialog.set_transient_for(Some(&win));
                if dialog.run_future().await == ResponseType::Accept {
                    if let Err(error) = win.save_file(&dialog.file().unwrap()) {
                        println!("Error saving the image: {error}");
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
    fn handle_min_rate(&self, button: &gtk::SpinButton) {
        self.options
            .borrow_mut()
            .set_min_rate(button.value() as u16);
    }

    #[template_callback]
    fn handle_max_rate(&self, button: &gtk::SpinButton) {
        self.options
            .borrow_mut()
            .set_max_rate(button.value() as u16);
    }

    #[template_callback]
    fn handle_pixelation(&self, button: &gtk::SpinButton) {
        self.options
            .borrow_mut()
            .set_pixelation(button.value() as u8);
    }

    #[template_callback]
    fn handle_line_shift(&self, button: &gtk::SpinButton) {
        self.options.borrow_mut().set_line_shift(button.value());
    }

    #[template_callback]
    fn handle_reverse(&self, button: &gtk::SpinButton) {
        self.options.borrow_mut().set_reverse(button.value());
    }

    #[template_callback]
    fn handle_flip(&self, button: &gtk::SpinButton) {
        self.options.borrow_mut().set_flip(button.value());
    }

    #[template_callback]
    fn handle_channel_swap(&self, button: &gtk::SpinButton) {
        self.options.borrow_mut().set_channel_swap(button.value());
    }

    #[template_callback]
    fn handle_channel_shift(&self, button: &gtk::SpinButton) {
        self.options.borrow_mut().set_channel_shift(button.value());
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
