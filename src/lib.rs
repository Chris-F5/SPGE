mod cell_components;
mod cell_storage;
mod cell_systems;
mod object_components;
mod object_systems;

//use specs::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    render_some_stuff();
    Ok(())
}

#[wasm_bindgen]
pub fn test_add_function(a: i32, b: i32) -> i32 {
    a + b
}

pub fn render_some_stuff() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context.begin_path();
    context.fill_rect(20.0, 20.0, 300.0, 150.0);
    context.stroke();
}
