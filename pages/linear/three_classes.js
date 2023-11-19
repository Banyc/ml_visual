import init, * as lib from "../../assets/wasm/ml_visual.js"

export function setup() {
    const size_x = 128
    const size_y = 128
    let pixels_wrapper = new lib.Pixels2DWrapper(size_x, size_y)

    let y_0_w_1 = document.getElementById("linear.three_classes.y_0_w_1")
    y_0_w_1.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_0_w_1_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_0_w_2 = document.getElementById("linear.three_classes.y_0_w_2")
    y_0_w_2.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_0_w_2_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_0_b = document.getElementById("linear.three_classes.y_0_b")
    y_0_b.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_0_b_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_0_w_1_range = document.getElementById("linear.three_classes.y_0_w_1_range")
    y_0_w_1_range.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_0_w_1").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_0_w_2_range = document.getElementById("linear.three_classes.y_0_w_2_range")
    y_0_w_2_range.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_0_w_2").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_0_b_range = document.getElementById("linear.three_classes.y_0_b_range")
    y_0_b_range.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_0_b").value = this.value
        draw_canvas(pixels_wrapper)
    })

    let y_1_w_1 = document.getElementById("linear.three_classes.y_1_w_1")
    y_1_w_1.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_1_w_1_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_1_w_2 = document.getElementById("linear.three_classes.y_1_w_2")
    y_1_w_2.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_1_w_2_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_1_b = document.getElementById("linear.three_classes.y_1_b")
    y_1_b.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_1_b_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_1_w_1_range = document.getElementById("linear.three_classes.y_1_w_1_range")
    y_1_w_1_range.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_1_w_1").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_1_w_2_range = document.getElementById("linear.three_classes.y_1_w_2_range")
    y_1_w_2_range.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_1_w_2").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_1_b_range = document.getElementById("linear.three_classes.y_1_b_range")
    y_1_b_range.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_1_b").value = this.value
        draw_canvas(pixels_wrapper)
    })

    let y_2_w_1 = document.getElementById("linear.three_classes.y_2_w_1")
    y_2_w_1.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_2_w_1_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_2_w_2 = document.getElementById("linear.three_classes.y_2_w_2")
    y_2_w_2.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_2_w_2_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_2_b = document.getElementById("linear.three_classes.y_2_b")
    y_2_b.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_2_b_range").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_2_w_1_range = document.getElementById("linear.three_classes.y_2_w_1_range")
    y_2_w_1_range.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_2_w_1").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_2_w_2_range = document.getElementById("linear.three_classes.y_2_w_2_range")
    y_2_w_2_range.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_2_w_2").value = this.value
        draw_canvas(pixels_wrapper)
    })
    let y_2_b_range = document.getElementById("linear.three_classes.y_2_b_range")
    y_2_b_range.addEventListener("input", function (ev) {
        document.getElementById("linear.three_classes.y_2_b").value = this.value
        draw_canvas(pixels_wrapper)
    })

    let examples = document.getElementById("linear.three_classes.examples")
    examples.addEventListener("change", function (ev) {
        draw_canvas(pixels_wrapper)
    })

    let perceptron_learn = document.getElementById("linear.three_classes.perceptron_learn")
    perceptron_learn.addEventListener("click", function (ev) {
        learn_and_draw("perceptron", pixels_wrapper)
    })

    let adaline_learn = document.getElementById("linear.three_classes.adaline_learn")
    adaline_learn.addEventListener("click", function (ev) {
        learn_and_draw("adaline", pixels_wrapper)
    })

    draw_canvas(pixels_wrapper)
}

