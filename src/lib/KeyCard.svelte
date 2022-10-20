<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { open } from "@tauri-apps/api/dialog";
	import {
		Button,
		Expander,
		TextBlock,
		ProgressRing,
		InfoBadge,
		ContentDialog,
		MenuFlyout,
		MenuFlyoutItem,
		IconButton,
		Tooltip,
		TextBox,
	} from "fluent-svelte";
	import { parse, gt, major } from "semver";
	import type { Solo2 } from "src/types";
	import { saveKeyName } from "$lib/keyName";

	export let key: Solo2;
	export let latest_version = "0.0.0";
	let dialogOpen = false;
	let updating = false;
	let expanding = false;
	let selected_file: string | undefined;
	let keyName = key.name ?? key.uuid;
	let editingKeyName = false;
	async function update() {
		updating = true;
		try {
			await invoke("update_key", {
				uuid: key.uuid,
				file: selected_file,
			});
		} finally {
			updating = false;
		}
		console.log("updated");
	}
	async function wink() {
		await invoke("wink", { uuid: key.uuid });
	}
	async function reboot() {
		await invoke("reboot", { uuid: key.uuid });
	}
	async function maintenance() {
		await invoke("maintenance", { uuid: key.uuid });
	}

	async function upload_firmware() {
		const selected = await open({
			multiple: false,
			title: "Select Firmware File",
			filters: [{ extensions: ["sb2"], name: "Firmware" }],
		});
		if (Array.isArray(selected)) selected_file = selected[0];
		else if (typeof selected === "string") selected_file = selected;
		else selected_file = undefined;
		console.log(selected_file);
	}

	function toCalver(version: string) {
		const semver = parse(version);
		if (semver !== null) {
			const date = new Date(new Date("2020-01-01").setDate(1 + semver.minor));
			const formatter = new Intl.DateTimeFormat("en-us", {
				year: "numeric",
				month: "2-digit",
				day: "2-digit",
			});
			const formatted_date = Object.fromEntries(
				formatter.formatToParts(date).map((x) => [x.type, x.value])
			);
			return `${semver.major}:${formatted_date.year}${formatted_date.month}${formatted_date.day}.${semver.patch}`;
		}

		return "Invalid version";
	}
	function expand() {
		expanding = true;
		setTimeout(
			() =>
				document
					.querySelector(`#expander-${key.uuid}`)
					?.classList.add("overflow-visible"),
			250
		);
	}
	function collapse() {
		if (expanding) {
			expanding = false;
			setTimeout(
				() =>
					document
						.querySelector(`#expander-${key.uuid}`)
						?.classList.remove("overflow-visible"),
				250
			);
		}
		document
			.querySelector(`#expander-${key.uuid}`)
			?.classList.remove("overflow-visible");
	}
</script>

