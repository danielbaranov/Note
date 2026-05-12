<script lang="ts">
    import Icon from "$lib/components/ui/Icon.svelte";
    import IconButton from "$lib/components/ui/IconButton.svelte";

    type Props = {
        title?: string;
        connectionsOpen?: boolean;
        onOpenCommand?: () => void;
        onOpenGraph?: () => void;
        onToggleConnections?: () => void;
    };

    let {
        title = "Note",
        connectionsOpen = true,
        onOpenCommand,
        onOpenGraph,
        onToggleConnections,
    }: Props = $props();
</script>

<header class="app-header" data-tauri-drag-region>
    <div
        class="traffic-light-space"
        aria-hidden="true"
        data-tauri-drag-region
    ></div>

    <div class="titlebar-title" data-tauri-drag-region>
        <span>{title}</span>
    </div>

    <button
        type="button"
        class="search-trigger"
        onclick={() => onOpenCommand?.()}
    >
        <span class="search-icon"><Icon name="search" size={12} /></span>
        <span>Search or jump…</span>
        <span class="kbd">⌘K</span>
    </button>

    <div
        class="titlebar-actions"
        aria-label="Window actions"
        data-tauri-drag-region
    >
        <IconButton title="Graph view" onclick={() => onOpenGraph?.()}>
            <Icon name="graph" />
        </IconButton>
        {#if onToggleConnections}
            <IconButton
                title={connectionsOpen ? "Hide connections" : "Show connections"}
                pressed={connectionsOpen}
                onclick={() => onToggleConnections()}
            >
                <Icon name="link" />
            </IconButton>
        {/if}
    </div>
</header>

<style>
    .app-header {
        height: var(--header-height);
        display: grid;
        grid-template-columns:
            var(--traffic-light-space) minmax(120px, auto) minmax(180px, 1fr)
            auto;
        align-items: start;
        gap: 12px;
        padding: 0 14px 0 0;
        background: var(--app-bg);
        border-bottom: 1px solid var(--border-color);
        user-select: none;
    }

    .traffic-light-space {
        width: var(--traffic-light-space);
        height: 100%;
    }

    .titlebar-title {
        min-width: 0;
        height: var(--titlebar-control-height);
        margin-top: var(--titlebar-control-top);
        display: flex;
        align-items: center;
        color: var(--fg-2);
        font-size: 13px;
        font-weight: 500;
    }

    .titlebar-title span {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .search-trigger {
        display: inline-flex;
        align-items: center;
        justify-self: stretch;
        min-width: 0;
        max-width: 520px;
        height: var(--titlebar-control-height);
        margin-top: var(--titlebar-control-top);
        gap: 8px;
        padding: 0 12px;
        border-radius: var(--r-sm);
        background: var(--bg-tint);
        color: var(--fg-3);
        border: 0;
        cursor: pointer;
        font: inherit;
        font-size: 12px;
        transition: background 0.12s;
    }

    .search-trigger:hover {
        background: var(--bg-tint-2);
    }

    .search-icon {
        display: inline-flex;
        color: var(--fg-3);
    }

    .kbd {
        font-family: var(--font-mono);
        font-size: 10.5px;
        padding: 1px 5px;
        border: 1px solid var(--line-2);
        border-radius: 4px;
        color: var(--fg-3);
        margin-left: auto;
        flex: 0 0 auto;
    }

    .titlebar-actions {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        height: var(--titlebar-control-height);
        margin-top: var(--titlebar-control-top);
        gap: 4px;
    }
</style>
