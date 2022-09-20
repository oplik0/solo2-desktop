<script lang="ts">
	import "fluent-svelte/theme.css";
	import Breadcrumbs from "$lib/Breadcrumbs.svelte";
	import "../app.css";
</script>

<Breadcrumbs />

<main data-sveltekit-prefetch>
	<slot />
</main>

<style>
	:global(body) {
		background-color: var(--fds-solid-background-base);
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
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 40px;
	}

	footer a {
		font-weight: bold;
	}

	@media (min-width: 480px) {
		footer {
			padding: 40px 0;
		}
	}
</style>
