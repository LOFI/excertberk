
use std::fs::File;
use std::path::Path;
use serde_json;
use serde_json::Value;
use ggez::graphics::Rect;

pub struct Image;
pub struct Object;

pub struct Track {
    tile_width: u32,
    tile_height: u32,
    tile_layer: TileLayerData,
    image: Image,
}


#[derive(Debug, Deserialize)]
pub struct TileLayerData {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub data: Vec<u32>,
}


pub struct ObjectLayerData {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub objects: Vec<Object>,
}


fn div_mod(n: u32, div: u32) -> (u32, u32) {
    (n / div, n % div)
}


impl Track {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Track {

        let reader = File::open(path).expect("read file");

        let obj: Value = serde_json::from_reader(reader).expect("parse json");

        let tile_width = obj["tilewidth"].as_u64().unwrap() as u32;
        let tile_height = obj["tileheight"].as_u64().unwrap() as u32;

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
/// Panics when gid is out of bounds.
fn gid_to_rect(
    gid: u32,
    tile_width: u32,
    tile_height: u32,
    tileset_width: u32,
    tileset_height: u32,
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

    #[test]
    fn test_gid_0_to_rect() {
        assert_eq!(gid_to_rect(0, 16, 16, 3, 3), Rect::new(0., 0., 16., 16.));
    }

    #[test]
    fn test_gid_1_to_rect() {
        assert_eq!(gid_to_rect(1, 16, 16, 3, 3), Rect::new(16., 0., 16., 16.));
    }

    #[test]
    fn test_gid_2_to_rect() {
        assert_eq!(gid_to_rect(2, 16, 16, 3, 3), Rect::new(32., 0., 16., 16.));
    }

    #[test]
    fn test_gid_3_to_rect() {
        assert_eq!(gid_to_rect(3, 16, 16, 3, 3), Rect::new(0., 16., 16., 16.));
    }

    #[test]
    fn test_gid_4_to_rect() {
        assert_eq!(gid_to_rect(4, 16, 16, 3, 3), Rect::new(16., 16., 16., 16.));
    }

    #[test]
    fn test_gid_5_to_rect() {
        assert_eq!(gid_to_rect(5, 16, 16, 3, 3), Rect::new(32., 16., 16., 16.));
    }

    #[test]
    fn test_gid_6_to_rect() {
        assert_eq!(gid_to_rect(6, 16, 16, 3, 3), Rect::new(0., 32., 16., 16.));
    }

    #[test]
    fn test_gid_7_to_rect() {
        assert_eq!(gid_to_rect(7, 16, 16, 3, 3), Rect::new(16., 32., 16., 16.));
    }

    #[test]
    fn test_gid_8_to_rect() {
        assert_eq!(gid_to_rect(8, 16, 16, 3, 3), Rect::new(32., 32., 16., 16.));
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_gid() {
        gid_to_rect(9, 16, 16, 3, 3);
    }

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
