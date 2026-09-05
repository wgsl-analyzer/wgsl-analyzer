/**
 * Adapter for `vscode-jsonrpc`, which is what `monaco-languageclient` and
 * `vscode-languageclient` consume.
 *
 * `vscode-jsonrpc` is an optional peer dependency: importing this entry point is
 * what pulls it in.
 */

import {
	AbstractMessageReader,
	AbstractMessageWriter,
	type DataCallback,
	type Disposable,
	type Message,
	type MessageReader,
	type MessageWriter,
} from "vscode-jsonrpc";

import type { WgslAnalyzerServer } from "./index.js";

class ServerMessageReader extends AbstractMessageReader implements MessageReader {
	readonly #server: WgslAnalyzerServer;
	#subscription: { dispose(): void } | null = null;

	constructor(server: WgslAnalyzerServer) {
		super();
		this.#server = server;
	}

	listen(callback: DataCallback): Disposable {
		this.#subscription = this.#server.onMessage((message) => {
			callback(message as Message);
		});
		return {
			dispose: () => {
				this.#subscription?.dispose();
				this.#subscription = null;
			},
		};
	}

	override dispose(): void {
		this.#subscription?.dispose();
		this.#subscription = null;
		super.dispose();
	}
}

class ServerMessageWriter extends AbstractMessageWriter implements MessageWriter {
	readonly #server: WgslAnalyzerServer;

	constructor(server: WgslAnalyzerServer) {
		super();
		this.#server = server;
	}

	async write(message: Message): Promise<void> {
		this.#server.sendMessage(message);
	}

	end(): void {
		// The ring buffer is closed by `WgslAnalyzerServer.dispose`.
	}
}

export interface MessageTransports {
	readonly reader: MessageReader;
	readonly writer: MessageWriter;
}

/** Wraps a running server as a vscode-jsonrpc reader/writer pair. */
export function createMessageTransports(server: WgslAnalyzerServer): MessageTransports {
	return { reader: new ServerMessageReader(server), writer: new ServerMessageWriter(server) };
}
