use super::{CellPos, Join};
use hibitset::BitSet;

pub struct CellMask {
    mask: BitSet,
}

impl CellMask {
    pub fn empty() -> CellMask {
        CellMask {
            mask: BitSet::with_capacity(crate::WORLD_CELL_COUNT),
        }
    }
    pub fn insert(&mut self, pos: &dyn CellPos) {
        self.mask.add(pos.index());
    }
    pub fn remove(&mut self, pos: &dyn CellPos) {
        self.mask.remove(pos.index());
    }
    pub fn contains(&self, pos: &dyn CellPos) -> bool {
        self.mask.contains(pos.index())
    }
}

impl<'a> Join for &'a CellMask {
    type Mask = &'a BitSet;
    fn get_mask(self) -> &'a BitSet {
        &self.mask
    }
}
