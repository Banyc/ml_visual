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
        const w_1 = parseFloat(document.getElementById("perceptron.w_1").value)
        const w_2 = parseFloat(document.getElementById("perceptron.w_2").value)
        const b = parseFloat(document.getElementById("perceptron.b").value)
        let param = new lib.PerceptronParam(w_1, w_2, b)
        const learning_rate = parseFloat(document.getElementById("perceptron.eta").value)
        const examples = document.getElementById("perceptron.examples").value

        const param_new = lib.perceptron_learn(examples, param, learning_rate)
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
    })

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

async function main() {
    await init()
    perceptron_setup()
}

main()
