<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	export let key: Solo2;
</script>

<section>
	<h1>{key.uuid}</h1>
	<h2>v{key.version}</h2>
	<h3>{key.secure ? "secure" : "hacker"}</h3>
	<button on:click={() => invoke("update_key", { uuid: key.uuid })}>Update</button>
</section>
