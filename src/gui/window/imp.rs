use adw::glib::subclass::InitializingObject;
use adw::subclass::prelude::ObjectSubclass;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/shiishiji/integration2/window.ui")]
pub struct Window {}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "AppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        // klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl Window {}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_callbacks();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
