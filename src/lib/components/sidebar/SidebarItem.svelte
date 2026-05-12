<script lang="ts">
    import Icon from "$lib/components/ui/Icon.svelte";
    import IconButton from "$lib/components/ui/IconButton.svelte";

    type ObjectKind = "note" | "pdf" | "web";

    type Props = {
        name: string;
        meta?: string;
        kind?: ObjectKind;
        active?: boolean;
        onSelect?: () => void;
        onDelete?: () => void | Promise<void>;
    };

    let {
        name,
        meta,
        kind = "note",
        active = false,
        onSelect,
        onDelete,
    }: Props = $props();

    const dotClass = $derived(`mp-dot mp-dot--${kind}`);
</script>

<div class="row" class:active>
    <button
        type="button"
        class="select"
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

    {#if onDelete}
        <span class="delete-action">
            <IconButton
                title="Delete note"
                size={22}
                onclick={(event) => {
                    event.stopPropagation();
                    void onDelete();
                }}
            >
                <Icon name="close" size={13} />
            </IconButton>
        </span>
    {/if}
</div>

<style>
    .row {
        position: relative;
    }

    .select {
        display: flex;
        align-items: center;
        gap: 9px;
        width: 100%;
        padding: 6px 30px 6px 10px;
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

    .row:hover .select {
        background: var(--bg-tint);
    }

    .row.active .select {
        color: var(--fg-1);
        background: var(--bg-tint);
        font-weight: 500;
    }

    .delete-action {
        position: absolute;
        top: 50%;
        right: 5px;
        opacity: 0;
        visibility: hidden;
        transform: translateY(-50%);
        transition:
            opacity 0.12s,
            visibility 0.12s;
    }

    .row:hover .delete-action,
    .delete-action:focus-within {
        opacity: 1;
        visibility: visible;
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
