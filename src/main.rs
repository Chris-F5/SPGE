use shred::{DispatcherBuilder, SystemData, World};
use spge::{
    components::cell_components::{CellColor, TestComp},
    storage::cell_storage::{MaskedCellStorage, WriteCellStorage},
    systems::{DrawSystem, SandSystem},
};

fn main() {
    // INIT world
    let mut world = World::empty();

    let cell_colors: MaskedCellStorage<CellColor> = Default::default();
    let test_cells: MaskedCellStorage<TestComp> = Default::default();

    world.insert(cell_colors);
    world.insert(test_cells);

    {
        let mut colors = WriteCellStorage::<CellColor>::fetch(&world);
        colors.insert(5, 5);
        colors.insert(4, 1);
        colors.insert(4, 5);
        colors.insert(10, 10);
    }
    let mut game_tick_dispatcher = DispatcherBuilder::new()
        .with(SandSystem, "sand", &[])
        .build();
    let mut draw_dispatcher = DispatcherBuilder::new()
        .with_thread_local(DrawSystem)
        .build();
    game_tick_dispatcher.dispatch(&world);
    draw_dispatcher.dispatch(&world);
    game_tick_dispatcher.dispatch(&world);
    draw_dispatcher.dispatch(&world);
}
