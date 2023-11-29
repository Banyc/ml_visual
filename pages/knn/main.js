import init, * as lib from "../../assets/wasm/ml_visual.js"

async function setup() {
    let builder = new lib.WasmKnnBuilder()
    let pixels_wrapper = new lib.Pixels2DWrapper(128, 128)

    const examples = document.getElementById("knn.examples").value
    let tree = builder.build(examples)

    let draw_on_event = function (ev) {
        const examples = document.getElementById("knn.examples").value
        tree = builder.build(examples)

        draw_canvas(pixels_wrapper, tree)
    }
    document.getElementById("knn.examples").addEventListener("input", draw_on_event)
    document.getElementById("knn.k").addEventListener("input", draw_on_event)
    
    let draw_canvas_on_event = function (ev) {
        draw_canvas(pixels_wrapper, tree)
    }
    document.getElementById("knn.two_features.x_axis_start").addEventListener("input", draw_canvas_on_event)
    document.getElementById("knn.two_features.x_axis_end").addEventListener("input", draw_canvas_on_event)
    document.getElementById("knn.two_features.y_axis_start").addEventListener("input", draw_canvas_on_event)
    document.getElementById("knn.two_features.y_axis_end").addEventListener("input", draw_canvas_on_event)

    draw_canvas(pixels_wrapper, tree)
}

function draw_canvas(pixels_wrapper, tree) {
    const x_axis_start = parseFloat(document.getElementById("knn.two_features.x_axis_start").value)
    const x_axis_end = parseFloat(document.getElementById("knn.two_features.x_axis_end").value)
    const y_axis_start = parseFloat(document.getElementById("knn.two_features.y_axis_start").value)
    const y_axis_end = parseFloat(document.getElementById("knn.two_features.y_axis_end").value)
    const examples = document.getElementById("knn.examples").value
    const k = document.getElementById("knn.k").value

    tree.draw(examples, x_axis_start, x_axis_end, y_axis_start, y_axis_end, pixels_wrapper, k)

    let canvas_perceptron = document.getElementById("knn.two_features.canvas")
    let ctx = canvas_perceptron.getContext("2d")
    let palette = ctx.getImageData(0, 0, pixels_wrapper.width(), pixels_wrapper.height())
    palette.data.set(new Uint8ClampedArray(pixels_wrapper.pixels().buffer))
    ctx.putImageData(palette, 0, 0)
}


async function main() {
    await init()
    await setup()
}

main()
