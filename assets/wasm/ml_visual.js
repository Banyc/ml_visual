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

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
* @param {PerceptronParam} param
* @param {Pixels2DWrapper} pixels
*/
export function perceptron_draw_classification(param, pixels) {
    _assertClass(param, PerceptronParam);
    var ptr0 = param.__destroy_into_raw();
    _assertClass(pixels, Pixels2DWrapper);
    wasm.perceptron_draw_classification(ptr0, pixels.__wbg_ptr);
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
/**
* @param {string} examples
* @param {Pixels2DWrapper} pixels
*/
export function perceptron_draw_examples(examples, pixels) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(pixels, Pixels2DWrapper);
    wasm.perceptron_draw_examples(ptr0, len0, pixels.__wbg_ptr);
}

/**
* @param {string} examples
* @param {PerceptronParam} param
* @param {number} learning_rate
* @returns {PerceptronParam | undefined}
*/
export function perceptron_learn(examples, param, learning_rate) {
    const ptr0 = passStringToWasm0(examples, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(param, PerceptronParam);
    var ptr1 = param.__destroy_into_raw();
    const ret = wasm.perceptron_learn(ptr0, len0, ptr1, learning_rate);
    return ret === 0 ? undefined : PerceptronParam.__wrap(ret);
}

/**
*/
export class PerceptronExample {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_perceptronexample_free(ptr);
    }
    /**
    * @param {number} x_1
    * @param {number} x_2
    * @param {boolean} y
    */
    constructor(x_1, x_2, y) {
        const ret = wasm.perceptronexample_new(x_1, x_2, y);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {number}
    */
    x_1() {
        const ret = wasm.perceptronexample_x_1(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    x_2() {
        const ret = wasm.perceptronexample_x_2(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {boolean}
    */
    y() {
        const ret = wasm.perceptronexample_y(this.__wbg_ptr);
        return ret !== 0;
    }
}
/**
*/
export class PerceptronParam {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PerceptronParam.prototype);
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
        wasm.__wbg_perceptronparam_free(ptr);
    }
    /**
    * @param {number} w_1
    * @param {number} w_2
    * @param {number} b
    */
    constructor(w_1, w_2, b) {
        const ret = wasm.perceptronparam_new(w_1, w_2, b);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {number}
    */
    w_1() {
        const ret = wasm.perceptronexample_x_1(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    w_2() {
        const ret = wasm.perceptronexample_x_2(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    b() {
        const ret = wasm.perceptronparam_b(this.__wbg_ptr);
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
    imports.wbg.__wbg_buffer_344d9b41efe96da7 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_666c0bd209289750 = function(arg0, arg1, arg2) {
        const ret = new Uint32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_3feb964f0aedb844 = function(arg0) {
        const ret = new Uint32Array(getObject(arg0));
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