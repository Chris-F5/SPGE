//mod components;
//mod systems;

//use specs::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn test_add_function(a: i32, b: i32) -> i32 {
    a + b + 1
}
/*
fn main() {
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::PrintPosition, "debug_position_system", &[])
        .build();
    dispatcher.setup(&mut world);
    world
        .create_entity()
        .with(components::Position { x: 5.0, y: 10.0 })
        .build();
    dispatcher.dispatch(&mut world);
}
*/
