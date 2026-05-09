<script lang="ts">
    import { onMount } from "svelte";

    import AppShell from "$lib/components/shell/AppShell.svelte";
    import TitleBar from "$lib/components/shell/TitleBar.svelte";
    import Sidebar from "$lib/components/sidebar/Sidebar.svelte";
    import EditorPane from "$lib/components/editor/EditorPane.svelte";
    import EditorEmpty from "$lib/components/editor/EditorEmpty.svelte";
    import ConnectionsPanel from "$lib/components/connections/ConnectionsPanel.svelte";
    import GraphDialog from "$lib/components/graph/GraphDialog.svelte";
    import CommandDialog from "$lib/components/CommandDialog.svelte";
    import {
        createMarkdownObject,
        loadObject,
        markdownTitleOrDefault,
        saveObjectContent,
        searchObjects,
        type ObjectRecord,
    } from "$lib/objects";
    import type { Connection } from "$lib/connections";

    let query = $state("");
    let objects = $state<ObjectRecord[]>([]);
    let object = $state<ObjectRecord | null>(null);
    let markdownText = $state("");
    let error = $state("");
    let commandOpen = $state(false);
    let graphOpen = $state(false);
    let connectionsOpen = $state(true);
    let loading = $state(false);
    let searchRequest = 0;

    // Connections aren't persisted yet — empty list keeps the panel honest
    // about its current state. Wire to the backend when the storage lands.
    const noteConnections: Connection[] = [];

    const graphTitle = $derived(
        object
            ? markdownTitleOrDefault(markdownText) || "Untitled"
            : "Note graph",
    );
    const titleBarTitle = $derived(
        object ? markdownTitleOrDefault(markdownText) : "Note",
    );

    onMount(() => {
        void search();
    });

    async function search(nextQuery = query) {
        const request = ++searchRequest;
        error = "";
        loading = true;

        try {
            const results = await searchObjects(nextQuery);
            if (request === searchRequest) {
                objects = results;
            }
        } catch (caught) {
            error = String(caught);
        } finally {
            if (request === searchRequest) {
                loading = false;
            }
        }
    }

    async function create(title: string) {
        const noteTitle = title.trim() || "Untitled";
        const initialContent = `# ${noteTitle}`;
        error = "";

        try {
            const created = await createMarkdownObject(initialContent);
            object = created;
            markdownText = initialContent;
            closeCommand();
            await search("");
        } catch (caught) {
            error = String(caught);
        }
    }

    async function open(id: string) {
        error = "";

        try {
            const loaded = await loadObject(id);
            object = loaded;
            markdownText = loaded.contentText;
            closeCommand();
            await search("");
        } catch (caught) {
            error = String(caught);
        }
    }

    async function save() {
        if (!object) return;

        error = "";

        try {
            object = await saveObjectContent(object.id, markdownText);
            await search(query);
        } catch (caught) {
            error = String(caught);
        }
    }

    function closeCommand() {
        query = "";
        commandOpen = false;
    }

    function openCommand() {
        commandOpen = true;
    }

    function handleWindowKeydown(event: KeyboardEvent) {
        const isCommandShortcut =
            event.key === "/" && (event.metaKey || event.ctrlKey);

        if (isCommandShortcut) {
            event.preventDefault();
            openCommand();
            return;
        }

        const isQuickSearch =
            event.key.toLowerCase() === "k" &&
            (event.metaKey || event.ctrlKey);

        if (isQuickSearch) {
            event.preventDefault();
            openCommand();
        }
    }
</script>

<svelte:window onkeydown={handleWindowKeydown} />

<AppShell {connectionsOpen}>
    {#snippet titleBar()}
        <TitleBar
            title={titleBarTitle}
            {connectionsOpen}
            onOpenCommand={openCommand}
            onOpenGraph={() => (graphOpen = true)}
            onToggleConnections={() => (connectionsOpen = !connectionsOpen)}
        />
    {/snippet}

    {#snippet sidebar()}
        <Sidebar
            {objects}
            currentObjectId={object?.id ?? null}
            onSelectObject={open}
            onOpenCommand={openCommand}
            onCreateObject={() => create("")}
        />
    {/snippet}

    {#snippet editor()}
        {#if object}
            <EditorPane
                {object}
                {markdownText}
                onChange={(value) => (markdownText = value)}
                onSave={save}
                onOpenCommand={openCommand}
            />
        {:else}
            <EditorEmpty onOpenCommand={openCommand} />
        {/if}
    {/snippet}

    {#snippet connections()}
        {#if object}
            <ConnectionsPanel connections={noteConnections} />
        {/if}
    {/snippet}
</AppShell>

<CommandDialog
    bind:open={commandOpen}
    bind:query
    {objects}
    {loading}
    currentObjectId={object?.id ?? null}
    onCreateObject={create}
    onOpenObject={open}
    onQueryChange={search}
/>

<GraphDialog bind:open={graphOpen} title={graphTitle} />

{#if error}
    <p class="error" role="alert">{error}</p>
{/if}

<style>
    .error {
        position: fixed;
        right: 16px;
        bottom: 16px;
        z-index: 40;
        max-width: min(520px, calc(100vw - 32px));
        margin: 0;
        border: 1px solid rgba(177, 74, 57, 0.3);
        border-radius: var(--r-md);
        background: #fbe9e5;
        color: #7a2a1f;
        font-size: 12.5px;
        padding: 10px 12px;
        white-space: pre-wrap;
        box-shadow: var(--shadow-md);
    }
</style>
