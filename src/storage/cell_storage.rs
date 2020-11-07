mod array_storage;
mod cell_component_joining;
mod null_storage;
mod slice_access_storage;

pub use self::array_storage::ArrayStorage;
pub use self::cell_component_joining::Join;
pub use self::null_storage::NullStorage;
pub use self::slice_access_storage::SliceAccessStorage;

use crate::components::cell_components::CellComponent;
use crate::WORLD_WIDTH;
use hibitset::BitSet;
use shred::{Fetch, FetchMut, ResourceId, SystemData, World};
use std::ops::{Deref, DerefMut};

fn cell_to_id(x: u32, y: u32) -> u32 {
    x + y * WORLD_WIDTH
}
fn id_to_cell(id: u32) -> (u32, u32) {
    (id % WORLD_WIDTH, id / WORLD_WIDTH)
}

pub struct CellStorage<D> {
    pub data: D,
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
    pub fn get_unchecked(&self, x: u32, y: u32) -> &C {
        let id = cell_to_id(x, y);
        self.data.inner.get(id)
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
    pub fn get_mut_unchecked(&mut self, x: u32, y: u32) -> &mut C {
        let id = cell_to_id(x, y);
        self.data.inner.get_mut(id)
    }
    pub fn insert(&mut self, x: u32, y: u32, component: C) {
        let id = cell_to_id(x, y);
        self.data.inner.insert(id, component);
        self.data.mask.add(id);
    }
    pub fn remove(&mut self, x: u32, y: u32) {
        let id = cell_to_id(x, y);
        self.data.inner.remove(id);
        self.data.mask.remove(id);
    }
    pub fn move_cell(&mut self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
        let from_id = cell_to_id(from_x, from_y);
        let to_id = cell_to_id(to_x, to_y);
        self.data.inner.move_cell(from_id, to_id);
        self.data.mask.remove(from_id);
        self.data.mask.add(to_id);
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
    pub inner: T::Storage,
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

pub trait InnerCellStorage<T>: Default + Sized
where
    T: CellComponent,
{
    fn get_mut(&mut self, id: u32) -> &mut T;
    fn get(&self, id: u32) -> &T;
    fn insert(&mut self, id: u32, component: T);
    fn remove(&mut self, id: u32);
    fn move_cell(&mut self, from_id: u32, to_id: u32) {
        let from_cell = *self.get(from_id);
        self.remove(from_id);
        self.insert(to_id, from_cell);
    }
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

#[derive(SystemData)]
pub struct WriteCells<'a> {
    pub color: WriteCellStorage<'a, crate::components::cell_components::CellColor>,
    pub sand: WriteCellStorage<'a, crate::components::cell_components::Sand>,
    pub solid: WriteCellStorage<'a, crate::components::cell_components::Solid>,
}

impl<'a> WriteCells<'a> {
    pub fn move_cell(&mut self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
        self.color.move_cell(from_x, from_y, to_x, to_y);
        self.sand.move_cell(from_x, from_y, to_x, to_y);
        self.solid.move_cell(from_x, from_y, to_x, to_y);
    }
}
