mod array_storage;
mod cell_component_joining;

pub use self::array_storage::ArrayStorage;
pub use self::cell_component_joining::Join;

use crate::components::cell_components::CellComponent;
use crate::CHUNK_SIZE;
use hibitset::BitSet;
use shred::{Fetch, FetchMut, ResourceId, SystemData, World};
use std::ops::{Deref, DerefMut};

fn cell_to_id(x: u32, y: u32) -> u32 {
    x + y * CHUNK_SIZE
}
fn id_to_cell(id: u32) -> (u32, u32) {
    (id % CHUNK_SIZE, id / CHUNK_SIZE)
}

pub struct CellStorage<D> {
    data: D,
}

impl<D> CellStorage<D> {
    pub fn new(data: D) -> Self {
        CellStorage::<D> { data: data }
    }
}

impl<C, D> CellStorage<D>
where
    C: CellComponent,
    D: Deref<Target = MaskedCellStorage<C>>,
{
    pub fn get(&self, x: u32, y: u32) -> Option<&C> {
        let id = cell_to_id(x, y);
        if self.data.mask.contains(id) {
            Some(self.data.inner.get(id))
        } else {
            None
        }
    }
}

impl<C, D> CellStorage<D>
where
    C: CellComponent,
    D: DerefMut<Target = MaskedCellStorage<C>>,
{
    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut C> {
        let id = cell_to_id(x, y);
        if self.data.mask.contains(id) {
            Some(self.data.inner.get_mut(id))
        } else {
            None
        }
    }
    pub fn insert(&mut self, x: u32, y: u32, component: C) {
        let id = cell_to_id(x, y);
        self.data.mask.add(id);
        self.data.inner.insert(id, component);
    }
    pub fn remove(&mut self, x: u32, y: u32) {
        let id = cell_to_id(x, y);
        self.data.mask.remove(id);
    }
}

impl<'a, T, C> Join for &'a CellStorage<T>
where
    C: CellComponent,
    T: Deref<Target = MaskedCellStorage<C>>,
{
    type Mask = &'a BitSet;
    fn get_mask(self) -> &'a BitSet {
        &self.data.mask
    }
}

pub struct MaskedCellStorage<T>
where
    T: CellComponent,
{
    mask: BitSet,
    inner: T::Storage,
}

impl<T> Default for MaskedCellStorage<T>
where
    T: CellComponent,
{
    fn default() -> Self {
        MaskedCellStorage::<T> {
            mask: Default::default(),
            inner: Default::default(),
        }
    }
}

pub trait InnerCellStorage<T>: Default + Sized {
    fn get_mut(&mut self, id: u32) -> &mut T;
    fn get(&self, id: u32) -> &T;
    fn insert(&mut self, id: u32, component: T);
    fn remove(&mut self, id: u32);
}

pub type ReadCellStorage<'a, T> = CellStorage<Fetch<'a, MaskedCellStorage<T>>>;

impl<'a, T> SystemData<'a> for ReadCellStorage<'a, T>
where
    T: CellComponent,
{
    fn setup(_res: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        CellStorage::new(res.fetch())
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<MaskedCellStorage<T>>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

pub type WriteCellStorage<'a, T> = CellStorage<FetchMut<'a, MaskedCellStorage<T>>>;

impl<'a, T> SystemData<'a> for WriteCellStorage<'a, T>
where
    T: CellComponent,
{
    fn setup(_res: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        CellStorage::new(res.fetch_mut())
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<MaskedCellStorage<T>>()]
    }
}
