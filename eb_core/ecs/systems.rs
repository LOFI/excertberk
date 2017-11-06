use specs::{System, SystemData, ReadStorage, WriteStorage, Join};
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
    tiles: Vec<Tile>,
}

/// I guess this will return a 2d vec, one tile slice per lane, using some
/// window to limit the tile count.
fn upcoming_tiles(track: &Track, from: f32, to: f32) -> Vec<&[Tile]> {
    unimplemented!();
}

impl<'a> System<'a> for ComputerRiderThink {
    type SystemData = (WriteStorage<'a, Bike>, ReadStorage<'a, ComputerRider>);

    fn run(&mut self, (mut bike, brain): Self::SystemData) {
        // FIXME: need access to the other bike positions, and terrain info.

        // Need to look at the forward terrain, from the current bike position
        // to the view distance of the brain.
        // Also check for nearby bikes that could be rear-ended, or cut off.

        let track = Track { tiles: vec![] };
        for (b, brain) in (&mut bike, &brain).join() {
            // ... think about what you'll do

            match b.active_state {
                RiderState::Riding => {
                    let tiles_by_lane =
                        upcoming_tiles(&track, b.distance, b.distance + brain.view_distance);
                }
                _ => ()
            };
        }
    }
}