function learn_and_draw(algorithm, pixels_wrapper) {
    const y_0_w_1 = parseFloat(document.getElementById("linear.three_classes.y_0_w_1").value)
    const y_0_w_2 = parseFloat(document.getElementById("linear.three_classes.y_0_w_2").value)
    const y_0_b = parseFloat(document.getElementById("linear.three_classes.y_0_b").value)

    const y_1_w_1 = parseFloat(document.getElementById("linear.three_classes.y_1_w_1").value)
    const y_1_w_2 = parseFloat(document.getElementById("linear.three_classes.y_1_w_2").value)
    const y_1_b = parseFloat(document.getElementById("linear.three_classes.y_1_b").value)

    const y_2_w_1 = parseFloat(document.getElementById("linear.three_classes.y_2_w_1").value)
    const y_2_w_2 = parseFloat(document.getElementById("linear.three_classes.y_2_w_2").value)
    const y_2_b = parseFloat(document.getElementById("linear.three_classes.y_2_b").value)

    let param_0 = new lib.LinearTwoFeatureParam(y_0_w_1, y_0_w_2, y_0_b)
    let param_1 = new lib.LinearTwoFeatureParam(y_1_w_1, y_1_w_2, y_1_b)
    let param_2 = new lib.LinearTwoFeatureParam(y_2_w_1, y_2_w_2, y_2_b)

    const examples = document.getElementById("linear.three_classes.examples").value
    const learning_rate = parseFloat(document.getElementById("linear.three_classes.eta").value)

    let param_0_new = null
    let param_1_new = null
    let param_2_new = null
    if (algorithm === "adaline") {
        param_0_new = lib.adaline_learn_multiclass(examples, 0, param_0, learning_rate)
        param_1_new = lib.adaline_learn_multiclass(examples, 1, param_1, learning_rate)
        param_2_new = lib.adaline_learn_multiclass(examples, 2, param_2, learning_rate)
    } else if (algorithm === "perceptron") {
        param_0_new = lib.perceptron_learn_multiclass(examples, 0, param_0, learning_rate)
        param_1_new = lib.perceptron_learn_multiclass(examples, 1, param_1, learning_rate)
        param_2_new = lib.perceptron_learn_multiclass(examples, 2, param_2, learning_rate)
    }

    document.getElementById("linear.three_classes.y_0_w_1").value = param_0_new.w_1()
    document.getElementById("linear.three_classes.y_0_w_1_range").value = param_0_new.w_1()
    document.getElementById("linear.three_classes.y_0_w_2").value = param_0_new.w_2()
    document.getElementById("linear.three_classes.y_0_w_2_range").value = param_0_new.w_2()
    document.getElementById("linear.three_classes.y_0_b").value = param_0_new.b()
    document.getElementById("linear.three_classes.y_0_b_range").value = param_0_new.b()

    document.getElementById("linear.three_classes.y_1_w_1").value = param_1_new.w_1()
    document.getElementById("linear.three_classes.y_1_w_1_range").value = param_1_new.w_1()
    document.getElementById("linear.three_classes.y_1_w_2").value = param_1_new.w_2()
    document.getElementById("linear.three_classes.y_1_w_2_range").value = param_1_new.w_2()
    document.getElementById("linear.three_classes.y_1_b").value = param_1_new.b()
    document.getElementById("linear.three_classes.y_1_b_range").value = param_1_new.b()

    document.getElementById("linear.three_classes.y_2_w_1").value = param_2_new.w_1()
    document.getElementById("linear.three_classes.y_2_w_1_range").value = param_2_new.w_1()
    document.getElementById("linear.three_classes.y_2_w_2").value = param_2_new.w_2()
    document.getElementById("linear.three_classes.y_2_w_2_range").value = param_2_new.w_2()
    document.getElementById("linear.three_classes.y_2_b").value = param_2_new.b()
    document.getElementById("linear.three_classes.y_2_b_range").value = param_2_new.b()

    draw_canvas(pixels_wrapper)
}

function draw_canvas(pixels_wrapper) {
    const y_0_w_1 = parseFloat(document.getElementById("linear.three_classes.y_0_w_1").value)
    const y_0_w_2 = parseFloat(document.getElementById("linear.three_classes.y_0_w_2").value)
    const y_0_b = parseFloat(document.getElementById("linear.three_classes.y_0_b").value)

    const y_1_w_1 = parseFloat(document.getElementById("linear.three_classes.y_1_w_1").value)
    const y_1_w_2 = parseFloat(document.getElementById("linear.three_classes.y_1_w_2").value)
    const y_1_b = parseFloat(document.getElementById("linear.three_classes.y_1_b").value)

    const y_2_w_1 = parseFloat(document.getElementById("linear.three_classes.y_2_w_1").value)
    const y_2_w_2 = parseFloat(document.getElementById("linear.three_classes.y_2_w_2").value)
    const y_2_b = parseFloat(document.getElementById("linear.three_classes.y_2_b").value)

    let param_0 = new lib.LinearTwoFeatureParam(y_0_w_1, y_0_w_2, y_0_b)
    let param_1 = new lib.LinearTwoFeatureParam(y_1_w_1, y_1_w_2, y_1_b)
    let param_2 = new lib.LinearTwoFeatureParam(y_2_w_1, y_2_w_2, y_2_b)

    const examples = document.getElementById("linear.three_classes.examples").value

    lib.linear_draw_classification_three_classes(param_0, param_1, param_2, pixels_wrapper)
    lib.draw_examples_three_classes(examples, pixels_wrapper)

    let canvas_perceptron = document.getElementById("linear.three_classes.canvas")
    let ctx = canvas_perceptron.getContext("2d")
    let palette = ctx.getImageData(0, 0, pixels_wrapper.width(), pixels_wrapper.height())
    palette.data.set(new Uint8ClampedArray(pixels_wrapper.pixels().buffer))
    ctx.putImageData(palette, 0, 0)
}
