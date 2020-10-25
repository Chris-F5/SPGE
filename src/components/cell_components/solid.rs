use crate::components::cell_components::CellComponent;
use crate::storage::cell_storage::NullStorage;

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
