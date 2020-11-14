use crate::WriteCells;
use shred::System;
use spge::cell_storage::Join;

pub struct PowderSystem;

impl<'a> System<'a> for PowderSystem {
    type SystemData = WriteCells<'a>;
    fn run(&mut self, mut cells: Self::SystemData) {
        for (x, y) in cells.powder.mask().join() {
            let this_powder = cells.powder.get_mut_unchecked(&(x, y));
            if !cells.solid.contains(&(x, y - 1)) {
                this_powder.down_vel += 4;
                let mut cells_to_move = this_powder.down_vel / 64;
                this_powder.sub_cell_y_pos += this_powder.down_vel % 64;
                if this_powder.sub_cell_y_pos >= 64 {
                    this_powder.sub_cell_y_pos -= 64;
                    cells_to_move += 1;
                }

                if cells_to_move != 0 {
                    let mut target_y = y;
                    for _i in 0..cells_to_move {
                        if cells.solid.contains(&(x, target_y - 1)) {
                            this_powder.down_vel = 0;
                            break;
                        } else {
                            target_y -= 1;
                        }
                    }
                    if target_y != y {
                        cells.move_cell(&(x, y), &(x, target_y));
                    }
                }
            } else if !cells.solid.contains(&(x - 1, y - 1)) {
                cells.move_cell(&(x, y), &(x - 1, y - 1));
            } else if !cells.solid.contains(&(x + 1, y - 1)) {
                cells.move_cell(&(x, y), &(x + 1, y - 1));
            }
        }
    }
}
