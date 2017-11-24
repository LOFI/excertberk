extern crate eb_core;

use eb_core::ecs::components::*;
use eb_core::ecs::systems::*;
use eb_core::{World, DispatcherBuilder};
use eb_core::types::{Track, DeltaTime};

fn main() {

    let mut world = World::new();
    world.register::<Bike>();
    world.register::<ComputerRider>();
    world.add_resource(Track { tiles: vec![] });
    world.add_resource(DeltaTime(0.0));

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

    let mut dispatcher = DispatcherBuilder::new()
        .add(ComputerRiderThink, "thinker", &[])
        .build();

    dispatcher.dispatch(&mut world.res);
}
