import init, * as lib from "../../assets/wasm/ml_visual.js"

function perceptron_setup() {
    const size_x = 128
    const size_y = 128
    let pixels_wrapper = new lib.Pixels2DWrapper(size_x, size_y)

    let w_1 = document.getElementById("perceptron.w_1")
    w_1.addEventListener("input", function(ev) {
        document.getElementById("perceptron.w_1_range").value = this.value
        perceptron_draw_canvas(pixels_wrapper)
    })
    let w_2 = document.getElementById("perceptron.w_2")
    w_2.addEventListener("input", function(ev) {
        document.getElementById("perceptron.w_2_range").value = this.value
        perceptron_draw_canvas(pixels_wrapper)
    })
    let b = document.getElementById("perceptron.b")
    b.addEventListener("input", function(ev) {
        document.getElementById("perceptron.b_range").value = this.value
        perceptron_draw_canvas(pixels_wrapper)
    })

    let w_1_range = document.getElementById("perceptron.w_1_range")
    w_1_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.w_1").value = this.value
        perceptron_draw_canvas(pixels_wrapper)
    })
    
    let w_2_range = document.getElementById("perceptron.w_2_range")
    w_2_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.w_2").value = this.value
        perceptron_draw_canvas(pixels_wrapper)
    })
    
    let b_range = document.getElementById("perceptron.b_range")
    b_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.b").value = this.value
        perceptron_draw_canvas(pixels_wrapper)
    })

    let examples = document.getElementById("perceptron.examples")
    examples.addEventListener("change", function(ev) {
        perceptron_draw_canvas(pixels_wrapper)
    })

    let learn = document.getElementById("perceptron.learn")
    learn.addEventListener("click", function(ev) {
        const adaline = false
        learn_and_draw(adaline, pixels_wrapper)
    })

    let adaline_learn = document.getElementById("perceptron.adaline_learn")
    adaline_learn.addEventListener("click", function(ev) {
        const adaline = true
        learn_and_draw(adaline, pixels_wrapper)
    })

    perceptron_draw_canvas(pixels_wrapper)
}

function learn_and_draw(adaline, pixels_wrapper) {
    const w_1 = parseFloat(document.getElementById("perceptron.w_1").value)
    const w_2 = parseFloat(document.getElementById("perceptron.w_2").value)
    const b = parseFloat(document.getElementById("perceptron.b").value)
    let param = new lib.PerceptronParam(w_1, w_2, b)
    const learning_rate = parseFloat(document.getElementById("perceptron.eta").value)
    const examples = document.getElementById("perceptron.examples").value

    let param_new = null
    if (adaline) {
        param_new = lib.perceptron_adaline_learn(examples, param, learning_rate)
    } else {
        param_new = lib.perceptron_learn(examples, param, learning_rate)
    }
    if (param_new === undefined) {
        return
    }

    document.getElementById("perceptron.w_1").value = param_new.w_1()
    document.getElementById("perceptron.w_1_range").value = param_new.w_1()
    document.getElementById("perceptron.w_2").value = param_new.w_2()
    document.getElementById("perceptron.w_2_range").value = param_new.w_2()
    document.getElementById("perceptron.b").value = param_new.b()
    document.getElementById("perceptron.b_range").value = param_new.b()

    perceptron_draw_canvas(pixels_wrapper)
}

function perceptron_draw_canvas(pixels_wrapper) {
    const w_1 = parseFloat(document.getElementById("perceptron.w_1").value)
    const w_2 = parseFloat(document.getElementById("perceptron.w_2").value)
    const b = parseFloat(document.getElementById("perceptron.b").value)
    const examples = document.getElementById("perceptron.examples").value
    let param = new lib.PerceptronParam(w_1, w_2, b)

    lib.perceptron_draw_classification(param, pixels_wrapper)
    lib.perceptron_draw_examples(examples, pixels_wrapper)

    let canvas_perceptron = document.getElementById("perceptron.canvas")
    let ctx = canvas_perceptron.getContext("2d")
    let palette = ctx.getImageData(0, 0, pixels_wrapper.width(), pixels_wrapper.height())
    palette.data.set(new Uint8ClampedArray(pixels_wrapper.pixels().buffer))
    ctx.putImageData(palette, 0, 0)
}


