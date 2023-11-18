use crate::service::models::models_xml::{
    XmlDisc, XmlGraphicCard, XmlLaptop, XmlProcessor, XmlScreen,
};
use crate::service::models::Laptop;

impl From<Laptop> for XmlLaptop {
    fn from(value: Laptop) -> Self {
        XmlLaptop {
            manufacturer: value.manufacturer,
            screen: Some(XmlScreen {
                size: value.screen.clone().expect("").size,
                resolution: value.screen.clone().expect("").resolution,
                r#type: value.screen.clone().expect("").r#type,
                touchscreen: value.screen.clone().expect("").touchscreen,
            }),
            processor: Some(XmlProcessor {
                name: value.processor.clone().expect("").name,
                physical_cores: value.processor.clone().expect("").physical_cores,
                clock_speed: value.processor.clone().expect("").clock_speed,
            }),
            ram: value.ram,
            disc: Some(XmlDisc {
                storage: value.disc.clone().expect("").storage,
                r#type: value.disc.clone().expect("").r#type,
            }),
            graphic_card: Some(XmlGraphicCard {
                name: value.graphic_card.clone().expect("").name,
                memory: value.graphic_card.clone().expect("").memory,
            }),
            os: value.os,
            disc_reader: value.disc_reader,
        }
    }
}
