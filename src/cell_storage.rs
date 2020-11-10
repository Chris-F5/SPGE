mod array_storage;
mod cell_component_joining;
mod cell_mask;
mod masked_array_storage;
mod null_storage;

pub use self::array_storage::ArrayStorage;
pub use self::cell_component_joining::Join;
pub use self::cell_mask::CellMask;
pub use self::masked_array_storage::MaskedArrayStorage;
pub use self::null_storage::NullStorage;

use crate::WORLD_WIDTH;
use shred::{Fetch, FetchMut, ResourceId, SystemData, World};
use std::ops::{Deref, DerefMut};

pub fn cell_to_id(x: u32, y: u32) -> u32 {
    x + y * WORLD_WIDTH
}
pub fn id_to_cell(id: u32) -> (u32, u32) {
    (id % WORLD_WIDTH, id / WORLD_WIDTH)
}

use std::any::Any;

pub trait CellComponent: Any + Sized + Default + Copy + Clone {
    type Storage: InnerCellStorage<Self> + Send + Sync;
}

pub struct CellStorageAccessor<D> {
    pub data: D,
}

impl<D> CellStorageAccessor<D> {
    pub fn new(data: D) -> CellStorageAccessor<D> {
        CellStorageAccessor::<D> { data: data }
    }
}
impl<C, D> Deref for CellStorageAccessor<D>
where
    C: CellComponent,
    D: Deref<Target = CellStorage<C>>,
{
    type Target = D;
    fn deref(&self) -> &D {
        &self.data
    }
}

impl<C, D> DerefMut for CellStorageAccessor<D>
where
    C: CellComponent,
    D: Deref<Target = CellStorage<C>>,
{
    fn deref_mut(&mut self) -> &mut D {
        &mut self.data
    }
}

pub struct CellStorage<C>
where
    C: CellComponent,
{
    inner: C::Storage,
}

impl<C> Default for CellStorage<C>
where
    C: CellComponent,
{
    fn default() -> Self {
        CellStorage::<C> {
            inner: Default::default(),
        }
    }
}

impl<C> Deref for CellStorage<C>
where
    C: CellComponent,
{
    type Target = C::Storage;
    fn deref(&self) -> &C::Storage {
        &self.inner
    }
}
impl<C> DerefMut for CellStorage<C>
where
    C: CellComponent,
{
    fn deref_mut(&mut self) -> &mut C::Storage {
        &mut self.inner
    }
}

pub trait InnerCellStorage<T>: Default + Sized
where
    T: CellComponent,
{
}

pub type ReadCellStorage<'a, C> = CellStorageAccessor<Fetch<'a, CellStorage<C>>>;

impl<'a, C> SystemData<'a> for ReadCellStorage<'a, C>
where
    C: CellComponent,
{
    fn setup(_res: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        CellStorageAccessor::new(res.fetch())
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<CellStorage<C>>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

pub type WriteCellStorage<'a, C> = CellStorageAccessor<FetchMut<'a, CellStorage<C>>>;

impl<'a, C> SystemData<'a> for WriteCellStorage<'a, C>
where
    C: CellComponent,
{
    fn setup(_res: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        CellStorageAccessor::new(res.fetch_mut())
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<CellStorage<C>>()]
    }
}
