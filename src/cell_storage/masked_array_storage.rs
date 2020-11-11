use super::{CellComponent, CellMask, CellPos, InnerCellStorage};
use crate::WORLD_CELL_COUNT;

pub struct MaskedArrayStorage<C>
where
    C: CellComponent,
{
    mask: CellMask,
    cells: [C; WORLD_CELL_COUNT as usize],
}

impl<C> Default for MaskedArrayStorage<C>
where
    C: CellComponent,
{
    fn default() -> MaskedArrayStorage<C> {
        MaskedArrayStorage::<C> {
            mask: CellMask::empty(),
            cells: [Default::default(); WORLD_CELL_COUNT as usize],
        }
    }
}

impl<C> InnerCellStorage<C> for MaskedArrayStorage<C> where C: CellComponent {}

impl<C> MaskedArrayStorage<C>
where
    C: CellComponent,
{
    pub fn mask(&self) -> &CellMask {
        &self.mask
    }
    pub fn get(&self, pos: &dyn CellPos) -> Option<&C> {
        if self.mask.contains(pos) {
            Some(self.get_unchecked(pos))
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, pos: &dyn CellPos) -> Option<&mut C> {
        if self.mask.contains(pos) {
            Some(self.get_mut_unchecked(pos))
        } else {
            None
        }
    }
    pub fn get_unchecked(&self, pos: &dyn CellPos) -> &C {
        &self.cells[pos.index() as usize]
    }
    pub fn get_mut_unchecked(&mut self, pos: &dyn CellPos) -> &mut C {
        &mut self.cells[pos.index() as usize]
    }
    pub fn insert(&mut self, pos: &dyn CellPos, component: C) {
        self.cells[pos.index() as usize] = component;
        self.mask.insert(pos);
    }
    pub fn remove(&mut self, pos: &dyn CellPos) {
        self.mask.remove(pos);
    }
    pub fn move_cell(&mut self, from_pos: &dyn CellPos, to_pos: &dyn CellPos) {
        if self.mask.contains(from_pos) {
            self.insert(to_pos, *self.get_unchecked(from_pos));
            self.remove(from_pos);
        } else {
            self.mask.remove(to_pos);
        }
    }
}
