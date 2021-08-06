mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
pub fn example(x: &str) -> usize { x.len() }

#[wasm_bindgen]
pub fn max(array: &[f64]) -> f64 {
    let mut max = array[0];

    for (_i, &value) in array.iter().enumerate() {
        if value > max {
            max = value;
        }
    }

    max
}

#[wasm_bindgen]
pub fn min(array: &[f64]) -> f64 {
    let mut min = array[0];

    for (_i, &value) in array.iter().enumerate() {
        if value < min {
            min = value;
        }
    }

    min
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}



