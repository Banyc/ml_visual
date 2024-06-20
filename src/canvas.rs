use olive_rs::{Canvas, HeapPixels2D, Pixels2D, BLACK};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Pixels2DWrapper {
    pixels: HeapPixels2D,
}

#[wasm_bindgen]
impl Pixels2DWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: HeapPixels2D::new(width, height, BLACK),
        }
    }

    pub(crate) fn canvas(&mut self) -> Canvas<HeapPixels2D> {
        Canvas::new_entire(&mut self.pixels)
    }

    pub fn pixels(&self) -> js_sys::Uint32Array {
        let u32_array: &[u32] = unsafe { std::mem::transmute(self.pixels.pixels()) };
        js_sys::Uint32Array::from(u32_array)
    }

    pub fn width(&self) -> usize {
        self.pixels.width()
    }

    pub fn height(&self) -> usize {
        self.pixels.height()
    }
}
