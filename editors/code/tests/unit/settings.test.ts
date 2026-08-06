/** biome-ignore-all lint/style/useNamingConvention: environment variables naming convention */
/** biome-ignore-all lint/suspicious/noTemplateCurlyInString: substituteVariablesInEnv */
import { deepStrictEqual } from "node:assert/strict";
import process from "node:process";
import { substituteVariablesInEnv } from "../../src/config.ts";
import type { Context } from "./index.ts";

export async function getTests(context: Context) {
	await context.suite("Server Env Settings", (suite) => {
		suite.addSyncTest("Replacing Env Variables", () => {
			const envJson = {
				USING_MY_VAR: "${env:MY_VAR} test ${env:MY_VAR}",
				MY_VAR: "test",
			};
			const expectedEnv = {
				USING_MY_VAR: "test test test",
				MY_VAR: "test",
			};
			const actualEnv = substituteVariablesInEnv(envJson);
			deepStrictEqual(actualEnv, expectedEnv);
		});

		suite.addSyncTest("Circular dependencies remain as is", () => {
			const envJson = {
				A_USES_B: "${env:B_USES_A}",
				B_USES_A: "${env:A_USES_B}",
				C_USES_ITSELF: "${env:C_USES_ITSELF}",
				D_USES_C: "${env:C_USES_ITSELF}",
				E_IS_ISOLATED: "test",
				F_USES_E: "${env:E_IS_ISOLATED}",
			};
			const expectedEnv = {
				A_USES_B: "${env:B_USES_A}",
				B_USES_A: "${env:A_USES_B}",
				C_USES_ITSELF: "${env:C_USES_ITSELF}",
				D_USES_C: "${env:C_USES_ITSELF}",
				E_IS_ISOLATED: "test",
				F_USES_E: "test",
			};
			const actualEnv = substituteVariablesInEnv(envJson);
			deepStrictEqual(actualEnv, expectedEnv);
		});

		suite.addSyncTest("Should support external variables", () => {
			process.env["TEST_VARIABLE"] = "test";
			const envJson = {
				USING_EXTERNAL_VAR: "${env:TEST_VARIABLE} test ${env:TEST_VARIABLE}",
			};
			const expectedEnv = {
				USING_EXTERNAL_VAR: "test test test",
			};

			const actualEnv = substituteVariablesInEnv(envJson);
			deepStrictEqual(actualEnv, expectedEnv);
			// biome-ignore lint/performance/noDelete: this is correct
			// biome-ignore lint/complexity/useLiteralKeys: Property 'TEST_VARIABLE' comes from an index signature
			delete process.env["TEST_VARIABLE"];
		});

		suite.addSyncTest("should support VSCode variables", () => {
			const envJson = {
				USING_VSCODE_VAR: "${workspaceFolderBasename}",
			};
			const actualEnv = substituteVariablesInEnv(envJson);
			deepStrictEqual(actualEnv["USING_VSCODE_VAR"], "code");
		});
	});
}
