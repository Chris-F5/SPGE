use crate::components::cell_components::CellComponent;
use crate::storage::cell_storage::NullStorage;

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
