use specs::{Component, HashMapStorage};

/// This is a computer controlled player. Eventually we'll
/// have fields on it to tune factors to customize the
/// decision making process.
pub struct ComputerRider {
    pub view_distance: f32,
}

impl Component for ComputerRider {
    type Storage = HashMapStorage<Self>;
}


/// This is a human/input controlled player.
pub struct HumanRider;

impl Component for HumanRider {
    type Storage = HashMapStorage<Self>;
}




/// This is a bike. Unsure if it also covers the rider (which may be a
/// separate entity).
pub struct Bike {
    /// which lane is the bike in
    pub lane: u8,
    /// distance along the track
    pub distance: f32,
}

impl Component for Bike {
    type Storage = HashMapStorage<Self>;
}
