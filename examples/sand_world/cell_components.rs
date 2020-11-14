use shred::{ResourceId, SystemData, World};
use spge::cell_storage::{
    ArrayStorage, CellComponent, CellPos, MaskedArrayStorage, NullStorage, WriteCellStorage,
};

#[derive(SystemData)]
pub struct WriteCells<'a> {
    pub color: WriteCellStorage<'a, CellColor>,
    pub powder: WriteCellStorage<'a, Powder>,
    pub solid: WriteCellStorage<'a, Solid>,
}

impl<'a> WriteCells<'a> {
    pub fn move_cell(&mut self, from_pos: &dyn CellPos, to_pos: &dyn CellPos) {
        let from_pos = &from_pos.index();
        let to_pos = &to_pos.index();
        self.color.move_cell(from_pos, to_pos);
        self.powder.move_cell(from_pos, to_pos);
        self.solid.move_cell(from_pos, to_pos);
    }
}

#[derive(Copy, Clone)]
pub struct CellColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl CellComponent for CellColor {
    type Storage = ArrayStorage<Self>;
}

impl Default for CellColor {
    fn default() -> Self {
        CellColor {
            r: 90,
            g: 0,
            b: 90,
            a: 0,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Solid;

impl CellComponent for Solid {
    type Storage = NullStorage<Self>;
}

#[derive(Copy, Clone)]
pub struct Powder {
    pub down_vel: u8,
    pub sub_cell_y_pos: u8,
}

impl Default for Powder {
    fn default() -> Powder {
        Powder {
            down_vel: 0,
            sub_cell_y_pos: 8,
        }
    }
}

impl CellComponent for Powder {
    type Storage = MaskedArrayStorage<Self>;
}
