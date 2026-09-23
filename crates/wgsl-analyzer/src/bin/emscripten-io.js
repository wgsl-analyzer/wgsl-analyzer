// Emscripten JS library for the wasm LSP transport.
// Linked in using -Clink-arg=--js-library=crates/wgsl-analyzer/src/bin/emscripten-io.js

addToLibrary({
    // `__proxy: 'sync'` runs a body on the main runtime thread (the top-level thread
    // of the Web Worker hosting the module) even though the caller is a pthread.
    // Only that thread holds the queue and the page's `postMessage`, and it must
    // never block.
    lsp_js_read__proxy: 'sync',
    // `__async` upgrades the proxy to PROXY_SYNC_ASYNC: synchronous on the
    // calling thread, asynchronous on the main one. The runtime thread starts the
    // read and returns to its event loop, and the promise's value becomes this
    // call's return value once it settles. That is what lets a blocking
    // `Read::read` wait for a frame without Asyncify, and what keeps the writer
    // thread's own proxied calls from queuing up behind a pending read.
    lsp_js_read__async: 'auto',
    lsp_js_read__sig: 'ipp',
    // Do NOT make this an `async function`, emscripten would rewrite it as
    //
    //   <async?> function (destination, capacity) {
    //     if (ENVIRONMENT_IS_PTHREAD) return proxyToMainThread(..., destination, capacity);
    //     <body>
    //   }
    //
    // and `async` would wrap that early `return` too. The pthread would then get
    // a Promise where wasm expects an i32, which converts to 0 — indistinguishable
    // from end of input, so the server's reader shuts down after one read and the
    // connection dies silently.
    lsp_js_read: function (destination, capacity) {
        // The main-thread path has to hand back a thenable, and never a rejected
        // one: the proxy dispatcher does a bare `promise.then(...)`, so a
        // rejection leaves the proxying context unfinished and the reader thread
        // blocked for good. `Promise.resolve().then` also catches a synchronous
        // throw, which is what a missing `--pre-js` would produce.
        return Promise.resolve()
            .then(() => Module['lspReadInto'](destination, capacity))
            .catch((error) => {
                console.error('[wgsl-analyzer] lsp_js_read failed', error);
                return -1;
            });
    },

    // Synchronous rather than async: the body is a live view into wasm memory, valid
    // only for the duration of the call, and the host takes all of it at once.
    lsp_js_write__proxy: 'sync',
    lsp_js_write__sig: 'ipp',
    lsp_js_write: function (source, length) {
        try {
            return Module['lspWriteFrom'](source, length);
        } catch (error) {
            console.error('[wgsl-analyzer] lsp_js_write failed', error);
            return -1;
        }
    },
});
