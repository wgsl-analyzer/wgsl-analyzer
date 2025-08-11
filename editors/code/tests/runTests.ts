import { runTests, TestOptions } from "@vscode/test-electron";
import { fold } from "fp-ts/Either";
import { pipe } from "fp-ts/function";
import * as fs from "fs";
import * as Decoder from "io-ts/Decoder";
import * as path from "path";

const PackageJson = Decoder.struct({
	engines: Decoder.struct({ vscode: Decoder.string }),
});
type PackageJson = Decoder.TypeOf<typeof PackageJson>;

async function main() {
	// The folder containing the Extension Manifest package.json
	// Passed to `--extensionDevelopmentPath`
	const extensionDevelopmentPath = path.resolve(__dirname, "../../");

	// Minimum supported version.
	const jsonData = fs.readFileSync(
		path.join(extensionDevelopmentPath, "package.json"),
	);
	const minimalVersion = pipe(
		PackageJson.decode(jsonData.toString()),
		fold(
			(errors) => {
				throw Error(`Invalid package.json: ${JSON.stringify(errors)}`);
			},
			(parsed) => {
				if (
					parsed.engines.vscode[0] &&
					"~^=".indexOf(parsed.engines.vscode[0])
				) {
					return parsed.engines.vscode.slice(1);
				}
				return parsed.engines.vscode;
			},
		),
	);

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

main().catch((error: unknown) => {
	// biome-ignore: noConsole
	console.error("Failed to run tests", error);
	process.exit(1);
});
