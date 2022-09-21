<script lang="ts">
	import "fluent-svelte/theme.css";
	import Breadcrumbs from "$lib/Breadcrumbs.svelte";
	import "../app.css";
	import Nav from "$lib/Nav.svelte";
	const pages = [
		{
			href: "/",
			name: "Key List",
		},
		{
			href: "/totp",
			name: "TOTP",
		},
	];
</script>

<Breadcrumbs />
<Nav items={pages} />
<main data-sveltekit-prefetch>
	<slot />
</main>

<style>
	:global(body) {
		/* background-color: var(--fds-solid-background-base); */
		color: var(--fds-text-primary);
	}
	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		width: 100%;
		max-width: 1024px;
		margin: 0 auto;
		box-sizing: border-box;
		grid-area: content;
		padding: 0 12px;
	}
</style>
