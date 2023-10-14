<script lang="ts">
	import "fluent-svelte/theme.css";
	import Breadcrumbs from "$lib/Breadcrumbs.svelte";
	import Router, { type AsyncSvelteComponent } from "svelte-spa-router";
	import { wrap } from "svelte-spa-router/wrap";
	import Nav from "$lib/Nav.svelte";
	import Keys from "$routes/keys.svelte";
	const routes = {
		"/": wrap({
			// @ts-ignore
			component: Keys,
			userData: {
				title: "Keys",
				description: "List of keys",
			},
		}),
		"/totp": wrap({
			asyncComponent: (() =>
				import("$routes/totp.svelte")) as AsyncSvelteComponent,
			userData: {
				title: "TOTP",
				description: "List of TOTP Credentials",
			},
		}),
		"/totp/:uuid": wrap({
			asyncComponent: (() =>
				import("$routes/totp.svelte")) as AsyncSvelteComponent,
			userData: {
				title: "TOTP",
				description: "List of TOTP Credentials",
			},
		}),
		"*": wrap({
			// @ts-ignore
			component: Keys,
		}),
	};
</script>

<Breadcrumbs />
<Nav items={routes} />
<main>
	<Router {routes} />
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
