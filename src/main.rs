use shred::{DispatcherBuilder, System, SystemData, World};
use spge::{
    components::cell_components::{CellColor, TestComp},
    storage::cell_storage::{Join, MaskedCellStorage, ReadCellStorage, WriteCellStorage},
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
    {
        let mut tests = WriteCellStorage::<TestComp>::fetch(&world);
        tests.insert(5, 5);
        tests.insert(2, 7);
        tests.insert(10, 10);
    }
    {
        let colors = ReadCellStorage::<CellColor>::fetch(&world);
        match colors.get(5, 5) {
            Some(_) => println!("Some"),
            None => println!("None"),
        }
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(TestSystem, "test_system", &[])
        .build();
    dispatcher.dispatch(&mut world);
}

struct TestSystem;

impl<'a> System<'a> for TestSystem {
    type SystemData = (
        WriteCellStorage<'a, CellColor>,
        ReadCellStorage<'a, TestComp>,
    );
    fn run(
        &mut self,
        (mut cell_colors, test_comps): (
            WriteCellStorage<'a, CellColor>,
            ReadCellStorage<'a, TestComp>,
        ),
    ) {
        for ((x, y), (cell_color, _)) in (&mut cell_colors, &test_comps).join() {
            println!(
                "(x: {}, y: {}) (r: {}, g: {}, b: {})",
                x, y, cell_color.r, cell_color.g, cell_color.b
            )
        }
    }
}
