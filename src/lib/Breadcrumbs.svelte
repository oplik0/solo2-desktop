<script lang="ts">
	import { page } from "$app/stores";
	import { TextBlock } from "fluent-svelte";

	function extractRoute(pathname: string, breadcrumb: string) {
		return pathname.substring(
			0,
			pathname.indexOf(breadcrumb) + breadcrumb.length
		);
	}
</script>

<header>
	<TextBlock variant="title" class="breadcrumb-text {$page.url?.pathname === "/" ? "current-link" : ""}"
		><a href="/">Key List</a></TextBlock
	>

	{#each Object.entries($page.url?.pathname?.split("/")?.slice(1) ?? []) as [i, breadcrumb]}
		{#if breadcrumb.length > 0}
			<svg
				width="16"
				height="16"
				fill="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="M8.293 4.293a1 1 0 0 0 0 1.414L14.586 12l-6.293 6.293a1 1 0 1 0 1.414 1.414l7-7a1 1 0 0 0 0-1.414l-7-7a1 1 0 0 0-1.414 0Z"
				/>
			</svg>
			<TextBlock variant="title" class="breadcrumb-text breadcrumb-text-next {$page.url?.pathname?.split("/").length - 2 === parseInt(i, 10) ? "current-link" : ""}" >
				<a href={extractRoute($page?.url?.pathname, breadcrumb)}>
					{breadcrumb}
				</a>
			</TextBlock>
		{/if}
	{/each}
</header>

<style>
	:global(.breadcrumb-text) {
		text-transform: capitalize;
		max-width: 30%;
		overflow: hidden;
		text-overflow: ellipsis;
		text-decoration: none;
	}
	:global(.breadcrumb-text > a) {
		text-transform: capitalize;
		overflow: hidden;
		text-overflow: ellipsis;
		text-decoration: none;
	}
	svg {
		align-self: center;
	}
	header {
		display: flex;
		flex-direction: row;
		justify-content: flex-start;
		flex: 1;
		width: 100%;
		max-width: 1200px;
		margin-left: auto;
		margin-right: auto;
		margin-bottom: 24px;
		gap: 6px;
		grid-area: breadcrumbs;
		padding: 0 12px;
	}
	a {
		color: var(--fds-text-secondary);
		cursor: default;
	}
	a:hover {
		text-decoration: none;
		color: var(--fds-text-primary);
		cursor: default;
	}
	:global(.current-link > a) {
		color: var(--fds-text-primary);
	}
</style>
