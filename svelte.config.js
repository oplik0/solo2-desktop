import adapter from "@sveltejs/adapter-static";
import preprocess from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter(),
		csp: {
			directives: {
				"default-src": ["self"],
				"script-src": ["self", "unsafe-inline"],
				"frame-src": ["self", "*.localhost"],
			},
			mode: "hash",
		},
	},
	vitePlugin: {
		experimental: {
			useVitePreprocess: true,
		},
	},
};

export default config;
