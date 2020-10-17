use super::id_to_cell;
use hibitset::{BitIter, BitSetAnd, BitSetLike};
use tuple_utils::Split;

pub trait BitSetMerger {
    type MergedSet: BitSetLike;
    fn merge(self) -> Self::MergedSet;
}

impl<A> BitSetMerger for (A,)
where
    A: BitSetLike,
{
    type MergedSet = A;

    fn merge(self) -> Self::MergedSet {
        self.0
    }
}

macro_rules! implement_tuple_bit_set_merging {
    ($($from:ident),*) => {
        impl<$($from),*> BitSetMerger for ($($from),*)
            where $($from: BitSetLike),*
        {
            type MergedSet = BitSetAnd<
                <<Self as Split>::Left as BitSetMerger>::MergedSet,
                <<Self as Split>::Right as BitSetMerger>::MergedSet
            >;

            fn merge(self) -> Self::MergedSet {
                let (l, r) = self.split();
                BitSetAnd(l.merge(), r.merge())
            }
        }
    }
}

pub trait Join {
    type Component;
    type InnerStorage;
    type Mask: BitSetLike;
    fn join(self) -> JoinIter<Self>
    where
        Self: Sized,
    {
        JoinIter::new(self)
    }
    fn open(self) -> (Self::Mask, Self::InnerStorage);
    unsafe fn get(value: &mut Self::InnerStorage, id: u32) -> Self::Component;
}

macro_rules! implement_tuple_joining {
    ($($from:ident),*) => {
        impl<$($from,)*> Join for ($($from),*,)
            where $($from: Join),*,
                  ($(<$from as Join>::Mask,)*): BitSetMerger,
        {
            type Component = ($($from::Component),*,);
            type InnerStorage = ($($from::InnerStorage),*,);
            type Mask = <($($from::Mask,)*) as BitSetMerger>::MergedSet;
            #[allow(non_snake_case)]
            fn open(self) -> (Self::Mask, Self::InnerStorage) {
                let ($($from,)*) = self;
                let ($($from,)*) = ($($from.open(),)*);
                (
                    ($($from.0),*,).merge(),
                    ($($from.1),*,)
                )
            }
            #[allow(non_snake_case)]
            unsafe fn get(v: &mut Self::InnerStorage, i: u32) -> Self::Component {
                let ($($from,)*) = v;
                ($($from::get($from, i),)*)
            }
        }
    }
}

pub struct JoinIter<J>
where
    J: Join,
{
    keys: BitIter<J::Mask>,
    inner_storages: J::InnerStorage,
}
impl<J> JoinIter<J>
where
    J: Join,
{
    fn new(j: J) -> Self {
        let (keys, inner_storages) = j.open();
        JoinIter {
            keys: keys.iter(),
            inner_storages: inner_storages,
        }
    }
}

impl<J: Join> std::iter::Iterator for JoinIter<J> {
    type Item = ((u32, u32), J::Component);

    fn next(&mut self) -> Option<((u32, u32), J::Component)> {
        self.keys.next().map(|i| {
            (id_to_cell(i), unsafe {
                J::get(&mut self.inner_storages, i)
            })
        })
    }
}

implement_tuple_joining! {A}
implement_tuple_joining! {A, B}
implement_tuple_joining! {A, B, C}
implement_tuple_joining! {A, B, C, D}
implement_tuple_joining! {A, B, C, D, E}
implement_tuple_joining! {A, B, C, D, E, F}
implement_tuple_joining! {A, B, C, D, E, F, G}
implement_tuple_joining! {A, B, C, D, E, F, G, H}
implement_tuple_joining! {A, B, C, D, E, F, G, H, I}
implement_tuple_joining! {A, B, C, D, E, F, G, H, I, J}
implement_tuple_joining! {A, B, C, D, E, F, G, H, I, J, K}
implement_tuple_joining! {A, B, C, D, E, F, G, H, I, J, K, L}
implement_tuple_joining! {A, B, C, D, E, F, G, H, I, J, K, L, M}
implement_tuple_joining! {A, B, C, D, E, F, G, H, I, J, K, L, M, N}
implement_tuple_joining! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O}
implement_tuple_joining! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}

implement_tuple_bit_set_merging! {A, B}
implement_tuple_bit_set_merging! {A, B, C}
implement_tuple_bit_set_merging! {A, B, C, D}
implement_tuple_bit_set_merging! {A, B, C, D, E}
implement_tuple_bit_set_merging! {A, B, C, D, E, F}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G, H}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G, H, I}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G, H, I, J}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G, H, I, J, K}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G, H, I, J, K, L}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G, H, I, J, K, L, M}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G, H, I, J, K, L, M, N}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O}
implement_tuple_bit_set_merging! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}
