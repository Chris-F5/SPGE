use shred::{ResourceId, SystemData, World};
use spge::cell_storage::{ArrayStorage, CellComponent, CellPos, NullStorage, WriteCellStorage};

#[derive(SystemData)]
pub struct WriteCells<'a> {
    pub color: WriteCellStorage<'a, CellColor>,
    pub sand: WriteCellStorage<'a, Sand>,
    pub solid: WriteCellStorage<'a, Solid>,
    pub water: WriteCellStorage<'a, Water>,
}

impl<'a> WriteCells<'a> {
    pub fn move_cell(&mut self, from_pos: &dyn CellPos, to_pos: &dyn CellPos) {
        let from_pos = &from_pos.index();
        let to_pos = &to_pos.index();
        self.color.move_cell(from_pos, to_pos);
        self.sand.move_cell(from_pos, to_pos);
        self.solid.move_cell(from_pos, to_pos);
        self.water.move_cell(from_pos, to_pos);
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
pub struct Sand;

impl CellComponent for Sand {
    type Storage = NullStorage<Self>;
}

#[derive(Copy, Clone, Default)]
pub struct Solid;

impl CellComponent for Solid {
    type Storage = NullStorage<Self>;
}

#[derive(Copy, Clone, Default)]
pub struct Water;

impl CellComponent for Water {
    type Storage = NullStorage<Self>;
}
