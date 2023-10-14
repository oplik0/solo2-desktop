import svg from "@poppanator/sveltekit-svg";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "node:path";
import type { UserConfig } from "vite";
const config: UserConfig = {
	plugins: [
		svelte(),
		svg({
			includePaths: ["src/assets/icons", "src/assets"],
		}),
	],
	resolve: {
		alias: {
			$lib: resolve(__dirname, "src/lib"),
			$routes: resolve(__dirname, "src/routes"),
			$assets: resolve(__dirname, "src/assets"),
			$icons: resolve(__dirname, "src/assets/icons"),
		},
	},
	build: {
		outDir: "build",
	},
};

export default config;
