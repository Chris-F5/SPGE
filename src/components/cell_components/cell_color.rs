use crate::components::cell_components::CellComponent;
use crate::storage::cell_storage::ArrayStorage;

#[derive(Copy, Clone)]
pub struct CellColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl CellComponent for CellColor {
    type Storage = ArrayStorage<Self>;
}

impl Default for CellColor {
    fn default() -> Self {
        CellColor {
            r: 255,
            g: 0,
            b: 255,
        }
    }
}
