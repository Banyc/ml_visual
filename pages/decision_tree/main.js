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

    let examples = document.getElementById("decision_tree.examples").value
    let tree = builder.build(examples)

    let render_dot_on_event = function (ev) {
        let examples = document.getElementById("decision_tree.examples").value
        tree = builder.build(examples)
        render_dot(viz_instance, tree)
    }
    document.getElementById("decision_tree.examples").addEventListener("input", render_dot_on_event)
    document.getElementById("decision_tree.features").addEventListener("input", render_dot_on_event)

    await render_dot(viz_instance, tree)
}

async function main() {
    await init()
    await setup()
}

main()
