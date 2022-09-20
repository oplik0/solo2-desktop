<script lang="ts">
    import { IconButton , TextBlock, ProgressRing, MenuFlyout} from "fluent-svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { writeText } from "@tauri-apps/api/clipboard";
    export let uuid: string;
    export let credential: string;
    let code = "";
    let display_code = false;
    let timeLeft: number;
    setInterval(() => {
        if (timeLeft>0) timeLeft-=50;
    }, 50);
    async function getCode() {
        timeLeft = 30 * 1000 - (Date.now() - Math.floor(Date.now() / 1000 / 30) * 1000 * 30);
        code = await invoke("get_oath_code", {uuid: uuid, credential: credential});
        if (display_code) setTimeout(getCode, timeLeft)
    }
    function flip_code() {
        display_code = !display_code; 
        if (display_code) {
            getCode();
        } else {
            code = "";
        }
    }
</script>

<section>
        <div class="oath-credential-id-container">
            <TextBlock variant="title" class="oath-credential-id">{credential}</TextBlock>
            <TextBlock variant="caption">({uuid})</TextBlock>
        </div>
        <button on:click={async () => await writeText(code)}>
            <TextBlock variant="title" class="oath-code {!display_code ? "hidden" : ""}">{code.substring(0, code.length/2)} {code.substring(code.length/2)}</TextBlock>
        </button>
        <ProgressRing value={Math.round(100*((timeLeft)/30000))} class="{!display_code ? "hidden" : ""}"></ProgressRing>
    <IconButton on:click={flip_code}>
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            {#if display_code}
                <path d="M10.1196 10.8267L14.1464 14.8536C14.3417 15.0488 14.6583 15.0488 14.8536 14.8536C15.0488 14.6583 15.0488 14.3417 14.8536 14.1464L1.85355 1.14645C1.65829 0.951184 1.34171 0.951184 1.14645 1.14645C0.951184 1.34171 0.951184 1.65829 1.14645 1.85355L4.37624 5.08334C3.90117 5.4183 3.5126 5.80026 3.19877 6.18295C2.75443 6.72477 2.46154 7.26493 2.27931 7.66977C2.18795 7.87274 2.12369 8.04329 2.08166 8.1653C2.06063 8.22636 2.03453 8.31047 2.03453 8.31047L2.01687 8.37186C2.01687 8.37186 1.94098 8.86907 2.37202 8.9833C2.63879 9.05404 2.91251 8.8948 2.98346 8.62815L2.98444 8.62471L2.99179 8.5997C2.9989 8.57616 3.01051 8.53927 3.02715 8.49095C3.06047 8.39421 3.11375 8.25227 3.19119 8.08023C3.34655 7.73507 3.59627 7.27523 3.97201 6.81706C4.26363 6.46146 4.63213 6.10494 5.09595 5.80306L6.67356 7.38067C5.9688 7.82277 5.50024 8.60667 5.50024 9.5C5.50024 10.8807 6.61953 12 8.00024 12C8.89358 12 9.67747 11.5314 10.1196 10.8267ZM9.3807 10.0878C9.15205 10.6241 8.62005 11 8.00024 11C7.17182 11 6.50024 10.3284 6.50024 9.5C6.50024 8.88019 6.87616 8.34819 7.41244 8.11955L9.3807 10.0878ZM6.31962 4.19853L7.174 5.05291C7.43366 5.01852 7.70875 5 8.00017 5C10.0445 5 11.2857 5.9115 12.0283 6.81706C12.4041 7.27523 12.6538 7.73507 12.8091 8.08023C12.8866 8.25227 12.9399 8.39421 12.9732 8.49095C12.9898 8.53927 13.0014 8.57616 13.0085 8.5997L13.0159 8.62471L13.0169 8.62815L13.0172 8.62937C13.0885 8.89555 13.3618 9.05397 13.6283 8.9833C13.8952 8.91253 14.0542 8.63878 13.9835 8.37186L13.9832 8.37069L13.9827 8.36916L13.9816 8.365L13.9781 8.35236C13.9752 8.34204 13.9711 8.328 13.9658 8.31047C13.9552 8.27541 13.9397 8.22636 13.9187 8.1653C13.8766 8.04329 13.8124 7.87274 13.721 7.66977C13.5388 7.26493 13.2459 6.72477 12.8016 6.18295C11.904 5.0885 10.3952 4 8.00017 4C7.38264 4 6.82403 4.07236 6.31962 4.19853Z" />
            {:else}
                <path d="M2.98444 8.62471L2.98346 8.62815C2.91251 8.8948 2.63879 9.05404 2.37202 8.9833C1.94098 8.86907 2.01687 8.37186 2.01687 8.37186L2.03453 8.31047C2.03453 8.31047 2.06063 8.22636 2.08166 8.1653C2.12369 8.04329 2.18795 7.87274 2.27931 7.66977C2.46154 7.26493 2.75443 6.72477 3.19877 6.18295C4.09629 5.08851 5.60509 4 8.00017 4C10.3952 4 11.904 5.08851 12.8016 6.18295C13.2459 6.72477 13.5388 7.26493 13.721 7.66977C13.8124 7.87274 13.8766 8.04329 13.9187 8.1653C13.9397 8.22636 13.9552 8.27541 13.9658 8.31047C13.9711 8.328 13.9752 8.34204 13.9781 8.35236L13.9816 8.365L13.9827 8.36916L13.9832 8.37069L13.9835 8.37186C14.0542 8.63878 13.8952 8.91253 13.6283 8.9833C13.3618 9.05397 13.0885 8.89556 13.0172 8.62937L13.0169 8.62815L13.0159 8.62471L13.0085 8.5997C13.0014 8.57616 12.9898 8.53927 12.9732 8.49095C12.9399 8.39422 12.8866 8.25227 12.8091 8.08023C12.6538 7.73508 12.4041 7.27523 12.0283 6.81706C11.2857 5.9115 10.0445 5 8.00017 5C5.95584 5 4.71464 5.9115 3.97201 6.81706C3.59627 7.27523 3.34655 7.73508 3.19119 8.08023C3.11375 8.25227 3.06047 8.39422 3.02715 8.49095C3.01051 8.53927 2.9989 8.57616 2.99179 8.5997L2.98444 8.62471ZM8.00024 7C6.61953 7 5.50024 8.11929 5.50024 9.5C5.50024 10.8807 6.61953 12 8.00024 12C9.38096 12 10.5002 10.8807 10.5002 9.5C10.5002 8.11929 9.38096 7 8.00024 7ZM6.50024 9.5C6.50024 8.67157 7.17182 8 8.00024 8C8.82867 8 9.50024 8.67157 9.50024 9.5C9.50024 10.3284 8.82867 11 8.00024 11C7.17182 11 6.50024 10.3284 6.50024 9.5Z" />
            {/if}
        </svg>
    </IconButton>
    
    
</section>

<style>
    button {
        background: none;
        border: none;
        padding: 0;
        margin: 0;
        color: var(--fds-card-text-default);
        cursor: pointer;
        opacity: 1;
    }
    button:active {
        opacity: 0.8;
    }
    :global(.oath-credential-id) {
        justify-self: flex-start;
        flex-grow: 8;
    }
    :global(.oath-code) {
        width: 100%;
        text-align: right;
    }
    section {
        display: grid;
        grid-template-columns: 1fr minmax(100px, max-content) min-content min-content min-content;
        flex-direction: row;
        justify-content: center;
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
    :global(.hidden) {
        visibility: hidden;
    }
    .oath-credential-id-container {
        justify-self: flex-start;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;
        gap: 12px;
    }
</style>