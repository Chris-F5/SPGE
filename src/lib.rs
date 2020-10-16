pub mod components;
pub mod storage;

pub const CHUNK_SIZE: u32 = 16;

#[cfg(test)]
mod tests {
    use crate::components::cell_components::CellColor;
    use crate::components::cell_components::TestComp;
    use crate::storage::cell_storage::Join;
    use crate::storage::cell_storage::MaskedCellStorage;
    use crate::storage::cell_storage::ReadCellStorage;
    use crate::storage::cell_storage::WriteCellStorage;
    use shred::DispatcherBuilder;
    use shred::System;
    use shred::SystemData;
    use shred::World;

    #[test]
    fn it_works() {
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
            WriteCellStorage<'a, TestComp>,
        );
        fn run(
            &mut self,
            (mut cell_colors, mut test_comps): (
                WriteCellStorage<'a, CellColor>,
                WriteCellStorage<'a, TestComp>,
            ),
        ) {
            for ((x, y), (cell_color, _)) in (&mut cell_colors, &mut test_comps).join() {
                println!("{} {} {}", x, y, cell_color.r)
            }
        }
    }
}
