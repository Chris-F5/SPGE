use super::id_to_cell;
use hibitset::{BitSetAnd, BitSetLike};
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
    type Mask: BitSetLike;
    fn join(self) -> CellIter
    where
        Self: Sized,
    {
        let mask = self.get_mask();
        let mut iter = mask.iter();
        let mut ids = Vec::new();
        loop {
            let id = iter.next();
            match id {
                Some(id) => ids.push(id),
                None => break,
            }
        }
        CellIter::new(ids)
    }
    fn get_mask(self) -> Self::Mask;
}

macro_rules! implement_tuple_joining {
    ($($from:ident),*) => {
        impl<$($from,)*> Join for ($($from),*,)
            where $($from: Join),*,
                  ($(<$from as Join>::Mask,)*): BitSetMerger,
        {
            type Mask = <($($from::Mask,)*) as BitSetMerger>::MergedSet;
            #[allow(non_snake_case)]
            fn get_mask(self) -> Self::Mask {
                let ($($from,)*) = self;
                let ($($from,)*) = ($($from.get_mask(),)*);
                ($($from),*,).merge()
            }
        }
    }
}

pub struct CellIter {
    ids: Vec<u32>,
}
impl CellIter {
    fn new(ids: Vec<u32>) -> Self {
        CellIter { ids: ids }
    }
}

impl std::iter::Iterator for CellIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        self.ids.pop().map(|i| id_to_cell(i))
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
