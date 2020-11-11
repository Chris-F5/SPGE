use crate::WriteCells;
use shred::System;
use spge::cell_storage::Join;

pub struct SandSystem;

impl<'a> System<'a> for SandSystem {
    type SystemData = WriteCells<'a>;
    fn run(&mut self, mut cells: Self::SystemData) {
        for (x, y) in (&cells.sand.get_mask()).join() {
            if !cells.solid.contains(&(x, y - 1)) {
                cells.move_cell(&(x, y), &(x, y - 1));
            } else if !cells.solid.contains(&(x - 1, y - 1)) {
                cells.move_cell(&(x, y), &(x - 1, y - 1));
            } else if !cells.solid.contains(&(x + 1, y - 1)) {
                cells.move_cell(&(x, y), &(x + 1, y - 1));
            }
        }
    }
}
