use crow::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    Context, DrawConfig, Texture,
};
use shred::{DispatcherBuilder, SystemData, World};
use spge::{
    components::cell_components::{CellColor, TestComp},
    storage::cell_storage::{MaskedCellStorage, WriteCellStorage},
    systems::DrawSystem,
};

fn main() -> Result<(), crow::Error> {
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
        colors.insert(10, 10);
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(DrawSystem, "test_system", &[])
        .build();
    dispatcher.dispatch(&mut world);

    // INIT crow window and context
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("SPGE Test");
    let mut ctx = Context::new(window, &event_loop)?;

    let texture = Texture::load(&mut ctx, "./test_texture.png")?;

    // Run crow event loop
    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => ctx.window().request_redraw(),
            Event::RedrawRequested(_) => {
                let mut surface = ctx.surface();
                ctx.clear_color(&mut surface, (0.4, 0.4, 0.8, 1.0));
                ctx.draw(&mut surface, &texture, (100, 150), &DrawConfig::default());
                ctx.present(surface).unwrap();
            }
            _ => (),
        },
    )
}
