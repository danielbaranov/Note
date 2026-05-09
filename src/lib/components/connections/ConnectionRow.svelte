<script lang="ts">
    import InlineMarkdown from "./InlineMarkdown.svelte";
    import type { Connection } from "$lib/connections";

    type Props = {
        connection: Connection;
        isLast?: boolean;
        onSelect?: () => void;
    };

    let { connection, isLast = false, onSelect }: Props = $props();

    const dotClass = $derived(`mp-dot mp-dot--${connection.kind}`);
</script>

<button
    type="button"
    class="row"
    class:no-divider={isLast}
    onclick={() => onSelect?.()}
>
    <div class="header">
        <span class={dotClass}></span>
        <div class="title-block">
            <div class="title">{connection.title}</div>
            {#if connection.meta}
                <div class="meta">{connection.meta}</div>
            {/if}
        </div>
    </div>

    <div class="label">
        <InlineMarkdown source={connection.label} />
    </div>
</button>

<style>
    .row {
        display: flex;
        flex-direction: column;
        gap: 9px;
        width: 100%;
        padding: 14px 16px 16px;
        background: transparent;
        border: 0;
        border-bottom: 1px solid var(--line-1);
        cursor: pointer;
        text-align: left;
        font: inherit;
        position: relative;
        transition: background 0.12s;
    }

    .row.no-divider {
        border-bottom: 0;
    }

    .row:hover {
        background: var(--bg-tint);
    }

    .header {
        display: flex;
        align-items: flex-start;
        gap: 8px;
    }

    .header :global(.mp-dot) {
        margin-top: 6px;
    }

    .title-block {
        flex: 1;
        min-width: 0;
    }

    .title {
        font-size: 13px;
        color: var(--fg-1);
        font-weight: 500;
        line-height: 1.35;
    }

    .meta {
        font-size: 11px;
        color: var(--fg-4);
        margin-top: 2px;
    }

    /* The label evokes a margin annotation: indented under the title with a
       thin accent rule on the left. */
    .label {
        margin-left: 14px;
        padding-left: 11px;
        border-left: 2px solid var(--accent-line);
        font-size: 12.5px;
        line-height: 1.55;
        color: var(--fg-2);
    }
</style>
