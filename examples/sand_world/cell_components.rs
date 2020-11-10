use shred::{ResourceId, SystemData, World};
use spge::cell_storage::{cell_to_id, ArrayStorage, CellComponent, NullStorage, WriteCellStorage};

#[derive(SystemData)]
pub struct WriteCells<'a> {
    pub color: WriteCellStorage<'a, CellColor>,
    pub sand: WriteCellStorage<'a, Sand>,
    pub solid: WriteCellStorage<'a, Solid>,
    pub water: WriteCellStorage<'a, Water>,
}

impl<'a> WriteCells<'a> {
    pub fn move_cell(&mut self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
        let from_id = cell_to_id(from_x, from_y);
        let to_id = cell_to_id(to_x, to_y);
        self.color.move_cell(from_id, to_id);
        self.sand.move_cell(from_id, to_id);
        self.solid.move_cell(from_id, to_id);
        self.water.move_cell(from_id, to_id);
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
