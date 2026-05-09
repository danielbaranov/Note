<script lang="ts">
    import { tick } from "svelte";
    import { Command } from "bits-ui";

    import {
        displayTitleForObject,
        type ObjectRecord,
    } from "$lib/objects";

    export let query = "";
    export let objects: ObjectRecord[] = [];
    export let loading = false;
    export let mode: "inline" | "modal" = "modal";
    export let currentObjectId: string | null = null;
    export let onCreateObject: (title: string) => void | Promise<void> =
        () => {};
    export let onOpenObject: (id: string) => void | Promise<void> = () => {};
    export let onQueryChange: (query: string) => void | Promise<void> =
        () => {};

    let inputElement: HTMLInputElement | null = null;
    let lastQuery = query;

    $: trimmedQuery = query.trim();
    $: isCreateQuery = trimmedQuery.startsWith("+");
    $: createTitle = titleFromCreateQuery(trimmedQuery);
    $: visibleObjects = isCreateQuery ? [] : objects;

    $: if (query !== lastQuery) {
        lastQuery = query;
        void onQueryChange(query);
    }

    export async function focusInput() {
        await tick();
        inputElement?.focus();
        inputElement?.select();
    }

    function titleFromCreateQuery(value: string): string {
        const title = value.slice(1).trim();
        return title || "Untitled";
    }

    function handleInputKeydown(event: KeyboardEvent) {
        if (event.key !== "Enter" || !isCreateQuery) return;

        event.preventDefault();
        event.stopPropagation();
        void onCreateObject(createTitle);
    }
</script>

<Command.Root
    class={`object-command ${mode}`}
    label="Object command"
    shouldFilter={false}
    loop
>
    <Command.Input
        class="command-input"
        placeholder="Search or +Title"
        bind:value={query}
        bind:ref={inputElement}
        onkeydown={handleInputKeydown}
    />

    <Command.List class="command-list">
        {#if isCreateQuery}
            <Command.Item
                class="command-item create-item"
                value={`create:${createTitle}`}
                onSelect={() => onCreateObject(createTitle)}
            >
                <span class="item-primary">Create {createTitle}</span>
                <span class="item-secondary"># {createTitle}</span>
            </Command.Item>
        {:else if loading}
            <Command.Loading class="command-empty">Searching</Command.Loading>
        {:else if visibleObjects.length === 0}
            <Command.Empty class="command-empty">No objects</Command.Empty>
        {:else}
            <Command.Group value="objects">
                {#each visibleObjects as item (item.id)}
                    <Command.Item
                        class={`command-item ${
                            item.id === currentObjectId ? "active-item" : ""
                        }`}
                        value={item.id}
                        keywords={[item.title, item.contentText]}
                        onSelect={() => onOpenObject(item.id)}
                    >
                        <span class="item-primary"
                            >{displayTitleForObject(item)}</span
                        >
                        {#if item.id === currentObjectId}
                            <span class="item-secondary">Open</span>
                        {/if}
                    </Command.Item>
                {/each}
            </Command.Group>
        {/if}
    </Command.List>
</Command.Root>

<style>
    :global(.object-command) {
        display: flex;
        width: min(640px, calc(100vw - 32px));
        flex-direction: column;
        overflow: hidden;
        border: 1px solid var(--line-1);
        border-radius: var(--r-md);
        background: var(--bg-elev);
        box-shadow: var(--shadow-lg);
    }

    :global(.object-command.inline) {
        box-shadow: var(--shadow-md);
    }

    :global(.command-input) {
        width: 100%;
        border: 0;
        border-bottom: 1px solid var(--line-1);
        box-sizing: border-box;
        font: inherit;
        font-size: 15px;
        color: var(--fg-1);
        background: transparent;
        outline: 0;
        padding: 14px 18px;
    }

    :global(.command-input)::placeholder {
        color: var(--fg-4);
    }

    :global(.command-list) {
        max-height: min(420px, 52vh);
        overflow: auto;
        padding: 6px;
    }

    :global(.command-item),
    :global(.command-empty) {
        min-height: 40px;
        border-radius: var(--r-sm);
        box-sizing: border-box;
        color: var(--fg-1);
        font-size: 13.5px;
        padding: 8px 12px;
    }

    :global(.command-item) {
        display: flex;
        cursor: default;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
    }

    :global(.command-item[data-selected]),
    :global(.command-item:hover) {
        background: var(--bg-tint);
    }

    :global(.active-item) {
        color: var(--accent);
        font-weight: 500;
    }

    .item-primary {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .item-secondary {
        flex: none;
        color: var(--fg-3);
        font-size: 11.5px;
        font-family: var(--font-mono);
    }

    :global(.command-empty) {
        display: flex;
        align-items: center;
        color: var(--fg-3);
    }
</style>
