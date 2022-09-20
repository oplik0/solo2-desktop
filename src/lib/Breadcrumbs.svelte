<script lang="ts">
    import { page } from '$app/stores';
	import { TextBlock } from "fluent-svelte";
</script>

<header>
    <TextBlock variant="title" class="breadcrumb-text"><a href="/">Key List</a></TextBlock>

    {#each $page.url?.pathname?.split("/")?.slice(1) ?? [] as breadcrumb}
        <svg width="16" height="16" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path d="M8.293 4.293a1 1 0 0 0 0 1.414L14.586 12l-6.293 6.293a1 1 0 1 0 1.414 1.414l7-7a1 1 0 0 0 0-1.414l-7-7a1 1 0 0 0-1.414 0Z" />
        </svg>
        <TextBlock variant="title" class="breadcrumb-text breadcrumb-text-next"><a href={$page.url?.pathname.substring(0, $page.url?.pathname.indexOf(breadcrumb) + breadcrumb.length)}>{breadcrumb}</a></TextBlock>
    {/each}
</header>

<style>
    :global(.breadcrumb-text) {
        text-transform: capitalize;
    }
    :global(.breadcrumb-text-next::before) {
        content: url({});
        height: 24px;
        width: 24px;
        position: relative;
        left: -12px;
    }
    svg {
        align-self: center;
    }
    header {
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        flex: 1;
        width: calc(100vw - 24px);
        max-width: 1200px;
        margin-left: auto;
        margin-right: auto;
        margin-bottom: 24px;
        gap: 6px;
    }
    a {
        color: var(--fds-text-primary);
    }
    a:hover {
        text-decoration: none;
    }

</style>