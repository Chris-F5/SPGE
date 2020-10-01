use specs::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}
