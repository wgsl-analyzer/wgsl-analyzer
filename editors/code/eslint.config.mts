import eslintConfigPrettier from "eslint-config-prettier";
import stylistic from "@stylistic/eslint-plugin";
import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import stylisticJs from "@stylistic/eslint-plugin-js";

const config = [
	eslintConfigPrettier,
	eslint.configs.recommended,
	stylisticJs.configs["disable-legacy"],
	...tseslint.configs.recommended,
	stylistic.configs.customize({
		indent: "tab",
		quotes: "double",
		semi: true,
		braceStyle: "1tbs",
		arrowParens: true,
	}),
	{
		rules: {
			"no-console": "warn",
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					args: "all",
					argsIgnorePattern: "^_",
					caughtErrors: "all",
					caughtErrorsIgnorePattern: "^_",
					destructuredArrayIgnorePattern: "^_",
					varsIgnorePattern: "^_",
					ignoreRestSiblings: true,
				},
			],
			// the following stylistic lints conflict with prettier
			"@stylistic/operator-linebreak": "off",
			"@stylistic/indent-binary-ops": "off",
			"@stylistic/indent": "off",
			"@stylistic/brace-style": "off",
			"@stylistic/quotes": "off",
			"@stylistic/no-mixed-spaces-and-tabs": "off",
		},
	},
	{
		ignores: ["out/", ".vscode-test/", "node_modules/"],
	},
];

export default config;
