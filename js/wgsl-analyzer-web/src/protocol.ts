/**
 * Control messages exchanged between the page and the worker hosting the
 * emscripten module.
 *
 * LSP traffic travels this way in both directions. The worker adds the
 * `Content-Length` framing on the way in and strips it on the way out, so the
 * page only ever sees parsed JSON-RPC objects.
 */

/** Contents of a workspace, keyed by path relative to the MEMFS root. */
export type WorkspaceFiles = Record<string, string | Uint8Array>;

/** Page to worker. */
export type HostMessage =
	| {
			readonly type: "boot";
			readonly root: string;
			readonly files: WorkspaceFiles;
			readonly args: readonly string[];
	  }
	| { readonly type: "lsp"; readonly message: unknown }
	| { readonly type: "writeFile"; readonly path: string; readonly contents: string | Uint8Array }
	| { readonly type: "deleteFile"; readonly path: string }
	| { readonly type: "close" };

/** Worker to page. */
export type WorkerMessage =
	| { readonly type: "ready" }
	| { readonly type: "lsp"; readonly message: unknown }
	| { readonly type: "stderr"; readonly line: string }
	| { readonly type: "exit"; readonly code: number }
	| { readonly type: "error"; readonly message: string; readonly stack?: string | undefined };
