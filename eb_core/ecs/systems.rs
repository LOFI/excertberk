use specs::{System, SystemData, Entities, ReadStorage, WriteStorage, Join};
use super::components::*;

pub struct ComputerRiderThink;

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
    tiles: Vec<Vec<Tile>>,
}

/// I guess this will return a 2d vec, one tile slice per lane, using some
/// window to limit the tile count.
fn upcoming_tiles(track: &Track, from: f32, to: f32) -> &[Vec<Tile>] {
    track.tiles.as_slice()
}

impl<'a> System<'a> for ComputerRiderThink {
    type SystemData = (WriteStorage<'a, Bike>, ReadStorage<'a, ComputerRider>, ReadStorage<'a, Bike>);

    fn run(&mut self, (mut wbikes, brains, rbikes): Self::SystemData) {
        // FIXME: need access to the other bike positions, and terrain info.

        let track = Track { tiles: vec![] }; // FIXME: bind Track to ECS as resource

        let others = rbikes.join().collect::<Vec<&Bike>>();

        for (bike, brain) in (&mut wbikes, &brains).join() {
            // ... think about what you'll do
            match bike.active_state {
                RiderState::Riding => {
                    let tiles_by_lane =
                        upcoming_tiles(&track, bike.distance, bike.distance + brain.view_distance);
                }
                _ => (),
            };
        }
    }
}
