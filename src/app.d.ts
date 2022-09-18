// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
declare namespace App {
	interface Locals {
		userid: string;
	}

	// interface PageData {}

	// interface PageError {}

	// interface Platform {}
}

interface Solo2 {
	uuid: string;
	version: string;
	secure: ?boolean;
}
interface Solo2List {
	[uuid: string]: Solo2;
}
