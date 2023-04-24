mod utils;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct Calculation {
    contents: i32,
}

#[wasm_bindgen]
impl Calculation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculation {
        Calculation { contents: 0 }
    }

    pub fn add(&mut self, x: i32) {
        self.contents += x;
    }

    pub fn subtract(&mut self, x: i32) {
        self.contents -= x;
    }

    pub fn get_contents(&self) -> i32 {
        self.contents
    }
}

#[wasm_bindgen(js_name = doTheThing)]
pub fn do_the_thing() -> u32 {
    42
}
