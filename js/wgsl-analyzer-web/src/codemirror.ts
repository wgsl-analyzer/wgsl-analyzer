/**
 * Adapter for `@marimo-team/codemirror-languageserver`.
 *
 * That package defines its own minimal `Transport` interface, which maps
 * directly onto {@link WgslAnalyzerServer}. It hands the client parsed
 * JSON-RPC objects and leaves the wire format to the transport. It is an
 * optional peer dependency; importing this entry point is what pulls it in.
 */

import type { JSONRPCMessage, Transport } from "@marimo-team/codemirror-languageserver";

import type { WgslAnalyzerServer } from "./index.js";

/** Carries JSON-RPC frames to a wgsl-analyzer server running in a worker. */
export class WgslAnalyzerTransport implements Transport {
	readonly #server: WgslAnalyzerServer;
	readonly #subscriptions = new Set<{ dispose(): void }>();

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

/** Convenience wrapper matching the shape of the other adapters. */
export function createTransport(server: WgslAnalyzerServer): Transport {
	return new WgslAnalyzerTransport(server);
}
