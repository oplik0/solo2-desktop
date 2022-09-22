<script lang="ts">
	import { browser } from "$app/environment";
	import TotpCard from "$lib/TotpCard.svelte";
	import AddOath from "$lib/AddOath.svelte";
	import { listen } from "@tauri-apps/api/event";
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	let oathList: Record<string, string>;
	async function refreshOathList() {
		oathList = await invoke("list_oath");
	}
	if (browser) {
		listen("usb_change", refreshOathList);
		listen("oath_change", refreshOathList);
	}
	onMount(refreshOathList);
</script>

<svelte:head>
	<title>TOTP List</title>
	<meta name="description" content="List of TOTP Credentials" />
</svelte:head>

<section>
	{#if oathList}
		{#each Object.entries(oathList) as [credential, uuid]}
			<TotpCard {credential} {uuid} />
		{/each}
	{/if}
	<AddOath uuid={Object.values(oathList ?? {})} />
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
