use crate::components::Position;
use specs::prelude::*;
use specs::Join;

pub struct PrintPosition;
impl<'a> System<'a> for PrintPosition {
    type SystemData = ReadStorage<'a, Position>;
    fn run(&mut self, positions: Self::SystemData) {
        for position in (&positions).join() {
            println!("{:?}", position);
        }
    }
}