function perceptron_2_setup() {
    const size_x = 128
    const size_y = 128
    let pixels_wrapper = new lib.Pixels2DWrapper(size_x, size_y)

    let y_0_w_1 = document.getElementById("perceptron.2.y_0_w_1")
    y_0_w_1.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_0_w_1_range").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_0_w_2 = document.getElementById("perceptron.2.y_0_w_2")
    y_0_w_2.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_0_w_2_range").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_0_b = document.getElementById("perceptron.2.y_0_b")
    y_0_b.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_0_b_range").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_0_w_1_range = document.getElementById("perceptron.2.y_0_w_1_range")
    y_0_w_1_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_0_w_1").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_0_w_2_range = document.getElementById("perceptron.2.y_0_w_2_range")
    y_0_w_2_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_0_w_2").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_0_b_range = document.getElementById("perceptron.2.y_0_b_range")
    y_0_b_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_0_b").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    
    let y_1_w_1 = document.getElementById("perceptron.2.y_1_w_1")
    y_1_w_1.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_1_w_1_range").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_1_w_2 = document.getElementById("perceptron.2.y_1_w_2")
    y_1_w_2.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_1_w_2_range").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_1_b = document.getElementById("perceptron.2.y_1_b")
    y_1_b.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_1_b_range").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_1_w_1_range = document.getElementById("perceptron.2.y_1_w_1_range")
    y_1_w_1_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_1_w_1").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_1_w_2_range = document.getElementById("perceptron.2.y_1_w_2_range")
    y_1_w_2_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_1_w_2").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_1_b_range = document.getElementById("perceptron.2.y_1_b_range")
    y_1_b_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_1_b").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    
    let y_2_w_1 = document.getElementById("perceptron.2.y_2_w_1")
    y_2_w_1.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_2_w_1_range").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_2_w_2 = document.getElementById("perceptron.2.y_2_w_2")
    y_2_w_2.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_2_w_2_range").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_2_b = document.getElementById("perceptron.2.y_2_b")
    y_2_b.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_2_b_range").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_2_w_1_range = document.getElementById("perceptron.2.y_2_w_1_range")
    y_2_w_1_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_2_w_1").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_2_w_2_range = document.getElementById("perceptron.2.y_2_w_2_range")
    y_2_w_2_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_2_w_2").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })
    let y_2_b_range = document.getElementById("perceptron.2.y_2_b_range")
    y_2_b_range.addEventListener("input", function(ev) {
        document.getElementById("perceptron.2.y_2_b").value = this.value
        perceptron_2_draw_canvas(pixels_wrapper)
    })

    let examples = document.getElementById("perceptron.2.examples")
    examples.addEventListener("change", function(ev) {
        perceptron_2_draw_canvas(pixels_wrapper)
    })

    let adaline_learn = document.getElementById("perceptron.2.adaline_learn")
    adaline_learn.addEventListener("click", function(ev) {
        learn_and_draw_2(pixels_wrapper)
    })

    perceptron_2_draw_canvas(pixels_wrapper)
}

