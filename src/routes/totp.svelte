<script lang="ts">
	import TotpCard from "$lib/TotpCard.svelte";
	import AddOath from "$lib/AddOath.svelte";
	import { listen } from "@tauri-apps/api/event";
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import type { Solo2List } from "$types";
	export let params: Record<string, unknown> = {};
	let oathList: Record<string, string> | undefined;
	let keyList: Solo2List | undefined;
	async function refreshOathList() {
		oathList = await invoke("list_oath");
		if (!oathList || Object.keys(oathList).length === 0) {
			oathList = undefined;
			keyList = await invoke("list_keys");
		}
	}
	listen("usb_change", refreshOathList);
	listen("oath_change", refreshOathList);
	onMount(refreshOathList);
</script>

<svelte:head>
	<title>TOTP List</title>
	<meta name="description" content="List of TOTP Credentials" />
</svelte:head>

<section>
	{#if oathList}
		{#each Object.entries(oathList ?? {}).filter(([_, uuid]) => !params.uuid || uuid === params.uuid) as [credential, uuid]}
			<TotpCard {credential} {uuid} />
		{/each}
	{/if}
	<AddOath
		uuid={Object.values(oathList ?? keyList ?? {})
			.map((val) => (typeof val === "string" ? val : val.uuid))
			.filter((uuid) => !params.uuid || uuid === params.uuid)}
	/>
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
