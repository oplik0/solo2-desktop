<script lang="ts">
	import { link, location } from "svelte-spa-router";
	import type { WrappedComponent } from "svelte-spa-router";
	import { ListItem } from "fluent-svelte";
	interface Items {
		[path: string]: WrappedComponent & {
			userData?: { title?: string; description?: string };
		};
	}
	export let items: Items;
	items = Object.fromEntries(
		Object.entries(items).filter(([path, userData]) =>
			path.match(/^\/(?![:*]+)[\w\/]*$/g)
		)
	);
</script>

<nav>
	{#each Object.keys(items) as href}
		<ListItem
			selected={href.split("/")[1] === $location.split("/")[1]}
			href={"#" + href}
			role="navigation"
		>
			{items[href]?.userData?.title}
		</ListItem>
	{/each}
</nav>

<style>
	nav {
		display: flex;
		flex-direction: column;
		height: 100%;
		max-height: 95vh;
		grid-column-start: 1;
		grid-column-end: 2;
		width: 100%;
		grid-area: nav;
		margin: 12px;
	}
</style>
