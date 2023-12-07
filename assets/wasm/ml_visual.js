let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
* @param {string} examples
* @param {LinearTwoFeatureParam} param
* @param {number} learning_rate
* @returns {LinearTwoFeatureParam | undefined}
*/
export function perceptron_learn_binary_class(examples, param, learning_rate) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(param, LinearTwoFeatureParam);
    var ptr1 = param.__destroy_into_raw();
    const ret = wasm.perceptron_learn_binary_class(ptr0, len0, ptr1, learning_rate);
    return ret === 0 ? undefined : LinearTwoFeatureParam.__wrap(ret);
}

/**
* @param {string} examples
* @param {number} _class
* @param {LinearTwoFeatureParam} param
* @param {number} learning_rate
* @returns {LinearTwoFeatureParam | undefined}
*/
export function perceptron_learn_multiclass(examples, _class, param, learning_rate) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(param, LinearTwoFeatureParam);
    var ptr1 = param.__destroy_into_raw();
    const ret = wasm.perceptron_learn_multiclass(ptr0, len0, _class, ptr1, learning_rate);
    return ret === 0 ? undefined : LinearTwoFeatureParam.__wrap(ret);
}

/**
* @param {string} examples
* @param {LinearTwoFeatureParam} param
* @param {number} learning_rate
* @returns {LinearTwoFeatureParam | undefined}
*/
export function adaline_learn_binary_class(examples, param, learning_rate) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(param, LinearTwoFeatureParam);
    var ptr1 = param.__destroy_into_raw();
    const ret = wasm.adaline_learn_binary_class(ptr0, len0, ptr1, learning_rate);
    return ret === 0 ? undefined : LinearTwoFeatureParam.__wrap(ret);
}

/**
* @param {string} examples
* @param {number} _class
* @param {LinearTwoFeatureParam} param
* @param {number} learning_rate
* @returns {LinearTwoFeatureParam | undefined}
*/
export function adaline_learn_multiclass(examples, _class, param, learning_rate) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(param, LinearTwoFeatureParam);
    var ptr1 = param.__destroy_into_raw();
    const ret = wasm.adaline_learn_multiclass(ptr0, len0, _class, ptr1, learning_rate);
    return ret === 0 ? undefined : LinearTwoFeatureParam.__wrap(ret);
}

/**
* @param {LinearTwoFeatureParam} param
* @param {Pixels2DWrapper} pixels
*/
export function linear_draw_classification_binary_class(param, pixels) {
    _assertClass(param, LinearTwoFeatureParam);
    var ptr0 = param.__destroy_into_raw();
    _assertClass(pixels, Pixels2DWrapper);
    wasm.linear_draw_classification_binary_class(ptr0, pixels.__wbg_ptr);
}

/**
* @param {LinearTwoFeatureParam} param
* @param {Pixels2DWrapper} pixels
*/
export function logistic_regression_draw_classification_binary_class(param, pixels) {
    _assertClass(param, LinearTwoFeatureParam);
    var ptr0 = param.__destroy_into_raw();
    _assertClass(pixels, Pixels2DWrapper);
    wasm.logistic_regression_draw_classification_binary_class(ptr0, pixels.__wbg_ptr);
}

/**
* @param {string} examples
* @param {Pixels2DWrapper} pixels
*/
export function draw_examples_binary_class(examples, pixels) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(pixels, Pixels2DWrapper);
    wasm.draw_examples_binary_class(ptr0, len0, pixels.__wbg_ptr);
}

/**
* @param {LinearTwoFeatureParam} param_0
* @param {LinearTwoFeatureParam} param_1
* @param {LinearTwoFeatureParam} param_2
* @param {Pixels2DWrapper} pixels
*/
export function linear_draw_classification_three_classes(param_0, param_1, param_2, pixels) {
    _assertClass(param_0, LinearTwoFeatureParam);
    var ptr0 = param_0.__destroy_into_raw();
    _assertClass(param_1, LinearTwoFeatureParam);
    var ptr1 = param_1.__destroy_into_raw();
    _assertClass(param_2, LinearTwoFeatureParam);
    var ptr2 = param_2.__destroy_into_raw();
    _assertClass(pixels, Pixels2DWrapper);
    wasm.linear_draw_classification_three_classes(ptr0, ptr1, ptr2, pixels.__wbg_ptr);
}

