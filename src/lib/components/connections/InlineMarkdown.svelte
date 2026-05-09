<script lang="ts">
    import { tokenizeInline } from "$lib/markdown/inline";

    type Props = {
        source: string;
    };

    let { source }: Props = $props();

    const tokens = $derived(tokenizeInline(source));
</script>

{#each tokens as token, i (i)}
    {#if token.type === "bold"}
        <strong>{token.value}</strong>
    {:else if token.type === "italic"}
        <em>{token.value}</em>
    {:else if token.type === "code"}
        <code>{token.value}</code>
    {:else}
        {token.value}
    {/if}
{/each}

<style>
    strong {
        font-weight: 600;
        color: var(--fg-1);
    }

    code {
        font-family: var(--font-mono);
        font-size: 0.88em;
        background: var(--bg-tint);
        padding: 1px 4px;
        border-radius: 3px;
    }
</style>
