use super::{CellComponent, CellPos, InnerCellStorage};
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
    pub fn get(&self, pos: &dyn CellPos) -> &C {
        &self.cells[pos.index() as usize]
    }
    pub fn get_mut(&mut self, pos: &dyn CellPos) -> &mut C {
        &mut self.cells[pos.index() as usize]
    }
    pub fn move_cell(&mut self, from_pos: &dyn CellPos, to_pos: &dyn CellPos) {
        let from = *self.get(from_pos);
        let to = self.get_mut(to_pos);
        *to = from;
        let from = self.get_mut(from_pos);
        *from = C::default();
    }
}
