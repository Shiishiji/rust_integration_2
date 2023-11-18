

#[derive(Debug, Default, Clone)]
pub struct ManufacturerFilter {
    pub manufacturer: String,
}

#[derive(Debug, Default, Clone)]
pub struct ScreenProportionsFilter {
    pub screen_proportion: String,
}

pub struct LaptopTraitsFilter {
    pub manufacturer: String,
    pub screen_type: String,

}

#[derive(Debug, Default, Clone)]
pub struct Laptop {
    pub manufacturer: Option<String>,
    pub screen: Option<Screen>,
    pub processor: Option<Processor>,
    pub ram: Option<String>,
    pub disc: Option<Disc>,
    pub graphic_card: Option<GraphicCard>,
    pub os: Option<String>,
    pub disc_reader: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Screen {
    pub size: Option<String>,
    pub resolution: Option<String>,
    pub r#type: Option<String>, // "type" is a reserved keyword, so we use "r#type"
    pub touchscreen: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Processor {
    pub name: Option<String>,
    pub physical_cores: Option<u8>,
    pub clock_speed: Option<i32>,
}

#[derive(Debug, Default, Clone)]
pub struct Disc {
    pub storage: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct GraphicCard {
    pub name: Option<String>,
    pub memory: Option<String>,
}