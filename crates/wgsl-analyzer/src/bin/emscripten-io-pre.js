// Host side of the wasm LSP transport.
// Linked in via -Clink-arg=--pre-js=crates/wgsl-analyzer/src/bin/emscripten-io-pre.js
//
// --pre-js runs inside the -sMODULARIZE factory, just after `var Module =
// moduleArg`, so the methods installed here are present before the factory's promise
// resolves. That is why the queue exists before -sPROXY_TO_PTHREAD starts `main()` on
//  a pthread, so anything pushed early is simply waiting when the reader first asks.

(() => {
    /** Framed messages waiting for the server, oldest first. */
    const input = [];
    /** How much of `input[0]` the server has already taken. */
    let inputOffset = 0;
    let inputClosed = false;
    /** `resolve` callbacks of reads parked on an empty queue. */
    let waiters = [];
    /** Where the server's output goes. Supplied by `lspStart`. */
    let outputHandler = null;

    const wake = () => {
        const parked = waiters;
        waiters = [];
        for (const resolve of parked) resolve();
    };

    /**
     * Start the transport and take hold of its two ends.
     *
     * Call this before `callMain`, which is when the server can first write.
     *
     * `onOutput` receives a view into wasm memory that is only valid for the
     * duration of the call, so it has to consume or copy the bytes synchronously.
     * They are a raw stream rather than messages: one call is not one frame, so it
     * needs a `Content-Length` parser over them.
     */
    Module["lspStart"] = (options) => {
        if (outputHandler !== null) {
            throw new Error("the wgsl-analyzer LSP transport is already started");
        }
        const handler = options && options["onOutput"];
        if (typeof handler !== "function") {
            throw new TypeError("lspStart needs an onOutput function");
        }
        outputHandler = handler;

        return {
            /** Queue one framed message for the server. */
            pushBytes: (bytes) => {
                if (inputClosed || bytes.length === 0) return;
                input.push(bytes);
                wake();
            },

            /**
             * Report end of input, so the reader thread unwinds instead of parking
             * forever and holding the runtime open.
             */
            closeInput: () => {
                inputClosed = true;
                wake();
            },
        };
    };

    /**
     * Fill `capacity` bytes at `destination`, waiting for a message if the queue
     * is empty. Backs `lsp_js_read`, and returns through its promise.
     *
     * Awaiting here is what keeps the runtime thread's event loop free while the
     * server's reader thread is blocked.
     */
    Module["lspReadInto"] = async (destination, capacity) => {
        while (input.length === 0 && !inputClosed) {
            await new Promise((resolve) => waiters.push(resolve));
        }
        // Zero only after an explicit close: `Read::read` defines 0 as end of
        // input, not as "nothing available yet".
        if (input.length === 0) return 0;

        // One chunk per call. Returning less than `capacity` is allowed, and the
        // caller loops.
        const chunk = input[0];
        const count = Math.min(chunk.length - inputOffset, capacity);
        // Read HEAPU8 fresh: ALLOW_MEMORY_GROWTH replaces the heap views.
        HEAPU8.set(chunk.subarray(inputOffset, inputOffset + count), destination);
        inputOffset += count;
        if (inputOffset === chunk.length) {
            input.shift();
            inputOffset = 0;
        }
        return count;
    };

    /** Hand the server's output to `lspStart`'s handler. Backs `lsp_js_write`. */
    Module["lspWriteFrom"] = (source, length) => {
        if (outputHandler === null) {
            // Only reachable by calling `callMain` without `lspStart`.
            console.error("[wgsl-analyzer] callMain ran before lspStart; the output is lost");
            return -1;
        }
        // A view, not a copy: the handler is expected to consume it synchronously.
        outputHandler(HEAPU8.subarray(source, source + length));
        return length;
    };
})();
