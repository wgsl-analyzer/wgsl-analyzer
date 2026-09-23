/**
 * Carries JSON-RPC frames between `@marimo-team/codemirror-languageserver` and
 * a wgsl-analyzer server.
 *
 * That package defines its own minimal `Transport` interface, which maps
 * directly onto {@link WgslAnalyzerServer}: it hands the client parsed JSON-RPC
 * objects and leaves the wire format to the transport.
 */

import type { JSONRPCMessage, Transport } from "@marimo-team/codemirror-languageserver";
import type { Disposable, WgslAnalyzerServer } from "wgsl-analyzer-web";

export class WgslAnalyzerTransport implements Transport {
	readonly #server: WgslAnalyzerServer;
	readonly #subscriptions = new Set<Disposable>();

	constructor(server: WgslAnalyzerServer) {
		this.#server = server;
	}

	/** The server is already running by the time it reaches this class. */
	connect(): Promise<void> {
		return Promise.resolve();
	}

	send(message: JSONRPCMessage): void {
		this.#server.sendMessage(message);
	}

	onMessage(handler: (message: JSONRPCMessage) => void): () => void {
		const subscription = this.#server.onMessage((message) => {
			handler(message as JSONRPCMessage);
		});
		this.#subscriptions.add(subscription);
		return () => {
			subscription.dispose();
			this.#subscriptions.delete(subscription);
		};
	}

	close(): void {
		for (const subscription of this.#subscriptions) subscription.dispose();
		this.#subscriptions.clear();
	}
}
