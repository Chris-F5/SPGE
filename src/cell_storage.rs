mod cell_component_joining;
mod cell_mask;
mod storage;

pub use self::cell_component_joining::Join;
pub use self::cell_mask::CellMask;
pub use self::storage::MaskedArrayStorage;
pub use self::storage::NullStorage;
pub use self::storage::StrictArrayStorage;
pub use self::storage::TagStorage;

use crate::WORLD_WIDTH;
use shred::{Fetch, FetchMut, ResourceId, SystemData, World};
use std::any::Any;
use std::ops::{Deref, DerefMut};

fn cell_to_id(x: u32, y: u32) -> u32 {
    x + y * WORLD_WIDTH
}
fn id_to_cell(id: u32) -> (u32, u32) {
    (id % WORLD_WIDTH, id / WORLD_WIDTH)
}

pub trait CellPos {
    fn index(&self) -> u32;
}

impl CellPos for u32 {
    fn index(&self) -> u32 {
        *self
    }
}

impl CellPos for (u32, u32) {
    fn index(&self) -> u32 {
        cell_to_id(self.0, self.1)
    }
}

pub trait CellManager<CellType, Storage>
where
    Storage: TagStorageCollection<CellType>,
{
    fn get_storage() -> Storage;
}

pub trait TagStorageCollection<CellType> {
    fn move_cell(&mut self, from: &dyn CellPos, to: &dyn CellPos);
    fn remove(&mut self, pos: &dyn CellPos);
    fn insert(&mut self, pos: &dyn CellPos, cell: CellType);
}

pub trait Tag: Any + Sized + Copy + Clone {
    type Storage: Default + Sized + Send + Sync;
}

pub trait CellTag<CellType>: Tag {
    fn make_tag(cell_type: CellType) -> Option<Self>;
}

pub struct CellTagStorageAccessor<D> {
    pub data: D,
}

impl<D> CellTagStorageAccessor<D> {
    pub fn new(data: D) -> CellTagStorageAccessor<D> {
        CellTagStorageAccessor::<D> { data: data }
    }
}
impl<T, D> Deref for CellTagStorageAccessor<D>
where
    T: Tag,
    D: Deref<Target = CellTagStorage<T>>,
{
    type Target = D;
    fn deref(&self) -> &D {
        &self.data
    }
}

impl<T, D> DerefMut for CellTagStorageAccessor<D>
where
    T: Tag,
    D: Deref<Target = CellTagStorage<T>>,
{
    fn deref_mut(&mut self) -> &mut D {
        &mut self.data
    }
}

pub trait CellType: Any {}

pub struct CellTagStorage<T>
where
    T: Tag,
{
    inner: T::Storage,
}

impl<T> Default for CellTagStorage<T>
where
    T: Tag,
{
    fn default() -> Self {
        CellTagStorage::<T> {
            inner: Default::default(),
        }
    }
}

impl<T> Deref for CellTagStorage<T>
where
    T: Tag,
{
    type Target = T::Storage;
    fn deref(&self) -> &T::Storage {
        &self.inner
    }
}
impl<T> DerefMut for CellTagStorage<T>
where
    T: Tag,
{
    fn deref_mut(&mut self) -> &mut T::Storage {
        &mut self.inner
    }
}

//pub trait InnerCellStorage<T>: Default + Sized
//where
//    T: CellTag,
//{
//}

pub type ReadTagStorage<'a, C> = CellTagStorageAccessor<Fetch<'a, CellTagStorage<C>>>;

impl<'a, T> SystemData<'a> for ReadTagStorage<'a, T>
where
    T: Tag,
{
    fn setup(_res: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        CellTagStorageAccessor::new(res.fetch())
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<CellTagStorage<T>>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

pub type WriteTagStorage<'a, T> = CellTagStorageAccessor<FetchMut<'a, CellTagStorage<T>>>;

impl<'a, T> SystemData<'a> for WriteTagStorage<'a, T>
where
    T: Tag,
{
    fn setup(_res: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        CellTagStorageAccessor::new(res.fetch_mut())
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<CellTagStorage<T>>()]
    }
}
