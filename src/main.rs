use shred::{DispatcherBuilder, SystemData, World};
use spge::{
    components::cell_components::{CellColor, TestComp},
    storage::cell_storage::{MaskedCellStorage, WriteCellStorage},
    systems::DrawSystem,
};

fn main() {
    let mut world = World::empty();

    let cell_colors: MaskedCellStorage<CellColor> = Default::default();
    let test_cells: MaskedCellStorage<TestComp> = Default::default();

    world.insert(cell_colors);
    world.insert(test_cells);

    {
        let mut colors = WriteCellStorage::<CellColor>::fetch(&world);
        colors.insert(5, 5);
        colors.insert(4, 1);
        colors.insert(10, 10);
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(DrawSystem, "test_system", &[])
        .build();
    dispatcher.dispatch(&mut world);
}
