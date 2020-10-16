use crate::components::cell_components::CellComponent;
use crate::storage::cell_storage::ArrayStorage;

#[derive(Copy, Clone)]
pub struct TestComp {
    pub num: u8,
}

impl CellComponent for TestComp {
    type Storage = ArrayStorage<Self>;
}

impl Default for TestComp {
    fn default() -> Self {
        TestComp { num: 99 }
    }
}
