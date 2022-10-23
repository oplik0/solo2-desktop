import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "node:path";
import type { UserConfig } from "vite";
import svelteSVG from "vite-plugin-svelte-svg";
const config: UserConfig = {
	plugins: [
		svelte(),
		svelteSVG({
			svgoConfig: {},
			requireSuffix: true,
		}),
	],
	resolve: {
		alias: {
			$lib: resolve(__dirname, "src/lib"),
			$assets: resolve(__dirname, "src/assets"),
			$icons: resolve(__dirname, "src/assets/icons"),
		},
	},
	build: {
		outDir: "build",
	},
};

export default config;
