pub struct DeltaTime(pub f32);

pub enum TileKind {
    Grass,
    Dirt,
    Hump,
    Boost,
    RampTiny,
    RampMedium,
    RampLarge,
}

pub struct Tile {
    /// what lane the tile appears in
    pub lane: u8,
    /// the x position of the left edge of the tile
    pub offset: f32,
    /// the right edge of the tile is offset + width
    pub width: f32,
    /// The kind of tile this is
    pub kind: TileKind,
}

pub struct Track {
    pub tiles: Vec<Vec<Tile>>,
}
