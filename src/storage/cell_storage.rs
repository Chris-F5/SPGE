mod array_storage;

pub use array_storage::ArrayStorage;

use crate::components::cell_components::CellComponent;
use crate::CHUNK_SIZE;
use hibitset::BitIter;
use hibitset::BitSet;
use hibitset::BitSetAnd;
use hibitset::BitSetLike;
use shred::{Fetch, FetchMut, ResourceId, SystemData, World};
use std::ops::{Deref, DerefMut};
use tuple_utils::Split;

fn cell_to_id(x: u32, y: u32) -> u32 {
    x + y * CHUNK_SIZE
}
fn id_to_cell(id: u32) -> (u32, u32) {
    (id % CHUNK_SIZE, id / CHUNK_SIZE)
}

pub struct MaskedCellStorage<T>
where
    T: CellComponent,
{
    mask: BitSet,
    inner: T::Storage,
}

impl<T> MaskedCellStorage<T>
where
    T: CellComponent,
{
    fn open_mut(&mut self) -> (&BitSet, &mut T::Storage) {
        (&self.mask, &mut self.inner)
    }
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

pub trait CellInnerStorage<T>: Default + Sized {
    fn get_mut(&mut self, id: u32) -> &mut T;
    fn get(&self, id: u32) -> &T;
    fn insert(&mut self, id: u32) -> &mut T;
    fn remove(&mut self, id: u32);
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
    pub fn insert(&mut self, x: u32, y: u32) -> &C {
        let id = cell_to_id(x, y);
        self.data.mask.add(id);
        self.data.inner.insert(id)
    }
    pub fn remove(&mut self, x: u32, y: u32) {
        let id = cell_to_id(x, y);
        self.data.mask.remove(id);
    }
}

impl<'a, T, C> Join for &'a CellStorage<T>
where
    C: CellComponent,
    T: DerefMut<Target = MaskedCellStorage<C>>,
{
    type Type = &'a C;
    type Value = &'a C::Storage;
    type Mask = &'a BitSet;
    fn open(self) -> (&'a BitSet, &'a C::Storage) {
        (&self.data.mask, &self.data.inner)
    }
    unsafe fn get(value: &mut Self::Value, id: u32) -> &'a C {
        value.get(id)
    }
}

impl<'a, T, C> Join for &'a mut CellStorage<T>
where
    C: CellComponent,
    T: DerefMut<Target = MaskedCellStorage<C>>,
{
    type Type = &'a mut C;
    type Value = &'a mut C::Storage;
    type Mask = &'a BitSet;
    fn open(self) -> (&'a BitSet, &'a mut C::Storage) {
        self.data.open_mut()
    }

    // TODO: audit unsafe
    unsafe fn get(value: &mut Self::Value, id: u32) -> &'a mut C {
        // This is horribly unsafe. Unfortunately, Rust doesn't provide a way
        // to abstract mutable/immutable state at the moment, so we have to hack
        // our way through it.
        let value: *mut Self::Value = value as *mut Self::Value;
        (*value).get_mut(id)
    }
}

pub trait Join {
    type Type;
    type Value;
    type Mask: BitSetLike;
    fn join(self) -> JoinIter<Self>
    where
        Self: Sized,
    {
        JoinIter::new(self)
    }
    fn open(self) -> (Self::Mask, Self::Value);
    unsafe fn get(value: &mut Self::Value, id: u32) -> Self::Type;
}
pub struct JoinIter<J>
where
    J: Join,
{
    keys: BitIter<J::Mask>,
    values: J::Value,
}
impl<J> JoinIter<J>
where
    J: Join,
{
    fn new(j: J) -> Self {
        let (keys, values) = j.open();
        JoinIter {
            keys: keys.iter(),
            values: values,
        }
    }
}

impl<J: Join> std::iter::Iterator for JoinIter<J> {
    type Item = ((u32, u32), J::Type);

