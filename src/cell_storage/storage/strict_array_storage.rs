use crate::cell_storage::{CellMask, CellPos, Tag, TagStorage};
use crate::WORLD_CELL_COUNT;

pub struct StrictArrayStorage<T>
where
    T: Tag + Default,
{
    mask: CellMask,
    cells: [T; WORLD_CELL_COUNT as usize],
}

impl<T> StrictArrayStorage<T>
where
    T: Tag + Default,
{
    pub fn read_cell_array(&self) -> &[T; WORLD_CELL_COUNT as usize] {
        &self.cells
    }
}

impl<T> Default for StrictArrayStorage<T>
where
    T: Tag + Default,
{
    fn default() -> StrictArrayStorage<T> {
        StrictArrayStorage::<T> {
            mask: CellMask::empty(),
            cells: [Default::default(); WORLD_CELL_COUNT as usize],
        }
    }
}

impl<T> TagStorage<T> for StrictArrayStorage<T>
where
    T: Tag + Default,
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
        self.cells[pos.index() as usize] = Default::default();
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
