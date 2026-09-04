/**
 * Unit tests for the LSP `Content-Length` framing in `src/framing.ts`.
 *
 * The decoder is an incremental parser over a growable buffer, so the cases
 * that matter are the ones where a frame does not arrive whole: chunks that
 * split the header terminator, several frames coalesced into one chunk, and
 * bodies that outgrow the initial buffer. `lsp_stdout_pop` produces all three
 * routinely.
 *
 * These import from `dist/`, so run `pnpm run build` first.
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { createFrameDecoder, encodeFrame } from "../../dist/framing.js";

const encoder = new TextEncoder();
const bytes = (text) => encoder.encode(text);

/** A decoder plus the messages and errors it has produced. */
function decoder() {
	const messages = [];
	const errors = [];
	const push = createFrameDecoder(
		(message) => messages.push(message),
		(reason) => errors.push(reason),
	);
	return { messages, errors, push };
}

/** Builds a raw frame without going through `encodeFrame`. */
function frame(body, header) {
	const payload = bytes(body);
	return concat(bytes(header ?? `Content-Length: ${payload.length}\r\n\r\n`), payload);
}

function concat(...chunks) {
	const total = chunks.reduce((sum, chunk) => sum + chunk.length, 0);
	const out = new Uint8Array(total);
	let offset = 0;
	for (const chunk of chunks) {
		out.set(chunk, offset);
		offset += chunk.length;
	}
	return out;
}

describe("createFrameDecoder", () => {
	it("decodes one frame in one chunk", () => {
		const { messages, push } = decoder();
		push(frame('{"id":1}'));
		assert.deepEqual(messages, [{ id: 1 }]);
	});

	it("decodes a frame delivered one byte at a time", () => {
		const { messages, push } = decoder();
		const raw = frame('{"id":1}');
		for (const byte of raw) push(Uint8Array.of(byte));
		assert.deepEqual(messages, [{ id: 1 }]);
	});

	it("handles a split inside the header terminator", () => {
		// The only case that exercises `searchFrom = Math.max(0, length - 3)`.
		// An off-by-one there hangs the decoder on a boundary rare enough to
		// reach production.
		const { messages, push } = decoder();
		const raw = frame('{"id":1}');
		const split = raw.indexOf(13) + 3; // ...\r\n\r | \n{...
		push(raw.subarray(0, split));
		push(raw.subarray(split));
		assert.deepEqual(messages, [{ id: 1 }]);
	});

	it("decodes three frames coalesced into one chunk", () => {
		const { messages, push } = decoder();
		push(concat(frame('{"id":1}'), frame('{"id":2}'), frame('{"id":3}')));
		assert.deepEqual(messages, [{ id: 1 }, { id: 2 }, { id: 3 }]);
	});

	it("holds a trailing partial frame until the rest arrives", () => {
		const { messages, push } = decoder();
		const tail = frame('{"id":2}');
		const cut = tail.length - 3;
		push(concat(frame('{"id":1}'), tail.subarray(0, cut)));
		assert.deepEqual(messages, [{ id: 1 }]);
		push(tail.subarray(cut));
		assert.deepEqual(messages, [{ id: 1 }, { id: 2 }]);
	});

	it("counts Content-Length in bytes, not characters", () => {
		// Anyone who reworks the decoder in terms of string indices breaks here
		// and nowhere else.
		const { messages, push } = decoder();
		const message = { text: "héllo — ✓" };
		const raw = encodeFrame(message);
		const header = new TextDecoder().decode(raw.subarray(0, raw.indexOf(13)));
		assert.equal(header, `Content-Length: ${bytes(JSON.stringify(message)).length}`);
		push(raw);
		assert.deepEqual(messages, [message]);
	});

	it("reassembles a multi-byte character split across chunks", () => {
		const { messages, push } = decoder();
		const raw = encodeFrame({ text: "—".repeat(8) });
		// Cut mid-body, which for this payload lands inside a 3-byte sequence.
		const cut = raw.length - 4;
		push(raw.subarray(0, cut));
		push(raw.subarray(cut));
		assert.deepEqual(messages, [{ text: "—".repeat(8) }]);
	});

	it("grows past the initial 64 KiB buffer", () => {
		const { messages, push } = decoder();
		const message = { padding: "x".repeat(200 * 1024) };
		const raw = encodeFrame(message);
		for (let offset = 0; offset < raw.length; offset += 4096) {
			push(raw.subarray(offset, Math.min(offset + 4096, raw.length)));
		}
		assert.deepEqual(messages, [message]);
	});

	it("accepts extra headers and any casing", () => {
		const { messages, push } = decoder();
		const body = '{"id":1}';
		push(
			frame(
				body,
				"Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n" +
					`content-length:  ${body.length}\r\n\r\n`,
			),
		);
		assert.deepEqual(messages, [{ id: 1 }]);
	});

	it("reports a malformed body and recovers on the next frame", () => {
		const { messages, errors, push } = decoder();
		push(frame("not json"));
		push(frame('{"id":2}'));
		assert.equal(errors.length, 1);
		assert.match(errors[0], /malformed JSON body/);
		assert.deepEqual(messages, [{ id: 2 }]);
	});

	it("reports a header with no Content-Length", () => {
		const { errors, push } = decoder();
		push(bytes("Content-Type: text/plain\r\n\r\n"));
		assert.equal(errors.length, 1);
		assert.match(errors[0], /missing Content-Length/);
	});

	it("reports an absurd Content-Length instead of stalling", () => {
		// Regression: an unbounded length parked `drain` in its
		// `length < contentLength` early return forever — no error, no
		// recovery, the stream simply stopped.
		const { messages, errors, push } = decoder();
		push(bytes("Content-Length: 99999999999\r\n\r\n"));
		assert.equal(errors.length, 1);
		assert.match(errors[0], /exceeds the \d+ byte limit/);
		push(frame('{"id":1}'));
		assert.deepEqual(messages, [{ id: 1 }]);
	});

	it("round trips through encodeFrame", () => {
		const { messages, push } = decoder();
		const sent = [
			{ jsonrpc: "2.0", id: 1, method: "initialize" },
			{ jsonrpc: "2.0", id: 1, result: {} },
		];
		for (const message of sent) push(encodeFrame(message));
		assert.deepEqual(messages, sent);
	});
});