    fn next(&mut self) -> Option<((u32, u32), J::Type)> {
        self.keys
            .next()
            .map(|i| (id_to_cell(i), unsafe { J::get(&mut self.values, i) }))
    }
}

/// `BitAnd` is a helper method to & bitsets together resulting in a tree.
pub trait BitAnd {
    /// The combined bitsets.
    type Value: BitSetLike;
    /// Combines `Self` into a single `BitSetLike` through `BitSetAnd`.
    fn and(self) -> Self::Value;
}

/// This needs to be special cased
impl<A> BitAnd for (A,)
where
    A: BitSetLike,
{
    type Value = A;

    fn and(self) -> Self::Value {
        self.0
    }
}

macro_rules! bitset_and {
    // use variables to indicate the arity of the tuple
    ($($from:ident),*) => {
        impl<$($from),*> BitAnd for ($($from),*)
            where $($from: BitSetLike),*
        {
            type Value = BitSetAnd<
                <<Self as Split>::Left as BitAnd>::Value,
                <<Self as Split>::Right as BitAnd>::Value
            >;

            fn and(self) -> Self::Value {
                let (l, r) = self.split();
                BitSetAnd(l.and(), r.and())
            }
        }
    }
}

bitset_and! {A, B}
bitset_and! {A, B, C}
bitset_and! {A, B, C, D}
bitset_and! {A, B, C, D, E}
bitset_and! {A, B, C, D, E, F}
bitset_and! {A, B, C, D, E, F, G}
bitset_and! {A, B, C, D, E, F, G, H}
bitset_and! {A, B, C, D, E, F, G, H, I}
bitset_and! {A, B, C, D, E, F, G, H, I, J}
bitset_and! {A, B, C, D, E, F, G, H, I, J, K}
bitset_and! {A, B, C, D, E, F, G, H, I, J, K, L}
bitset_and! {A, B, C, D, E, F, G, H, I, J, K, L, M}
bitset_and! {A, B, C, D, E, F, G, H, I, J, K, L, M, N}
bitset_and! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O}
bitset_and! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}

macro_rules! define_open {
    // use variables to indicate the arity of the tuple
    ($($from:ident),*) => {
        impl<$($from,)*> Join for ($($from),*,)
            where $($from: Join),*,
                  ($(<$from as Join>::Mask,)*): BitAnd,
        {
            type Type = ($($from::Type),*,);
            type Value = ($($from::Value),*,);
            type Mask = <($($from::Mask,)*) as BitAnd>::Value;
            #[allow(non_snake_case)]
            // SAFETY: While we do expose the mask and the values and therefore would allow swapping them,
            // this method is `unsafe` and relies on the same invariants.
            fn open(self) -> (Self::Mask, Self::Value) {
                let ($($from,)*) = self;
                let ($($from,)*) = ($($from.open(),)*);
                (
                    ($($from.0),*,).and(),
                    ($($from.1),*,)
                )
            }
            #[allow(non_snake_case)]
            unsafe fn get(v: &mut Self::Value, i: u32) -> Self::Type {
                let &mut ($(ref mut $from,)*) = v;
                ($($from::get($from, i),)*)
            }
        }
    }
}

define_open! {A}
define_open! {A, B}
define_open! {A, B, C}
define_open! {A, B, C, D}
define_open! {A, B, C, D, E}
define_open! {A, B, C, D, E, F}
define_open! {A, B, C, D, E, F, G}
define_open! {A, B, C, D, E, F, G, H}
define_open! {A, B, C, D, E, F, G, H, I}
define_open! {A, B, C, D, E, F, G, H, I, J}
define_open! {A, B, C, D, E, F, G, H, I, J, K}
define_open! {A, B, C, D, E, F, G, H, I, J, K, L}
define_open! {A, B, C, D, E, F, G, H, I, J, K, L, M}
define_open! {A, B, C, D, E, F, G, H, I, J, K, L, M, N}
define_open! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O}
define_open! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}

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
