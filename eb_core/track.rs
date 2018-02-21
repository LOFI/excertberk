use std::path::Path;
use serde_json;
use serde_json::Value;
use ggez::graphics::Rect;

type GID = u32;
pub struct Object;

pub struct TrackData {
    /// how wide each tile is in pixels
    pub tile_width: u32,
    /// how tall each tile is in pixels
    pub tile_height: u32,
    /// layer data
    pub tile_layer: TileLayerData,
    /// size of the texture atlas
    pub image_size: ImageSize,
}

#[derive(Debug, Deserialize)]
pub struct TileLayerData {
    pub name: String,
    /// column count (number of tiles wide) of the layer
    pub width: u32,
    /// row count (number of tiles tall) of the layer
    pub height: u32,
    pub data: Vec<GID>,
}

pub struct ObjectLayerData {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub objects: Vec<Object>,
}

pub fn divmod(n: u32, div: u32) -> (u32, u32) {
    (n / div, n % div)
}

pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

impl TrackData {
    pub fn from_str(s: &str, image_size: ImageSize) -> TrackData {
        let obj: Value = serde_json::from_str(s).expect("parse json");

        let tile_width = obj["tilewidth"].as_u64().unwrap() as u32;
        let tile_height = obj["tileheight"].as_u64().unwrap() as u32;

        let layer: TileLayerData = serde_json::from_value(obj["layers"][0].clone()).unwrap();

        TrackData {
            tile_width,
            tile_height,
            tile_layer: layer,
            image_size,
        }
    }

    /// Converts the tile ids in the layer into rects in UV space for the image associated with
    /// the data.
    pub fn uv_rects(&self) -> Vec<Option<Rect>> {
        // the uv conversion will be dividing values by these so we can convert to f32
        // eagerly here to save repeated casts
        let img_size = (self.image_size.width as f32, self.image_size.height as f32);
        let tile_width = self.tile_width as f32;
        let tile_height = self.tile_height as f32;

        self.tile_layer
            .data
            .iter()
            .map(|gid| match *gid {
                0 => None,
                id => Some(tile_idx_to_uv_rect(id, img_size, tile_width, tile_height)),
            })
            .collect()
    }
}

/// Converts a tile index into a `Rect` region of a tileset image.
pub fn tile_idx_to_uv_rect(
    gid: u32,
    img_size: (f32, f32),
    tile_width: f32,
    tile_height: f32,
) -> Rect {
    let (img_width, img_height) = img_size;

    let cols = img_width / tile_width;
    let rows = img_height / tile_height;

    // FIXME: assumes a start GID=1, but really it could be a higher starting number.
    let (row, col) = divmod(gid - 1, cols as u32);

    debug_assert!(
        row < rows as u32,
        "gid bounds check failed, {} >= {}.",
        row,
        rows
    );

    let x = col as f32 / cols;
    let y = row as f32 / rows;

    let w = tile_width / img_width;
    let h = tile_height / img_height;

    //    println!(
    //        "grid size={:?}, col/row={:?}, uv pos={:?}, uv size={:?}, tile size={:?}",
    //        (cols, rows),
    //        (col, row),
    //        (x, y),
    //        (w, h),
    //        (tile_width, tile_height),
    //    );

    // with all the dimensions converted to UV space, they should
    // all be between 0 and 1.
    debug_assert!(x <= 1. && x >= 0.);
    debug_assert!(y <= 1. && y >= 0.);
    debug_assert!(w <= 1. && w >= 0.);
    debug_assert!(h <= 1. && h >= 0.);

    Rect::new(x, y, w, h)
}

#[cfg(test)]
mod tests {
    use super::*;

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
