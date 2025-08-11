import * as assert from "node:assert/strict";
import { readdir } from "fs/promises";
import * as path from "path";
import { pathToFileURL } from "url";

class Test {
	readonly name: string;
	readonly promise: Promise<void>;

	constructor(name: string, promise: Promise<void>) {
		this.name = name;
		this.promise = promise;
	}
}

class Suite {
	tests: Test[];

	constructor() {
		this.tests = [];
	}

	public addSyncTest(name: string, test_function: () => void): void {
		const test = new Test(name, new Promise(test_function));
		this.tests.push(test);
	}

	public addTest(name: string, test_function: () => Promise<void>): void {
		const test = new Test(name, test_function());
		this.tests.push(test);
	}

	public async run(): Promise<void> {
		let failed = 0;
		for (const test of this.tests) {
			try {
				await test.promise;
				ok(`  ✔ ${test.name}`);
			} catch (exception) {
				assert.ok(exception instanceof Error);
				assert.ok(exception.stack);
				error(`  ✖︎ ${test.name}\n  ${exception.stack}`);
				failed += 1;
			}
		}
		if (failed) {
			const plural = failed > 1 ? "s" : "";
			throw new Error(`${failed} failed test${plural}`);
		}
	}
}

export class Context {
	public async suite(name: string, fn: (ctx: Suite) => void): Promise<void> {
		const ctx = new Suite();
		fn(ctx);
		try {
			ok(`⌛︎ ${name}`);
			await ctx.run();
			ok(`✔ ${name}`);
		} catch (exception) {
			assert.ok(exception instanceof Error);
			assert.ok(exception.stack);
			error(`✖︎ ${name}\n  ${exception.stack}`);
			throw exception;
		}
	}
}

export async function run(): Promise<void> {
	const context = new Context();

	const testFiles = (await readdir(path.resolve(__dirname))).filter((name) =>
		name.endsWith(".test.js"),
	);
	for (const testFile of testFiles) {
		try {
			const testModule = await import(
				pathToFileURL(path.resolve(__dirname, testFile)).href
			);
			await testModule.getTests(context);
		} catch (exception) {
			assert.ok(exception instanceof Error);
			error(`${exception}`);
			throw exception;
		}
	}
}

function ok(message: string): void {
	// biome-ignore lint/suspicious/noConsole: needed here
	console.log(`\x1b[32m${message}\x1b[0m`);
}

function error(message: string): void {
	// biome-ignore lint/suspicious/noConsole: needed here
	console.error(`\x1b[31m${message}\x1b[0m`);
}
