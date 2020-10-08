mod sand;
pub use sand::Sand;

use crate::cell_storage::InnerCellStorage;

pub trait CellComponent: Sized + Default + Clone + Copy + 'static {
    type CellStorage: InnerCellStorage<Self>;
}
