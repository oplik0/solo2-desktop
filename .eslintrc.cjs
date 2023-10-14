module.exports = {
	root: true,
	parser: "@typescript-eslint/parser",
	extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended", "plugin:svelte/recommended"],
	plugins: ["@typescript-eslint"],
	ignorePatterns: [
		"*.cjs",
		"**/target/**",
	],
	overrides: [
		{
			files: ["*.svelte"],
			parser: "svelte-eslint-parser",
			parserOptions: {
				parser: {
					// Specify a parser for each lang.
					ts: "@typescript-eslint/parser",
					js: "espree",
					typescript: "@typescript-eslint/parser",
				},
			},
		},
	],
	rules: {
		"@typescript-eslint/no-unused-vars": ["warn", { argsIgnorePattern: "^_" }],
		"@typescript-eslint/ban-ts-comment": ["error", { "ts-ignore": "allow-with-description" }],
	},
	parserOptions: {
		sourceType: "module",
		ecmaVersion: 2020,
	},
	env: {
		browser: true,
		es2017: true,
		node: true,
	},
};
