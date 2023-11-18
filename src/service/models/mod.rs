#[derive(Debug, Default, Clone)]
pub struct ManufacturerFilter {
    pub manufacturer: String,
}

impl ManufacturerFilter {
    pub fn new(manufacturer: &str) -> Self {
        ManufacturerFilter {
            manufacturer: manufacturer.to_string(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ScreenProportionsFilter {
    pub screen_proportion: String,
}

impl ScreenProportionsFilter {
    pub fn new(screen_proportions: &str) -> Self {
        ScreenProportionsFilter {
            screen_proportion: screen_proportions.to_string(),
        }
    }
}

pub struct LaptopTraitsFilter {
    pub manufacturer: String,
    pub screen_type: String,
    pub screen_size: String,
}

impl LaptopTraitsFilter {
    pub fn new(manufacturer: &str, screen_type: &str, screen_size: &str) -> Self {
        LaptopTraitsFilter {
            manufacturer: manufacturer.to_string(),
            screen_type: screen_type.to_string(),
            screen_size: screen_size.to_string(),
        }
    }
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
