<script lang="ts">
	import {
		Button,
		ComboBox,
		TextBlock,
		ContentDialog,
		TextBox,
		Expander,
		RadioButton,
	} from "fluent-svelte-extra";
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import { loadKeyName } from "./keyName";
	export let uuid: string[];
	let selected_uuid: string;
	let open = false;
	let label: string;
	let issuer: string;
	let secret: string;
	let kind: "totp" | "hotp" = "totp";
	let algorithm: "sha1" | "sha256" = "sha1";
	let period = 30;
	let digits = 6;
	let keyNames: Record<string, string> = {};
	onMount(async () => {
		for (const id of uuid) {
			keyNames[id] = await loadKeyName(id);
		}
	});
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
	function openDialog() {
		open = !open;
		selected_uuid = selected_uuid ?? uuid[0];
	}
</script>

<section>
	<TextBlock variant="subtitle"
		>Register a new {kind.toUpperCase()} credential</TextBlock
	>
	<Button variant="accent" on:click={openDialog}>Add crededntial</Button>
	<ContentDialog {open} class="register-dialog">
		<div class="dialog-title">
			<TextBlock variant="subtitle"
				>Register a new {kind.toUpperCase()} credential</TextBlock
			>
		</div>
		<form>
			{#if uuid.length > 1}
				<fieldset>
					<TextBlock>Key</TextBlock>
					<ComboBox
						items={[...new Set(uuid)].map((uuid) => ({
							name: keyNames[uuid] ?? uuid,
							value: uuid,
						}))}
						bind:value={selected_uuid}
					/>
				</fieldset>
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
				<TextBlock>{kind.toUpperCase()} Secret</TextBlock>
				<TextBox bind:value={secret} type="text" />
			</fieldset>
			<Expander class="addTotpExpander">
				<TextBlock>Advanced</TextBlock>
				<svelte:fragment slot="content">
					<fieldset>
						<TextBlock>Kind</TextBlock>
						<div class="radio">
							<RadioButton
								bind:group={kind}
								on:input={() => (period = 30)}
								value="totp">TOTP</RadioButton
							>
							<RadioButton
								bind:group={kind}
								on:input={() => (period = 0)}
								value="hotp">HOTP</RadioButton
							>
						</div>
					</fieldset>
					<fieldset>
						<TextBlock>Algorithm</TextBlock>
						<div class="radio">
							<RadioButton bind:group={algorithm} value="sha1">SHA1</RadioButton
							>
							<RadioButton bind:group={algorithm} value="sha256"
								>SHA256</RadioButton
							>
						</div>
					</fieldset>
					<fieldset>
						<TextBlock
							>{kind == "totp"
								? "TOTP Period"
								: "HOTP initial counter"}</TextBlock
						>
						<TextBox
							bind:value={period}
							type="number"
							placeholder={kind == "totp" ? "30" : "0"}
						/>
					</fieldset>
					<fieldset>
						<TextBlock>Digits</TextBlock>
						<TextBox bind:value={digits} type="number" placeholder="6" />
					</fieldset>
				</svelte:fragment>
			</Expander>
		</form>
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
	:global(.register-dialog > .content-dialog-body > form) {
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
		gap: 12px;
		overflow-y: scroll;
		padding: 12px;
	}
	:global(.register-dialog > .content-dialog-body) {
		display: flex;
		flex-direction: column;
		gap: 12px;
		overflow-y: hidden;
		max-height: 90%;
		padding: 0;
	}
	.dialog-title {
		display: block;
		padding: 12px 24px 0;
		position: sticky;
	}
	:global(.register-dialog > .content-dialog-footer) {
		box-sizing: border-box;
		min-height: 50px;
	}
	:global(.register-dialog) {
		max-height: 80vh;
		display: flex;
		flex-direction: column;
	}
	:global(.addTotpExpander) {
		padding: 0 10px;
		box-sizing: border-box;
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
