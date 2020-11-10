use super::{cell_to_id, CellComponent, InnerCellStorage};
use crate::WORLD_CELL_COUNT;

pub struct ArrayStorage<C>
where
    C: CellComponent,
{
    pub cells: [C; WORLD_CELL_COUNT as usize],
}

impl<C> Default for ArrayStorage<C>
where
    C: CellComponent,
{
    fn default() -> Self {
        ArrayStorage::<C> {
            cells: [Default::default(); WORLD_CELL_COUNT as usize],
        }
    }
}

impl<C> InnerCellStorage<C> for ArrayStorage<C> where C: CellComponent {}

impl<C> ArrayStorage<C>
where
    C: CellComponent,
{
    pub fn get_cell(&self, x: u32, y: u32) -> &C {
        let id = cell_to_id(x, y);
        self.get(id)
    }
    pub fn get_cell_mut(&mut self, x: u32, y: u32) -> &mut C {
        let id = cell_to_id(x, y);
        self.get_mut(id)
    }
    pub fn get(&self, id: u32) -> &C {
        &self.cells[id as usize]
    }
    pub fn get_mut(&mut self, id: u32) -> &mut C {
        &mut self.cells[id as usize]
    }
    pub fn move_cell(&mut self, from_id: u32, to_id: u32) {
        let from = *self.get(from_id);
        let to = self.get_mut(to_id);
        *to = from;
        let from = self.get_mut(from_id);
        *from = C::default();
    }
}
