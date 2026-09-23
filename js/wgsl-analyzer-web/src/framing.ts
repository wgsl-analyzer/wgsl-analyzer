/** LSP `Content-Length` framing, inbound only. */

const encoder = new TextEncoder();

/** Encodes one JSON-RPC message as an LSP frame. */
export function encodeFrame(message: unknown): Uint8Array {
	const body = encoder.encode(JSON.stringify(message));
	const header = encoder.encode(`Content-Length: ${body.length}\r\n\r\n`);
	const frame = new Uint8Array(header.length + body.length);
	frame.set(header, 0);
	frame.set(body, header.length);
	return frame;
}
