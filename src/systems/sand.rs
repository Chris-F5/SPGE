use crate::{
    components::cell_components::CellColor,
    storage::cell_storage::{Join, WriteCellStorage},
};
use shred::System;

pub struct SandSystem;

impl<'a> System<'a> for SandSystem {
    type SystemData = WriteCellStorage<'a, CellColor>;
    fn run(&mut self, mut cell_colors: Self::SystemData) {
        for (x, y) in (&cell_colors).join() {
            if y > 0 && !cell_colors.get(x, y - 1).is_some() {
                let cell = *cell_colors.get(x, y).unwrap();
                cell_colors.insert(x, y - 1, cell);
                cell_colors.remove(x, y);
            } else if y > 0 && !cell_colors.get(x - 1, y - 1).is_some() {
                let cell = *cell_colors.get(x, y).unwrap();
                cell_colors.insert(x - 1, y - 1, cell);
                cell_colors.remove(x, y);
            } else if y > 0 && !cell_colors.get(x + 1, y - 1).is_some() {
                let cell = *cell_colors.get(x, y).unwrap();
                cell_colors.insert(x + 1, y - 1, cell);
                cell_colors.remove(x, y);
            }
        }
    }
}
