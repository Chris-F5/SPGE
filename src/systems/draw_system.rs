use crate::{
    components::cell_components::CellColor,
    storage::cell_storage::{Join, ReadCellStorage},
};
use shred::System;

pub struct DrawSystem;

impl<'a> System<'a> for DrawSystem {
    type SystemData = ReadCellStorage<'a, CellColor>;
    fn run(&mut self, cell_colors: Self::SystemData) {
        for (x, y) in (&cell_colors).join() {
            let color = cell_colors.get(x, y).unwrap();
            println!(
                "(x: {}, y: {}) (r: {}, g: {}, b: {})",
                x, y, color.r, color.g, color.b
            )
        }
    }
}
