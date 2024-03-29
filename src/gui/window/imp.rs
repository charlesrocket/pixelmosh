use adw::{prelude::*, subclass::prelude::*};
use glib::subclass::InitializingObject;
use gtk::{gio, glib, Button, CompositeTemplate, Entry, Label, SpinButton, Stack};
use png::ColorType;

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
    #[template_child]
    pub btn_rewind: TemplateChild<Button>,
    #[template_child]
    pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    pub dialog_open: gtk::FileDialog,
    pub dialog_save: gtk::FileDialog,
    pub image: RefCell<Image>,
    #[template_child]
    pub picture: TemplateChild<gtk::Picture>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
    #[template_child]
    pub seed: TemplateChild<Entry>,
    #[template_child]
    pub color_type: TemplateChild<Label>,
    pub style_manager: adw::StyleManager,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "PixelmoshWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn new() -> Self {
        let filters = gio::ListStore::new::<gtk::FileFilter>();
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
            .modal(true)
            .build();

        let dialog_save = gtk::FileDialog::builder()
            .title("Save file")
            .accept_label("Save")
            .initial_name("moshed.png")
            .filters(&filters)
            .modal(true)
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
            btn_rewind: TemplateChild::default(),
            toast_overlay: TemplateChild::default(),
            dialog_open,
            dialog_save,
            image: RefCell::new(Image::default()),
            picture: TemplateChild::default(),
            stack: TemplateChild::default(),
            seed: TemplateChild::default(),
            color_type: TemplateChild::default(),
            style_manager: adw::StyleManager::default(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
        klass.install_action(
            "win.mosh-file",
            None,
            |win, _action_name, _action_target| {
                win.mosh();
                win.set_rewind_button();
            },
        );

        klass.install_action(
            "win.mosh-rewind",
            None,
            |win, _action_name, _action_target| {
                win.mosh_rewind();
                win.set_rewind_button();
            },
        );

        klass.install_action_async(
            "win.open-file",
            None,
            |win, _action_name, _action_target| async move {
                let dialog = &win.imp().dialog_open;
                if let Ok(file) = dialog.open_future(Some(&win)).await {
                    win.load_file(&file);

                    let color_type = match win.imp().image.borrow_mut().core.data.color_type {
                        ColorType::Grayscale => "Grayscale",
                        ColorType::Indexed => "Indexed",
                        ColorType::GrayscaleAlpha => "Grayscale/A",
                        ColorType::Rgb => "RGB",
                        ColorType::Rgba => "RGB/A",
                    };

                    if let Some(file_path) = file.path() {
                        win.set_title(file_path.file_name().and_then(|name| name.to_str()))
                    }

                    win.set_color_type(color_type);
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
                        win.show_message(&format!("Error saving the image: {}", error), 0);
                    } else {
                        win.show_message("DONE", 3);
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
        obj.setup_buttons();
        #[cfg(debug_assertions)]
        obj.setup_debug();
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
