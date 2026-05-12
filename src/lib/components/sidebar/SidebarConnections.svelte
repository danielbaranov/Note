<script lang="ts">
    import { Command } from "bits-ui";

    import {
        otherObjectIdOf,
        type ObjectConnectionRecord,
    } from "$lib/connections";
    import {
        displayTitleForObject,
        type ObjectRecord,
    } from "$lib/objects";
    import Icon from "$lib/components/ui/Icon.svelte";
    import IconButton from "$lib/components/ui/IconButton.svelte";
    import SidebarConnectionRow from "./SidebarConnectionRow.svelte";

    type Props = {
        currentObjectId: string | null;
        objects: ObjectRecord[];
        connections: ObjectConnectionRecord[];
        selectedConnectionId?: string | null;
        onCreateConnection: (targetObjectId: string) => void | Promise<void>;
        onOpenObject: (id: string) => void | Promise<void>;
        onEditConnection: (connection: ObjectConnectionRecord) => void;
        onDeleteConnection: (id: string) => void | Promise<void>;
    };

    let {
        currentObjectId,
        objects,
        connections,
        selectedConnectionId = null,
        onCreateConnection,
        onOpenObject,
        onEditConnection,
        onDeleteConnection,
    }: Props = $props();

    let creating = $state(false);
    let creatingConnection = $state(false);
    let targetObjectId = $state("");
    let targetQuery = $state("");

    const objectsById = $derived(
        new Map(objects.map((object) => [object.id, object])),
    );
    const targetObjects = $derived(
        currentObjectId
            ? objects.filter((object) => object.id !== currentObjectId)
            : [],
    );
    const hasPossibleTarget = $derived(targetObjects.length > 0);
    const selectedTarget = $derived(
        targetObjectId ? (objectsById.get(targetObjectId) ?? null) : null,
    );

    $effect(() => {
        if (!hasPossibleTarget) {
            closeCreate();
        }
    });

    $effect(() => {
        currentObjectId;
        closeCreate();
    });

    function resetForm() {
        targetObjectId = "";
        targetQuery = "";
    }

    function openCreate() {
        resetForm();
        creating = true;
    }

    function closeCreate() {
        creating = false;
        resetForm();
    }

    function toggleCreate() {
        if (creating) closeCreate();
        else openCreate();
    }

    function clearTarget() {
        targetObjectId = "";
        targetQuery = "";
    }

    function selectTarget(id: string) {
        if (creatingConnection) return;

        targetObjectId = id;
        targetQuery = "";
    }

    function handleTargetKeydown(event: KeyboardEvent) {
        if (event.key === "Escape" && targetQuery) {
            event.preventDefault();
            targetQuery = "";
        }
    }

    function otherObjectFor(connection: ObjectConnectionRecord) {
        return objectsById.get(otherObjectIdOf(connection, currentObjectId!)) ?? null;
    }

    async function submit() {
        if (!currentObjectId || !selectedTarget || creatingConnection) return;

        creatingConnection = true;
        try {
            await onCreateConnection(selectedTarget.id);
            closeCreate();
        } finally {
            creatingConnection = false;
        }
    }
</script>

