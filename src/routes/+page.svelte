<script lang="ts">
    import { onMount } from "svelte";

    import AppShell from "$lib/components/shell/AppShell.svelte";
    import TitleBar from "$lib/components/shell/TitleBar.svelte";
    import Sidebar from "$lib/components/sidebar/Sidebar.svelte";
    import SidebarConnections from "$lib/components/sidebar/SidebarConnections.svelte";
    import EditorPane from "$lib/components/editor/EditorPane.svelte";
    import EditorEmpty from "$lib/components/editor/EditorEmpty.svelte";
    import GraphDialog from "$lib/components/graph/GraphDialog.svelte";
    import CommandDialog from "$lib/components/CommandDialog.svelte";
    import {
        connectionDirectionOf,
        createObjectConnection,
        deleteObjectConnection,
        listAllConnections,
        listObjectConnections,
        otherObjectIdOf,
        updateObjectConnectionContent,
        type ObjectConnectionRecord,
    } from "$lib/connections";
    import {
        createMarkdownObject,
        deleteObject,
        displayTitleForObject,
        loadObject,
        markdownTitleOrDefault,
        saveObjectContent,
        searchObjects,
        type ObjectRecord,
    } from "$lib/objects";

    type EditorTarget =
        | { kind: "object"; id: string }
        | { kind: "connection"; id: string };

    let query = $state("");
    let objects = $state<ObjectRecord[]>([]);
    let object = $state<ObjectRecord | null>(null);
    let objectConnections = $state<ObjectConnectionRecord[]>([]);
    let graphNodes = $state<ObjectRecord[]>([]);
    let graphEdges = $state<ObjectConnectionRecord[]>([]);
    let markdownText = $state("");
    let error = $state("");
    let commandOpen = $state(false);
    let graphOpen = $state(false);
    let connectionsOpen = $state(true);
    let loading = $state(false);
    let editorTarget = $state<EditorTarget | null>(null);
    let searchRequest = 0;
    let connectionsRequest = 0;

    const selectedConnection = $derived.by(() => {
        if (editorTarget?.kind !== "connection") return null;

        const connectionId = editorTarget.id;
        return (
            objectConnections.find(
                (connection) => connection.id === connectionId,
            ) ?? null
        );
    });
    const editorKey = $derived(
        editorTarget ? `${editorTarget.kind}:${editorTarget.id}` : "",
    );
    const editorTitle = $derived(
        selectedConnection
            ? connectionEditorTitle(selectedConnection)
            : object
              ? markdownTitleOrDefault(markdownText)
              : "Untitled",
    );
    const editorFolder = $derived(
        editorTarget?.kind === "connection" ? "Connections" : "Notes",
    );
    const editorUpdatedAtMs = $derived(
        selectedConnection ? selectedConnection.updatedAtMs : object?.updatedAtMs,
    );
    const currentObjectTitle = $derived(
        object
            ? editorTarget?.kind === "object"
                ? markdownTitleOrDefault(markdownText)
                : displayTitleForObject(object)
            : "Note graph",
    );
    const graphTitle = $derived(
        object ? currentObjectTitle || "Untitled" : "Note graph",
    );
    const titleBarTitle = $derived(editorTarget ? editorTitle : "Note");

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
            objectConnections = [];
            editorTarget = { kind: "object", id: created.id };
            connectionsRequest += 1;
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
            editorTarget = { kind: "object", id: loaded.id };
            markdownText = loaded.contentText;
            closeCommand();
            await loadConnections(loaded.id);
            await search("");
        } catch (caught) {
            error = String(caught);
        }
    }

    async function save() {
        if (!editorTarget) return;

        error = "";

        try {
            if (editorTarget.kind === "connection") {
                const updated = await updateObjectConnectionContent(
                    editorTarget.id,
                    markdownText,
                );
                objectConnections = objectConnections.map((connection) =>
                    connection.id === updated.id ? updated : connection,
                );
                graphEdges = graphEdges.map((connection) =>
                    connection.id === updated.id ? updated : connection,
                );
                return;
            }

            const updated = await saveObjectContent(editorTarget.id, markdownText);
            if (object?.id === updated.id) {
                object = updated;
            }
            await search(query);
        } catch (caught) {
            error = String(caught);
        }
    }

    async function loadConnections(objectId: string) {
        const request = ++connectionsRequest;

        try {
            const connections = await listObjectConnections(objectId);
            if (request === connectionsRequest && object?.id === objectId) {
                objectConnections = connections;
            }
        } catch (caught) {
            if (request === connectionsRequest) {
                error = String(caught);
            }
        }
    }

    async function withError<T>(work: () => Promise<T>): Promise<T> {
        error = "";
        try {
            return await work();
        } catch (caught) {
            error = String(caught);
            throw caught;
        }
    }

    function createConnection(targetObjectId: string) {
        if (!object) return Promise.resolve();
        const source = object;

        return withError(async () => {
            const created = await createObjectConnection(
                source.id,
                targetObjectId,
                "",
            );
            objectConnections = [created, ...objectConnections];
            editConnection(created);
        });
    }

    function editConnection(connection: ObjectConnectionRecord) {
        editorTarget = { kind: "connection", id: connection.id };
        markdownText = connection.contentText;
    }

    function deleteConnection(id: string) {
        return withError(async () => {
            await deleteObjectConnection(id);
            objectConnections = objectConnections.filter(
                (connection) => connection.id !== id,
            );
            graphEdges = graphEdges.filter((connection) => connection.id !== id);

            if (editorTarget?.kind === "connection" && editorTarget.id === id) {
                editorTarget = object ? { kind: "object", id: object.id } : null;
                markdownText = object?.contentText ?? "";
            }
        });
    }

    async function deleteObjectById(id: string) {
        await withError(() => deleteObject(id));

        if (object?.id === id) {
            object = null;
            editorTarget = null;
            markdownText = "";
            objectConnections = [];
            connectionsRequest += 1;
        }

        await search(query);
    }

    async function openGraph() {
        error = "";
        graphOpen = true;

        try {
            const [nextNodes, nextEdges] = await Promise.all([
                searchObjects(""),
                listAllConnections(),
            ]);

            graphNodes = nextNodes;
            graphEdges = nextEdges;
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

    function connectionEditorTitle(connection: ObjectConnectionRecord): string {
        const currentObject = object;
        if (!currentObject) return "Connection";

        const otherObject = objects.find(
            (candidate) =>
                candidate.id === otherObjectIdOf(connection, currentObject.id),
        );
        const otherTitle = otherObject
            ? displayTitleForObject(otherObject)
            : "Unknown object";
        const direction = connectionDirectionOf(connection, currentObject.id);

        return direction === "incoming"
            ? `Connection from ${otherTitle}`
            : `Connection to ${otherTitle}`;
    }
</script>

<svelte:window onkeydown={handleWindowKeydown} />

<AppShell {connectionsOpen}>
    {#snippet titleBar()}
        <TitleBar
            title={titleBarTitle}
            {connectionsOpen}
            onOpenCommand={openCommand}
            onOpenGraph={openGraph}
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
            onDeleteObject={deleteObjectById}
        />
    {/snippet}

    {#snippet editor()}
        {#if object && editorTarget}
            <EditorPane
                {editorKey}
                title={editorTitle}
                folder={editorFolder}
                updatedAtMs={editorUpdatedAtMs}
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
        <SidebarConnections
            {objects}
            connections={objectConnections}
            currentObjectId={object?.id ?? null}
            onOpenObject={open}
            onCreateConnection={createConnection}
            selectedConnectionId={selectedConnection?.id ?? null}
            onEditConnection={editConnection}
            onDeleteConnection={deleteConnection}
        />
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

<GraphDialog
    bind:open={graphOpen}
    title={graphTitle}
    nodes={graphNodes}
    edges={graphEdges}
    currentObjectId={object?.id ?? null}
    onSelectNode={(id) => {
        graphOpen = false;
        void open(id);
    }}
/>

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
