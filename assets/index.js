let lib_wasm_filename = "lib.wasm"
let lib_wasm = null

async function import_lib_wasm() {
    let results = await WebAssembly.instantiateStreaming(fetch(lib_wasm_filename))
    lib_wasm = results.instance.exports
    console.log("Imported `%s`", lib_wasm_filename)
}

async function main() {
    await import_lib_wasm()

    let span_sum = document.getElementById("sum")
    let sum = lib_wasm.add(1, 1)
    span_sum.textContent = sum
}

main()
