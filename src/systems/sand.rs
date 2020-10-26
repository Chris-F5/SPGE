use crate::{
    components::cell_components::{CellColor, Sand, Solid},
    storage::cell_storage::{Join, WriteCellStorage},
};
use shred::System;

pub struct SandSystem;

impl<'a> System<'a> for SandSystem {
    type SystemData = (
        WriteCellStorage<'a, CellColor>,
        WriteCellStorage<'a, Solid>,
        WriteCellStorage<'a, Sand>,
    );
    fn run(&mut self, (mut cell_colors, mut solids, mut sands): Self::SystemData) {
        for (x, y) in (&sands).join() {
            if !solids.get(x, y - 1).is_some() {
                cell_colors.move_cell(x, y, x, y - 1);
                solids.move_cell(x, y, x, y - 1);
                sands.move_cell(x, y, x, y - 1);
            } else if !solids.get(x - 1, y - 1).is_some() {
                cell_colors.move_cell(x, y, x - 1, y - 1);
                solids.move_cell(x, y, x - 1, y - 1);
                sands.move_cell(x, y, x - 1, y - 1);
            } else if !solids.get(x + 1, y - 1).is_some() {
                cell_colors.move_cell(x, y, x + 1, y - 1);
                solids.move_cell(x, y, x + 1, y - 1);
                sands.move_cell(x, y, x + 1, y - 1);
            }
        }
    }
}
