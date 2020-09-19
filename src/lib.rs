extern crate kagura;
extern crate wasm_bindgen;

mod page;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(page::index::new().with(page::index::Props {}), "app");
}
