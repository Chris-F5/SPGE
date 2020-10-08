use super::CellComponent;
use crate::cell_storage::ArrayStorage;

#[derive(Copy, Clone)]
pub struct Sand {
    pub color: (u8, u8, u8),
}
impl Default for Sand {
    fn default() -> Self {
        Sand {
            color: (255, 0, 255),
        }
    }
}

impl CellComponent for Sand {
    type CellStorage = ArrayStorage<Self>;
}
