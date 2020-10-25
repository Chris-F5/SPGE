use crate::components::cell_components::CellComponent;
use crate::storage::cell_storage::InnerCellStorage;

pub struct NullStorage<T>(T);

impl<T> Default for NullStorage<T>
where
    T: Default,
{
    fn default() -> Self {
        NullStorage(Default::default())
    }
}

impl<T> InnerCellStorage<T> for NullStorage<T>
where
    T: CellComponent,
{
    fn get_mut(&mut self, _id: u32) -> &mut T {
        &mut self.0
    }
    fn get(&self, _id: u32) -> &T {
        &self.0
    }
    fn insert(&mut self, _id: u32, _component: T) {}
    fn remove(&mut self, _id: u32) {}
}
