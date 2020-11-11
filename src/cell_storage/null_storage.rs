use super::{CellComponent, CellMask, CellPos, InnerCellStorage};
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
    pub fn insert(&mut self, pos: &dyn CellPos) {
        self.mask.insert(pos);
    }
    pub fn get_mask(&self) -> &CellMask {
        &self.mask
    }
    pub fn contains(&self, pos: &dyn CellPos) -> bool {
        self.mask.contains(pos)
    }
    pub fn move_cell(&mut self, from_pos: &dyn CellPos, to_pos: &dyn CellPos) {
        if self.mask.contains(from_pos) {
            self.mask.insert(to_pos);
            self.mask.remove(from_pos);
        } else {
            self.mask.remove(to_pos);
        }
    }
}
