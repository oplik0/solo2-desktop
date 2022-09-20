<script lang="ts">
	import {
		Button,
		ComboBox,
		TextBlock,
		ProgressRing,
		ContentDialog,
		TextBox,
		Expander,
		RadioButton,
	} from "fluent-svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	export let uuid: string[];
	let selected_uuid: string = uuid[0];
	let open = false;
	let label: string;
	let issuer: string;
	let secret: string;
	let kind: "totp" | "hotp" = "totp";
	let algorithm: "sha1" | "sha256" = "sha1";
	let period = 30;
	let digits = 6;
	async function registerOath() {
		await invoke("register_oath", {
			uuid: selected_uuid,
			label,
			issuer: issuer.length > 0 ? issuer : undefined,
			secret,
			kind,
			algorithm,
			period,
			digits,
		});
		open = false;
	}
</script>

<section>
	<TextBlock variant="subtitle">Register a new TOTP credential</TextBlock>
	<Button variant="accent" on:click={() => (open = !open)}
		>Add crededntial</Button
	>
	<ContentDialog {open} class="register-dialog">
		{#if uuid.length > 1}
			<TextBlock variant="title">Register a new TOTP credential</TextBlock>
			<ComboBox
				items={[...new Set(uuid)].map((uuid) => ({ name: uuid, value: uuid }))}
				bind:value={selected_uuid}
			/>
		{:else}
			<TextBlock variant="title">Register a new TOTP credential</TextBlock>
		{/if}
		<fieldset>
			<TextBlock>Label</TextBlock>
			<TextBox bind:value={label} type="text" placeholder="label" />
		</fieldset>
		<fieldset>
			<TextBlock>Issuer (optional)</TextBlock>
			<TextBox bind:value={issuer} type="text" />
		</fieldset>
		<fieldset>
			<TextBlock>TOTP Secret</TextBlock>
			<TextBox bind:value={secret} type="text" />
		</fieldset>
		<Expander>
			<TextBlock>Advanced</TextBlock>
			<svelte:fragment slot="content">
				<fieldset>
					<TextBlock>Algorithm</TextBlock>
					<div class="radio">
						<RadioButton bind:group={algorithm} value="sha1">SHA1</RadioButton>
						<RadioButton bind:group={algorithm} value="sha256"
							>SHA256</RadioButton
						>
					</div>
				</fieldset>
				<fieldset>
					<TextBlock>TOTP Period</TextBlock>
					<TextBox bind:value={period} type="number" placeholder="30" />
				</fieldset>
				<fieldset>
					<TextBlock>Digits</TextBlock>
					<TextBox bind:value={digits} type="number" placeholder="6" />
				</fieldset>
			</svelte:fragment>
		</Expander>
		<svelte:fragment slot="footer">
			<Button
				slot="footer"
				variant="accent"
				disabled={!label?.length || !secret?.length}
				on:click={registerOath}>Register</Button
			>
			<Button slot="footer" on:click={() => (open = false)}>Cancel</Button>
		</svelte:fragment>
	</ContentDialog>
</section>

<style>
	section {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		flex: 1;
		border: 1px solid var(--fds-card-stroke-default);
		border-radius: var(--fds-control-corner-radius);
		background-clip: padding-box;
		background-color: var(--fds-card-background-default);
		box-shadow: var(--fds-card-shadow);
		padding: 24px;
		outline: none;
		width: 100%;
		max-width: 1200px;
		margin-left: auto;
		margin-right: auto;
		box-sizing: border-box;
		gap: 12px;
	}
	:global(.register-dialog > div) {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}
	fieldset {
		display: flex;
		flex-direction: column;
		gap: 6px;
		outline: none;
		border: none;
	}
	.radio {
		display: flex;
		flex-direction: row;
		gap: 12px;
	}
</style>
