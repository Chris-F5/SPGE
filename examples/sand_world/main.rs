mod cell_components;
mod powder_system;

pub use cell_components::{CellColor, Powder, Solid, WriteCells};

use ggez::{
    event::{self, EventHandler, MouseButton},
    graphics,
    graphics::Image,
    nalgebra::{Point2, Vector2},
    Context, ContextBuilder, GameResult,
};
use shred::{DispatcherBuilder, SystemData, World};
use spge::{
    cell_storage::{CellPos, CellStorage, ReadCellStorage, WriteCellStorage},
    WORLD_HEIGHT, WORLD_WIDTH,
};

const CELL_SIZE: u32 = 5;

fn main() {
    // INIT world
    let mut world = World::empty();
    world
        .entry::<CellStorage<CellColor>>()
        .or_insert_with(|| Default::default());
    world
        .entry::<CellStorage<Powder>>()
        .or_insert_with(|| Default::default());
    world
        .entry::<CellStorage<Solid>>()
        .or_insert_with(|| Default::default());

    let update_dispatcher: shred::Dispatcher<'static, 'static> = DispatcherBuilder::new()
        .with(powder_system::PowderSystem, "powder", &[])
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
        game.insert_wall(&(x, 0));
        game.insert_wall(&(x, WORLD_HEIGHT - 1))
    }
    for y in 1..WORLD_HEIGHT - 1 {
        game.insert_wall(&(0, y));
        game.insert_wall(&(WORLD_WIDTH - 1, y))
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
    left_mouse_down: bool,
    right_mouse_down: bool,
    mouse_x: f32,
    mouse_y: f32,
}

impl<'a> Game<'a> {
    pub fn new(world: shred::World, update_dispatcher: shred::Dispatcher<'a, 'a>) -> Game<'a> {
        Game::<'a> {
            world: world,
            update_dispatcher: update_dispatcher,
            renderer: Renderer::new(),
            left_mouse_down: false,
            right_mouse_down: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
    pub fn insert_sand(&self, pos: &dyn CellPos) {
        let mut solids = WriteCellStorage::<Solid>::fetch(&self.world);
        if !solids.contains(pos) {
            let sand_col = CellColor {
                r: 224,
                g: 188,
                b: 27,
                a: 255,
            };
            solids.insert(pos);
            let mut color = WriteCellStorage::<CellColor>::fetch(&self.world);
            *color.get_mut(pos) = sand_col;
            let mut powder = WriteCellStorage::<Powder>::fetch(&self.world);
            powder.insert(pos, Default::default());
        }
    }
    pub fn insert_wall(&self, pos: &dyn CellPos) {
        let col = CellColor {
            r: 89,
            g: 8,
            b: 12,
            a: 255,
        };
        let mut colors = WriteCellStorage::<CellColor>::fetch(&self.world);
        *colors.get_mut(pos) = col;
        let mut solids = WriteCellStorage::<Solid>::fetch(&self.world);
        solids.insert(pos);
    }
    fn left_mouse_down_on_pixel(&self, x: f32, y: f32) {
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
        self.insert_sand(&(x, y));
    }
    fn right_mouse_down_on_pixel(&self, x: f32, y: f32) {
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
        self.insert_wall(&(x, y));
    }
}

impl<'a> EventHandler for Game<'a> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.left_mouse_down {
            self.left_mouse_down_on_pixel(self.mouse_x, self.mouse_y)
        } else if self.right_mouse_down {
            self.right_mouse_down_on_pixel(self.mouse_x, self.mouse_y)
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
            self.left_mouse_down = true;
        }
        if button == MouseButton::Right {
            self.right_mouse_down = true;
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.left_mouse_down = false;
        }
        if button == MouseButton::Right {
            self.right_mouse_down = false;
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
        let cell_colors = ReadCellStorage::<CellColor>::fetch(&world);

        let colors = cell_colors.cells;
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