function learn_and_draw_2(pixels_wrapper) {
    const y_0_w_1 = parseFloat(document.getElementById("perceptron.2.y_0_w_1").value)
    const y_0_w_2 = parseFloat(document.getElementById("perceptron.2.y_0_w_2").value)
    const y_0_b = parseFloat(document.getElementById("perceptron.2.y_0_b").value)

    const y_1_w_1 = parseFloat(document.getElementById("perceptron.2.y_1_w_1").value)
    const y_1_w_2 = parseFloat(document.getElementById("perceptron.2.y_1_w_2").value)
    const y_1_b = parseFloat(document.getElementById("perceptron.2.y_1_b").value)

    const y_2_w_1 = parseFloat(document.getElementById("perceptron.2.y_2_w_1").value)
    const y_2_w_2 = parseFloat(document.getElementById("perceptron.2.y_2_w_2").value)
    const y_2_b = parseFloat(document.getElementById("perceptron.2.y_2_b").value)

    let param_0 = new lib.PerceptronParam(y_0_w_1, y_0_w_2, y_0_b)
    let param_1 = new lib.PerceptronParam(y_1_w_1, y_1_w_2, y_1_b)
    let param_2 = new lib.PerceptronParam(y_2_w_1, y_2_w_2, y_2_b)

    const examples = document.getElementById("perceptron.2.examples").value
    const learning_rate = parseFloat(document.getElementById("perceptron.2.eta").value)

    const param_0_new = lib.perceptron_2_adaline_learn(examples, 0, param_0, learning_rate)

    document.getElementById("perceptron.2.y_0_w_1").value = param_0_new.w_1()
    document.getElementById("perceptron.2.y_0_w_1_range").value = param_0_new.w_1()
    document.getElementById("perceptron.2.y_0_w_2").value = param_0_new.w_2()
    document.getElementById("perceptron.2.y_0_w_2_range").value = param_0_new.w_2()
    document.getElementById("perceptron.2.y_0_b").value = param_0_new.b()
    document.getElementById("perceptron.2.y_0_b_range").value = param_0_new.b()

    const param_1_new = lib.perceptron_2_adaline_learn(examples, 1, param_1, learning_rate)

    document.getElementById("perceptron.2.y_1_w_1").value = param_1_new.w_1()
    document.getElementById("perceptron.2.y_1_w_1_range").value = param_1_new.w_1()
    document.getElementById("perceptron.2.y_1_w_2").value = param_1_new.w_2()
    document.getElementById("perceptron.2.y_1_w_2_range").value = param_1_new.w_2()
    document.getElementById("perceptron.2.y_1_b").value = param_1_new.b()
    document.getElementById("perceptron.2.y_1_b_range").value = param_1_new.b()

    const param_2_new = lib.perceptron_2_adaline_learn(examples, 2, param_2, learning_rate)

    document.getElementById("perceptron.2.y_2_w_1").value = param_2_new.w_1()
    document.getElementById("perceptron.2.y_2_w_1_range").value = param_2_new.w_1()
    document.getElementById("perceptron.2.y_2_w_2").value = param_2_new.w_2()
    document.getElementById("perceptron.2.y_2_w_2_range").value = param_2_new.w_2()
    document.getElementById("perceptron.2.y_2_b").value = param_2_new.b()
    document.getElementById("perceptron.2.y_2_b_range").value = param_2_new.b()

    perceptron_2_draw_canvas(pixels_wrapper)
}

function perceptron_2_draw_canvas(pixels_wrapper) {
    const y_0_w_1 = parseFloat(document.getElementById("perceptron.2.y_0_w_1").value)
    const y_0_w_2 = parseFloat(document.getElementById("perceptron.2.y_0_w_2").value)
    const y_0_b = parseFloat(document.getElementById("perceptron.2.y_0_b").value)

    const y_1_w_1 = parseFloat(document.getElementById("perceptron.2.y_1_w_1").value)
    const y_1_w_2 = parseFloat(document.getElementById("perceptron.2.y_1_w_2").value)
    const y_1_b = parseFloat(document.getElementById("perceptron.2.y_1_b").value)

    const y_2_w_1 = parseFloat(document.getElementById("perceptron.2.y_2_w_1").value)
    const y_2_w_2 = parseFloat(document.getElementById("perceptron.2.y_2_w_2").value)
    const y_2_b = parseFloat(document.getElementById("perceptron.2.y_2_b").value)

    let param_0 = new lib.PerceptronParam(y_0_w_1, y_0_w_2, y_0_b)
    let param_1 = new lib.PerceptronParam(y_1_w_1, y_1_w_2, y_1_b)
    let param_2 = new lib.PerceptronParam(y_2_w_1, y_2_w_2, y_2_b)

    const examples = document.getElementById("perceptron.2.examples").value

    lib.perceptron_2_draw_classification(param_0, param_1, param_2, pixels_wrapper)
    lib.perceptron_2_draw_examples(examples, pixels_wrapper)

    let canvas_perceptron = document.getElementById("perceptron.2.canvas")
    let ctx = canvas_perceptron.getContext("2d")
    let palette = ctx.getImageData(0, 0, pixels_wrapper.width(), pixels_wrapper.height())
    palette.data.set(new Uint8ClampedArray(pixels_wrapper.pixels().buffer))
    ctx.putImageData(palette, 0, 0)
}

async function main() {
    await init()
    perceptron_setup()
    perceptron_2_setup()
}

main()
