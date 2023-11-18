use adw::glib::subclass::InitializingObject;
use adw::subclass::prelude::ObjectSubclass;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, template_callbacks, Button};

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
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[template_callbacks]
impl Window {

    #[template_callback]
    pub async fn handle_nr_of_laptops_by_manufacturer(&self, _: Button) {
        println!("handle handle_nr_of_laptops_by_manufacturer");
    }

    #[template_callback]
    pub async fn handle_nr_of_laptops_by_screen_proportions(&self, _: Button) {
        println!("handle handle_nr_of_laptops_by_screen_proportions");
    }

    #[template_callback]
    pub async fn handle_export_laptops(&self, _: Button) {
        println!("handle handle_export_laptops");
    }

}

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
