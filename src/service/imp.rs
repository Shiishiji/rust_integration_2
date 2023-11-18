use crate::service::models::{
    Disc, GraphicCard, Laptop, LaptopTraitsFilter, ManufacturerFilter, Processor, Screen,
    ScreenProportionsFilter,
};
use crate::service::Service;

impl Service {
    pub fn new() -> Self {
        Service {}
    }

    pub fn get_number_of_laptops_by_manufacturer(&self, filter: ManufacturerFilter) -> i32 {
        println!("Mocking request for filters: {:?}", filter);

        5
    }

    pub fn get_number_of_laptops_by_screen_proportions(
        &self,
        filter: ScreenProportionsFilter,
    ) -> i32 {
        println!("Mocking request for filters: {:?}", filter);

        2
    }

    pub fn get_laptops_by_selected_traits(&self, filter: LaptopTraitsFilter) -> Vec<Laptop> {
        println!("Mocking request for filters: {:?}", filter);

        vec![
            Laptop {
                manufacturer: Some("Manufacturer1".to_string()),
                screen: Some(Screen {
                    size: Some("15 inches".to_string()),
                    resolution: Some("1920x1080".to_string()),
                    r#type: Some("LCD".to_string()),
                    touchscreen: Some("Yes".to_string()),
                }),
                processor: Some(Processor {
                    name: Some("Intel Core i7".to_string()),
                    physical_cores: Some(4),
                    clock_speed: Some(2_500),
                }),
                ram: Some("8 GB".to_string()),
                disc: Some(Disc {
                    storage: Some("512 GB SSD".to_string()),
                    r#type: Some("SSD".to_string()),
                }),
                graphic_card: Some(GraphicCard {
                    name: Some("NVIDIA GeForce GTX 1650".to_string()),
                    memory: Some("4 GB GDDR5".to_string()),
                }),
                os: Some("Windows 10".to_string()),
                disc_reader: Some("DVD-RW".to_string()),
            },
            Laptop {
                manufacturer: Some("Manufacturer2".to_string()),
                screen: Some(Screen {
                    size: Some("13 inches".to_string()),
                    resolution: Some("2560x1600".to_string()),
                    r#type: Some("OLED".to_string()),
                    touchscreen: Some("No".to_string()),
                }),
                processor: Some(Processor {
                    name: Some("AMD Ryzen 5".to_string()),
                    physical_cores: Some(6),
                    clock_speed: Some(3_000),
                }),
                ram: Some("16 GB".to_string()),
                disc: Some(Disc {
                    storage: Some("1 TB HDD".to_string()),
                    r#type: Some("HDD".to_string()),
                }),
                graphic_card: Some(GraphicCard {
                    name: Some("Intel UHD Graphics".to_string()),
                    memory: Some("Shared".to_string()),
                }),
                os: Some("Ubuntu 20.04".to_string()),
                disc_reader: Some("None".to_string()),
            },
            Laptop {
                manufacturer: Some("Manufacturer3".to_string()),
                screen: Some(Screen {
                    size: Some("17 inches".to_string()),
                    resolution: Some("3840x2160".to_string()),
                    r#type: Some("LED".to_string()),
                    touchscreen: Some("Yes".to_string()),
                }),
                processor: Some(Processor {
                    name: Some("Apple M1".to_string()),
                    physical_cores: Some(8),
                    clock_speed: Some(3_200),
                }),
                ram: Some("32 GB".to_string()),
                disc: Some(Disc {
                    storage: Some("1 TB SSD".to_string()),
                    r#type: Some("SSD".to_string()),
                }),
                graphic_card: Some(GraphicCard {
                    name: Some("Apple M1 GPU".to_string()),
                    memory: Some("8 GB Unified".to_string()),
                }),
                os: Some("macOS Big Sur".to_string()),
                disc_reader: Some("None".to_string()),
            },
        ]
    }
}
