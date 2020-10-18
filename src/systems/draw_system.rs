use crate::{
    components::cell_components::CellColor,
    storage::cell_storage::{CellStorage, Join, ReadCellStorage},
    CHUNK_SIZE,
};
use crow::{
    image::{
        imageops::{resize, Nearest},
        ImageBuffer, Rgba,
    },
    Context, DrawConfig, Texture,
};
use shred::System;

pub struct DrawSystem {
    pub ctx: Context,
}

impl<'a> System<'a> for DrawSystem {
    type SystemData = ReadCellStorage<'a, CellColor>;
    fn run(&mut self, cell_colors: Self::SystemData) {
        let mut surface = self.ctx.surface();
        let img = ImageBuffer::from_fn(CHUNK_SIZE, CHUNK_SIZE, |x, y| {
            let cell_color = cell_colors.get(x, y);
            match cell_color {
                Some(cell_color) => return Rgba([cell_color.r, cell_color.g, cell_color.b, 255]),
                None => return Rgba([0, 0, 0, 255]),
            }
        });
        let img = resize(&img, CHUNK_SIZE * 10, CHUNK_SIZE * 10, Nearest);
        let texture = Texture::from_image(&mut self.ctx, img).unwrap();
        self.ctx.clear_color(&mut surface, (0.4, 0.4, 0.8, 1.0));
        self.ctx
            .draw(&mut surface, &texture, (100, 150), &DrawConfig::default());
        self.ctx.present(surface).unwrap();
    }
}
