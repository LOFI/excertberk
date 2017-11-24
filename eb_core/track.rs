
use std::fs::File;
use std::path::Path;
use serde_json;

pub struct Track;

pub struct TileLayerData {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub data: Option<Vec<u32>>,
}

pub struct ObjectLayerData {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub objects: Option<Vec<u32>>,
}


impl Track {
    pub fn new() -> Track {
        Track
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Track {
        // - read json from file
        // - iterate over the "layers" array and extract each
        //   into the appropriate data struct
        // - pack layers into a `Track`

        let reader = File::open(path).expect("read file");
        let json_data: serde_json::Value = serde_json::from_reader(reader).expect("parse json");

        Track
    }
}
