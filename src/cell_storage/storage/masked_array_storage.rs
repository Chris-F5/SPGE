use crate::cell_storage::{CellMask, CellPos, Tag, TagStorage};
use crate::WORLD_CELL_COUNT;

pub struct MaskedArrayStorage<T>
where
    T: Tag,
{
    mask: CellMask,
    cells: [T; WORLD_CELL_COUNT as usize],
}

impl<T> Default for MaskedArrayStorage<T>
where
    T: Tag,
{
    fn default() -> MaskedArrayStorage<T> {
        MaskedArrayStorage::<T> {
            mask: CellMask::empty(),
            cells: [unsafe { std::mem::MaybeUninit::uninit().assume_init() };
                WORLD_CELL_COUNT as usize],
        }
    }
}

impl<T> TagStorage<T> for MaskedArrayStorage<T>
where
    T: Tag,
{
    fn mask(&self) -> &CellMask {
        &self.mask
    }
    fn get_unchecked(&self, pos: &dyn CellPos) -> &T {
        &self.cells[pos.index() as usize]
    }
    fn get_mut_unchecked(&mut self, pos: &dyn CellPos) -> &mut T {
        &mut self.cells[pos.index() as usize]
    }
    fn insert(&mut self, pos: &dyn CellPos, tag: T) {
        self.cells[pos.index() as usize] = tag;
        self.mask.insert(pos);
    }
    fn remove(&mut self, pos: &dyn CellPos) {
        self.mask.remove(pos);
    }
    fn move_tag(&mut self, from_pos: &dyn CellPos, to_pos: &dyn CellPos) {
        if self.mask.contains(from_pos) {
            self.insert(to_pos, self.cells[from_pos.index() as usize]);
            self.remove(from_pos);
        } else {
            self.mask.remove(to_pos);
        }
    }
}
