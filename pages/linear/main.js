import init, * as lib from "../../assets/wasm/ml_visual.js"
import * as binary_class from "./binary_class.js"
import * as three_classes from "./three_classes.js"

async function main() {
    await init()
    binary_class.setup()
    three_classes.setup()
}

main()
