/**
 * Carries JSON-RPC frames between `monaco.lsp` and a wgsl-analyzer server.
 *
 * `monaco.lsp` ships `createTransportToWorker`, which is not usable here: it
 * posts bare JSON-RPC to the worker, while `wgsl-analyzer-web`'s worker speaks
 * an enveloped protocol (`{ type: "lsp", message }` alongside `boot`,
 * `writeFile` and `deleteFile`), so bare messages would fall through its switch.
 * This goes against `WgslAnalyzerServer` directly instead.
 */

import type * as monaco from "monaco-editor";
import type { Disposable, WgslAnalyzerServer } from "wgsl-analyzer-web";

// `monaco.lsp` exports only MonacoLspClient, WebSocketTransport and the two
// createTransportTo* helpers, so the transport interface has no name to import.
type MessageTransport = ConstructorParameters<typeof monaco.lsp.MonacoLspClient>[0];
type MessageListener = NonNullable<Parameters<MessageTransport["setListener"]>[0]>;
type Message = Parameters<MessageListener>[0];
type ConnectionState = MessageTransport["state"]["value"];

const OPEN: MessageTransport["state"] = {
	get value(): ConnectionState {
		return { state: "open" };
	},
	get onChange() {
		return () => ({ dispose: () => {} });
	},
};

export class WgslAnalyzerTransport implements MessageTransport {
	readonly #server: WgslAnalyzerServer;
	#subscription: Disposable | undefined;

	constructor(server: WgslAnalyzerServer) {
		this.#server = server;
	}

	get state(): MessageTransport["state"] {
		return OPEN;
	}

	send(message: Message): Promise<void> {
		this.#server.sendMessage(message);
		return Promise.resolve();
	}

	setListener(listener: MessageListener | undefined): void {
		this.#subscription?.dispose();
		this.#subscription = listener
			? this.#server.onMessage((message) => listener(message as Message))
			: undefined;
	}

	toString(): string {
		return "wgsl-analyzer";
	}
}
