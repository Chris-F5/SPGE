use crate::WriteCells;
use shred::System;
use spge::cell_storage::Join;

pub struct WaterSystem;

impl<'a> System<'a> for WaterSystem {
    type SystemData = WriteCells<'a>;
    fn run(&mut self, mut cells: Self::SystemData) {
        for (x, y) in (&cells.water).join() {
            if !cells.water.get(x, y - 1).is_some() && !cells.solid.get(x, y - 1).is_some() {
                cells.move_cell(x, y, x, y - 1);
            } else if !cells.water.get(x - 1, y - 1).is_some()
                && !cells.solid.get(x - 1, y - 1).is_some()
            {
                cells.move_cell(x, y, x - 1, y - 1);
            } else if !cells.water.get(x + 1, y - 1).is_some()
                && !cells.solid.get(x + 1, y - 1).is_some()
            {
                cells.move_cell(x, y, x + 1, y - 1);
            } else if !cells.water.get(x - 1, y).is_some() && !cells.solid.get(x - 1, y).is_some() {
                cells.move_cell(x, y, x - 1, y);
            } else if !cells.water.get(x + 1, y).is_some() && !cells.solid.get(x + 1, y).is_some() {
                cells.move_cell(x, y, x + 1, y);
            }
        }
    }
}