/**
* @param {LinearTwoFeatureParam} param_0
* @param {LinearTwoFeatureParam} param_1
* @param {LinearTwoFeatureParam} param_2
* @param {Pixels2DWrapper} pixels
*/
export function logistic_regression_draw_classification_three_classes(param_0, param_1, param_2, pixels) {
    _assertClass(param_0, LinearTwoFeatureParam);
    var ptr0 = param_0.__destroy_into_raw();
    _assertClass(param_1, LinearTwoFeatureParam);
    var ptr1 = param_1.__destroy_into_raw();
    _assertClass(param_2, LinearTwoFeatureParam);
    var ptr2 = param_2.__destroy_into_raw();
    _assertClass(pixels, Pixels2DWrapper);
    wasm.logistic_regression_draw_classification_three_classes(ptr0, ptr1, ptr2, pixels.__wbg_ptr);
}

/**
* @param {string} examples
* @param {Pixels2DWrapper} pixels
*/
export function draw_examples_three_classes(examples, pixels) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(pixels, Pixels2DWrapper);
    wasm.draw_examples_binary_class(ptr0, len0, pixels.__wbg_ptr);
}

/**
* @param {string} examples
* @param {LinearTwoFeatureParam} param
* @param {number} learning_rate
* @param {number} regularization_parameter
* @returns {LinearTwoFeatureParam | undefined}
*/
export function logistic_regression_learn_binary_class(examples, param, learning_rate, regularization_parameter) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(param, LinearTwoFeatureParam);
    var ptr1 = param.__destroy_into_raw();
    const ret = wasm.logistic_regression_learn_binary_class(ptr0, len0, ptr1, learning_rate, regularization_parameter);
    return ret === 0 ? undefined : LinearTwoFeatureParam.__wrap(ret);
}

/**
* @param {string} examples
* @param {number} _class
* @param {LinearTwoFeatureParam} param
* @param {number} learning_rate
* @param {number} regularization_parameter
* @returns {LinearTwoFeatureParam | undefined}
*/
export function logistic_regression_learn_multiclass(examples, _class, param, learning_rate, regularization_parameter) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(param, LinearTwoFeatureParam);
    var ptr1 = param.__destroy_into_raw();
    const ret = wasm.logistic_regression_learn_multiclass(ptr0, len0, _class, ptr1, learning_rate, regularization_parameter);
    return ret === 0 ? undefined : LinearTwoFeatureParam.__wrap(ret);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
