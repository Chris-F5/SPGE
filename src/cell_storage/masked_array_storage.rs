use super::{cell_to_id, CellComponent, CellMask, InnerCellStorage};
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
    pub fn get(&self, x: u32, y: u32) -> Option<&C> {
        let id = cell_to_id(x, y);
        if self.mask.contains(id) {
            Some(self.get_unchecked(id))
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut C> {
        let id = cell_to_id(x, y);
        if self.mask.contains(id) {
            Some(self.get_mut_unchecked(id))
        } else {
            None
        }
    }
    pub fn get_unchecked(&self, id: u32) -> &C {
        &self.cells[id as usize]
    }
    pub fn get_mut_unchecked(&mut self, id: u32) -> &mut C {
        &mut self.cells[id as usize]
    }
    pub fn insert(&mut self, id: u32, component: C) {
        self.cells[id as usize] = component;
        self.mask.insert(id);
    }
    pub fn remove(&mut self, id: u32) {
        self.mask.remove(id);
    }
    pub fn move_cell(&mut self, from: u32, to: u32) {
        if self.mask.contains(from) {
            self.insert(to, *self.get_unchecked(from));
            self.remove(from);
        } else {
            self.mask.remove(to);
        }
    }
}
