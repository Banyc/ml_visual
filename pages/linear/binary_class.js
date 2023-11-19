import init, * as lib from "../../assets/wasm/ml_visual.js"

export function setup() {
    const size_x = 128
    const size_y = 128
    let pixels_wrapper = new lib.Pixels2DWrapper(size_x, size_y)

    let w_1 = document.getElementById("linear.binary_class.w_1")
    w_1.addEventListener("input", function(ev) {
        document.getElementById("linear.binary_class.w_1_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let w_2 = document.getElementById("linear.binary_class.w_2")
    w_2.addEventListener("input", function(ev) {
        document.getElementById("linear.binary_class.w_2_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let b = document.getElementById("linear.binary_class.b")
    b.addEventListener("input", function(ev) {
        document.getElementById("linear.binary_class.b_range").value = this.value
        draw_canvas(pixels_wrapper)
    })

    let w_1_range = document.getElementById("linear.binary_class.w_1_range")
    w_1_range.addEventListener("input", function(ev) {
        document.getElementById("linear.binary_class.w_1").value = this.value
        draw_canvas(pixels_wrapper)
    })
    
    let w_2_range = document.getElementById("linear.binary_class.w_2_range")
    w_2_range.addEventListener("input", function(ev) {
        document.getElementById("linear.binary_class.w_2").value = this.value
        draw_canvas(pixels_wrapper)
    })
    
    let b_range = document.getElementById("linear.binary_class.b_range")
    b_range.addEventListener("input", function(ev) {
        document.getElementById("linear.binary_class.b").value = this.value
        draw_canvas(pixels_wrapper)
    })

    let examples = document.getElementById("linear.binary_class.examples")
    examples.addEventListener("change", function(ev) {
        draw_canvas(pixels_wrapper)
    })

    let perceptron_learn = document.getElementById("linear.binary_class.perceptron_learn")
    perceptron_learn.addEventListener("click", function(ev) {
        learn_and_draw("perceptron", pixels_wrapper)
    })

    let adaline_learn = document.getElementById("linear.binary_class.adaline_learn")
    adaline_learn.addEventListener("click", function(ev) {
        learn_and_draw("adaline", pixels_wrapper)
    })

    draw_canvas(pixels_wrapper)
}

function learn_and_draw(algorithm, pixels_wrapper) {
    const w_1 = parseFloat(document.getElementById("linear.binary_class.w_1").value)
    const w_2 = parseFloat(document.getElementById("linear.binary_class.w_2").value)
    const b = parseFloat(document.getElementById("linear.binary_class.b").value)
    let param = new lib.LinearTwoFeatureParam(w_1, w_2, b)
    const learning_rate = parseFloat(document.getElementById("linear.binary_class.eta").value)
    const examples = document.getElementById("linear.binary_class.examples").value

    let param_new = null
    if (algorithm === "adaline") {
        param_new = lib.adaline_learn_binary_class(examples, param, learning_rate)
    } else if (algorithm === "perceptron") {
        param_new = lib.perceptron_learn_binary_class(examples, param, learning_rate)
    }
    if (param_new === undefined) {
        return
    }

    document.getElementById("linear.binary_class.w_1").value = param_new.w_1()
    document.getElementById("linear.binary_class.w_1_range").value = param_new.w_1()
    document.getElementById("linear.binary_class.w_2").value = param_new.w_2()
    document.getElementById("linear.binary_class.w_2_range").value = param_new.w_2()
    document.getElementById("linear.binary_class.b").value = param_new.b()
    document.getElementById("linear.binary_class.b_range").value = param_new.b()

    draw_canvas(pixels_wrapper)
}

function draw_canvas(pixels_wrapper) {
    const w_1 = parseFloat(document.getElementById("linear.binary_class.w_1").value)
    const w_2 = parseFloat(document.getElementById("linear.binary_class.w_2").value)
    const b = parseFloat(document.getElementById("linear.binary_class.b").value)
    const examples = document.getElementById("linear.binary_class.examples").value
    let param = new lib.LinearTwoFeatureParam(w_1, w_2, b)

    lib.linear_draw_classification_binary_class(param, pixels_wrapper)
    lib.draw_examples_binary_class(examples, pixels_wrapper)

    let canvas_perceptron = document.getElementById("linear.binary_class.canvas")
    let ctx = canvas_perceptron.getContext("2d")
    let palette = ctx.getImageData(0, 0, pixels_wrapper.width(), pixels_wrapper.height())
    palette.data.set(new Uint8ClampedArray(pixels_wrapper.pixels().buffer))
    ctx.putImageData(palette, 0, 0)
}
