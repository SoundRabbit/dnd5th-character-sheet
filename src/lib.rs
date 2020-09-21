extern crate kagura;
extern crate wasm_bindgen;
extern crate web_sys;

mod component;
mod model;
mod util;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(component::app::new().with(component::app::Props {}), "app");
}
