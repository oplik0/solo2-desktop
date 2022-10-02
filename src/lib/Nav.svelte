<script lang="ts">
	import { page } from "$app/stores";
	import { ListItem } from "fluent-svelte";
	interface Item {
		href: string;
		name: string;
		icon?: string;
	}

	export let items: Item[];
</script>

<nav>
	{#each items as { href, name, icon }}
		<ListItem
			selected={$page.url.pathname === href ||
				($page.url.pathname.split("/").length > 2 &&
					href.split("/").length > 2 &&
					$page.url.pathname.startsWith(href))}
			{href}
			role="navigation"
		>
			{#if icon && icon.startsWith("<svg")}
				{@html icon}
			{/if}
			{name}
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
