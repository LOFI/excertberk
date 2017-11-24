
use std::fs::File;
use std::path::Path;
use serde_json;

pub struct Track;

pub struct TileLayerData {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub data: Vec<u32>,
}

pub struct Object;

pub struct ObjectLayerData {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub objects: Vec<Object>,
}


impl Track {
    pub fn new() -> Track {
        Track
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Track {
        // - read json from file
        // - iterate over the "layers" array and extract each
        //   into the appropriate data struct (based on the type field)
        // - pack layers into a `Track`

        let reader = File::open(path).expect("read file");
        let obj: serde_json::Map<String, serde_json::Value> = serde_json::from_reader(reader)
            .expect("parse json");
        let layers = obj.get("layers").expect("get layers").as_array().unwrap();
        for layer in layers.iter() {
            let obj = layer.as_object().unwrap();

            println!("{}", obj["type"]);

        }
        Track
    }
}
