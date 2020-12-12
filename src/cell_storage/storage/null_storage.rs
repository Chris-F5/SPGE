use crate::cell_storage::{CellMask, CellPos, Tag, TagStorage};
use std::marker::PhantomData;

pub struct NullStorage<T> {
    mask: CellMask,
    phantom: PhantomData<T>,
}

impl<T> Default for NullStorage<T> {
    fn default() -> NullStorage<T> {
        NullStorage {
            mask: CellMask::empty(),
            phantom: PhantomData,
        }
    }
}

impl<T> TagStorage<T> for NullStorage<T>
where
    T: Tag,
{
    fn mask(&self) -> &CellMask {
        &self.mask
    }
    fn insert(&mut self, pos: &dyn CellPos, _tag: T) {
        self.mask.insert(pos);
    }
    fn remove(&mut self, pos: &dyn CellPos) {
        self.mask.remove(pos);
    }
    fn get_unchecked(&self, _pos: &dyn CellPos) -> &T {
        // TODO: make this return something
        panic!("cant get value from null storage");
    }
    fn get_mut_unchecked(&mut self, _pos: &dyn CellPos) -> &mut T {
        // TODO: make this return something
        panic!("cant get value from null storage");
    }
    fn move_tag(&mut self, from_pos: &dyn CellPos, to_pos: &dyn CellPos) {
        if self.mask.contains(from_pos) {
            self.mask.insert(to_pos);
            self.mask.remove(from_pos);
        } else {
            self.mask.remove(to_pos);
        }
    }
}
