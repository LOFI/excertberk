use specs::{System, Entities, ReadStorage, WriteStorage, Join, Fetch};
use super::components::*;
use types::{Track, Tile};


pub struct ComputerRiderThink;


/// I guess this will return a 2d vec, one tile slice per lane, using some
/// window to limit the tile count.
fn upcoming_tiles(track: &Track, from: f32, to: f32) -> &[Vec<Tile>] {
    track.tiles.as_slice()
}

impl<'a> System<'a> for ComputerRiderThink {
    type SystemData = (WriteStorage<'a, Bike>, ReadStorage<'a, ComputerRider>, Fetch<'a, Track>);

    fn run(&mut self, (mut bikes, brains, track): Self::SystemData) {
        // FIXME: need access to the other bike positions, and terrain info.
        // Snapshotting the bike data so we can refer to it as we visit each one.
        // Each bike will need to know where other nearby bikes are before deciding what to do.
        let others = (&bikes).join().map(|x| x.clone()).collect::<Vec<Bike>>();

        for (bike, brain) in (&mut bikes, &brains).join() {
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
