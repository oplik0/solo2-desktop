<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { Button, Expander, TextBlock, ProgressRing, InfoBadge, ContentDialog } from "fluent-svelte";
	import { parse, gt, major } from "semver";

	export let key: Solo2;
	export let latest_version: string = "0.0.0";
	let dialogOpen = false;
	let updating = false;
	async function update() {
		updating = true;
		try {
			await invoke("update_key", {uuid: key.uuid});
		} finally {
			updating = false;
		}
		console.log("updated")
	}
	function toCalver(version: string) {
		const semver = parse(version);
		if (semver!==null) {
			const date = new Date((new Date("2020-01-01")).setDate(1 + semver.minor));
			const formatter = new Intl.DateTimeFormat('en-us', {
				year: 'numeric',
				month: '2-digit',
				day: '2-digit'
			});
			const formatted_date = Object.fromEntries(formatter.formatToParts(date).map(x => [x.type, x.value]));
			return `${semver.major}:${formatted_date.year}${formatted_date.month}${formatted_date.day}:${semver.patch}`;
		}
			
		return "Invalid version";
	}
</script>
<section>
	<Expander headingLevel={2} expanded class="key-expander">
		<svg slot="icon" width="128" height="128" viewBox="0 0 16 16" fill="currentColor" class="expander-icon" xmlns="http://www.w3.org/2000/svg">
			{#if key.secure}
			<!-- MIT Copyright (c) 2020 Microsoft Corporation - static/licenses/fluentui-icons-license-->
				<path d="M7.83562 1.50532L8 1.5C9.32548 1.5 10.41 2.53154 10.4947 3.83562L10.5 4V5H11.5C12.3284 5 13 5.67157 13 6.5V12.5C13 13.3284 12.3284 14 11.5 14H4.5C3.67157 14 3 13.3284 3 12.5V6.5C3 5.67157 3.67157 5 4.5 5H5.5V4C5.5 2.67452 6.53154 1.58996 7.83562 1.50532ZM11.5 6H4.5C4.22386 6 4 6.22386 4 6.5V12.5C4 12.7761 4.22386 13 4.5 13H11.5C11.7761 13 12 12.7761 12 12.5V6.5C12 6.22386 11.7761 6 11.5 6ZM8 8.5C8.55228 8.5 9 8.94772 9 9.5C9 10.0523 8.55228 10.5 8 10.5C7.44772 10.5 7 10.0523 7 9.5C7 8.94772 7.44772 8.5 8 8.5ZM8.14446 2.50687L8 2.5C7.2203 2.5 6.57955 3.09489 6.50687 3.85554L6.5 4V5H9.5V4C9.5 3.2203 8.90511 2.57955 8.14446 2.50687Z" />
			{:else}
			<!-- MIT Copyright (c) 2020 Microsoft Corporation - static/licenses/fluentui-icons-license-->
				<path d="M8 11.5C8.55228 11.5 9 11.0523 9 10.5C9 9.94771 8.55228 9.5 8 9.5C7.44772 9.5 7 9.94771 7 10.5C7 11.0523 7.44772 11.5 8 11.5ZM8 2.5C7.17157 2.5 6.5 3.17157 6.5 4V6H11.5C12.3284 6 13 6.67157 13 7.5V13.5C13 14.3284 12.3284 15 11.5 15H4.5C3.67157 15 3 14.3284 3 13.5V7.5C3 6.67157 3.67157 6 4.5 6H5.5V4C5.5 2.61929 6.61929 1.5 8 1.5C9.38071 1.5 10.5 2.61929 10.5 4C10.5 4.27614 10.2761 4.5 10 4.5C9.72386 4.5 9.5 4.27614 9.5 4C9.5 3.17157 8.82843 2.5 8 2.5ZM4.5 7C4.22386 7 4 7.22386 4 7.5V13.5C4 13.7761 4.22386 14 4.5 14H11.5C11.7761 14 12 13.7761 12 13.5V7.5C12 7.22386 11.7761 7 11.5 7H4.5Z" />
			{/if}
		</svg>		
		<div class="expander-title">
			<TextBlock variant="subtitle">{key.uuid}</TextBlock>
			<TextBlock variant="caption">v{toCalver(key.version)}</TextBlock>
			{#if gt(latest_version, key.version)}
				<InfoBadge severity="attention" class="new-version-badge">New version available</InfoBadge>
			{/if}
		</div>
		<svelte:fragment slot="content">
			<div class="expanded-content">
				<TextBlock variant="bodyStrong">semver: v{key.version}</TextBlock>
				<TextBlock variant="bodyStrong">variant: {key.secure ? "secure" : "hacker"}</TextBlock>
				<div>
					<Button variant="hyperlink" href="/totp/{key.uuid}">TOTP</Button>
					<Button variant={gt(latest_version, key.version) ? "accent" : "standard"} on:click={() => {dialogOpen = true}} disabled={updating} style="width: 85px">
						{#if updating}
							<ProgressRing size={20}></ProgressRing>
						{:else}
							<TextBlock>Update</TextBlock>
						{/if}
					</Button>
				</div>
				
			</div>
			
		</svelte:fragment>
	</Expander>
	<ContentDialog bind:open={dialogOpen} title="Are you sure you want to update?">
		<TextBlock variant="body">This will attempt to update the key with UUID {key.uuid}.</TextBlock>
		<TextBlock variant="body">If you want to continue press "Yes" and touch the key once the LED changes color. Unplugging the before touching any buttons will cancel the update</TextBlock>
		{#if major(latest_version) == major(key.version)}
			<TextBlock variant="body"><InfoBadge severity="caution">Warning </InfoBadge> This is is major update and it could risk breaking any current credentials on your key.</TextBlock>
			<TextBlock variant="body">Check latest release notes here to double check: <a href="https://github.com/solokeys/solo2/releases" target="_blank">https://github.com/solokeys/solo2/releases</a></TextBlock>
		{/if}	
		
		<svelte:fragment slot="footer">
			<Button slot="footer" on:click={() => (dialogOpen = false)}>No</Button>
			<Button slot="footer" variant="accent" on:click={() => {dialogOpen = false; update()}}>Yes</Button>
		</svelte:fragment>
		
	</ContentDialog>
</section>


<style>
	section {
		width: calc(100vw - 24px);
		max-width: 1200px;
	}
	.expanded-content {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
	}
	.expander-title {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		vertical-align: baseline;
	}
</style>	