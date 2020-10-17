mod cell_color;
mod test_comp;

pub use cell_color::CellColor;
pub use test_comp::TestComp;

use crate::storage::cell_storage::InnerCellStorage;
use std::any::Any;

pub trait CellComponent: Any + Sized + Default + Copy {
    type Storage: InnerCellStorage<Self> + Send + Sync;
}
