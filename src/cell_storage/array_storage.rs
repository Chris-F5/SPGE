use super::{CellComponent, InnerCellStorage};
use crate::{WORLD_HEIGHT, WORLD_WIDTH};

pub struct ArrayStorage<T>
where
    T: CellComponent,
{
    cells: [T; (WORLD_HEIGHT * WORLD_WIDTH) as usize],
}

impl<T> Default for ArrayStorage<T>
where
    T: CellComponent,
{
    fn default() -> Self {
        ArrayStorage::<T> {
            cells: [Default::default(); (WORLD_HEIGHT * WORLD_WIDTH) as usize],
        }
    }
}

impl<T> InnerCellStorage<T> for ArrayStorage<T>
where
    T: CellComponent,
{
    fn get_mut(&mut self, id: u32) -> &mut T {
        &mut self.cells[id as usize]
    }
    fn get(&self, id: u32) -> &T {
        &self.cells[id as usize]
    }
    fn insert(&mut self, id: u32, component: T) {
        self.cells[id as usize] = component;
    }
    fn remove(&mut self, _id: u32) {}
}
