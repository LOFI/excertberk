extern crate eb_core;

use eb_core::ecs::components::*;
use eb_core::ecs::systems::*;
use eb_core::{World, DispatcherBuilder};

fn main() {
    // The `World` is our
    // container for components
    // and other resources.

    let mut world = World::new();
    world.register::<Bike>();
    world.register::<ComputerRider>();

    // An entity may or may not contain some component.
    for i in 0..3 {
        world
            .create_entity()
            .with(Bike {
                id: i,
                lane: i,
                distance: 0.0,
                active_state: RiderState::Riding,
                angle: 0.0,
            })
            .with(ComputerRider { view_distance: 10.0 })
            .build();
    }

    // This builds a dispatcher.
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let mut dispatcher = DispatcherBuilder::new()
        .add(ComputerRiderThink, "thinker", &[])
        .build();

    // This dispatches all the systems in parallel (but blocking).
    dispatcher.dispatch(&mut world.res);
}
