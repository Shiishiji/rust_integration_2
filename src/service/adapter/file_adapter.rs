use crate::service::models::models_xml::{XmlLaptop, XmlLaptops};
use crate::service::models::Laptops;
use std::fs::File;
use std::io::Write;

pub struct FileAdapter {}

impl FileAdapter {
    pub fn save_to_xml(&self, filename: &str, data: Laptops) {
        let mut file = File::create(filename).expect("Cannot create file.");

        let mut i = 0;
        let mut vec_laptops = vec![];
        for laptop in data.laptops {
            let xml_laptop = XmlLaptop::from(laptop);
            vec_laptops.push(xml_laptop);
            i += 1;
        }

        let xml_laptops = XmlLaptops {
            laptop: vec_laptops,
        };

        let xml = yaserde::ser::to_string(&xml_laptops).expect("Unable to serialize");

        file.write(xml.as_ref()).expect("Unable to write");
        println!("Saved {} records to {}.", i, filename);
    }
}
