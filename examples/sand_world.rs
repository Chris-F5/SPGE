use ggez::{
    event::{self, EventHandler, MouseButton},
    graphics,
    graphics::Image,
    nalgebra::{Point2, Vector2},
    Context, ContextBuilder, GameResult,
};
use shred::{DispatcherBuilder, SystemData, World};
use spge::{
    components::cell_components::{CellColor, Sand, Solid, TestComp},
    storage::cell_storage::{CellStorage, MaskedCellStorage, WriteCellStorage},
    systems::SandSystem,
    WORLD_HEIGHT, WORLD_WIDTH,
};

const CELL_SIZE: u32 = 5;

fn main() {
    // INIT world
    let mut world = World::empty();

    let cell_colors: MaskedCellStorage<CellColor> = Default::default();
    let test_cells: MaskedCellStorage<TestComp> = Default::default();
    let sands: MaskedCellStorage<Sand> = Default::default();
    let solids: MaskedCellStorage<Solid> = Default::default();

    world.insert(cell_colors);
    world.insert(test_cells);
    world.insert(sands);
    world.insert(solids);

    let update_dispatcher: shred::Dispatcher<'static, 'static> = DispatcherBuilder::new()
        .with(SandSystem, "sand", &[])
        .build();

    // Make window and run event loop
    let (mut ctx, mut event_loop) = ContextBuilder::new("spge_game", "Chris Lang Games")
        .window_setup(ggez::conf::WindowSetup::default().title("Sand World Example"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            (WORLD_WIDTH * CELL_SIZE) as f32,
            (WORLD_HEIGHT * CELL_SIZE) as f32,
        ))
        .build()
        .expect("error creating ggez context!");

    let mut game = Game::new(world, update_dispatcher);

    for x in 0..WORLD_WIDTH {
        game.insert_wall(x, 0);
        game.insert_wall(x, WORLD_HEIGHT - 1)
    }
    for y in 1..WORLD_HEIGHT - 1 {
        game.insert_wall(0, y);
        game.insert_wall(WORLD_WIDTH - 1, y)
    }

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Game<'a> {
    world: shred::World,
    update_dispatcher: shred::Dispatcher<'a, 'a>,
    renderer: Renderer,
    mouse_down: bool,
    mouse_x: f32,
    mouse_y: f32,
}

impl<'a> Game<'a> {
    pub fn new(world: shred::World, update_dispatcher: shred::Dispatcher<'a, 'a>) -> Game<'a> {
        Game::<'a> {
            world: world,
            update_dispatcher: update_dispatcher,
            renderer: Renderer::new(),
            mouse_down: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
    pub fn insert_sand(&self, x: u32, y: u32) {
        let mut solids = WriteCellStorage::<Solid>::fetch(&self.world);
        if !solids.get(x, y).is_some() {
            let sand_col = CellColor {
                r: 224,
                g: 188,
                b: 27,
                a: 255,
            };
            let mut colors = WriteCellStorage::<CellColor>::fetch(&self.world);
            colors.insert(x, y, sand_col);
            solids.insert(x, y, Solid);
            let mut sands = WriteCellStorage::<Sand>::fetch(&self.world);
            sands.insert(x, y, Sand);
        }
    }
    pub fn insert_wall(&self, x: u32, y: u32) {
        let col = CellColor {
            r: 89,
            g: 8,
            b: 12,
            a: 255,
        };
        let mut colors = WriteCellStorage::<CellColor>::fetch(&self.world);
        colors.insert(x, y, col);
        let mut solids = WriteCellStorage::<Solid>::fetch(&self.world);
        solids.insert(x, y, Solid);
    }
    fn mouse_down_on_pixel(&self, x: f32, y: f32) {
        if y > (WORLD_HEIGHT * CELL_SIZE) as f32
            || y <= 0.0
            || x > (WORLD_WIDTH * CELL_SIZE) as f32
            || x <= 0.0
        {
            return;
        }
        let y = (y - (WORLD_HEIGHT * CELL_SIZE) as f32) * -1.0;

        let x = x.floor() as u32 / CELL_SIZE;
        let y = y.floor() as u32 / CELL_SIZE;
        self.insert_sand(x, y);
    }
}

impl<'a> EventHandler for Game<'a> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.mouse_down {
            self.mouse_down_on_pixel(self.mouse_x, self.mouse_y)
        }
        self.update_dispatcher.dispatch(&mut self.world);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::clear(ctx, ggez::graphics::WHITE);
        self.renderer.render(ctx, &mut self.world);
        ggez::graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if button == MouseButton::Left {
            self.mouse_down = true;
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.mouse_down = false;
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }
}

pub struct Renderer;
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }

    pub fn render(&mut self, ctx: &mut Context, world: &mut World) {
        let cell_colors = world.fetch::<MaskedCellStorage<CellColor>>();
        let cell_colors = CellStorage::new(cell_colors);

        let colors = cell_colors.data.inner.cells;
        let mut rgba_colors: [u8; (WORLD_WIDTH * WORLD_HEIGHT * 4) as usize] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        for i in 0..(WORLD_WIDTH * WORLD_HEIGHT) as usize {
            rgba_colors[i * 4] = colors[i].r;
            rgba_colors[i * 4 + 1] = colors[i].g;
            rgba_colors[i * 4 + 2] = colors[i].b;
            rgba_colors[i * 4 + 3] = colors[i].a;
        }

        let mut cells_image = Image::from_rgba8(
            ctx,
            WORLD_WIDTH as u16,
            WORLD_HEIGHT as u16,
            &rgba_colors[..],
        )
        .unwrap();

        cells_image.set_filter(graphics::FilterMode::Nearest);

        graphics::draw(
            ctx,
            &cells_image,
            graphics::DrawParam::new()
                .dest(Point2::new(0.0, (WORLD_HEIGHT * CELL_SIZE) as f32))
                .scale(Vector2::new(CELL_SIZE as f32, -(CELL_SIZE as f32))),
        )
        .unwrap();
    }
}
