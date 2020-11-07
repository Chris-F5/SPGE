use shred::{ResourceId, SystemData, World};
use spge::cell_storage::{CellComponent, NullStorage, SliceAccessStorage, WriteCellStorage};

#[derive(SystemData)]
pub struct WriteCells<'a> {
    pub color: WriteCellStorage<'a, CellColor>,
    pub sand: WriteCellStorage<'a, Sand>,
    pub solid: WriteCellStorage<'a, Solid>,
}

impl<'a> WriteCells<'a> {
    pub fn move_cell(&mut self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
        self.color.move_cell(from_x, from_y, to_x, to_y);
        self.sand.move_cell(from_x, from_y, to_x, to_y);
        self.solid.move_cell(from_x, from_y, to_x, to_y);
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
    type Storage = SliceAccessStorage<Self>;
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

#[derive(Copy, Clone)]
pub struct Sand;

impl CellComponent for Sand {
    type Storage = NullStorage<Self>;
}

impl Default for Sand {
    fn default() -> Self {
        Sand
    }
}

#[derive(Copy, Clone)]
pub struct Solid;

impl CellComponent for Solid {
    type Storage = NullStorage<Self>;
}

impl Default for Solid {
    fn default() -> Self {
        Solid
    }
}
