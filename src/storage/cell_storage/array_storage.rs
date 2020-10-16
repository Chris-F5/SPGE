use crate::components::cell_components::CellComponent;
use crate::storage::cell_storage::CellInnerStorage;
use crate::CHUNK_SIZE;

pub struct ArrayStorage<T>
where
    T: CellComponent,
{
    cells: [T; (CHUNK_SIZE * CHUNK_SIZE) as usize],
}

impl<T> Default for ArrayStorage<T>
where
    T: CellComponent,
{
    fn default() -> Self {
        ArrayStorage::<T> {
            cells: [Default::default(); (CHUNK_SIZE * CHUNK_SIZE) as usize],
        }
    }
}

impl<T> CellInnerStorage<T> for ArrayStorage<T>
where
    T: CellComponent,
{
    fn get_mut(&mut self, id: u32) -> &mut T {
        &mut self.cells[id as usize]
    }
    fn get(&self, id: u32) -> &T {
        &self.cells[id as usize]
    }
    fn insert(&mut self, id: u32) -> &mut T {
        &mut self.cells[id as usize]
    }
    fn remove(&mut self, _id: u32) {}
}
