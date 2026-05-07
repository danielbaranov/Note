<script lang="ts">
    import { onMount, tick } from "svelte";

    import CommandDialog from "$lib/components/CommandDialog.svelte";
    import EditorWorkspace from "$lib/components/EditorWorkspace.svelte";
    import ObjectCommand from "$lib/components/ObjectCommand.svelte";
    import {
        createMarkdownObject,
        loadObject,
        saveObjectContent,
        searchObjects,
        type ObjectRecord,
    } from "$lib/objects";

    let query = $state("");
    let objects = $state<ObjectRecord[]>([]);
    let object = $state<ObjectRecord | null>(null);
    let markdownText = $state("");
    let error = $state("");
    let commandOpen = $state(false);
    let loading = $state(false);
    let inlineCommand = $state<ObjectCommand>();
    let searchRequest = 0;

    onMount(() => {
        void search();
        void focusInlineCommand();
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
            const created = await createMarkdownObject(noteTitle);
            await saveObjectContent(created.id, initialContent);

            object = {
                ...created,
                contentText: initialContent,
                updatedAtMs: Date.now(),
            };
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
            await saveObjectContent(object.id, markdownText);
            object = {
                ...object,
                contentText: markdownText,
                updatedAtMs: Date.now(),
            };
            await search(query);
        } catch (caught) {
            error = String(caught);
        }
    }

    function closeCommand() {
        query = "";
        commandOpen = false;
    }

    async function focusInlineCommand() {
        await tick();
        await inlineCommand?.focusInput();
    }

    function openCommand() {
        if (object) {
            commandOpen = true;
        } else {
            void focusInlineCommand();
        }
    }

    function handleWindowKeydown(event: KeyboardEvent) {
        const isCommandShortcut =
            event.key === "/" && (event.metaKey || event.ctrlKey);

        if (!isCommandShortcut) return;

        event.preventDefault();
        openCommand();
    }
</script>

<svelte:window onkeydown={handleWindowKeydown} />

<main class:empty={!object}>
    {#if error}
        <p class="error" role="alert">{error}</p>
    {/if}

    {#if object}
        <EditorWorkspace
            {object}
            {markdownText}
            onChange={(value) => (markdownText = value)}
            onSave={save}
            onOpenCommand={openCommand}
        />
        <CommandDialog
            bind:open={commandOpen}
            bind:query
            {objects}
            {loading}
            currentObjectId={object.id}
            onCreateObject={create}
            onOpenObject={open}
            onQueryChange={search}
        />
    {:else}
        <section class="empty-command" aria-label="Open object">
            <ObjectCommand
                bind:this={inlineCommand}
                bind:query
                mode="inline"
                {objects}
                {loading}
                onCreateObject={create}
                onOpenObject={open}
                onQueryChange={search}
            />
        </section>
    {/if}
</main>

<style>
    :global(body) {
        margin: 0;
    }

    :global(*) {
        box-sizing: border-box;
    }

    main {
        position: relative;
        height: 100vh;
        min-height: 0;
        background: #fbfbfa;
        color: #18181b;
        font-family:
            Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont,
            "Segoe UI", sans-serif;
    }

    main.empty {
        display: grid;
        place-items: start center;
        padding-top: 18vh;
    }

    .empty-command {
        width: min(720px, calc(100vw - 32px));
    }

    .error {
        position: fixed;
        right: 16px;
        bottom: 16px;
        z-index: 20;
        max-width: min(520px, calc(100vw - 32px));
        margin: 0;
        border: 1px solid rgba(185, 28, 28, 0.22);
        border-radius: 8px;
        background: #fef2f2;
        color: #991b1b;
        font-size: 0.86rem;
        padding: 10px 12px;
        white-space: pre-wrap;
    }
</style>
