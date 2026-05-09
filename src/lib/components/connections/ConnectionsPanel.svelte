<script lang="ts">
    import Icon from "$lib/components/ui/Icon.svelte";
    import IconButton from "$lib/components/ui/IconButton.svelte";
    import ConnectionRow from "./ConnectionRow.svelte";
    import type { Connection } from "$lib/connections";

    type Props = {
        connections: Connection[];
        onAddConnection?: () => void;
        onSelectConnection?: (id: string) => void;
    };

    let {
        connections,
        onAddConnection,
        onSelectConnection,
    }: Props = $props();
</script>

<div class="panel">
    <header class="panel-head">
        <div class="title-group">
            <div class="title">Connections</div>
            <div class="subtitle">{connections.length} labelled edges</div>
        </div>
        <IconButton title="Add connection" onclick={onAddConnection}>
            <Icon name="plus" />
        </IconButton>
    </header>

    <div class="panel-body">
        {#if connections.length === 0}
            <div class="empty">
                <p>No connections yet.</p>
                <p class="hint">
                    Connections are labelled links between this note and other
                    objects.
                </p>
            </div>
        {:else}
            {#each connections as connection, i (connection.id)}
                <ConnectionRow
                    {connection}
                    isLast={i === connections.length - 1}
                    onSelect={() => onSelectConnection?.(connection.id)}
                />
            {/each}
        {/if}
    </div>

    <footer class="panel-foot">
        <span class="foot-icon"><Icon name="link" size={12} /></span>
        <span>Drag from edge to connect</span>
    </footer>
</div>

<style>
    .panel {
        display: flex;
        flex-direction: column;
        flex: 1;
        min-height: 0;
    }

    .panel-head {
        padding: 16px 18px 12px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        border-bottom: 1px solid var(--line-1);
    }

    .title {
        font-size: 13px;
        font-weight: 600;
        color: var(--fg-1);
    }

    .subtitle {
        font-size: 11px;
        color: var(--fg-3);
        margin-top: 2px;
    }

    .panel-body {
        flex: 1;
        min-height: 0;
        overflow-y: auto;
    }

    .empty {
        padding: 24px 18px;
        color: var(--fg-3);
    }

    .empty p {
        margin: 0;
        font-size: 12.5px;
    }

    .empty .hint {
        margin-top: 6px;
        color: var(--fg-4);
        line-height: 1.5;
    }

    .panel-foot {
        padding: 10px 14px;
        border-top: 1px solid var(--line-1);
        font-size: 11.5px;
        color: var(--fg-3);
        display: flex;
        align-items: center;
        gap: 6px;
        justify-content: center;
    }

    .foot-icon {
        display: inline-flex;
        width: 12px;
        height: 12px;
    }
</style>
