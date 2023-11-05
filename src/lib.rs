use std::io::Write;

const WASM_MEMORY_BUFFER_SIZE: usize = 1024;
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
