<script lang="ts">
    import { browser } from '$app/environment';
	import OathCard from '$lib/OathCard.svelte';
    import AddOath from '$lib/AddOath.svelte';
    import { listen } from '@tauri-apps/api/event';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
	
	let oathList: Record<string, string>;
    if (browser) {
        listen("usb_change", async () => {
            oathList = await invoke("list_oath");
        })
    }
    onMount(async () => {
        oathList = await invoke("list_oath");
    })
</script>

<svelte:head>
	<title>TOTP List</title>
	<meta name="description" content="List of TOTP Credentials" />
</svelte:head>

<section>
	{#if oathList}
        {#each Object.entries(oathList) as [credential, uuid]}
            <OathCard {credential} {uuid} />
        {/each}
    {/if}
    <AddOath></AddOath>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 1;
	}
</style>
