<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import {browser} from "$app/environment";
    import KeyListCard from "./KeyListCard.svelte";
    let solo2list: Solo2List;
    if (browser) {
        listen("usb_change", async () => {
            solo2list = await invoke("list_keys");
        })
    }
    onMount(async () => {
        solo2list = await invoke("list_keys");
    })
</script>

<section>
    {#if solo2list}
        {#each Object.values(solo2list) as key}
            <KeyListCard key={key} />
        {/each}
    {/if}
</section>