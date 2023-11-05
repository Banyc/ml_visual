use std::io::Write;

#[allow(dead_code)]
mod canvas;
#[allow(dead_code)]
mod math;
#[allow(dead_code)]
mod pixel;

const WASM_MEMORY_BUFFER_SIZE: usize = 1 << 10;
pub static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

#[no_mangle]
pub extern "C" fn buffer() -> *const u8 {
    unsafe { WASM_MEMORY_BUFFER.as_ptr() }
}

#[no_mangle]
pub extern "C" fn add(a: isize, b: isize) -> isize {
    a + b
}

#[no_mangle]
pub extern "C" fn buffer_write_hello_world() -> usize {
    let mut buffer = unsafe { WASM_MEMORY_BUFFER.as_mut() };
    let str = b"hello world";
    buffer.write_all(str).unwrap();
    str.len()
}

#[no_mangle]
pub extern "C" fn draw_binary_classification_two_features(
    w_1: f64,
    w_2: f64,
    b: f64,
    canvas_size_x: usize,
    canvas_size_y: usize,
) {
    fn decision_function(x_1: f64, w_1: f64, x_2: f64, w_2: f64, b: f64) -> bool {
        x_1 * w_1 + x_2 * w_2 + b >= 0.
    }

    fn draw_binary_classification_two_features_impl(
        w_1: f64,
        w_2: f64,
        b: f64,
        canvas_size: canvas::RectangleSize,
    ) {
        const X_1_RANGE: std::ops::RangeInclusive<f64> = -1.0..=1.0;
        const X_2_RANGE: std::ops::RangeInclusive<f64> = -1.0..=1.0;

        canvas::draw_canvas(canvas_size, X_1_RANGE, X_2_RANGE, |x_1, x_2| {
            let res = decision_function(x_1, w_1, x_2, w_2, b);
            match res {
                true => pixel::COLOR_BLUE,
                false => pixel::COLOR_RED,
            }
        });
    }

    draw_binary_classification_two_features_impl(
        w_1,
        w_2,
        b,
        canvas::RectangleSize {
            x: canvas_size_x,
            y: canvas_size_y,
        },
    )
}
