/**
 * Unit tests for the LSP `Content-Length` framing in `src/framing.ts`.
 *
 * These import from `dist/`, so run `pnpm run build` first.
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { encodeFrame } from "../../dist/framing.js";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

describe("encodeFrame", () => {
	it("prefixes the body with a Content-Length header", () => {
		const message = { jsonrpc: "2.0", id: 1, method: "shutdown" };
		const body = JSON.stringify(message);
		assert.equal(
			decoder.decode(encodeFrame(message)),
			`Content-Length: ${body.length}\r\n\r\n${body}`,
		);
	});

	it("counts Content-Length in bytes, not characters", () => {
		// Anyone who reworks this in terms of string indices breaks here and
		// nowhere else.
		const message = { text: "héllo — ✓" };
		const raw = encodeFrame(message);
		const header = decoder.decode(raw.subarray(0, raw.indexOf(13)));
		const body = JSON.stringify(message);
		const length = encoder.encode(body).length;
		assert.equal(header, `Content-Length: ${length}`);
		assert.notEqual(length, body.length, "the fixture needs a multi-byte character");
	});
});
