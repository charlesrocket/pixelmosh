use adw::{prelude::*, subclass::prelude::*};
use glib::subclass::InitializingObject;
use gtk::{gio, glib, CompositeTemplate, Entry, SpinButton, Stack};

use std::cell::RefCell;

use crate::gui::window::Image;

#[derive(CompositeTemplate)]
#[template(resource = "/org/hellbyte/pixelmosh/window.ui")]
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
    pub dialog_open: gtk::FileDialog,
    pub dialog_save: gtk::FileDialog,
    pub image: RefCell<Image>,
    #[template_child]
    pub picture: TemplateChild<gtk::Picture>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
    #[template_child]
    pub seed: TemplateChild<Entry>,
    pub style_manager: adw::StyleManager,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "PixelmoshWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn new() -> Self {
        let filters = gio::ListStore::new(gtk::FileFilter::static_type());
        let png_filter = gtk::FileFilter::new();

        png_filter.add_mime_type("image/png");
        png_filter.set_name(Some("PNG"));

        if !cfg!(target_os = "macos") {
            filters.append(&png_filter);
        }

        let dialog_open = gtk::FileDialog::builder()
            .title("Open file")
            .accept_label("Open")
            .filters(&filters)
            .build();

        let dialog_save = gtk::FileDialog::builder()
            .title("Save file")
            .accept_label("Save")
            .build();

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
            picture: TemplateChild::default(),
            stack: TemplateChild::default(),
            seed: TemplateChild::default(),
            style_manager: adw::StyleManager::default(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
        klass.install_action_async(
            "win.mosh-file",
            None,
            |win, _action_name, _action_target| async move {
                win.mosh();
            },
        );

        klass.install_action_async(
            "win.open-file",
            None,
            |win, _action_name, _action_target| async move {
                let dialog = &win.imp().dialog_open;
                if let Ok(file) = dialog.open_future(Some(&win)).await {
                    win.load_file(&file);
                }
            },
        );

        klass.install_action_async(
            "win.save-file",
            None,
            |win, _action_name, _action_target| async move {
                let dialog = &win.imp().dialog_save;
                if let Ok(file) = dialog.save_future(Some(&win)).await {
                    if let Err(error) = win.save_file(&file) {
                        let error_dialog = gtk::AlertDialog::builder()
                            .modal(true)
                            .message(format!("Error saving the image: {}", error))
                            .build();

                        error_dialog.show(Some(&win));
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
        self.image.borrow_mut().set_min_rate(button.value() as u16);
    }

    #[template_callback]
    fn handle_max_rate(&self, button: &gtk::SpinButton) {
        self.image.borrow_mut().set_max_rate(button.value() as u16);
    }

    #[template_callback]
    fn handle_pixelation(&self, button: &gtk::SpinButton) {
        self.image.borrow_mut().set_pixelation(button.value() as u8);
    }

    #[template_callback]
    fn handle_line_shift(&self, button: &gtk::SpinButton) {
        self.image.borrow_mut().set_line_shift(button.value());
    }

    #[template_callback]
    fn handle_reverse(&self, button: &gtk::SpinButton) {
        self.image.borrow_mut().set_reverse(button.value());
    }

    #[template_callback]
    fn handle_flip(&self, button: &gtk::SpinButton) {
        self.image.borrow_mut().set_flip(button.value());
    }

    #[template_callback]
    fn handle_channel_swap(&self, button: &gtk::SpinButton) {
        self.image.borrow_mut().set_channel_swap(button.value());
    }

    #[template_callback]
    fn handle_channel_shift(&self, button: &gtk::SpinButton) {
        self.image.borrow_mut().set_channel_shift(button.value());
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
