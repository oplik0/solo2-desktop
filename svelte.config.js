import preprocess from "svelte-preprocess";

const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),
	vitePlugin: {
		experimental: {
			useVitePreprocess: true,
		},
	},
};

export default config;
