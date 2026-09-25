// The glue reads Atomics.waitAsync when its factory runs, so this forces the mailbox's postMessage fallback.
Reflect.deleteProperty(Atomics, "waitAsync");
await import(new URL("./lsp.test.ts", import.meta.url).href);

// Makes this a module, so the top-level `await` typechecks.
export {};
