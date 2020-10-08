mod array_storage;

pub use array_storage::ArrayStorage;

use crate::cell_components::CellComponent;
use specs::join::Join;
use specs::BitSet;

pub const CHUNK_SIZE: u32 = 64;

pub trait InnerCellStorage<T>: Default
where
    T: CellComponent,
{
    fn get(&mut self, x: u32, y: u32) -> &mut T;
}

pub struct MaskedChunkStorage<T>
where
    T: CellComponent,
{
    mask: BitSet,
    inner_storage: T::CellStorage,
}

impl<T> Default for MaskedChunkStorage<T>
where
    T: CellComponent,
{
    fn default() -> Self {
        MaskedChunkStorage {
            mask: Default::default(),
            inner_storage: Default::default(),
        }
    }
}

impl<'a, T> Join for &'a mut MaskedChunkStorage<T>
where
    T: CellComponent,
{
    type Type = &'a mut T;
    type Value = &'a mut T::CellStorage;
    type Mask = &'a BitSet;
    unsafe fn open(self) -> (Self::Mask, Self::Value) {
        (&self.mask, &mut self.inner_storage)
    }
    unsafe fn get(v: &mut Self::Value, i: u32) -> &'b mut T {
        v.get(i / CHUNK_SIZE, i % CHUNK_SIZE)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn array_storage() {
        use super::ArrayStorage;
        use super::InnerCellStorage;
        use crate::cell_components::Sand;

        let mut storage: ArrayStorage<Sand> = Default::default();
        let mut t = storage.get(5, 5);
        t = Default::default();
    }
}
