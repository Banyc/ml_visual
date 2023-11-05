use std::io::Write;

#[allow(dead_code)]
mod math;
#[allow(dead_code)]
mod pixel;

const WASM_MEMORY_BUFFER_SIZE: usize = 1 << 18;
pub static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

#[no_mangle]
pub extern "C" fn buffer() -> *const u8 {
    unsafe { WASM_MEMORY_BUFFER.as_ptr() }
}

const WASM_PIXEL_BUFFER_SIZE: usize = 1 << 18;
pub static mut WASM_PIXEL_BUFFER: [pixel::Pixel; WASM_PIXEL_BUFFER_SIZE] =
    [pixel::COLOR_WHITE; WASM_PIXEL_BUFFER_SIZE];

#[no_mangle]
pub extern "C" fn pixel_buffer() -> *const pixel::Pixel {
    unsafe { WASM_PIXEL_BUFFER.as_ptr() }
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

#[repr(C)]
pub struct RectangleSize {
    x: usize,
    y: usize,
}

#[no_mangle]
pub extern "C" fn draw_binary_classification_two_features(
    w_1: f64,
    w_2: f64,
    b: f64,
    size_x: usize,
    size_y: usize,
) {
    fn decision_function(x_1: f64, w_1: f64, x_2: f64, w_2: f64, b: f64) -> bool {
        x_1 * w_1 + x_2 * w_2 + b >= 0.
    }

    fn draw_binary_classification_two_features_impl(
        w_1: f64,
        w_2: f64,
        b: f64,
        size: RectangleSize,
    ) {
        const X_1_RANGE: std::ops::RangeInclusive<f64> = -1.0..=1.0;
        const X_2_RANGE: std::ops::RangeInclusive<f64> = -1.0..=1.0;
        let mut writer = std::io::Cursor::new(unsafe { &mut WASM_PIXEL_BUFFER[..] });

        for pixel_y in 0..size.y {
            let t = (size.y - pixel_y) as f64 / size.y as f64;
            let x_2 = math::lerp(X_2_RANGE, t);
            for pixel_x in 0..size.x {
                let t = pixel_x as f64 / size.x as f64;
                let x_1 = math::lerp(X_1_RANGE, t);

                let res = decision_function(x_1, w_1, x_2, w_2, b);

                // Write pixel
                let pos = writer.position() as usize;
                match res {
                    true => writer.get_mut()[pos] = pixel::COLOR_BLUE,
                    false => writer.get_mut()[pos] = pixel::COLOR_RED,
                }
                // writer.get_mut()[pos] = pixel::COLOR_WHITE;
                writer.set_position(pos as u64 + 1);
            }
        }
    }

    draw_binary_classification_two_features_impl(
        w_1,
        w_2,
        b,
        RectangleSize {
            x: size_x,
            y: size_y,
        },
    )
}
