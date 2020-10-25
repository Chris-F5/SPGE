mod cell_color;
mod sand;
mod solid;
mod test_comp;

pub use cell_color::CellColor;
pub use sand::Sand;
pub use solid::Solid;
pub use test_comp::TestComp;

use crate::storage::cell_storage::InnerCellStorage;
use std::any::Any;

pub trait CellComponent: Any + Sized + Default + Copy {
    type Storage: InnerCellStorage<Self> + Send + Sync;
}
