use specs::{System, SystemData, ReadStorage, WriteStorage, Join};
use super::components::*;

pub struct ComputerRiderThink;

impl<'a> System<'a> for ComputerRiderThink {
    type SystemData = (WriteStorage<'a, Bike>, ReadStorage<'a, ComputerRider>);

    fn run(&mut self, (mut bike, brain): Self::SystemData) {
        // FIXME: need access to the other bike positions, and terrain info.

        // Need to look at the forward terrain, from the current bike position
        // to the view distance of the brain.
        // Also check for nearby bikes that could be rear-ended, or cut off.

        for (b, brain) in (&mut bike, &brain).join() {
            // ... think about what you'll do
        }
    }
}
