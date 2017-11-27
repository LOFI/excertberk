
use std::fs::File;
use std::path::Path;
use serde_json;
use serde_json::Value;
use ggez::graphics::Rect;

pub struct Image;
pub struct Object;

pub struct Track {
    tile_width: u64,
    tile_height: u64,
    tile_layer: TileLayerData,
    image: Image,
}


#[derive(Debug, Deserialize)]
pub struct TileLayerData {
    pub name: String,
    pub height: u64,
    pub width: u64,
    pub data: Vec<u64>,
}


pub struct ObjectLayerData {
    pub name: String,
    pub height: u64,
    pub width: u64,
    pub objects: Vec<Object>,
}


fn div_mod(n: u64, div: u64) -> (u64, u64) {
    let times = n / div;
    (times, n - div * times)
}


impl Track {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Track {

        let reader = File::open(path).expect("read file");

        let obj: Value = serde_json::from_reader(reader).expect("parse json");

        let tile_height = obj["tileheight"].as_u64().unwrap();
        let tile_width = obj["tilewidth"].as_u64().unwrap();

        let layer: TileLayerData = serde_json::from_value(obj["layers"][0].clone()).unwrap();

        println!("{:?}: {}", layer, ::std::mem::size_of::<TileLayerData>());

        Track {
            tile_width,
            tile_height,
            tile_layer: layer,
            image: Image, // may need to box this, depending on the size
        }
    }
}



/// Converts a GID into a `Rect` region of a tileset image.
///
/// The `tileset_width` and `tileset_height` params are the number of cells (not
/// pixels) that make up the entire image.
/// The image size is therefore:
///   `(tileset_width * tile_width) * (tileset_height * tile_height)`
fn gid_to_rect(
    gid: u64,
    tile_width: u64,
    tile_height: u64,
    tileset_width: u64,
    tileset_height: u64,
) -> Rect {
    let (row, col) = div_mod(gid, tileset_width);

    assert!(
        row < tileset_height,
        "gid bounds check failed, row too large."
    );

    Rect::new(
        (col * tile_width) as f32,
        (row * tile_height) as f32,
        tile_width as f32,
        tile_height as f32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // I probably don't need these divmod tests, but it is very late...
    #[test]
    fn test_div_mod_3_2() {
        assert_eq!(div_mod(3, 2), (1, 1));
    }

    #[test]
    fn test_div_mod_5_5() {
        assert_eq!(div_mod(5, 5), (1, 0));
    }

    #[test]
    fn test_div_mod_5_10() {
        assert_eq!(div_mod(5, 10), (0, 5));
    }

    #[test]
    fn test_div_mod_5_2() {
        assert_eq!(div_mod(5, 2), (2, 1));
    }

}