/**
*/
export class LinearTwoFeatureParam {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LinearTwoFeatureParam.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lineartwofeatureparam_free(ptr);
    }
    /**
    * @param {number} w_1
    * @param {number} w_2
    * @param {number} b
    */
    constructor(w_1, w_2, b) {
        const ret = wasm.lineartwofeatureparam_new(w_1, w_2, b);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {number}
    */
    w_1() {
        const ret = wasm.lineartwofeatureparam_w_1(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    w_2() {
        const ret = wasm.lineartwofeatureparam_w_2(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    b() {
        const ret = wasm.lineartwofeatureparam_b(this.__wbg_ptr);
        return ret;
    }
}
/**
*/
export class MulticlassExample {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_multiclassexample_free(ptr);
    }
    /**
    * @param {TwoFeatures} feature
    * @param {number} y
    */
    constructor(feature, y) {
        _assertClass(feature, TwoFeatures);
        var ptr0 = feature.__destroy_into_raw();
        const ret = wasm.multiclassexample_new(ptr0, y);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {TwoFeatures}
    */
    feature() {
        const ret = wasm.multiclassexample_feature(this.__wbg_ptr);
        return TwoFeatures.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    y() {
        const ret = wasm.multiclassexample_y(this.__wbg_ptr);
        return ret;
    }
}
/**
*/
export class Pixels2DWrapper {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pixels2dwrapper_free(ptr);
    }
    /**
    * @param {number} width
    * @param {number} height
    */
    constructor(width, height) {
        const ret = wasm.pixels2dwrapper_new(width, height);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {Uint32Array}
    */
    pixels() {
        const ret = wasm.pixels2dwrapper_pixels(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {number}
    */
    width() {
        const ret = wasm.pixels2dwrapper_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    height() {
        const ret = wasm.pixels2dwrapper_height(this.__wbg_ptr);
        return ret >>> 0;
    }
}
/**
*/
export class TwoFeatures {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TwoFeatures.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_twofeatures_free(ptr);
    }
}
/**
*/
export class WasmBinaryDecisionTree {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmBinaryDecisionTree.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmbinarydecisiontree_free(ptr);
    }
    /**
    * @param {string} feature_names
    * @returns {string | undefined}
    */
    dot(feature_names) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(feature_names, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.wasmbinarydecisiontree_dot(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v2;
            if (r0 !== 0) {
                v2 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} examples
    * @param {number} x_axis_start
    * @param {number} x_axis_end
    * @param {number} y_axis_start
    * @param {number} y_axis_end
    * @param {Pixels2DWrapper} pixels
    */
    draw(examples, x_axis_start, x_axis_end, y_axis_start, y_axis_end, pixels) {
        const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(pixels, Pixels2DWrapper);
        wasm.wasmbinarydecisiontree_draw(this.__wbg_ptr, ptr0, len0, x_axis_start, x_axis_end, y_axis_start, y_axis_end, pixels.__wbg_ptr);
    }
}
/**
*/
export class WasmBinaryDecisionTreeBuilder {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmbinarydecisiontreebuilder_free(ptr);
    }
    /**
    */
    constructor() {
        const ret = wasm.wasmbinarydecisiontreebuilder_new();
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @param {string} examples
    * @returns {WasmBinaryDecisionTree | undefined}
    */
    build(examples) {
        const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmbinarydecisiontreebuilder_build(this.__wbg_ptr, ptr0, len0);
        return ret === 0 ? undefined : WasmBinaryDecisionTree.__wrap(ret);
    }
}
/**
*/
export class WasmKnn {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmKnn.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmknn_free(ptr);
    }
    /**
    * @param {string} examples
    * @param {number} x_axis_start
    * @param {number} x_axis_end
    * @param {number} y_axis_start
    * @param {number} y_axis_end
    * @param {Pixels2DWrapper} pixels
    * @param {number} k
    */
    draw(examples, x_axis_start, x_axis_end, y_axis_start, y_axis_end, pixels, k) {
        const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(pixels, Pixels2DWrapper);
        wasm.wasmknn_draw(this.__wbg_ptr, ptr0, len0, x_axis_start, x_axis_end, y_axis_start, y_axis_end, pixels.__wbg_ptr, k);
    }
}
/**
*/
export class WasmKnnBuilder {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmknnbuilder_free(ptr);
    }
    /**
    */
    constructor() {
        const ret = wasm.wasmknnbuilder_new();
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @param {string} examples
    * @returns {WasmKnn | undefined}
    */
    build(examples) {
        const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmknnbuilder_build(this.__wbg_ptr, ptr0, len0);
        return ret === 0 ? undefined : WasmKnn.__wrap(ret);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_crypto_58f13aa23ffcb166 = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_process_5b786e71d465a513 = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_versions_c2ab80650590b6a2 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_523d7bd03ef69fba = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbg_msCrypto_abcb1295e768d1f2 = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_2784e593a4674877 = function() { return handleError(function () {
        const ret = module.require;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_randomFillSync_a0d98aa11c81fe89 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).randomFillSync(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_504510b5564925af = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_newnoargs_c62ea9419c21fbac = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_90c26b09837aba1c = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_f0e34d89f33b99fd = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_d3b084224f4774d7 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_9caa27ff917c6860 = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_35dfdd59a4da3e74 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_call_5da1969d7cd31ccd = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_buffer_a448f833075b71ba = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_d0482f893617af71 = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_8f67e318f15d7254 = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_2357bf09366ee480 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_7a23ee7b263abe07 = function(arg0, arg1, arg2) {
        const ret = new Uint32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_be6b1e731abc472c = function(arg0) {
        const ret = new Uint32Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithlength_6c2df9e2f3028c43 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_2e940e41c0f5a1d9 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('ml_visual_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
