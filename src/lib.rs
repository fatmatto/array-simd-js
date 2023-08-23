
extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
use packed_simd::f32x8 as f32s;
use std::panic;

#[wasm_bindgen]
pub fn sum_ver(x: &[f32]) -> f32 {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    assert_eq!(x.len() % f32s::lanes(), 0);

    x.chunks_exact(f32s::lanes())
        .map(f32s::from_slice_unaligned)
        .sum::<f32s>()
        .sum()
}

#[wasm_bindgen]
pub fn sum_hor(x: &[f32]) -> f32 {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    assert_eq!(x.len() % f32s::lanes(), 0);

    x.chunks_exact(f32s::lanes())
        .map(f32s::from_slice_unaligned)
        .map(f32s::sum)
        .sum()
}