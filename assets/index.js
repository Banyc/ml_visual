const lib_wasm_filename = "lib.wasm"
let lib_wasm = null
let lib_wasm_buffer = null

async function import_lib_wasm() {
    const results = await WebAssembly.instantiateStreaming(fetch(lib_wasm_filename))
    lib_wasm = results.instance.exports
    lib_wasm_buffer = results.instance.exports.memory.buffer
    console.log("Imported `%s`", lib_wasm_filename)
}

function read_wasm_buffer(buffer_offset, length) {
    const buffer_ptr = lib_wasm.buffer()
    const buffer = new Uint8Array(lib_wasm_buffer).slice(buffer_ptr)
    return buffer.slice(buffer_offset, length)
}

function read_wasm_string(buffer_offset, length) {
    const buffer = read_wasm_buffer(buffer_offset, length)
    const string = new TextDecoder().decode(buffer);
    return string
}

async function main() {
    await import_lib_wasm()

    let span_sum = document.getElementById("sum")
    const sum = lib_wasm.add(1, 1)
    span_sum.textContent = sum

    const len = lib_wasm.buffer_write_hello_world()
    console.log("`%s` said: \"%s\"", lib_wasm_filename, read_wasm_string(0, len))
}

main()
