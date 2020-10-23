use crate::components::cell_components::CellComponent;
use crate::storage::cell_storage::SliceAccessStorage;

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
