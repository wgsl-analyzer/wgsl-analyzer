import * as path from "path";
import * as fs from "fs";

import { runTests, TestOptions } from "@vscode/test-electron";

async function main() {
	// The folder containing the Extension Manifest package.json
	// Passed to `--extensionDevelopmentPath`
	const extensionDevelopmentPath = path.resolve(__dirname, "../../");

	// Minimum supported version.
	const jsonData = fs.readFileSync(path.join(extensionDevelopmentPath, "package.json"));
	const json = JSON.parse(jsonData.toString());
	let minimalVersion: string = json.engines.vscode;
	if (minimalVersion.startsWith("^")) minimalVersion = minimalVersion.slice(1);

	const launchArgs = ["--disable-extensions", extensionDevelopmentPath];

	// All test suites (either unit tests or integration tests) should be in subfolders.
	const extensionTestsPath = path.resolve(__dirname, "./unit/index");

	const test1Options: TestOptions = {
		version: minimalVersion,
		launchArgs,
		extensionDevelopmentPath,
		extensionTestsPath,
	};
	// Run tests using the minimal supported version.
	await runTests(test1Options);

	const test2Options: TestOptions = {
		version: "stable",
		launchArgs,
		extensionDevelopmentPath,
		extensionTestsPath,
	};
	// and the latest one
	await runTests(test2Options);
}

main().catch((error) => {
	// eslint-disable-next-line no-console
	console.error("Failed to run tests", error);
	process.exit(1);
});
