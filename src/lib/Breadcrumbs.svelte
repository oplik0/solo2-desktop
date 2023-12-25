<script lang="ts">
	import { TextBlock } from "fluent-svelte-extra";
	import ChevronRight from "$icons/chevron-right.svg?component";
	import { location } from "svelte-spa-router";
	function extractRoute(pathname: string, breadcrumb: string) {
		return pathname.substring(
			0,
			pathname.indexOf(breadcrumb) + breadcrumb.length
		);
	}
</script>

<header>
	<TextBlock
		variant="title"
		class="breadcrumb-text {$location === '/' ? 'current-link' : ''}"
		><a href="#/">Key List</a></TextBlock
	>

	{#each Object.entries($location
			?.split("/")
			?.slice(1) ?? []) as [i, breadcrumb]}
		{#if breadcrumb.length > 0}
			<ChevronRight class="breadcrumb-separator" />
			<TextBlock
				variant="title"
				class="breadcrumb-text breadcrumb-text-next {$location?.split('/')
					.length -
					2 ===
				parseInt(i, 10)
					? 'current-link'
					: ''}"
			>
				<a href={extractRoute($location, breadcrumb)}>
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
	:global(.breadcrumb-separator) {
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
