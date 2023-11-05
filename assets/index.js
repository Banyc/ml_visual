const lib_wasm_filename = "lib.wasm"
let lib_wasm = null
let lib_wasm_memory = null

async function import_lib_wasm() {
    const results = await WebAssembly.instantiateStreaming(fetch(lib_wasm_filename))
    lib_wasm = results.instance.exports
    lib_wasm_memory = results.instance.exports.memory.buffer
    console.log("Imported `%s`", lib_wasm_filename)
}

function read_wasm_buffer(buffer_offset, length) {
    const buffer_ptr = lib_wasm.buffer()
    const buffer = new Uint8Array(lib_wasm_memory).slice(buffer_ptr)
    return buffer.slice(buffer_offset, length)
}

function read_wasm_string(buffer_offset, length) {
    const buffer = read_wasm_buffer(buffer_offset, length)
    const string = new TextDecoder().decode(buffer);
    return string
}

function read_wasm_pixels(x, y) {
    const buffer_ptr = lib_wasm.pixel_buffer()
    const buffer = new Uint8Array(lib_wasm_memory).slice(buffer_ptr)
    return new Uint32Array(buffer.buffer).slice(0, x * y)
}

function render_one_plus_one() {
    let span_sum = document.getElementById("sum")
    const sum = lib_wasm.add(1, 1)
    span_sum.textContent = sum
}

function log_hello_world() {
    const len = lib_wasm.buffer_write_hello_world()
    console.log("`%s` said: \"%s\"", lib_wasm_filename, read_wasm_string(0, len))
}

function binary_classification_draw_canvas() {
    const size_x = 128
    const size_y = 128

    const w_1 = parseFloat(document.getElementById("binary_classification.w_1").value)
    const w_2 = parseFloat(document.getElementById("binary_classification.w_2").value)
    const b = parseFloat(document.getElementById("binary_classification.b").value)

    lib_wasm.draw_binary_classification_two_features(w_1, w_2, b, size_x, size_y)

    const pixels = read_wasm_pixels(size_x, size_y)

    let canvas_binary_classification = document.getElementById("binary_classification.canvas")
    let ctx = canvas_binary_classification.getContext("2d")
    let palette = ctx.getImageData(0, 0, size_x, size_y)
    palette.data.set(new Uint8ClampedArray(pixels.buffer))
    ctx.putImageData(palette, 0, 0)
}

async function main() {
    await import_lib_wasm()
    render_one_plus_one()
    log_hello_world()
}

main()