<div class="panel">
    <header class="panel-head">
        <span class="panel-title">Connections</span>
        {#if currentObjectId && hasPossibleTarget}
            <IconButton
                title="Add connection"
                size={22}
                pressed={creating}
                onclick={toggleCreate}
            >
                <Icon name="plus" size={14} />
            </IconButton>
        {/if}
    </header>

    <div class="panel-body">
        {#if !currentObjectId}
            <p class="empty">No note selected</p>
        {:else}
            {#if creating}
                <form
                    class="create-form"
                    onsubmit={(event) => {
                        event.preventDefault();
                        void submit();
                    }}
                >
                    {#if selectedTarget}
                        <div class="selected-target">
                            <span class="selected-title">
                                {displayTitleForObject(selectedTarget)}
                            </span>
                            <button
                                type="button"
                                class="clear-target"
                                aria-label="Clear selected target"
                                disabled={creatingConnection}
                                onclick={clearTarget}
                            >
                                <Icon name="close" size={12} />
                            </button>
                        </div>
                    {:else}
                        <Command.Root
                            class="target-command"
                            label="Connection target"
                            shouldFilter={true}
                            loop
                        >
                            <Command.Input
                                class="target-command-input"
                                placeholder="Search target"
                                bind:value={targetQuery}
                                disabled={creatingConnection}
                                onkeydown={handleTargetKeydown}
                            />

                            <Command.List class="target-command-list">
                                <Command.Empty class="target-command-empty">
                                    No targets
                                </Command.Empty>
                                <Command.Group value="targets">
                                    {#each targetObjects as target (target.id)}
                                        <Command.Item
                                            class="target-command-item"
                                            value={target.id}
                                            keywords={[
                                                target.title,
                                                target.contentText,
                                            ]}
                                            disabled={creatingConnection}
                                            onSelect={() =>
                                                selectTarget(target.id)}
                                        >
                                            <span class="item-primary">
                                                {displayTitleForObject(target)}
                                            </span>
                                        </Command.Item>
                                    {/each}
                                </Command.Group>
                            </Command.List>
                        </Command.Root>
                    {/if}

                    <div class="form-actions">
                        <button
                            type="button"
                            class="text-button"
                            disabled={creatingConnection}
                            onclick={closeCreate}
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            class="primary-button"
                            disabled={!selectedTarget || creatingConnection}
                        >
                            Add
                        </button>
                    </div>
                </form>
            {/if}

            {#if connections.length === 0}
                <p class="empty">No connections yet</p>
            {:else}
                <div class="connection-list">
                    {#each connections as connection (connection.id)}
                        <SidebarConnectionRow
                            {connection}
                            currentObjectId={currentObjectId}
                            otherObject={otherObjectFor(connection)}
                            selected={connection.id === selectedConnectionId}
                            {onOpenObject}
                            {onEditConnection}
                            {onDeleteConnection}
                        />
                    {/each}
                </div>
            {/if}
        {/if}
    </div>
</div>

<style>
    .panel {
        display: flex;
        flex: 1;
        min-height: 0;
        flex-direction: column;
    }

    .panel-head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        min-height: 50px;
        padding: 16px 16px 12px;
    }

    .panel-title {
        font-size: 10.5px;
        color: var(--fg-4);
        letter-spacing: 0.06em;
        text-transform: uppercase;
        font-weight: 500;
    }

    .panel-body {
        flex: 1;
        min-height: 0;
        overflow-y: auto;
        padding: 0 8px 14px;
    }

    .empty {
        margin: 6px 10px;
        font-size: 12px;
        color: var(--fg-4);
    }

    .create-form {
        display: flex;
        flex-direction: column;
        gap: 7px;
        padding: 6px 8px 10px;
        border-bottom: 1px solid var(--line-1);
    }

    .selected-target {
        display: flex;
        align-items: center;
        gap: 6px;
        min-height: 30px;
        border: 1px solid var(--accent-line);
        border-radius: var(--r-sm);
        background: var(--accent-soft);
        color: var(--accent);
        padding: 3px 4px 3px 8px;
    }

    .selected-title {
        min-width: 0;
        flex: 1;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        font-size: 12px;
        font-weight: 500;
    }

    .clear-target {
        flex: 0 0 auto;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 22px;
        height: 22px;
        border: 0;
        border-radius: 5px;
        background: transparent;
        color: inherit;
        cursor: pointer;
        padding: 0;
    }

    .clear-target:hover {
        background: rgba(31, 107, 74, 0.1);
    }

    .clear-target:disabled {
        cursor: default;
        opacity: 0.55;
    }

    :global(.target-command) {
        display: flex;
        flex-direction: column;
        overflow: hidden;
        border: 1px solid var(--line-2);
        border-radius: var(--r-sm);
        background: rgba(255, 255, 255, 0.52);
    }

    :global(.target-command:focus-within) {
        border-color: var(--accent-line);
        background: var(--bg-elev);
    }

    :global(.target-command-input) {
        width: 100%;
        height: 30px;
        border: 0;
        border-bottom: 1px solid var(--line-1);
        background: transparent;
        color: var(--fg-1);
        font: inherit;
        font-size: 12px;
        outline: 0;
        padding: 0 8px;
    }

    :global(.target-command-input)::placeholder {
        color: var(--fg-4);
    }

    :global(.target-command-list) {
        max-height: 200px;
        overflow-y: auto;
        padding: 4px;
    }

    :global(.target-command-item),
    :global(.target-command-empty) {
        min-height: 30px;
        border-radius: 5px;
        box-sizing: border-box;
        color: var(--fg-1);
        font-size: 12px;
        padding: 7px 8px;
    }

    :global(.target-command-item) {
        display: flex;
        cursor: default;
        align-items: center;
    }

    :global(.target-command-item[data-selected]),
    :global(.target-command-item:hover) {
        background: var(--bg-tint);
    }

    :global(.target-command-empty) {
        display: flex;
        align-items: center;
        color: var(--fg-3);
    }

    .item-primary {
        min-width: 0;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .form-actions {
        display: flex;
        justify-content: flex-end;
        gap: 6px;
    }

    .text-button,
    .primary-button {
        height: 28px;
        border-radius: var(--r-sm);
        padding: 0 9px;
        font: inherit;
        font-size: 12px;
        cursor: pointer;
    }

    .text-button {
        border: 0;
        background: transparent;
        color: var(--fg-3);
    }

    .text-button:hover {
        color: var(--fg-1);
        background: var(--bg-tint);
    }

    .primary-button {
        border: 0;
        background: var(--accent);
        color: white;
        font-weight: 500;
    }

    .primary-button:disabled,
    .text-button:disabled {
        cursor: default;
        opacity: 0.55;
    }

    .connection-list {
        display: flex;
        flex-direction: column;
        margin: 0 2px;
    }
</style>
