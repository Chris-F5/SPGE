use crow::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    Context,
};
use shred::{DispatcherBuilder, SystemData, World};
use spge::{
    components::cell_components::{CellColor, TestComp},
    storage::cell_storage::{MaskedCellStorage, WriteCellStorage},
    systems::{DrawSystem, SandSystem},
};

fn main() -> Result<(), crow::Error> {
    // INIT crow window and context
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("SPGE Test");
    let ctx = Context::new(window, &event_loop)?;

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
        .with_thread_local(DrawSystem { ctx })
        .build();

    // Run crow event loop
    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                game_tick_dispatcher.dispatch(&mut world);
                draw_dispatcher.dispatch(&mut world);
            }
            Event::RedrawRequested(_) => {
                // TODO: call draw dispatcher in here and somehow pass window context into it
            }
            _ => (),
        },
    )
}
