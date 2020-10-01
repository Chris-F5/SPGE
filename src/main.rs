use specs::prelude::*;
use specs::Join;

#[derive(Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct DebugPositionSystem;
impl<'a> System<'a> for DebugPositionSystem {
    type SystemData = ReadStorage<'a, Position>;
    fn run(&mut self, positions: Self::SystemData) {
        for position in (&positions).join() {
            println!("{:?}", position);
        }
    }
}

fn main() {
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(DebugPositionSystem, "debug_position_system", &[])
        .build();
    dispatcher.setup(&mut world);
    world
        .create_entity()
        .with(Position { x: 5.0, y: 10.0 })
        .build();
    dispatcher.dispatch(&mut world);
}
