<script lang="ts">
	import KeyCard from "$lib/KeyCard.svelte";
	import { addNamesToKeys } from "$lib/keyName";
	import { listen } from "@tauri-apps/api/event";
	import { invoke } from "@tauri-apps/api/tauri";
	import type { Solo2List } from "$types";
	import { onMount } from "svelte";
	import type { UpdateData } from "../types";

	let keyList: Solo2List;
	let latest_version: string;
	listen("usb_change", async () => {
		keyList = await addNamesToKeys(await invoke("list_keys"));
	});
	listen("latest_version", (version) => {
		latest_version = version.payload as string;
	});
	onMount(async () => {
		keyList = await addNamesToKeys(await invoke("list_keys"));
		latest_version = await invoke("latest_version");
	});
</script>

<svelte:head>
	<title>Key List</title>
	<meta name="description" content="List of Solokeys" />
</svelte:head>

<section>
	{#if keyList}
		{#each Object.values(keyList) as key}
			<KeyCard {key} {latest_version} />
		{/each}
	{/if}
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
