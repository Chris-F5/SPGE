use super::{cell_to_id, CellComponent, CellMask, InnerCellStorage};
use std::marker::PhantomData;

pub struct NullStorage<C> {
    mask: CellMask,
    phantom: PhantomData<C>,
}

impl<C> Default for NullStorage<C> {
    fn default() -> NullStorage<C> {
        NullStorage {
            mask: CellMask::empty(),
            phantom: PhantomData,
        }
    }
}

impl<C> InnerCellStorage<C> for NullStorage<C> where C: CellComponent {}

impl<C> NullStorage<C>
where
    C: CellComponent,
{
    pub fn insert(&mut self, x: u32, y: u32) {
        self.mask.insert(cell_to_id(x, y));
    }
    pub fn get_mask(&self) -> &CellMask {
        &self.mask
    }
    pub fn contains(&self, x: u32, y: u32) -> bool {
        let id = cell_to_id(x, y);
        self.mask.contains(id)
    }
    pub fn move_cell(&mut self, from_id: u32, to_id: u32) {
        if self.mask.contains(from_id) {
            self.mask.insert(to_id);
            self.mask.remove(from_id);
        } else {
            self.mask.remove(to_id);
        }
    }
}
