// The transport crates/wgsl-analyzer/src/bin/emscripten_io.rs imports, linked in with
// `--js-library`. Both functions run on the main runtime thread, possibly inside a futex wait in
// the middle of another call into the module, so the host is only ever called from a microtask.
addToLibrary({
	lsp_next_message__proxy: "sync",
	lsp_next_message__async: true,
	lsp_next_message__sig: "p",
	lsp_next_message__deps: ["$stringToNewUTF8"],
	lsp_next_message: () => Module.lspNextMessage().then(stringToNewUTF8),

	lsp_send_message__proxy: "sync",
	lsp_send_message__sig: "vp",
	lsp_send_message__deps: ["$UTF8ToString"],
	lsp_send_message: (body) => {
		const text = UTF8ToString(body);
		queueMicrotask(() => Module.lspOnMessage(text));
	},
});
