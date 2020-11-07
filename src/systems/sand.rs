use crate::storage::cell_storage::{Join, WriteCells};
use shred::System;

pub struct SandSystem;

impl<'a> System<'a> for SandSystem {
    type SystemData = WriteCells<'a>;
    fn run(&mut self, mut cells: Self::SystemData) {
        for (x, y) in (&cells.sand).join() {
            if !cells.solid.get(x, y - 1).is_some() {
                cells.move_cell(x, y, x, y - 1);
            } else if !cells.solid.get(x - 1, y - 1).is_some() {
                cells.move_cell(x, y, x - 1, y - 1);
            } else if !cells.solid.get(x + 1, y - 1).is_some() {
                cells.move_cell(x, y, x + 1, y - 1);
            }
        }
    }
}
