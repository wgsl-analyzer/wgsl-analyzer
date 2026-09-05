/** LSP `Content-Length` framing. */

const encoder = new TextEncoder();
const decoder = new TextDecoder();

/**
 * Largest body the decoder will wait for, well above any real LSP payload.
 * Its only job is to keep a corrupt header from stalling the stream forever.
 */
const MAX_CONTENT_LENGTH = 1 << 28;

/** Encodes one JSON-RPC message as an LSP frame. */
export function encodeFrame(message: unknown): Uint8Array {
	const body = encoder.encode(JSON.stringify(message));
	const header = encoder.encode(`Content-Length: ${body.length}\r\n\r\n`);
	const frame = new Uint8Array(header.length + body.length);
	frame.set(header, 0);
	frame.set(body, header.length);
	return frame;
}

/**
 * Accumulates the raw stdout stream and reports each complete frame.
 *
 * Chunks arrive straight from `lsp_stdout_pop`, so a frame is routinely split
 * across several of them and several frames routinely arrive in one.
 */
export function createFrameDecoder(
	onMessage: (message: unknown) => void,
	onError?: (reason: string) => void,
): (chunk: Uint8Array) => void {
	let buffer = new Uint8Array(1 << 16);
	let length = 0;
	let contentLength = -1;
	let searchFrom = 0;

	const reserve = (extra: number): void => {
		if (length + extra <= buffer.length) return;
		let capacity = buffer.length;
		while (capacity < length + extra) capacity *= 2;
		const grown = new Uint8Array(capacity);
		grown.set(buffer.subarray(0, length));
		buffer = grown;
	};

	const drain = (): void => {
		for (;;) {
			if (contentLength < 0) {
				if (length < 4) return;
				let bodyStart = -1;
				for (let index = searchFrom; index + 3 < length; index++) {
					if (
						buffer[index] === 13 &&
						buffer[index + 1] === 10 &&
						buffer[index + 2] === 13 &&
						buffer[index + 3] === 10
					) {
						bodyStart = index + 4;
						break;
					}
				}
				if (bodyStart < 0) {
					searchFrom = Math.max(0, length - 3);
					return;
				}
				const header = decoder.decode(buffer.subarray(0, bodyStart));
				const match = /content-length:\s*(\d+)/i.exec(header);
				if (!match) {
					onError?.(`missing Content-Length in header ${JSON.stringify(header)}`);
					length = 0;
					searchFrom = 0;
					return;
				}
				const declared = Number(match[1]);
				// An unbounded length would park `drain` in the `length < contentLength`
				// return below forever: no error, no recovery, the stream just stops.
				// fd 1 is shared with anything else that writes to stdout, so a
				// nonsense header is not purely hypothetical.
				if (declared > MAX_CONTENT_LENGTH) {
					onError?.(`Content-Length ${declared} exceeds the ${MAX_CONTENT_LENGTH} byte limit`);
					length = 0;
					searchFrom = 0;
					return;
				}
				contentLength = declared;
				buffer.copyWithin(0, bodyStart, length);
				length -= bodyStart;
				searchFrom = 0;
			}

			if (length < contentLength) return;
			const body = decoder.decode(buffer.subarray(0, contentLength));
			buffer.copyWithin(0, contentLength, length);
			length -= contentLength;
			contentLength = -1;
			try {
				onMessage(JSON.parse(body));
			} catch (error) {
				onError?.(`malformed JSON body: ${String(error)}`);
			}
		}
	};

	return (chunk: Uint8Array): void => {
		reserve(chunk.length);
		buffer.set(chunk, length);
		length += chunk.length;
		drain();
	};
}
