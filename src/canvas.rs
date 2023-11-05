use crate::{math, pixel};

const WASM_PIXEL_BUFFER_SIZE: usize = 1 << 14;
pub static mut WASM_PIXEL_BUFFER: [pixel::Pixel; WASM_PIXEL_BUFFER_SIZE] =
    [pixel::COLOR_ZERO; WASM_PIXEL_BUFFER_SIZE];

#[no_mangle]
pub extern "C" fn pixel_buffer() -> *const pixel::Pixel {
    unsafe { WASM_PIXEL_BUFFER.as_ptr() }
}

pub fn draw_canvas(
    canvas_size: RectangleSize,
    x_axis_range: std::ops::RangeInclusive<f64>,
    y_axis_range: std::ops::RangeInclusive<f64>,
    f: impl Fn(f64, f64) -> pixel::Pixel,
) {
    assert!(canvas_size.x * canvas_size.y <= WASM_PIXEL_BUFFER_SIZE);

    let mut writer = std::io::Cursor::new(unsafe { &mut WASM_PIXEL_BUFFER[..] });

    for pixel_y in 0..canvas_size.y {
        let t = (canvas_size.y - pixel_y) as f64 / canvas_size.y as f64;
        let y = math::lerp(&y_axis_range, t);
        for pixel_x in 0..canvas_size.x {
            let t = pixel_x as f64 / canvas_size.x as f64;
            let x = math::lerp(&x_axis_range, t);

            // Write pixel
            let pos = writer.position() as usize;
            let pixel = f(x, y);
            writer.get_mut()[pos] = pixel;
            writer.set_position(pos as u64 + 1);
        }
    }
}

#[repr(C)]
pub struct RectangleSize {
    pub x: usize,
    pub y: usize,
}
