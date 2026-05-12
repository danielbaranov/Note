<script lang="ts">
    import {
        connectionDirectionOf,
        otherObjectIdOf,
        type ObjectConnectionRecord,
    } from "$lib/connections";
    import {
        displayTitleForObject,
        type ObjectRecord,
    } from "$lib/objects";
    import ConfirmDeleteDialog from "$lib/components/ui/ConfirmDeleteDialog.svelte";
    import Icon from "$lib/components/ui/Icon.svelte";
    import IconButton from "$lib/components/ui/IconButton.svelte";

    type Props = {
        connection: ObjectConnectionRecord;
        currentObjectId: string;
        otherObject: ObjectRecord | null;
        selected?: boolean;
        onOpenObject: (id: string) => void | Promise<void>;
        onEditConnection: (connection: ObjectConnectionRecord) => void;
        onDeleteConnection?: (id: string) => void | Promise<void>;
    };

    let {
        connection,
        currentObjectId,
        otherObject,
        selected = false,
        onOpenObject,
        onEditConnection,
        onDeleteConnection,
    }: Props = $props();

    let deleteDialogOpen = $state(false);
    let deleting = $state(false);

    const otherObjectId = $derived(otherObjectIdOf(connection, currentObjectId));
    const direction = $derived(connectionDirectionOf(connection, currentObjectId));
    const title = $derived(
        otherObject ? displayTitleForObject(otherObject) : "Unknown object",
    );
    const preview = $derived(connection.contentText.trim());

    async function confirmDelete() {
        if (!onDeleteConnection || deleting) return;

        deleting = true;
        try {
            await onDeleteConnection(connection.id);
            deleteDialogOpen = false;
        } finally {
            deleting = false;
        }
    }
</script>

<div class="connection-row" class:selected>
    <span class="row-actions">
        <IconButton
            title="Open connected object"
            size={22}
            onclick={(event) => {
                event.stopPropagation();
                onOpenObject(otherObjectId);
            }}
        >
            <Icon name="arrow" size={13} />
        </IconButton>

        {#if onDeleteConnection}
            <IconButton
                title="Delete connection"
                size={22}
                onclick={(event) => {
                    event.stopPropagation();
                    deleteDialogOpen = true;
                }}
            >
                <Icon name="close" size={13} />
            </IconButton>
        {/if}
    </span>

    <button
        type="button"
        class="edit-connection"
        aria-pressed={selected}
        title={`Edit connection with ${title}`}
        onclick={() => onEditConnection(connection)}
    >
        <span class="mp-dot mp-dot--note"></span>
        <span class="connection-text">
            <span class="title">{title}</span>
            <span class="meta">{direction === "outgoing" ? "Outgoing" : "Incoming"}</span>
            <span class="preview" class:empty-preview={!preview}>
                {preview || "No connection text"}
            </span>
        </span>
    </button>

    {#if onDeleteConnection}
        <ConfirmDeleteDialog
            bind:open={deleteDialogOpen}
            title="Delete connection?"
            description="This permanently removes this connection. The connected objects are not affected."
            busy={deleting}
            onConfirm={confirmDelete}
        />
    {/if}
</div>

<style>
    .connection-row {
        position: relative;
        display: flex;
        flex-direction: column;
        gap: 6px;
        padding: 6px 60px 7px 6px;
        border-bottom: 1px solid var(--line-1);
    }

    .connection-row.selected {
        background: var(--accent-soft);
    }

    .connection-row:last-child {
        border-bottom: 0;
    }

    .row-actions {
        position: absolute;
        top: 6px;
        right: 6px;
        display: flex;
        gap: 3px;
        opacity: 0;
        visibility: hidden;
        transition:
            opacity 0.12s,
            visibility 0.12s;
    }

    .connection-row:hover .row-actions,
    .row-actions:focus-within {
        opacity: 1;
        visibility: visible;
    }

    .edit-connection {
        display: flex;
        align-items: flex-start;
        gap: 8px;
        width: 100%;
        border: 0;
        border-radius: 5px;
        padding: 5px 6px;
        background: transparent;
        color: var(--fg-2);
        cursor: pointer;
        font: inherit;
        text-align: left;
    }

    .edit-connection:hover {
        background: var(--bg-tint);
    }

    .edit-connection:hover .title {
        color: var(--fg-1);
    }

    .edit-connection :global(.mp-dot) {
        margin-top: 6px;
    }

    .connection-text {
        min-width: 0;
        flex: 1;
    }

    .title {
        display: block;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        font-size: 12.5px;
        font-weight: 500;
        color: var(--fg-1);
    }

    .meta {
        display: block;
        margin-top: 1px;
        font-size: 10.5px;
        color: var(--fg-4);
    }

    .preview {
        display: block;
        margin-top: 5px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        color: var(--fg-2);
        font-size: 12px;
        line-height: 1.35;
    }

    .empty-preview {
        color: var(--fg-3);
    }
</style>
