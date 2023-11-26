import init, * as lib from "../../assets/wasm/ml_visual.js"
import * as viz from "https://cdnjs.cloudflare.com/ajax/libs/viz.js/2.1.2/viz.es.js"

// Returns a blob:// URL which points
// to a javascript file which will call
// importScripts with the given URL
function getWorkerURL( url ) {
    const content = `importScripts( "${ url }" );`;
    return URL.createObjectURL( new Blob( [ content ], { type: "text/javascript" } ) );
}

async function render_dot(viz_instance, tree) {
    if (tree === undefined) {
        return
    }
    let features = document.getElementById("decision_tree.features").value
    const dot = tree.dot(features)
    if (dot === undefined) {
        return
    }

    const svg = await viz_instance.renderSVGElement(dot)
    let graph = document.getElementById("graph")
    graph.innerHTML = ""
    graph.appendChild(svg)
}

async function setup() {
    // Bypass the cross-origin restriction: <https://stackoverflow.com/a/62914052/9920172>
    let viz_worker_url = getWorkerURL("https://cdnjs.cloudflare.com/ajax/libs/viz.js/2.1.2/full.render.js")
    let viz_worker = new Worker(viz_worker_url)
    let viz_instance = new viz.default({ worker: viz_worker })

    let builder = new lib.WasmBinaryDecisionTreeBuilder()
    let pixels_wrapper = new lib.Pixels2DWrapper(128, 128)

    const examples = document.getElementById("decision_tree.examples").value
    let tree = builder.build(examples)

    let draw_on_event = function (ev) {
        const examples = document.getElementById("decision_tree.examples").value
        tree = builder.build(examples)

        draw_canvas(pixels_wrapper, tree)
        render_dot(viz_instance, tree)
    }
    document.getElementById("decision_tree.examples").addEventListener("input", draw_on_event)
    document.getElementById("decision_tree.features").addEventListener("input", draw_on_event)
    
    let draw_canvas_on_event = function (ev) {
        draw_canvas(pixels_wrapper, tree)
    }
    document.getElementById("decision_tree.two_features.x_axis_start").addEventListener("input", draw_canvas_on_event)
    document.getElementById("decision_tree.two_features.x_axis_end").addEventListener("input", draw_canvas_on_event)
    document.getElementById("decision_tree.two_features.y_axis_start").addEventListener("input", draw_canvas_on_event)
    document.getElementById("decision_tree.two_features.y_axis_end").addEventListener("input", draw_canvas_on_event)

    draw_canvas(pixels_wrapper, tree)
    await render_dot(viz_instance, tree)
}

function draw_canvas(pixels_wrapper, tree) {
    const x_axis_start = parseFloat(document.getElementById("decision_tree.two_features.x_axis_start").value)
    const x_axis_end = parseFloat(document.getElementById("decision_tree.two_features.x_axis_end").value)
    const y_axis_start = parseFloat(document.getElementById("decision_tree.two_features.y_axis_start").value)
    const y_axis_end = parseFloat(document.getElementById("decision_tree.two_features.y_axis_end").value)
    const examples = document.getElementById("decision_tree.examples").value

    tree.draw(examples, x_axis_start, x_axis_end, y_axis_start, y_axis_end, pixels_wrapper)

    let canvas_perceptron = document.getElementById("decision_tree.two_features.canvas")
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
