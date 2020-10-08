use super::InnerCellStorage;
use super::CHUNK_SIZE;
use crate::cell_components::CellComponent;

pub struct ArrayStorage<T>
where
    T: CellComponent,
{
    cells: [[T; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
}

impl<T> Default for ArrayStorage<T>
where
    T: CellComponent,
{
    fn default() -> Self {
        ArrayStorage::<T> {
            cells: [[Default::default(); CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
        }
    }
}

impl<T> InnerCellStorage<T> for ArrayStorage<T>
where
    T: CellComponent,
{
    fn get(&mut self, x: u32, y: u32) -> &mut T {
        &mut self.cells[x as usize][y as usize]
    }
}
