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
            let color = *cell_colors.get(x, y).unwrap();
            if !solids.get(x, y - 1).is_some() {
                cell_colors.insert(x, y - 1, color);
                cell_colors.remove(x, y);
                solids.insert(x, y - 1, Solid);
                solids.remove(x, y);
                sands.insert(x, y - 1, Sand);
                sands.remove(x, y);
            } else if !solids.get(x - 1, y - 1).is_some() {
                cell_colors.insert(x - 1, y - 1, color);
                cell_colors.remove(x, y);
                solids.insert(x - 1, y - 1, Solid);
                solids.remove(x, y);
                sands.insert(x - 1, y - 1, Sand);
                sands.remove(x, y);
            } else if !solids.get(x + 1, y - 1).is_some() {
                cell_colors.insert(x + 1, y - 1, color);
                cell_colors.remove(x, y);
                solids.insert(x + 1, y - 1, Solid);
                solids.remove(x, y);
                sands.insert(x + 1, y - 1, Sand);
                sands.remove(x, y);
            }
        }
    }
}
