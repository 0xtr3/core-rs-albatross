let wasm_bindgen;
(function() {
    const __exports = {};
    let script_src;
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }
    let wasm = undefined;

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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
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

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let cachedBigInt64Memory0 = null;

function getBigInt64Memory0() {
    if (cachedBigInt64Memory0 === null || cachedBigInt64Memory0.byteLength === 0) {
        cachedBigInt64Memory0 = new BigInt64Array(wasm.memory.buffer);
    }
    return cachedBigInt64Memory0;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

const CLOSURE_DTORS = new FinalizationRegistry(state => {
    wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state)
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_48(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures__invoke0_mut__hef811265daa3a595(arg0, arg1);
}

function __wbg_adapter_51(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures__invoke0_mut__h44c50f1b99f904ff(arg0, arg1);
}

function __wbg_adapter_54(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures__invoke1_mut__h21b0590f73469ed7(arg0, arg1, addHeapObject(arg2));
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;
                CLOSURE_DTORS.unregister(state)
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_61(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5ecdb55d58a32349(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_64(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures__invoke1_mut__he76e9651c19926b4(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_67(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures__invoke0_mut__h6ead74bc406c3080(arg0, arg1);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
function __wbg_adapter_136(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h2ff626fa09ccddc4(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
*/
__exports.TransactionFormat = Object.freeze({ Basic:0,"0":"Basic",Extended:1,"1":"Extended", });
/**
*/
__exports.AccountType = Object.freeze({ Basic:0,"0":"Basic",Vesting:1,"1":"Vesting",HTLC:2,"2":"HTLC",Staking:3,"3":"Staking", });

const AddressFinalization = new FinalizationRegistry(ptr => wasm.__wbg_address_free(ptr >>> 0));
/**
* An object representing a Nimiq address.
* Offers methods to parse and format addresses from and to strings.
*/
class Address {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Address.prototype);
        obj.__wbg_ptr = ptr;
        AddressFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        AddressFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_address_free(ptr);
    }
    /**
    * @param {Uint8Array} bytes
    */
    constructor(bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.address_new(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Parses an address from an {@link Address} instance or a string representation.
    *
    * Throws when an address cannot be parsed from the argument.
    * @param {string} addr
    * @returns {Address}
    */
    static fromAny(addr) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.address_fromAny(retptr, addBorrowedObject(addr));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Address.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Parses an address from a string representation, either user-friendly or hex format.
    *
    * Throws when an address cannot be parsed from the string.
    * @param {string} str
    * @returns {Address}
    */
    static fromString(str) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.address_fromString(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Address.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Formats the address into a plain string format.
    * @returns {string}
    */
    toPlain() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.address_toPlain(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
__exports.Address = Address;

const ClientFinalization = new FinalizationRegistry(ptr => wasm.__wbg_client_free(ptr >>> 0));
/**
* Nimiq Albatross client that runs in browsers via WASM and is exposed to Javascript.
*
* ### Usage:
*
* ```js
* import init, * as Nimiq from "./pkg/nimiq_web_client.js";
*
* init().then(async () => {
*     const config = new Nimiq.ClientConfiguration();
*     const client = await config.instantiateClient();
*     // ...
* });
* ```
*/
class Client {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Client.prototype);
        obj.__wbg_ptr = ptr;
        ClientFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ClientFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_client_free(ptr);
    }
    /**
    * Creates a new Client that automatically starts connecting to the network.
    * @param {PlainClientConfiguration} config
    * @returns {Promise<Client>}
    */
    static create(config) {
        const ret = wasm.client_create(addHeapObject(config));
        return takeObject(ret);
    }
    /**
    * Adds an event listener for consensus-change events, such as when consensus is established or lost.
    * @param {(state: ConsensusState) => any} listener
    * @returns {Promise<number>}
    */
    addConsensusChangedListener(listener) {
        const ret = wasm.client_addConsensusChangedListener(this.__wbg_ptr, addHeapObject(listener));
        return takeObject(ret);
    }
    /**
    * Adds an event listener for new blocks added to the blockchain.
    * @param {(hash: string, reason: string, reverted_blocks: string[], adopted_blocks: string[]) => any} listener
    * @returns {Promise<number>}
    */
    addHeadChangedListener(listener) {
        const ret = wasm.client_addHeadChangedListener(this.__wbg_ptr, addHeapObject(listener));
        return takeObject(ret);
    }
    /**
    * Adds an event listener for peer-change events, such as when a new peer joins, or a peer leaves.
    * @param {(peer_id: string, reason: 'joined' | 'left', peer_count: number, peer_info?: PlainPeerInfo) => any} listener
    * @returns {Promise<number>}
    */
    addPeerChangedListener(listener) {
        const ret = wasm.client_addPeerChangedListener(this.__wbg_ptr, addHeapObject(listener));
        return takeObject(ret);
    }
    /**
    * Adds an event listener for transactions to and from the provided addresses.
    *
    * The listener is called for transactions when they are _included_ in the blockchain.
    * @param {(transaction: PlainTransactionDetails) => any} listener
    * @param {string[]} addresses
    * @returns {Promise<number>}
    */
    addTransactionListener(listener, addresses) {
        const ret = wasm.client_addTransactionListener(this.__wbg_ptr, addHeapObject(listener), addHeapObject(addresses));
        return takeObject(ret);
    }
    /**
    * Removes an event listener by its handle.
    * @param {number} handle
    * @returns {Promise<void>}
    */
    removeListener(handle) {
        const ret = wasm.client_removeListener(this.__wbg_ptr, handle);
        return takeObject(ret);
    }
    /**
    * Returns the network ID that the client is connecting to.
    * @returns {Promise<number>}
    */
    getNetworkId() {
        const ret = wasm.client_getNetworkId(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns if the client currently has consensus with the network.
    * @returns {Promise<boolean>}
    */
    isConsensusEstablished() {
        const ret = wasm.client_isConsensusEstablished(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns a promise that resolves when the client has established consensus with the network.
    * @returns {Promise<void>}
    */
    waitForConsensusEstablished() {
        const ret = wasm.client_waitForConsensusEstablished(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns the block hash of the current blockchain head.
    * @returns {Promise<string>}
    */
    getHeadHash() {
        const ret = wasm.client_getHeadHash(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns the block number of the current blockchain head.
    * @returns {Promise<number>}
    */
    getHeadHeight() {
        const ret = wasm.client_getHeadHeight(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns the current blockchain head block.
    * Note that the web client is a light client and does not have block bodies, i.e. no transactions.
    * @returns {Promise<PlainBlock>}
    */
    getHeadBlock() {
        const ret = wasm.client_getHeadBlock(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Fetches a block by its hash.
    *
    * Throws if the client does not have the block.
    *
    * Fetching blocks from the network is not yet available.
    * @param {string} hash
    * @returns {Promise<PlainBlock>}
    */
    getBlock(hash) {
        const ptr0 = passStringToWasm0(hash, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.client_getBlock(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * Fetches a block by its height (block number).
    *
    * Throws if the client does not have the block.
    *
    * Fetching blocks from the network is not yet available.
    * @param {number} height
    * @returns {Promise<PlainBlock>}
    */
    getBlockAt(height) {
        const ret = wasm.client_getBlockAt(this.__wbg_ptr, height);
        return takeObject(ret);
    }
    /**
    * Fetches the account for the provided address from the network.
    *
    * Throws if the address cannot be parsed and on network errors.
    * @param {string} address
    * @returns {Promise<PlainAccount>}
    */
    getAccount(address) {
        const ret = wasm.client_getAccount(this.__wbg_ptr, addHeapObject(address));
        return takeObject(ret);
    }
    /**
    * Fetches the accounts for the provided addresses from the network.
    *
    * Throws if an address cannot be parsed and on network errors.
    * @param {string[]} addresses
    * @returns {Promise<PlainAccount[]>}
    */
    getAccounts(addresses) {
        const ret = wasm.client_getAccounts(this.__wbg_ptr, addHeapObject(addresses));
        return takeObject(ret);
    }
    /**
    * Fetches the staker for the provided address from the network.
    *
    * Throws if the address cannot be parsed and on network errors.
    * @param {string} address
    * @returns {Promise<PlainStaker | undefined>}
    */
    getStaker(address) {
        const ret = wasm.client_getStaker(this.__wbg_ptr, addHeapObject(address));
        return takeObject(ret);
    }
    /**
    * Fetches the stakers for the provided addresses from the network.
    *
    * Throws if an address cannot be parsed and on network errors.
    * @param {string[]} addresses
    * @returns {Promise<(PlainStaker | undefined)[]>}
    */
    getStakers(addresses) {
        const ret = wasm.client_getStakers(this.__wbg_ptr, addHeapObject(addresses));
        return takeObject(ret);
    }
    /**
    * Fetches the validator for the provided address from the network.
    *
    * Throws if the address cannot be parsed and on network errors.
    * @param {string} address
    * @returns {Promise<PlainValidator | undefined>}
    */
    getValidator(address) {
        const ret = wasm.client_getValidator(this.__wbg_ptr, addHeapObject(address));
        return takeObject(ret);
    }
    /**
    * Fetches the validators for the provided addresses from the network.
    *
    * Throws if an address cannot be parsed and on network errors.
    * @param {string[]} addresses
    * @returns {Promise<(PlainValidator | undefined)[]>}
    */
    getValidators(addresses) {
        const ret = wasm.client_getValidators(this.__wbg_ptr, addHeapObject(addresses));
        return takeObject(ret);
    }
    /**
    * Sends a transaction to the network and returns {@link PlainTransactionDetails}.
    *
    * Throws in case of network errors.
    * @param {PlainTransaction | string} transaction
    * @returns {Promise<PlainTransactionDetails>}
    */
    sendTransaction(transaction) {
        const ret = wasm.client_sendTransaction(this.__wbg_ptr, addHeapObject(transaction));
        return takeObject(ret);
    }
    /**
    * Fetches the transaction details for the given transaction hash.
    * @param {string} hash
    * @returns {Promise<PlainTransactionDetails>}
    */
    getTransaction(hash) {
        const ptr0 = passStringToWasm0(hash, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.client_getTransaction(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * This function is used to query the network for transaction receipts from and to a
    * specific address, that have been included in the chain.
    *
    * The obtained receipts are _not_ verified before being returned.
    *
    * Up to a `limit` number of transaction receipts are returned from newest to oldest.
    * If the network does not have at least `min_peers` to query, then an error is returned.
    * @param {string} address
    * @param {number | undefined} [limit]
    * @param {number | undefined} [min_peers]
    * @returns {Promise<PlainTransactionReceipt[]>}
    */
    getTransactionReceiptsByAddress(address, limit, min_peers) {
        const ret = wasm.client_getTransactionReceiptsByAddress(this.__wbg_ptr, addHeapObject(address), isLikeNone(limit) ? 0xFFFFFF : limit, !isLikeNone(min_peers), isLikeNone(min_peers) ? 0 : min_peers);
        return takeObject(ret);
    }
    /**
    * This function is used to query the network for transactions from and to a specific
    * address, that have been included in the chain.
    *
    * The obtained transactions are verified before being returned.
    *
    * Up to a `limit` number of transactions are returned from newest to oldest.
    * If the network does not have at least `min_peers` to query, then an error is returned.
    * @param {string} address
    * @param {number | undefined} [since_block_height]
    * @param {PlainTransactionDetails[] | undefined} [known_transaction_details]
    * @param {number | undefined} [limit]
    * @param {number | undefined} [min_peers]
    * @returns {Promise<PlainTransactionDetails[]>}
    */
    getTransactionsByAddress(address, since_block_height, known_transaction_details, limit, min_peers) {
        const ret = wasm.client_getTransactionsByAddress(this.__wbg_ptr, addHeapObject(address), !isLikeNone(since_block_height), isLikeNone(since_block_height) ? 0 : since_block_height, isLikeNone(known_transaction_details) ? 0 : addHeapObject(known_transaction_details), isLikeNone(limit) ? 0xFFFFFF : limit, !isLikeNone(min_peers), isLikeNone(min_peers) ? 0 : min_peers);
        return takeObject(ret);
    }
}
__exports.Client = Client;

const ClientConfigurationFinalization = new FinalizationRegistry(ptr => wasm.__wbg_clientconfiguration_free(ptr >>> 0));
/**
* Use this to provide initialization-time configuration to the Client.
* This is a simplified version of the configuration that is used for regular nodes,
* since not all configuration knobs are available when running inside a browser.
*/
class ClientConfiguration {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ClientConfigurationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_clientconfiguration_free(ptr);
    }
}
__exports.ClientConfiguration = ClientConfiguration;

const PolicyFinalization = new FinalizationRegistry(ptr => wasm.__wbg_policy_free(ptr >>> 0));
/**
*/
class Policy {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PolicyFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_policy_free(ptr);
    }
    /**
    * Number of batches a transaction is valid with Albatross consensus.
    * @returns {number}
    */
    static get TRANSACTION_VALIDITY_WINDOW() {
        const ret = wasm.policy_transaction_validity_window();
        return ret >>> 0;
    }
    /**
    * Number of blocks a transaction is valid with Albatross consensus.
    * @returns {number}
    */
    static get TRANSACTION_VALIDITY_WINDOW_BLOCKS() {
        const ret = wasm.policy_transaction_validity_window_blocks();
        return ret >>> 0;
    }
    /**
    * How many batches constitute an epoch
    * @returns {number}
    */
    static get BATCHES_PER_EPOCH() {
        const ret = wasm.policy_batches_per_epoch();
        return ret;
    }
    /**
    * Length of a batch including the macro block
    * @returns {number}
    */
    static get BLOCKS_PER_BATCH() {
        const ret = wasm.policy_blocks_per_batch();
        return ret >>> 0;
    }
    /**
    * Length of an epoch including the election block
    * @returns {number}
    */
    static get BLOCKS_PER_EPOCH() {
        const ret = wasm.policy_blocks_per_epoch();
        return ret >>> 0;
    }
    /**
    * Genesis block number
    * @returns {number}
    */
    static get GENESIS_BLOCK_NUMBER() {
        const ret = wasm.policy_genesis_block_number();
        return ret >>> 0;
    }
    /**
    * Tendermint's initial timeout, in milliseconds.
    *
    * See <https://arxiv.org/abs/1807.04938v3> for more information.
    * @returns {bigint}
    */
    static get TENDERMINT_TIMEOUT_INIT() {
        const ret = wasm.policy_tendermint_timeout_init();
        return BigInt.asUintN(64, ret);
    }
    /**
    * Tendermint's timeout delta, in milliseconds.
    *
    * See <https://arxiv.org/abs/1807.04938v3> for more information.
    * @returns {bigint}
    */
    static get TENDERMINT_TIMEOUT_DELTA() {
        const ret = wasm.policy_tendermint_timeout_delta();
        return BigInt.asUintN(64, ret);
    }
    /**
    * Maximum size of accounts trie chunks.
    * @returns {number}
    */
    static get STATE_CHUNKS_MAX_SIZE() {
        const ret = wasm.policy_state_chunks_max_size();
        return ret >>> 0;
    }
    /**
    * Returns the epoch number at a given block number (height).
    * @param {number} block_number
    * @returns {number}
    */
    static epochAt(block_number) {
        const ret = wasm.policy_epochAt(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the epoch index at a given block number. The epoch index is the number of a block relative
    * to the epoch it is in. For example, the first block of any epoch always has an epoch index of 0.
    * @param {number} block_number
    * @returns {number}
    */
    static epochIndexAt(block_number) {
        const ret = wasm.policy_epochIndexAt(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the batch number at a given `block_number` (height)
    * @param {number} block_number
    * @returns {number}
    */
    static batchAt(block_number) {
        const ret = wasm.policy_batchAt(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the batch index at a given block number. The batch index is the number of a block relative
    * to the batch it is in. For example, the first block of any batch always has an batch index of 0.
    * @param {number} block_number
    * @returns {number}
    */
    static batchIndexAt(block_number) {
        const ret = wasm.policy_batchIndexAt(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the number (height) of the next election macro block after a given block number (height).
    * @param {number} block_number
    * @returns {number}
    */
    static electionBlockAfter(block_number) {
        const ret = wasm.policy_electionBlockAfter(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the block number (height) of the preceding election macro block before a given block number (height).
    * If the given block number is an election macro block, it returns the election macro block before it.
    * @param {number} block_number
    * @returns {number}
    */
    static electionBlockBefore(block_number) {
        const ret = wasm.policy_electionBlockBefore(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the block number (height) of the last election macro block at a given block number (height).
    * If the given block number is an election macro block, then it returns that block number.
    * @param {number} block_number
    * @returns {number}
    */
    static lastElectionBlock(block_number) {
        const ret = wasm.policy_lastElectionBlock(block_number);
        return ret >>> 0;
    }
    /**
    * Returns a boolean expressing if the block at a given block number (height) is an election macro block.
    * @param {number} block_number
    * @returns {boolean}
    */
    static isElectionBlockAt(block_number) {
        const ret = wasm.policy_isElectionBlockAt(block_number);
        return ret !== 0;
    }
    /**
    * Returns the block number (height) of the next macro block after a given block number (height).
    * @param {number} block_number
    * @returns {number}
    */
    static macroBlockAfter(block_number) {
        const ret = wasm.policy_macroBlockAfter(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the block number (height) of the preceding macro block before a given block number (height).
    * If the given block number is a macro block, it returns the macro block before it.
    * @param {number} block_number
    * @returns {number}
    */
    static macroBlockBefore(block_number) {
        const ret = wasm.policy_macroBlockBefore(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the block number (height) of the last macro block at a given block number (height).
    * If the given block number is a macro block, then it returns that block number.
    * @param {number} block_number
    * @returns {number}
    */
    static lastMacroBlock(block_number) {
        const ret = wasm.policy_lastMacroBlock(block_number);
        return ret >>> 0;
    }
    /**
    * Returns a boolean expressing if the block at a given block number (height) is a macro block.
    * @param {number} block_number
    * @returns {boolean}
    */
    static isMacroBlockAt(block_number) {
        const ret = wasm.policy_isMacroBlockAt(block_number);
        return ret !== 0;
    }
    /**
    * Returns a boolean expressing if the block at a given block number (height) is a micro block.
    * @param {number} block_number
    * @returns {boolean}
    */
    static isMicroBlockAt(block_number) {
        const ret = wasm.policy_isMicroBlockAt(block_number);
        return ret !== 0;
    }
    /**
    * Returns the block number of the first block of the given epoch (which is always a micro block).
    * If the index is out of bounds, None is returned
    * @param {number} epoch
    * @returns {number | undefined}
    */
    static firstBlockOf(epoch) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.policy_firstBlockOf(retptr, epoch);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the block number of the first block of the given batch (which is always a micro block).
    * If the index is out of bounds, None is returned
    * @param {number} batch
    * @returns {number | undefined}
    */
    static firstBlockOfBatch(batch) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.policy_firstBlockOfBatch(retptr, batch);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the block number of the election macro block of the given epoch (which is always the last block).
    * If the index is out of bounds, None is returned
    * @param {number} epoch
    * @returns {number | undefined}
    */
    static electionBlockOf(epoch) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.policy_electionBlockOf(retptr, epoch);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the block number of the macro block (checkpoint or election) of the given batch (which
    * is always the last block).
    * If the index is out of bounds, None is returned
    * @param {number} batch
    * @returns {number | undefined}
    */
    static macroBlockOf(batch) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.policy_macroBlockOf(retptr, batch);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a boolean expressing if the batch at a given block number (height) is the first batch
    * of the epoch.
    * @param {number} block_number
    * @returns {boolean}
    */
    static firstBatchOfEpoch(block_number) {
        const ret = wasm.policy_firstBatchOfEpoch(block_number);
        return ret !== 0;
    }
    /**
    * Returns the block height for the last block of the reporting window of a given block number.
    * Note: This window is meant for reporting malicious behaviour (aka `jailable` behaviour).
    * @param {number} block_number
    * @returns {number}
    */
    static lastBlockOfReportingWindow(block_number) {
        const ret = wasm.policy_lastBlockOfReportingWindow(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the first block after the reporting window of a given block number has ended.
    * @param {number} block_number
    * @returns {number}
    */
    static blockAfterReportingWindow(block_number) {
        const ret = wasm.policy_blockAfterReportingWindow(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the first block after the jail period of a given block number has ended.
    * @param {number} block_number
    * @returns {number}
    */
    static blockAfterJail(block_number) {
        const ret = wasm.policy_blockAfterJail(block_number);
        return ret >>> 0;
    }
    /**
    * Returns the supply at a given time (as Unix time) in Lunas (1 NIM = 100,000 Lunas). It is
    * calculated using the following formula:
    * Supply (t) = Genesis_supply + Initial_supply_velocity / Supply_decay * (1 - e^(- Supply_decay * t))
    * Where e is the exponential function, t is the time in milliseconds since the genesis block and
    * Genesis_supply is the supply at the genesis of the Nimiq 2.0 chain.
    * @param {bigint} genesis_supply
    * @param {bigint} genesis_time
    * @param {bigint} current_time
    * @returns {bigint}
    */
    static supplyAt(genesis_supply, genesis_time, current_time) {
        const ret = wasm.policy_supplyAt(genesis_supply, genesis_time, current_time);
        return BigInt.asUintN(64, ret);
    }
    /**
    * Returns the percentage reduction that should be applied to the rewards due to a delayed batch.
    * This function returns a float in the range [0, 1]
    * I.e 1 means that the full rewards should be given, whereas 0.5 means that half of the rewards should be given
    * The input to this function is the batch delay, in milliseconds
    * The function is: [(1 - MINIMUM_REWARDS_PERCENTAGE) * e ^(-BLOCKS_DELAY_DECAY * t^2)] + MINIMUM_REWARDS_PERCENTAGE
    * @param {bigint} delay
    * @returns {number}
    */
    static batchDelayPenalty(delay) {
        const ret = wasm.policy_batchDelayPenalty(delay);
        return ret;
    }
    /**
    * This is the address for the staking contract.
    * @returns {string}
    */
    static get STAKING_CONTRACT_ADDRESS() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.policy_wasm_staking_contract_address(retptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * This is the address for the coinbase. Note that this is not a real account, it is just the
    * address we use to denote that some coins originated from a coinbase event.
    * @returns {string}
    */
    static get COINBASE_ADDRESS() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.policy_wasm_coinbase_address(retptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * The maximum allowed size, in bytes, for a micro block body.
    * @returns {number}
    */
    static get MAX_SIZE_MICRO_BODY() {
        const ret = wasm.policy_wasm_max_size_micro_body();
        return ret >>> 0;
    }
    /**
    * The current version number of the protocol. Changing this always results in a hard fork.
    * @returns {number}
    */
    static get VERSION() {
        const ret = wasm.policy_wasm_min_epochs_stored();
        return ret;
    }
    /**
    * Number of available validator slots. Note that a single validator may own several validator slots.
    * @returns {number}
    */
    static get SLOTS() {
        const ret = wasm.policy_wasm_slots();
        return ret;
    }
    /**
    * Calculates 2f+1 slots which is the minimum number of slots necessary to produce a macro block,
    * a skip block and other actions.
    * It is also the minimum number of slots necessary to be guaranteed to have a majority of honest
    * slots. That's because from a total of 3f+1 slots at most f will be malicious. If in a group of
    * 2f+1 slots we have f malicious ones (which is the worst case scenario), that still leaves us
    * with f+1 honest slots. Which is more than the f slots that are not in this group (which must all
    * be honest).
    * It is calculated as `ceil(SLOTS*2/3)` and we use the formula `ceil(x/y) = (x+y-1)/y` for the
    * ceiling division.
    * @returns {number}
    */
    static get TWO_F_PLUS_ONE() {
        const ret = wasm.policy_wasm_two_f_plus_one();
        return ret;
    }
    /**
    * Calculates f+1 slots which is the minimum number of slots necessary to be guaranteed to have at
    * least one honest slots. That's because from a total of 3f+1 slots at most f will be malicious.
    * It is calculated as `ceil(SLOTS/3)` and we use the formula `ceil(x/y) = (x+y-1)/y` for the
    * ceiling division.
    * @returns {number}
    */
    static get F_PLUS_ONE() {
        const ret = wasm.policy_wasm_f_plus_one();
        return ret;
    }
    /**
    * The timeout in milliseconds for a validator to produce a block (2s)
    * @returns {bigint}
    */
    static get BLOCK_PRODUCER_TIMEOUT() {
        const ret = wasm.policy_wasm_block_producer_timeout();
        return BigInt.asUintN(64, ret);
    }
    /**
    * The optimal time in milliseconds between blocks (1s)
    * @returns {bigint}
    */
    static get BLOCK_SEPARATION_TIME() {
        const ret = wasm.policy_wasm_block_separation_time();
        return BigInt.asUintN(64, ret);
    }
    /**
    * Minimum number of epochs that the ChainStore will store fully
    * @returns {number}
    */
    static get MIN_EPOCHS_STORED() {
        const ret = wasm.policy_wasm_min_epochs_stored();
        return ret >>> 0;
    }
    /**
    * The maximum drift, in milliseconds, that is allowed between any block's timestamp and the node's
    * system time. We only care about drifting to the future.
    * @returns {bigint}
    */
    static get TIMESTAMP_MAX_DRIFT() {
        const ret = wasm.policy_wasm_timestamp_max_drift();
        return BigInt.asUintN(64, ret);
    }
    /**
    * The slope of the exponential decay used to punish validators for not producing block in time
    * @returns {number}
    */
    static get BLOCKS_DELAY_DECAY() {
        const ret = wasm.policy_wasm_blocks_delay_decay();
        return ret;
    }
    /**
    * The minimum rewards percentage that we allow
    * @returns {number}
    */
    static get MINIMUM_REWARDS_PERCENTAGE() {
        const ret = wasm.policy_wasm_minimum_rewards_percentage();
        return ret;
    }
    /**
    * The deposit necessary to create a validator in Lunas (1 NIM = 100,000 Lunas).
    * A validator is someone who actually participates in block production. They are akin to miners
    * in proof-of-work.
    * @returns {bigint}
    */
    static get VALIDATOR_DEPOSIT() {
        const ret = wasm.policy_wasm_validator_deposit();
        return BigInt.asUintN(64, ret);
    }
    /**
    * The number of epochs a validator is put in jail for. The jailing only happens for severe offenses.
    * @returns {number}
    */
    static get JAIL_EPOCHS() {
        const ret = wasm.policy_wasm_jail_epochs();
        return ret >>> 0;
    }
    /**
    * Total supply in units.
    * @returns {bigint}
    */
    static get TOTAL_SUPPLY() {
        const ret = wasm.policy_wasm_total_supply();
        return BigInt.asUintN(64, ret);
    }
    /**
    * This is the number of Lunas (1 NIM = 100,000 Lunas) created by millisecond at the genesis of the
    * Nimiq 2.0 chain. The velocity then decreases following the formula:
    * Supply_velocity (t) = Initial_supply_velocity * e^(- Supply_decay * t)
    * Where e is the exponential function and t is the time in milliseconds since the genesis block.
    * @returns {number}
    */
    static get INITIAL_SUPPLY_VELOCITY() {
        const ret = wasm.policy_wasm_initial_supply_velocity();
        return ret;
    }
    /**
    * The supply decay is a constant that is calculated so that the supply velocity decreases at a
    * steady 1.47% per year.
    * @returns {number}
    */
    static get SUPPLY_DECAY() {
        const ret = wasm.policy_wasm_supply_decay();
        return ret;
    }
    /**
    * The maximum size of the BLS public key cache.
    * @returns {number}
    */
    static get BLS_CACHE_MAX_CAPACITY() {
        const ret = wasm.policy_wasm_bls_cache_max_capacity();
        return ret >>> 0;
    }
    /**
    * Maximum size of history chunks.
    * 25 MB.
    * @returns {bigint}
    */
    static get HISTORY_CHUNKS_MAX_SIZE() {
        const ret = wasm.policy_wasm_history_chunks_max_size();
        return BigInt.asUintN(64, ret);
    }
}
__exports.Policy = Policy;

const TransactionFinalization = new FinalizationRegistry(ptr => wasm.__wbg_transaction_free(ptr >>> 0));
/**
* Transactions describe a transfer of value, usually from the sender to the recipient.
* However, transactions can also have no value, when they are used to _signal_ a change in the staking contract.
*
* Transactions can be used to create contracts, such as vesting contracts and HTLCs.
*
* Transactions require a valid signature proof over their serialized content.
* Furthermore, transactions are only valid for 2 hours after their validity-start block height.
*/
class Transaction {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Transaction.prototype);
        obj.__wbg_ptr = ptr;
        TransactionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TransactionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_transaction_free(ptr);
    }
    /**
    * Creates a new unsigned transaction that transfers `value` amount of luna (NIM's smallest unit)
    * from the sender to the recipient, where both sender and recipient can be any account type,
    * and custom extra data can be added to the transaction.
    *
    * ### Basic transactions
    * If both the sender and recipient types are omitted or `0` and both data and flags are empty,
    * a smaller basic transaction is created.
    *
    * ### Extended transactions
    * If no flags are given, but sender type is not basic (`0`) or data is set, an extended
    * transaction is created.
    *
    * ### Contract creation transactions
    * To create a new vesting or HTLC contract, set `flags` to `0b1` and specify the contract
    * type as the `recipient_type`: `1` for vesting, `2` for HTLC. The `data` bytes must have
    * the correct format of contract creation data for the respective contract type.
    *
    * ### Signaling transactions
    * To interact with the staking contract, signaling transaction are often used to not
    * transfer any value, but to simply _signal_ a state change instead, such as changing one's
    * delegation from one validator to another. To create such a transaction, set `flags` to `
    * 0b10` and populate the `data` bytes accordingly.
    *
    * The returned transaction is not yet signed. You can sign it e.g. with `tx.sign(keyPair)`.
    *
    * Throws when an account type is unknown, the numbers given for value and fee do not fit
    * within a u64 or the networkId is unknown. Also throws when no data or recipient type is
    * given for contract creation transactions, or no data is given for signaling transactions.
    * @param {Address} sender
    * @param {number | undefined} sender_type
    * @param {Uint8Array | undefined} sender_data
    * @param {Address} recipient
    * @param {number | undefined} recipient_type
    * @param {Uint8Array | undefined} recipient_data
    * @param {bigint} value
    * @param {bigint} fee
    * @param {number | undefined} flags
    * @param {number} validity_start_height
    * @param {number} network_id
    */
    constructor(sender, sender_type, sender_data, recipient, recipient_type, recipient_data, value, fee, flags, validity_start_height, network_id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(sender, Address);
            var ptr0 = isLikeNone(sender_data) ? 0 : passArray8ToWasm0(sender_data, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            _assertClass(recipient, Address);
            var ptr1 = isLikeNone(recipient_data) ? 0 : passArray8ToWasm0(recipient_data, wasm.__wbindgen_malloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.transaction_new(retptr, sender.__wbg_ptr, isLikeNone(sender_type) ? 0xFFFFFF : sender_type, ptr0, len0, recipient.__wbg_ptr, isLikeNone(recipient_type) ? 0xFFFFFF : recipient_type, ptr1, len1, value, fee, isLikeNone(flags) ? 0xFFFFFF : flags, validity_start_height, network_id);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Computes the transaction's hash, which is used as its unique identifier on the blockchain.
    * @returns {string}
    */
    hash() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_hash(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Verifies that a transaction has valid properties and a valid signature proof.
    * Optionally checks if the transaction is valid on the provided network.
    *
    * **Throws with any transaction validity error.** Returns without exception if the transaction is valid.
    *
    * Throws when the given networkId is unknown.
    * @param {number | undefined} [network_id]
    */
    verify(network_id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_verify(retptr, this.__wbg_ptr, isLikeNone(network_id) ? 0xFFFFFF : network_id);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Tests if the transaction is valid at the specified block height.
    * @param {number} block_height
    * @returns {boolean}
    */
    isValidAt(block_height) {
        const ret = wasm.transaction_isValidAt(this.__wbg_ptr, block_height);
        return ret !== 0;
    }
    /**
    * Returns the address of the contract that is created with this transaction.
    * @returns {Address}
    */
    getContractCreationAddress() {
        const ret = wasm.transaction_getContractCreationAddress(this.__wbg_ptr);
        return Address.__wrap(ret);
    }
    /**
    * Serializes the transaction's content to be used for creating its signature.
    * @returns {Uint8Array}
    */
    serializeContent() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_serializeContent(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes the transaction to a byte array.
    * @returns {Uint8Array}
    */
    serialize() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_serialize(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * The transaction's {@link TransactionFormat}.
    * @returns {TransactionFormat}
    */
    get format() {
        const ret = wasm.transaction_format(this.__wbg_ptr);
        return ret;
    }
    /**
    * The transaction's sender address.
    * @returns {Address}
    */
    get sender() {
        const ret = wasm.transaction_sender(this.__wbg_ptr);
        return Address.__wrap(ret);
    }
    /**
    * The transaction's sender {@link AccountType}.
    * @returns {AccountType}
    */
    get senderType() {
        const ret = wasm.transaction_senderType(this.__wbg_ptr);
        return ret;
    }
    /**
    * The transaction's recipient address.
    * @returns {Address}
    */
    get recipient() {
        const ret = wasm.transaction_recipient(this.__wbg_ptr);
        return Address.__wrap(ret);
    }
    /**
    * The transaction's recipient {@link AccountType}.
    * @returns {AccountType}
    */
    get recipientType() {
        const ret = wasm.transaction_recipientType(this.__wbg_ptr);
        return ret;
    }
    /**
    * The transaction's value in luna (NIM's smallest unit).
    * @returns {bigint}
    */
    get value() {
        const ret = wasm.transaction_value(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * The transaction's fee in luna (NIM's smallest unit).
    * @returns {bigint}
    */
    get fee() {
        const ret = wasm.transaction_fee(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * The transaction's fee per byte in luna (NIM's smallest unit).
    * @returns {number}
    */
    get feePerByte() {
        const ret = wasm.transaction_feePerByte(this.__wbg_ptr);
        return ret;
    }
    /**
    * The transaction's validity-start height. The transaction is valid for 2 hours after this block height.
    * @returns {number}
    */
    get validityStartHeight() {
        const ret = wasm.transaction_validityStartHeight(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * The transaction's network ID.
    * @returns {number}
    */
    get networkId() {
        const ret = wasm.transaction_networkId(this.__wbg_ptr);
        return ret;
    }
    /**
    * The transaction's flags: `0b1` = contract creation, `0b10` = signaling.
    * @returns {number}
    */
    get flags() {
        const ret = wasm.transaction_flags(this.__wbg_ptr);
        return ret;
    }
    /**
    * The transaction's data as a byte array.
    * @returns {Uint8Array}
    */
    get data() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_data(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the transaction's data
    * @param {Uint8Array} data
    */
    set data(data) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.transaction_set_data(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * The transaction's sender data as a byte array.
    * @returns {Uint8Array}
    */
    get senderData() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_senderData(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * The transaction's signature proof as a byte array.
    * @returns {Uint8Array}
    */
    get proof() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_proof(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the transaction's signature proof.
    * @param {Uint8Array} proof
    */
    set proof(proof) {
        const ptr0 = passArray8ToWasm0(proof, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.transaction_set_proof(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * The transaction's byte size.
    * @returns {number}
    */
    get serializedSize() {
        const ret = wasm.transaction_serializedSize(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Serializes the transaction into a HEX string.
    * @returns {string}
    */
    toHex() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toHex(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Creates a JSON-compatible plain object representing the transaction.
    * @returns {PlainTransaction}
    */
    toPlain() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_toPlain(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Parses a transaction from a {@link Transaction} instance, a plain object, or a serialized
    * string representation.
    *
    * Throws when a transaction cannot be parsed from the argument.
    * @param {PlainTransaction | string} tx
    * @returns {Transaction}
    */
    static fromAny(tx) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_fromAny(retptr, addBorrowedObject(tx));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Transaction.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Parses a transaction from a plain object.
    *
    * Throws when a transaction cannot be parsed from the argument.
    * @param {PlainTransaction} plain
    * @returns {Transaction}
    */
    static fromPlain(plain) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transaction_fromPlain(retptr, addBorrowedObject(plain));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Transaction.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
__exports.Transaction = Transaction;

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
    imports.wbg.__wbg_randomFillSync_6894564c2c334c42 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_subarray_6ca5cfa7fbb9abbe = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getRandomValues_805f1c3d65988a5a = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_crypto_e1d53a1d73fb10b8 = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_process_038c26bf42b093f8 = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_versions_ab37218d2f0b24a8 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_080f4b19d15bc1fe = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbg_msCrypto_6e7d3e1f92610cbb = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithlength_13b5319ab422dcf6 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_78a3dcfbdba9cbce = function() { return handleError(function () {
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
    imports.wbg.__wbg_call_53fc3abd42e24ec8 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_clearTimeout_76877dbc010e786d = function(arg0) {
        const ret = clearTimeout(takeObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setTimeout_75cb9b6991a4031d = function() { return handleError(function (arg0, arg1) {
        const ret = setTimeout(getObject(arg0), arg1);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_get_2aff440840bb6202 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_now_0cfdc90c97d0c24b = function(arg0) {
        const ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__wbg_now_4579335d3581594c = function() {
        const ret = Date.now();
        return ret;
    };
    imports.wbg.__wbg_get_4a9aa5157afeb382 = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_cace2e0b3ddc0502 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_next_15da6a3df9290720 = function(arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_1989a20442400aaa = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_bc26bf4ada718266 = function(arg0) {
        const ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_0570714ff7d75f35 = function(arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_iterator_7ee1a391d310f8e4 = function() {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_669127b9d730c650 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_3fad056edded10bd = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_a4f46c98a61d4089 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_17eff828815f7d84 = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_46f939f6541643c5 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_newnoargs_ccdcae30fd002262 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_08236689f0afb357 = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_0ac78a2bc07da03c = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    };
    imports.wbg.__wbg_isArray_38525be7442aa21e = function(arg0) {
        const ret = Array.isArray(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_push_fd3233d09cf81821 = function(arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_instanceof_ArrayBuffer_c7cc317e5c29cc0d = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof ArrayBuffer;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_apply_1c259fc7880fb101 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).apply(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_isSafeInteger_c38b0a16d0c7cef7 = function(arg0) {
        const ret = Number.isSafeInteger(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_getTime_ed6ee333b702f8fc = function(arg0) {
        const ret = getObject(arg0).getTime();
        return ret;
    };
    imports.wbg.__wbg_new0_ad75dd38f92424e2 = function() {
        const ret = new Date();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_create_9b9e7caad35d0488 = function(arg0) {
        const ret = Object.create(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_entries_6d727b73ee02b7ce = function(arg0) {
        const ret = Object.entries(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_is_c74aa9bb973d6109 = function(arg0, arg1) {
        const ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_new_c728d68b8b34487e = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_feb65b865d980ae2 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_136(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_a3252b2860f0a09e = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_89e1c559530b85cf = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_344d9b41efe96da7 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_d8a000788389a31e = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_dcfd613a3420f908 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_a5587d6cd79ab197 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Uint8Array_19e6f142a5e7e1e1 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Uint8Array;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_set_40f7786a25a9cc7e = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbg_new_b66404b6322c59bf = function() { return handleError(function (arg0, arg1) {
        const ret = new WebSocket(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setbinaryType_096c70c4a9d97499 = function(arg0, arg1) {
        getObject(arg0).binaryType = takeObject(arg1);
    };
    imports.wbg.__wbg_setonopen_419ca0e52d8f19e8 = function(arg0, arg1) {
        getObject(arg0).onopen = getObject(arg1);
    };
    imports.wbg.__wbg_setonmessage_809f60b68c2a6938 = function(arg0, arg1) {
        getObject(arg0).onmessage = getObject(arg1);
    };
    imports.wbg.__wbg_setonerror_2fa7120354e9ec15 = function(arg0, arg1) {
        getObject(arg0).onerror = getObject(arg1);
    };
    imports.wbg.__wbg_setonclose_4210cf3908b79b31 = function(arg0, arg1) {
        getObject(arg0).onclose = getObject(arg1);
    };
    imports.wbg.__wbg_data_ab99ae4a2e1e8bc9 = function(arg0) {
        const ret = getObject(arg0).data;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_send_1a008ea2eb3a1951 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).send(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_close_dfa389d8fddb52fc = function() { return handleError(function (arg0) {
        getObject(arg0).close();
    }, arguments) };
    imports.wbg.__wbg_warn_d60e832f9882c1b2 = function(arg0) {
        console.warn(getObject(arg0));
    };
    imports.wbg.__wbg_performance_0666aaff1a1b75ec = function() {
        const ret = globalThis.performance;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_mark_1e2d127a70590942 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).mark(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_measure_b8146f54a3c651a7 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).measure(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), getStringFromWasm0(arg5, arg6));
    }, arguments) };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_client_new = function(arg0) {
        const ret = Client.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
        const ret = getObject(arg0) == getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_is_bigint = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'bigint';
        return ret;
    };
    imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
        const v = getObject(arg1);
        const ret = typeof(v) === 'bigint' ? v : undefined;
        getBigInt64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? BigInt(0) : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        const ret = getObject(arg0) === getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
        const ret = getObject(arg0) in getObject(arg1);
        return ret;
    };
    imports.wbg.__wbg_getwithrefkey_3b3c46ba20582127 = function(arg0, arg1) {
        const ret = getObject(arg0)[getObject(arg1)];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_8761474ad72b9bf1 = function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    };
    imports.wbg.__wbg_String_917f38a1211cf44b = function(arg0, arg1) {
        const ret = String(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_addEventListener_45198c46e231596e = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_mark_61aa1c1a9bb53ab8 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).mark(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_measure_db00dca1e38efa83 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).measure(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_debug_9a6b3243fbbebb61 = function(arg0) {
        console.debug(getObject(arg0));
    };
    imports.wbg.__wbg_info_2e30e8204b29d91d = function(arg0) {
        console.info(getObject(arg0));
    };
    imports.wbg.__wbg_error_788ae33f81d3b84b = function(arg0) {
        console.error(getObject(arg0));
    };
    imports.wbg.__wbg_log_1d3ae0273d8f4f8a = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_queueMicrotask_e5949c35d772a669 = function(arg0) {
        queueMicrotask(getObject(arg0));
    };
    imports.wbg.__wbg_queueMicrotask_2be8b97a81fe4d00 = function(arg0) {
        const ret = getObject(arg0).queueMicrotask;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setTimeout_d249305a43a65e93 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_performance_5c63fba5394e8a34 = function(arg0) {
        const ret = getObject(arg0).performance;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_now_f5bc2e45fa836a60 = function(arg0) {
        const ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper608 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 165, __wbg_adapter_48);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2124 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 569, __wbg_adapter_51);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2125 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 569, __wbg_adapter_54);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2126 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 569, __wbg_adapter_54);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2127 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 569, __wbg_adapter_54);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper6164 = function(arg0, arg1, arg2) {
        const ret = makeClosure(arg0, arg1, 2006, __wbg_adapter_61);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper9050 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 2881, __wbg_adapter_64);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper9099 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 2908, __wbg_adapter_67);
        return addHeapObject(ret);
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedBigInt64Memory0 = null;
    cachedFloat64Memory0 = null;
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

    if (typeof input === 'undefined' && script_src !== 'undefined') {
        input = script_src.replace(/\.js$/, '_bg.wasm');
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports);

})();
