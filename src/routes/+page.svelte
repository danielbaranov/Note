<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    import Editor from "../editor/editor.svelte";

    type ObjectRecord = {
        id: string;
        objectType: string;
        title: string;
        contentText: string;
        metadataJson: string;
        sourceUri: string | null;
        createdAtMs: number;
        updatedAtMs: number;
    };

    let query = $state("");
    let objects = $state<ObjectRecord[]>([]);
    let object = $state<ObjectRecord | null>(null);
    let markdownText = $state("");
    let error = $state("");

    onMount(search);

    async function run<T>(command: string, args = {}) {
        error = "";

        try {
            return await invoke<T>(command, args);
        } catch (caught) {
            error = String(caught);
        }
    }

    async function search() {
        objects =
            (await run<ObjectRecord[]>("search_objects", { query })) ?? [];
    }

    async function create() {
        const created = await run<ObjectRecord>("create_markdown_object", {
            title: query,
        });
        if (!created) return;

        object = created;
        markdownText = created.contentText;
        query = "";
        await search();
    }

    async function open(id: string) {
        const loaded = await run<ObjectRecord>("load_object", { id });
        if (!loaded) return;

        object = loaded;
        markdownText = loaded.contentText;
    }

    async function save() {
        if (!object) return;

        await run("save_object_content", {
            id: object.id,
            contentText: markdownText,
        });
    }
</script>

<main>
    <input
        placeholder="Search or new filename"
        bind:value={query}
        oninput={search}
    />
    <button type="button" onclick={create}>+</button>

    {#if error}
        <pre>{error}</pre>
    {/if}

    {#each objects as item (item.id)}
        <button type="button" onclick={() => open(item.id)}>{item.title}</button
        >
    {/each}

    {#if object}
        <p>
            <button type="button" onclick={save}>Save</button>
            {object.title}
        </p>

        <div class="editor">
            {#key object.id}
                <Editor
                    initialText={markdownText}
                    onChange={(value) => (markdownText = value)}
                    onSave={save}
                />
            {/key}
        </div>
    {/if}
</main>

<style>
    main {
        display: flex;
        height: 100vh;
        flex-direction: column;
        gap: 8px;
        padding: 12px;
    }

    .editor {
        flex: 1;
        min-height: 0;
        border: 1px solid #ccc;
    }
</style>
