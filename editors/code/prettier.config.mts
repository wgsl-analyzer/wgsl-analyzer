import { type Config } from "prettier";

const config: Config = {
	bracketSpacing: true,
	endOfLine: "lf",
	experimentalOperatorPosition: "start",
	experimentalTernaries: false,
	// use 100 because it is rustfmt's default
	// https://rust-lang.github.io/rustfmt/?version=v1.4.38&search=#max_width
	printWidth: 100,
	// use 4 because it is rustfmt's default
	// https://rust-lang.github.io/rustfmt/?version=v1.4.38&search=#%5C34%20%5C%20%5C(default%5C)%5C%3A
	tabWidth: 4,
	trailingComma: "all",
	useTabs: true,
};

export default config;
