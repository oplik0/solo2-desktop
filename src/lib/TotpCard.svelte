<script lang="ts">
	import {
		IconButton,
		TextBlock,
		ProgressRing,
		MenuFlyout,
		MenuFlyoutItem,
	} from "fluent-svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { writeText } from "@tauri-apps/api/clipboard";
	export let uuid: string;
	export let credential: string;
	let code = "";
	let display_code = false;
	let timeLeft: number = 0;
	setInterval(() => {
		if (timeLeft > 0) timeLeft -= 50;
	}, 50);
	async function getCode() {
		timeLeft =
			30 * 1000 - (Date.now() - Math.floor(Date.now() / 1000 / 30) * 1000 * 30);
		code = await invoke("get_oath_code", {
			uuid: uuid,
			credential: credential,
		});
		if (display_code) setTimeout(getCode, timeLeft);
	}
	function flip_code() {
		display_code = !display_code;
		if (display_code) {
			getCode();
		} else {
			timeLeft = 0;
			code = "";
		}
	}
	async function deleteCredential() {
		await invoke("delete_oath", { uuid: uuid, credential: credential });
	}
</script>

<section>
	<div class="oath-credential-id-container">
		<TextBlock variant="subtitle" class="oath-credential-id">
			{credential}
		</TextBlock>
		<a href="/totp/{uuid}">
			<TextBlock variant="caption" class="oathKeyId">({uuid})</TextBlock>
		</a>
	</div>
	<button
		on:click={async () => await writeText(code)}
		class={!display_code ? "hidden" : ""}
	>
		<TextBlock variant="title" class="oath-code"
			>{code.substring(0, code.length / 2)}
			{code.substring(code.length / 2)}</TextBlock
		>
	</button>
	<ProgressRing
		value={Math.round(100 * (timeLeft / 30000))}
		class={!display_code ? "hidden" : ""}
	/>
	<IconButton on:click={flip_code}>
		<svg
			width="16"
			height="16"
			viewBox="0 0 16 16"
			fill="currentColor"
			xmlns="http://www.w3.org/2000/svg"
		>
			{#if display_code}
				<path
					d="M10.1196 10.8267L14.1464 14.8536C14.3417 15.0488 14.6583 15.0488 14.8536 14.8536C15.0488 14.6583 15.0488 14.3417 14.8536 14.1464L1.85355 1.14645C1.65829 0.951184 1.34171 0.951184 1.14645 1.14645C0.951184 1.34171 0.951184 1.65829 1.14645 1.85355L4.37624 5.08334C3.90117 5.4183 3.5126 5.80026 3.19877 6.18295C2.75443 6.72477 2.46154 7.26493 2.27931 7.66977C2.18795 7.87274 2.12369 8.04329 2.08166 8.1653C2.06063 8.22636 2.03453 8.31047 2.03453 8.31047L2.01687 8.37186C2.01687 8.37186 1.94098 8.86907 2.37202 8.9833C2.63879 9.05404 2.91251 8.8948 2.98346 8.62815L2.98444 8.62471L2.99179 8.5997C2.9989 8.57616 3.01051 8.53927 3.02715 8.49095C3.06047 8.39421 3.11375 8.25227 3.19119 8.08023C3.34655 7.73507 3.59627 7.27523 3.97201 6.81706C4.26363 6.46146 4.63213 6.10494 5.09595 5.80306L6.67356 7.38067C5.9688 7.82277 5.50024 8.60667 5.50024 9.5C5.50024 10.8807 6.61953 12 8.00024 12C8.89358 12 9.67747 11.5314 10.1196 10.8267ZM9.3807 10.0878C9.15205 10.6241 8.62005 11 8.00024 11C7.17182 11 6.50024 10.3284 6.50024 9.5C6.50024 8.88019 6.87616 8.34819 7.41244 8.11955L9.3807 10.0878ZM6.31962 4.19853L7.174 5.05291C7.43366 5.01852 7.70875 5 8.00017 5C10.0445 5 11.2857 5.9115 12.0283 6.81706C12.4041 7.27523 12.6538 7.73507 12.8091 8.08023C12.8866 8.25227 12.9399 8.39421 12.9732 8.49095C12.9898 8.53927 13.0014 8.57616 13.0085 8.5997L13.0159 8.62471L13.0169 8.62815L13.0172 8.62937C13.0885 8.89555 13.3618 9.05397 13.6283 8.9833C13.8952 8.91253 14.0542 8.63878 13.9835 8.37186L13.9832 8.37069L13.9827 8.36916L13.9816 8.365L13.9781 8.35236C13.9752 8.34204 13.9711 8.328 13.9658 8.31047C13.9552 8.27541 13.9397 8.22636 13.9187 8.1653C13.8766 8.04329 13.8124 7.87274 13.721 7.66977C13.5388 7.26493 13.2459 6.72477 12.8016 6.18295C11.904 5.0885 10.3952 4 8.00017 4C7.38264 4 6.82403 4.07236 6.31962 4.19853Z"
				/>
			{:else}
				<path
					d="M2.98444 8.62471L2.98346 8.62815C2.91251 8.8948 2.63879 9.05404 2.37202 8.9833C1.94098 8.86907 2.01687 8.37186 2.01687 8.37186L2.03453 8.31047C2.03453 8.31047 2.06063 8.22636 2.08166 8.1653C2.12369 8.04329 2.18795 7.87274 2.27931 7.66977C2.46154 7.26493 2.75443 6.72477 3.19877 6.18295C4.09629 5.08851 5.60509 4 8.00017 4C10.3952 4 11.904 5.08851 12.8016 6.18295C13.2459 6.72477 13.5388 7.26493 13.721 7.66977C13.8124 7.87274 13.8766 8.04329 13.9187 8.1653C13.9397 8.22636 13.9552 8.27541 13.9658 8.31047C13.9711 8.328 13.9752 8.34204 13.9781 8.35236L13.9816 8.365L13.9827 8.36916L13.9832 8.37069L13.9835 8.37186C14.0542 8.63878 13.8952 8.91253 13.6283 8.9833C13.3618 9.05397 13.0885 8.89556 13.0172 8.62937L13.0169 8.62815L13.0159 8.62471L13.0085 8.5997C13.0014 8.57616 12.9898 8.53927 12.9732 8.49095C12.9399 8.39422 12.8866 8.25227 12.8091 8.08023C12.6538 7.73508 12.4041 7.27523 12.0283 6.81706C11.2857 5.9115 10.0445 5 8.00017 5C5.95584 5 4.71464 5.9115 3.97201 6.81706C3.59627 7.27523 3.34655 7.73508 3.19119 8.08023C3.11375 8.25227 3.06047 8.39422 3.02715 8.49095C3.01051 8.53927 2.9989 8.57616 2.99179 8.5997L2.98444 8.62471ZM8.00024 7C6.61953 7 5.50024 8.11929 5.50024 9.5C5.50024 10.8807 6.61953 12 8.00024 12C9.38096 12 10.5002 10.8807 10.5002 9.5C10.5002 8.11929 9.38096 7 8.00024 7ZM6.50024 9.5C6.50024 8.67157 7.17182 8 8.00024 8C8.82867 8 9.50024 8.67157 9.50024 9.5C9.50024 10.3284 8.82867 11 8.00024 11C7.17182 11 6.50024 10.3284 6.50024 9.5Z"
				/>
			{/if}
		</svg>
	</IconButton>
	<MenuFlyout placement="bottom">
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
			<MenuFlyoutItem on:click={deleteCredential}>
				<svg
					slot="icon"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M12 1.75C13.733 1.75 15.1492 3.10645 15.2449 4.81558L15.25 5H20.5C20.9142 5 21.25 5.33579 21.25 5.75C21.25 6.1297 20.9678 6.44349 20.6018 6.49315L20.5 6.5H19.704L18.4239 19.5192C18.2912 20.8683 17.1984 21.91 15.8626 21.9945L15.6871 22H8.31293C6.95734 22 5.81365 21.0145 5.59883 19.6934L5.57614 19.5192L4.295 6.5H3.5C3.1203 6.5 2.80651 6.21785 2.75685 5.85177L2.75 5.75C2.75 5.3703 3.03215 5.05651 3.39823 5.00685L3.5 5H8.75C8.75 3.20507 10.2051 1.75 12 1.75ZM18.197 6.5H5.802L7.06893 19.3724C7.12768 19.9696 7.60033 20.4343 8.18585 20.4936L8.31293 20.5H15.6871C16.2872 20.5 16.7959 20.0751 16.9123 19.4982L16.9311 19.3724L18.197 6.5ZM13.75 9.25C14.1297 9.25 14.4435 9.53215 14.4932 9.89823L14.5 10V17C14.5 17.4142 14.1642 17.75 13.75 17.75C13.3703 17.75 13.0565 17.4678 13.0068 17.1018L13 17V10C13 9.58579 13.3358 9.25 13.75 9.25ZM10.25 9.25C10.6297 9.25 10.9435 9.53215 10.9932 9.89823L11 10V17C11 17.4142 10.6642 17.75 10.25 17.75C9.8703 17.75 9.55651 17.4678 9.50685 17.1018L9.5 17V10C9.5 9.58579 9.83579 9.25 10.25 9.25ZM12 3.25C11.0818 3.25 10.3288 3.95711 10.2558 4.85647L10.25 5H13.75C13.75 4.0335 12.9665 3.25 12 3.25Z"
						fill="currentColor"
					/>
				</svg>
				Delete
			</MenuFlyoutItem>
		</svelte:fragment>
	</MenuFlyout>
</section>

<style>
	a {
		color: var(--fds-card-text-default);
	}
	button {
		background: none;
		border: none;
		padding: 0;
		margin: 0;
		color: var(--fds-card-text-default);
		cursor: pointer;
		opacity: 1;
		width: min-content;
		max-width: max-content;
	}
	button:active {
		opacity: 0.8;
	}
	:global(.oath-credential-id) {
		justify-self: flex-start;
		flex-grow: 8;
		text-align: left;
		overflow: hidden;
		text-overflow: ellipsis;
	}
	:global(.oath-code) {
		width: max-content;
		text-align: right;
	}
	section {
		display: grid;
		grid-template-columns: 1fr minmax(100px, max-content) 32px 32px 32px;
		flex-direction: row;
		justify-content: left;
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
		gap: 6px;
		line-height: 36px;
	}
	.hidden {
		visibility: hidden;
	}
	:global(.hidden) {
		visibility: hidden;
	}
	.oath-credential-id-container {
		justify-self: flex-start;
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		justify-content: flex-start;
		max-width: 100%;
		overflow: hidden;
		text-overflow: ellipsis;
		margin: -24px 0;
	}
	:global(.oathKeyId) {
		max-width: 100%;
		overflow: hidden;
		text-overflow: "...)";
	}
</style>
