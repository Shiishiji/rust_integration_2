use crate::service::models::{LaptopTraitsFilter, ManufacturerFilter, ScreenProportionsFilter};
use crate::service::Service;
use adw::glib::subclass::InitializingObject;
use adw::prelude::Cast;
use adw::subclass::prelude::ObjectSubclass;
use adw::subclass::prelude::*;
use gtk::ffi::GtkStringObject;
use gtk::prelude::EditableExt;
use gtk::{
    glib, template_callbacks, Button, CompositeTemplate, DropDown, Entry, Label, StringObject,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/shiishiji/integration2/window.ui")]
pub struct Window {
    #[template_child]
    pub label_nr_of_laptops_by_manufacturer: TemplateChild<Label>,
    #[template_child]
    pub entry_manufacturer: TemplateChild<Entry>,

    #[template_child]
    pub label_nr_of_laptops_by_screen_proportions: TemplateChild<Label>,
    #[template_child]
    pub drop_down_screen_proportions: TemplateChild<DropDown>,

    #[template_child]
    pub entry_traits_manufacturer: TemplateChild<Entry>,
    #[template_child]
    pub entry_traits_screen_type: TemplateChild<Entry>,
    #[template_child]
    pub entry_traits_screen_size: TemplateChild<Entry>,
}

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

        let service = Service::new();
        let response = service.get_number_of_laptops_by_manufacturer(ManufacturerFilter::new(
            self.entry_manufacturer.get().text().as_str(),
        ));

        self.label_nr_of_laptops_by_manufacturer
            .set_label(&*format!("{}", response));
    }

    #[template_callback]
    pub async fn handle_nr_of_laptops_by_screen_proportions(&self, _: Button) {
        println!("handle handle_nr_of_laptops_by_screen_proportions");

        let screen_proportions = self
            .drop_down_screen_proportions
            .get()
            .selected_item()
            .expect("Cannot get selected item")
            .downcast_ref::<StringObject>()
            .expect("Cannot downcast to StringObject")
            .string();

        let service = Service::new();
        let response = service.get_number_of_laptops_by_screen_proportions(
            ScreenProportionsFilter::new(screen_proportions.as_str()),
        );

        self.label_nr_of_laptops_by_screen_proportions
            .set_label(&*format!("{}", response));
    }

    #[template_callback]
    pub async fn handle_export_laptops(&self, _: Button) {
        println!("handle handle_export_laptops");

        let filter = LaptopTraitsFilter::new(
            self.entry_traits_manufacturer.get().text().as_str(),
            self.entry_traits_screen_type.get().text().as_str(),
            self.entry_traits_screen_size.get().text().as_str(),
        );

        let service = Service::new();

        let response = service.get_laptops_by_selected_traits(filter);
        println!("{:?}", response);
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
