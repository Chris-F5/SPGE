mod components;
mod systems;

use specs::prelude::*;

fn main() {
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::PrintPosition, "debug_position_system", &[])
        .build();
    dispatcher.setup(&mut world);
    world
        .create_entity()
        .with(components::Position { x: 5.0, y: 10.0 })
        .build();
    dispatcher.dispatch(&mut world);
}
