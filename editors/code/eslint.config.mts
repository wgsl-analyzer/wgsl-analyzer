import eslint from "@eslint/js";
import stylistic from "@stylistic/eslint-plugin";
import safeTsPlugin from "@susisu/eslint-plugin-safe-typescript";
import eslintConfigPrettier from "eslint-config-prettier";
import importPlugin from "eslint-plugin-import-x";
// import perfectionist from "eslint-plugin-perfectionist";
import regexpPlugin from "eslint-plugin-regexp";
import simpleImportSortPlugin from "eslint-plugin-simple-import-sort";
import unicornPlugin from "eslint-plugin-unicorn";
import tseslint from "typescript-eslint";

type Config = Parameters<typeof tseslint.config>[number];

const GLOB_TS = ["*.{ts,tsx,cts,mts}", "**/*.{ts,tsx,cts,mts}"];
const dirname = import.meta.dirname;
const sourceTsconfigArray = ["packages/*/tsconfig.json", "packages/*/*/tsconfig.json"];

const plugins: Config = {
	// register all of the plugins up-front
	// note - intentionally uses computed syntax to make it easy to sort the keys
	plugins: {
		["@stylistic/eslint-plugin"]: stylistic,
		["@susisu/safe-typescript"]: safeTsPlugin,
		["@typescript-eslint"]: tseslint.plugin,
		["eslint"]: eslint,
		["import-x"]: importPlugin,
		["simple-import-sort"]: simpleImportSortPlugin,
		["unicorn"]: unicornPlugin,
	},
};

const config: Config[] = [
	plugins,
	eslintConfigPrettier,
	eslint.configs.recommended,
	...tseslint.configs.strictTypeChecked,
	regexpPlugin.configs["flat/recommended"],
	// perfectionist.configs["recommended-natural"],
	stylistic.configs.customize({
		arrowParens: true,
		braceStyle: "1tbs",
		indent: "tab",
		quotes: "double",
		semi: true,
	}),
	// base ts language options
	{
		files: GLOB_TS,
		languageOptions: {
			parser: tseslint.parser,
			parserOptions: {
				allowAutomaticSingleRunInference: true,
				project: sourceTsconfigArray,
				projectService: true,
				tsconfigRootDir: dirname,
				warnOnUnsupportedTypeScriptVersion: false,
			},
		},
	},
	{
		rules: {
			"@stylistic/brace-style": "warn",
			"@stylistic/indent": "off",
			"@stylistic/indent-binary-ops": "off",
			"@stylistic/no-mixed-spaces-and-tabs": "off",
			// the following stylistic lints conflict with prettier
			"@stylistic/operator-linebreak": "off",
			"@stylistic/quotes": "off",
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					args: "all",
					argsIgnorePattern: "^_",
					caughtErrors: "all",
					caughtErrorsIgnorePattern: "^_",
					destructuredArrayIgnorePattern: "^_",
					ignoreRestSiblings: true,
					varsIgnorePattern: "^_",
				},
			],
			"@typescript-eslint/restrict-template-expressions": [
				"error",
				{
					allowNumber: true,
				},
			],
			"@typescript-eslint/no-empty-object-type": "off",
			"no-console": "warn",
			"sort-objects": "off",
			"sort-modules": "off",
			"sort-object-types": "off",
			"sort-union-types": "off",
		},
	},
	{
		ignores: ["out/", ".vscode-test/", "node_modules/"],
	},
];

export default tseslint.config(...config);
