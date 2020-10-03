mod components;
mod systems;

use specs::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_add_function(a: i32, b: i32) -> i32 {
    a + b
}
/*
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
*/
