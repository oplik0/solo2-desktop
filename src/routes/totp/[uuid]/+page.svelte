<script lang="ts">
    import { browser } from '$app/environment';
	import OathCard from '$lib/OathCard.svelte';
    import { listen } from '@tauri-apps/api/event';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import type {PageData} from "./$types";
    export let data: PageData;
	
	let oathList: Array<string>;
    let uuid = data.uuid ?? "";
    if (browser) {
        listen("usb_change", async () => {
            oathList = await invoke("list_oath", { uuid });
        })
    }
    onMount(async () => {
        oathList = await invoke("list_oath", { uuid });
    })
</script>

<svelte:head>
	<title>TOTP List</title>
	<meta name="description" content="List of TOTP Credentials for " />
</svelte:head>

<section>
	{#if oathList}
        {#each Object.keys(oathList) as credential}
            <OathCard {credential} {uuid} />
        {/each}
    {/if}
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		flex: 1;
        width: 100%;
        max-width: 1200px;
        padding: 0;
	}
</style>
