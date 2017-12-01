
use std::fs::File;
use std::path::Path;
use serde_json;
use serde_json::Value;
use ggez::graphics::Rect;


type GID = u32;
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
    pub data: Vec<GID>,
}


pub struct ObjectLayerData {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub objects: Vec<Object>,
}


fn divmod(n: u32, div: u32) -> (u32, u32) {
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


/// Eh, this will position a rect using the bottom left corner instead of the center).
fn place_rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect::new(x + (w / 2.), y + (h / 2.), w, h)
}


/// Converts a tile index into a `Rect` region of a tileset image.
///
/// The `tileset_width` and `tileset_height` params are the number of cells (not
/// pixels) that make up the entire image.
/// The image size is therefore:
///   `(tileset_width * tile_width) * (tileset_height * tile_height)`
/// Panics when gid is out of bounds.
fn tile_idx_to_rect(
    idx: u32,
    tile_width: u32,
    tile_height: u32,
    tileset_width: u32,
    tileset_height: u32,
) -> Rect {
    let (row, col) = divmod(idx, tileset_width);
    assert!(
        row < tileset_height,
        "gid bounds check failed, row too large."
    );
    place_rect(
        (col * tile_width) as f32,
        ((tileset_height - row - 1) * tile_height) as f32,
        tile_width as f32,
        tile_height as f32,
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_rect_0_0() {
        let r = place_rect(0., 0., 16., 16.);
        assert_eq!(r.top(), 16.);
        assert_eq!(r.bottom(), 0.);
        assert_eq!(r.left(), 0.);
        assert_eq!(r.right(), 16.);
    }


    #[test]
    fn test_place_rect_neg_10_0() {
        let r = place_rect(-10., 0., 16., 16.);
        assert_eq!(r.top(), 16.);
        assert_eq!(r.bottom(), 0.);
        assert_eq!(r.left(), -10.);
        assert_eq!(r.right(), 6.);
    }

    #[test]
    fn test_place_rect_10_0() {
        let r = place_rect(10., 0., 16., 16.);
        assert_eq!(r.top(), 16.);
        assert_eq!(r.bottom(), 0.);
        assert_eq!(r.left(), 10.);
        assert_eq!(r.right(), 26.);
    }

    #[test]
    fn test_place_rect_0_10() {
        let r = place_rect(0., 10., 16., 16.);
        assert_eq!(r.top(), 26.);
        assert_eq!(r.bottom(), 10.);
        assert_eq!(r.left(), 00.);
        assert_eq!(r.right(), 16.);
    }

    #[test]
    fn test_gid_0_to_rect() {
        assert_eq!(
            tile_idx_to_rect(0, 16, 16, 3, 3),
            place_rect(0., 32., 16., 16.)
        );
    }

    #[test]
    fn test_gid_1_to_rect() {
        assert_eq!(
            tile_idx_to_rect(1, 16, 16, 3, 3),
            place_rect(16., 32., 16., 16.)
        );
    }

    #[test]
    fn test_gid_2_to_rect() {
        assert_eq!(
            tile_idx_to_rect(2, 16, 16, 3, 3),
            place_rect(32., 32., 16., 16.)
        );
    }

    #[test]
    fn test_gid_3_to_rect() {
        assert_eq!(
            tile_idx_to_rect(3, 16, 16, 3, 3),
            place_rect(0., 16., 16., 16.)
        );
    }

    #[test]
    fn test_gid_4_to_rect() {
        assert_eq!(
            tile_idx_to_rect(4, 16, 16, 3, 3),
            place_rect(16., 16., 16., 16.)
        );
    }

    #[test]
    fn test_gid_5_to_rect() {
        assert_eq!(
            tile_idx_to_rect(5, 16, 16, 3, 3),
            place_rect(32., 16., 16., 16.)
        );
    }

    #[test]
    fn test_gid_6_to_rect() {
        assert_eq!(
            tile_idx_to_rect(6, 16, 16, 3, 3),
            place_rect(0., 0., 16., 16.)
        );
    }

    #[test]
    fn test_gid_7_to_rect() {
        assert_eq!(
            tile_idx_to_rect(7, 16, 16, 3, 3),
            place_rect(16., 0., 16., 16.)
        );
    }

    #[test]
    fn test_gid_8_to_rect() {
        assert_eq!(
            tile_idx_to_rect(8, 16, 16, 3, 3),
            place_rect(32., 0., 16., 16.)
        );
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_gid() {
        tile_idx_to_rect(9, 16, 16, 3, 3);
    }

    // I probably don't need these divmod tests, but it is very late...
    #[test]
    fn test_divmod_3_2() {
        assert_eq!(divmod(3, 2), (1, 1));
    }

    #[test]
    fn test_divmod_5_5() {
        assert_eq!(divmod(5, 5), (1, 0));
    }

    #[test]
    fn test_divmod_5_10() {
        assert_eq!(divmod(5, 10), (0, 5));
    }

    #[test]
    fn test_divmod_5_2() {
        assert_eq!(divmod(5, 2), (2, 1));
    }

}
