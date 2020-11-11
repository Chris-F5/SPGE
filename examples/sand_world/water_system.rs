use crate::WriteCells;
use shred::System;
use spge::cell_storage::Join;

pub struct WaterSystem;

impl<'a> System<'a> for WaterSystem {
    type SystemData = WriteCells<'a>;
    fn run(&mut self, mut cells: Self::SystemData) {
        for (x, y) in (&cells.water.get_mask()).join() {
            if !cells.water.contains(&(x, y - 1)) && !cells.solid.contains(&(x, y - 1)) {
                cells.move_cell(&(x, y), &(x, y - 1));
            } else if !cells.water.contains(&(x - 1, y - 1))
                && !cells.solid.contains(&(x - 1, y - 1))
            {
                cells.move_cell(&(x, y), &(x - 1, y - 1));
            } else if !cells.water.contains(&(x + 1, y - 1))
                && !cells.solid.contains(&(x + 1, y - 1))
            {
                cells.move_cell(&(x, y), &(x + 1, y - 1));
            } else if !cells.water.contains(&(x - 1, y)) && !cells.solid.contains(&(x - 1, y)) {
                cells.move_cell(&(x, y), &(x - 1, y));
            } else if !cells.water.contains(&(x + 1, y)) && !cells.solid.contains(&(x + 1, y)) {
                cells.move_cell(&(x, y), &(x + 1, y));
            }
        }
    }
}