<section>
	<Expander
		headingLevel={2}
		expanded
		class="key-expander overflow-visible"
		id="expander-{key.uuid}"
		on:expand={expand}
		on:collapse={collapse}
	>
		<svg
			slot="icon"
			width="128"
			height="128"
			viewBox="0 0 16 16"
			fill="currentColor"
			class="expander-icon"
			xmlns="http://www.w3.org/2000/svg"
		>
			{#if key.secure}
				<!-- MIT Copyright (c) 2020 Microsoft Corporation - static/licenses/fluentui-icons-license-->
				<path
					d="M7.83562 1.50532L8 1.5C9.32548 1.5 10.41 2.53154 10.4947 3.83562L10.5 4V5H11.5C12.3284 5 13 5.67157 13 6.5V12.5C13 13.3284 12.3284 14 11.5 14H4.5C3.67157 14 3 13.3284 3 12.5V6.5C3 5.67157 3.67157 5 4.5 5H5.5V4C5.5 2.67452 6.53154 1.58996 7.83562 1.50532ZM11.5 6H4.5C4.22386 6 4 6.22386 4 6.5V12.5C4 12.7761 4.22386 13 4.5 13H11.5C11.7761 13 12 12.7761 12 12.5V6.5C12 6.22386 11.7761 6 11.5 6ZM8 8.5C8.55228 8.5 9 8.94772 9 9.5C9 10.0523 8.55228 10.5 8 10.5C7.44772 10.5 7 10.0523 7 9.5C7 8.94772 7.44772 8.5 8 8.5ZM8.14446 2.50687L8 2.5C7.2203 2.5 6.57955 3.09489 6.50687 3.85554L6.5 4V5H9.5V4C9.5 3.2203 8.90511 2.57955 8.14446 2.50687Z"
				/>
			{:else}
				<!-- MIT Copyright (c) 2020 Microsoft Corporation - static/licenses/fluentui-icons-license-->
				<path
					d="M8 11.5C8.55228 11.5 9 11.0523 9 10.5C9 9.94771 8.55228 9.5 8 9.5C7.44772 9.5 7 9.94771 7 10.5C7 11.0523 7.44772 11.5 8 11.5ZM8 2.5C7.17157 2.5 6.5 3.17157 6.5 4V6H11.5C12.3284 6 13 6.67157 13 7.5V13.5C13 14.3284 12.3284 15 11.5 15H4.5C3.67157 15 3 14.3284 3 13.5V7.5C3 6.67157 3.67157 6 4.5 6H5.5V4C5.5 2.61929 6.61929 1.5 8 1.5C9.38071 1.5 10.5 2.61929 10.5 4C10.5 4.27614 10.2761 4.5 10 4.5C9.72386 4.5 9.5 4.27614 9.5 4C9.5 3.17157 8.82843 2.5 8 2.5ZM4.5 7C4.22386 7 4 7.22386 4 7.5V13.5C4 13.7761 4.22386 14 4.5 14H11.5C11.7761 14 12 13.7761 12 13.5V7.5C12 7.22386 11.7761 7 11.5 7H4.5Z"
				/>
			{/if}
		</svg>
		<div class="expander-title">
			<div class="keyNameBlock">
				<TextBlock variant="bodyLarge" class="keyId">
					{key.name ?? key.uuid}
				</TextBlock>
				<IconButton on:click={(e) => (editingKeyName = true)}>
					<svg
						width="24"
						height="24"
						fill="none"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							d="M13.94 5 19 10.06 9.062 20a2.25 2.25 0 0 1-.999.58l-5.116 1.395a.75.75 0 0 1-.92-.921l1.395-5.116a2.25 2.25 0 0 1 .58-.999L13.938 5Zm7.09-2.03a3.578 3.578 0 0 1 0 5.06l-.97.97L15 3.94l.97-.97a3.578 3.578 0 0 1 5.06 0Z"
							fill="currentColor"
						/>
					</svg>
				</IconButton>
			</div>

			<TextBlock variant="caption">v{toCalver(key.version)}</TextBlock>
			{#if gt(latest_version, key.version)}
				<InfoBadge severity="attention" class="new-version-badge"
					>New version available</InfoBadge
				>
			{/if}
		</div>
		<svelte:fragment slot="content">
			<div class="expanded-content">
				{#if key.name}
					<TextBlock variant="bodyStrong">uuid: {key.uuid}</TextBlock>
				{/if}
				<TextBlock variant="bodyStrong">semver: v{key.version}</TextBlock>
				<TextBlock variant="bodyStrong"
					>variant: {key.secure ? "secure" : "hacker"}</TextBlock
				>
				<div>
					<Button variant="hyperlink" href="/totp/{key.uuid}">TOTP</Button>

					<IconButton on:click={upload_firmware}>
						<Tooltip
							text={selected_file}
							placement="bottom"
							delay={selected_file?.length ? 200 : 2147483647}
							offset={6}
						>
							<svg
								width="16"
								height="16"
								fill="none"
								viewBox="0 0 24 24"
								xmlns="http://www.w3.org/2000/svg"
							>
								<path
									class:selected_file={selected_file?.length}
									d="M5.25 3.495h13.498a.75.75 0 0 0 .101-1.493l-.101-.007H5.25a.75.75 0 0 0-.102 1.493l.102.007Zm6.633 18.498L12 22a1 1 0 0 0 .993-.884L13 21V8.41l3.294 3.292a1 1 0 0 0 1.32.083l.094-.083a1 1 0 0 0 .083-1.32l-.083-.094-4.997-4.997a1 1 0 0 0-1.32-.083l-.094.083-5.004 4.996a1 1 0 0 0 1.32 1.499l.094-.083L11 8.415V21a1 1 0 0 0 .883.993Z"
									fill="currentColor"
								/>
							</svg>
						</Tooltip>
					</IconButton>
					<Button
						variant={gt(latest_version, key.version) || selected_file?.length
							? "accent"
							: "standard"}
						on:click={() => {
							dialogOpen = true;
						}}
						disabled={updating}
						style="width: 85px"
					>
						{#if updating}
							<ProgressRing size={20} />
						{:else}
							<TextBlock>Update</TextBlock>
						{/if}
					</Button>
					<MenuFlyout placement="bottom" class="keyFlyout">
						<IconButton>
							<svg
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								xmlns="http://www.w3.org/2000/svg"
							>
								<path
									d="M12 7.75C11.0335 7.75 10.25 6.9665 10.25 6C10.25 5.0335 11.0335 4.25 12 4.25C12.9665 4.25 13.75 5.0335 13.75 6C13.75 6.9665 12.9665 7.75 12 7.75ZM12 13.75C11.0335 13.75 10.25 12.9665 10.25 12C10.25 11.0335 11.0335 10.25 12 10.25C12.9665 10.25 13.75 11.0335 13.75 12C13.75 12.9665 12.9665 13.75 12 13.75ZM10.25 18C10.25 18.9665 11.0335 19.75 12 19.75C12.9665 19.75 13.75 18.9665 13.75 18C13.75 17.0335 12.9665 16.25 12 16.25C11.0335 16.25 10.25 17.0335 10.25 18Z"
									fill="currentColor"
								/>
							</svg>
						</IconButton>
						<svelte:fragment slot="flyout">
							<MenuFlyoutItem on:click={wink}>
								<svg
									width="16"
									height="16"
									fill="none"
									viewBox="0 0 24 24"
									xmlns="http://www.w3.org/2000/svg"
								>
									<path
										d="M11.996 19.01a.75.75 0 0 1 .743.649l.007.102v1.5a.75.75 0 0 1-1.493.101l-.007-.101v-1.5a.75.75 0 0 1 .75-.75Zm6.022-2.072 1.06 1.06a.75.75 0 1 1-1.06 1.061l-1.06-1.06a.75.75 0 0 1 1.06-1.061Zm-10.983 0a.75.75 0 0 1 0 1.06L5.974 19.06a.75.75 0 0 1-1.06-1.06l1.06-1.061a.75.75 0 0 1 1.06 0ZM12 6.475a5.525 5.525 0 1 1 0 11.05 5.525 5.525 0 0 1 0-11.05Zm0 1.5a4.025 4.025 0 1 0 0 8.05 4.025 4.025 0 0 0 0-8.05Zm9.25 3.293a.75.75 0 0 1 .102 1.493l-.102.007h-1.5a.75.75 0 0 1-.102-1.493l.102-.007h1.5Zm-17-.029a.75.75 0 0 1 .102 1.494l-.102.006h-1.5a.75.75 0 0 1-.102-1.493l.102-.007h1.5Zm1.64-6.37.084.072 1.06 1.06a.75.75 0 0 1-.976 1.134l-.084-.073-1.06-1.06a.75.75 0 0 1 .976-1.134Zm13.188.072a.75.75 0 0 1 .073.977l-.073.084-1.06 1.06a.75.75 0 0 1-1.133-.976l.072-.084 1.06-1.061a.75.75 0 0 1 1.061 0ZM12 1.99a.75.75 0 0 1 .743.648l.007.102v1.5a.75.75 0 0 1-1.493.101l-.007-.102v-1.5a.75.75 0 0 1 .75-.75Z"
										fill="currentColor"
									/>
								</svg>
								Wink
							</MenuFlyoutItem>
							<MenuFlyoutItem on:click={reboot}>
								<svg
									width="16"
									height="16"
									fill="none"
									viewBox="0 0 24 24"
									xmlns="http://www.w3.org/2000/svg"
								>
									<path
										d="M12 4.5a7.5 7.5 0 1 1-7.419 6.392c.067-.454-.265-.892-.724-.892a.749.749 0 0 0-.752.623A9 9 0 1 0 6 5.292V4.25a.75.75 0 0 0-1.5 0v3c0 .414.336.75.75.75h3a.75.75 0 0 0 0-1.5H6.9a7.473 7.473 0 0 1 5.1-2Z"
										fill="currentColor"
									/>
								</svg>
								Reboot
							</MenuFlyoutItem>
							<MenuFlyoutItem on:click={maintenance}>
								<svg
									width="24"
									height="24"
									fill="none"
									viewBox="0 0 24 24"
									xmlns="http://www.w3.org/2000/svg"
								>
									<path
										d="M10.5 7.751a5.75 5.75 0 0 1 8.38-5.114.75.75 0 0 1 .186 1.197L16.301 6.6l1.06 1.06 2.779-2.778a.75.75 0 0 1 1.193.179 5.75 5.75 0 0 1-6.422 8.284l-7.365 7.618a3.05 3.05 0 0 1-4.387-4.24l7.475-7.734a5.766 5.766 0 0 1-.134-1.238Zm5.75-4.25a4.25 4.25 0 0 0-4.067 5.489.75.75 0 0 1-.178.74l-7.768 8.035a1.55 1.55 0 1 0 2.23 2.156l7.676-7.941a.75.75 0 0 1 .775-.191 4.25 4.25 0 0 0 5.466-5.03l-2.492 2.492a.75.75 0 0 1-1.061 0L14.71 7.13a.75.75 0 0 1 0-1.06l2.466-2.467a4.268 4.268 0 0 0-.926-.102Z"
										fill="currentColor"
									/>
								</svg>
								Maintenance
							</MenuFlyoutItem>
						</svelte:fragment>
					</MenuFlyout>
				</div>
			</div>
		</svelte:fragment>
	</Expander>
	<ContentDialog
		bind:open={dialogOpen}
		title="Are you sure you want to update?"
	>
		<TextBlock variant="body"
			>This will attempt to update the key with UUID {key.uuid}.</TextBlock
		>
		<TextBlock variant="body"
			>If you want to continue press "Yes" and touch the key once the LED
			changes color. Unplugging the before touching any buttons will cancel the
			update</TextBlock
		>
		{#if major(latest_version) !== major(key.version)}
			<TextBlock variant="body"
				><InfoBadge severity="caution">Warning</InfoBadge> This is is major update
				and it could risk breaking any current credentials on your key.</TextBlock
			>
			<TextBlock variant="body"
				>Check latest release notes here to double check: <a
					href="https://github.com/solokeys/solo2/releases"
					target="_blank">https://github.com/solokeys/solo2/releases</a
				></TextBlock
			>
		{/if}

		<svelte:fragment slot="footer">
			<Button slot="footer" on:click={() => (dialogOpen = false)}>No</Button>
			<Button
				slot="footer"
				variant="accent"
				on:click={() => {
					dialogOpen = false;
					update();
				}}>Yes</Button
			>
		</svelte:fragment>
	</ContentDialog>
	<ContentDialog bind:open={editingKeyName} title="Edit your key name">
		<TextBlock variant="body">Enter a new name for your key</TextBlock>
		<TextBlock
			><InfoBadge severity="information" /> note: this is local to the application,
			the name will not be carried with the key</TextBlock
		>
		<TextBox bind:value={key.name} placeholder={key.name} />
		<svelte:fragment slot="footer">
			<Button slot="footer" on:click={() => (editingKeyName = false)}
				>Cancel</Button
			>
			<Button
				slot="footer"
				variant="accent"
				on:click={async () => {
					if (key.name) await saveKeyName(key.uuid, key.name);
					editingKeyName = false;
				}}>Save</Button
			>
		</svelte:fragment>
	</ContentDialog>
</section>

<style>
	section {
		width: 100%;
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
		vertical-align: sub;
	}
	:global(.keyId) {
		max-width: 80%;
		overflow: hidden;
		text-overflow: ellipsis;
		vertical-align: sub;
	}
	:global(.overflow-visible) {
		overflow: visible !important;
	}
	:global(.overflow-visible > .expander-content-anchor) {
		overflow: visible !important;
	}
	.selected_file {
		fill: var(--fds-accent-default);
	}
</style>
