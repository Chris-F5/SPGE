use super::Join;
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
    pub fn insert(&mut self, id: u32) {
        self.mask.add(id);
    }
    pub fn remove(&mut self, id: u32) {
        self.mask.remove(id);
    }
    pub fn contains(&self, id: u32) -> bool {
        self.mask.contains(id)
    }
}

impl<'a> Join for &'a CellMask {
    type Mask = &'a BitSet;
    fn get_mask(self) -> &'a BitSet {
        &self.mask
    }
}
