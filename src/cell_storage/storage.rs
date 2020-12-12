mod masked_array_storage;
mod null_storage;
mod strict_array_storage;
pub use masked_array_storage::MaskedArrayStorage;
pub use null_storage::NullStorage;
pub use strict_array_storage::StrictArrayStorage;

use crate::cell_storage::{CellMask, CellPos, Tag};

pub trait TagStorage<T>: Default + Sized
where
    T: Tag,
{
    fn mask(&self) -> &CellMask;
    fn insert(&mut self, pos: &dyn CellPos, tag: T);
    fn remove(&mut self, pos: &dyn CellPos);
    fn get(&self, pos: &dyn CellPos) -> Option<&T> {
        if self.mask().contains(pos) {
            Some(self.get_unchecked(pos))
        } else {
            None
        }
    }
    fn get_mut(&mut self, pos: &dyn CellPos) -> Option<&mut T> {
        if self.mask().contains(pos) {
            Some(self.get_mut_unchecked(pos))
        } else {
            None
        }
    }
    fn get_unchecked(&self, pos: &dyn CellPos) -> &T;
    fn get_mut_unchecked(&mut self, pos: &dyn CellPos) -> &mut T;
    fn move_tag(&mut self, from: &dyn CellPos, to: &dyn CellPos);
    fn set(&mut self, pos: &dyn CellPos, value: Option<T>) {
        if let Some(tag) = value {
            self.insert(pos, tag)
        } else {
            self.remove(pos);
        }
    }
}
