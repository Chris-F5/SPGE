use shred::{ResourceId, SystemData, World};
use spge::cell_storage::{
    CellPos, CellTag, MaskedArrayStorage, NullStorage, StrictArrayStorage, Tag, TagStorage,
    TagStorageCollection, WriteTagStorage,
};

#[derive(Clone, Copy)]
pub enum Matter {
    StaticSolid,
    Powder,
}

#[derive(Clone, Copy)]
pub struct CellType {
    pub matter: Matter,
    pub color: CellColor,
}

impl Tag for CellType {
    type Storage = MaskedArrayStorage<Self>;
}

impl CellTag<CellType> for CellType {
    fn make_tag(cell: CellType) -> Option<Self> {
        Some(cell)
    }
}

#[derive(SystemData)]
pub struct WriteCells<'a> {
    pub cells: WriteTagStorage<'a, CellType>,
    pub color: WriteTagStorage<'a, CellColor>,
    pub powder: WriteTagStorage<'a, Powder>,
    pub solid: WriteTagStorage<'a, Solid>,
}

impl<'a> TagStorageCollection<CellType> for WriteCells<'a> {
    fn move_cell(&mut self, from_pos: &dyn CellPos, to_pos: &dyn CellPos) {
        let from_pos = &from_pos.index();
        let to_pos = &to_pos.index();
        self.cells.move_tag(from_pos, to_pos);
        self.color.move_tag(from_pos, to_pos);
        self.powder.move_tag(from_pos, to_pos);
        self.solid.move_tag(from_pos, to_pos);
    }
    fn remove(&mut self, pos: &dyn CellPos) {
        let pos = &pos.index();
        self.cells.remove(pos);
        self.color.remove(pos);
        self.powder.remove(pos);
        self.solid.remove(pos);
    }
    fn insert(&mut self, pos: &dyn CellPos, cell: CellType) {
        let pos = &pos.index();
        self.cells.set(pos, CellType::make_tag(cell));
        self.color.set(pos, CellColor::make_tag(cell));
        self.powder.set(pos, Powder::make_tag(cell));
        self.solid.set(pos, Solid::make_tag(cell));
    }
}

#[derive(Copy, Clone)]
pub struct CellColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for CellColor {
    fn default() -> Self {
        CellColor {
            r: 255,
            g: 0,
            b: 255,
            a: 0,
        }
    }
}

impl CellColor {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> CellColor {
        CellColor { r, g, b, a }
    }
}

impl Tag for CellColor {
    type Storage = StrictArrayStorage<Self>;
}

impl CellTag<CellType> for CellColor {
    fn make_tag(cell_type: CellType) -> Option<Self> {
        Some(cell_type.color)
    }
}

#[derive(Copy, Clone, Default)]
pub struct Solid;

impl Tag for Solid {
    type Storage = NullStorage<Self>;
}

impl CellTag<CellType> for Solid {
    fn make_tag(cell_type: CellType) -> Option<Self> {
        match cell_type.matter {
            Matter::Powder => Some(Solid),
            Matter::StaticSolid => Some(Solid),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Powder {
    pub down_vel: u8,
    pub sub_cell_y_pos: u8,
}

impl Tag for Powder {
    type Storage = MaskedArrayStorage<Self>;
}

impl CellTag<CellType> for Powder {
    fn make_tag(cell_type: CellType) -> Option<Self> {
        match cell_type.matter {
            Matter::Powder => Some(Powder {
                down_vel: 0,
                sub_cell_y_pos: 8,
            }),
            _ => None,
        }
    }
}
