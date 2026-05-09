<script lang="ts">
    type ObjectKind = "note" | "pdf" | "web";

    type Props = {
        name: string;
        meta?: string;
        kind?: ObjectKind;
        active?: boolean;
        onSelect?: () => void;
    };

    let {
        name,
        meta,
        kind = "note",
        active = false,
        onSelect,
    }: Props = $props();

    const dotClass = $derived(`mp-dot mp-dot--${kind}`);
</script>

<button
    type="button"
    class="row"
    class:active
    onclick={() => onSelect?.()}
    title={name}
>
    <span class={dotClass}></span>
    <span class="text">
        <span class="name">{name}</span>
        {#if meta}<span class="meta">{meta}</span>{/if}
    </span>
    {#if active}
        <span class="active-dot" aria-hidden="true"></span>
    {/if}
</button>

<style>
    .row {
        display: flex;
        align-items: center;
        gap: 9px;
        padding: 6px 10px;
        font-size: 13px;
        color: var(--fg-2);
        background: transparent;
        border: 0;
        border-radius: 6px;
        cursor: pointer;
        user-select: none;
        font-weight: 400;
        text-align: left;
        font: inherit;
        font-size: 13px;
    }

    .row:hover {
        background: var(--bg-tint);
    }

    .row.active {
        color: var(--fg-1);
        background: var(--bg-tint);
        font-weight: 500;
    }

    .text {
        flex: 1;
        min-width: 0;
        display: block;
    }

    .name {
        display: block;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .meta {
        display: block;
        font-size: 10.5px;
        color: var(--fg-4);
        margin-top: 1px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .active-dot {
        width: 4px;
        height: 4px;
        background: var(--accent);
        border-radius: 50%;
        flex: 0 0 4px;
    }
</style>
