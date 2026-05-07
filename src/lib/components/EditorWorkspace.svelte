<script lang="ts">
    import Editor from "../../editor/editor.svelte";
    import {
        displayTitleForObject,
        type ObjectRecord,
    } from "$lib/objects";

    export let object: ObjectRecord;
    export let markdownText = "";
    export let onChange: (value: string) => void = () => {};
    export let onSave: () => void | Promise<void> = () => {};
    export let onOpenCommand: () => void = () => {};

    $: displayTitle = displayTitleForObject({
        ...object,
        contentText: markdownText,
    });
</script>

<section class="workspace" aria-label={displayTitle}>
    <header class="workspace-header">
        <h1 title={displayTitle}>{displayTitle}</h1>
        <button type="button" onclick={onSave}>Save</button>
    </header>

    <div class="editor-shell">
        {#key object.id}
            <Editor
                initialText={markdownText}
                {onChange}
                onSave={() => onSave()}
                {onOpenCommand}
            />
        {/key}
    </div>
</section>

<style>
    .workspace {
        display: flex;
        height: 100%;
        min-height: 0;
        flex-direction: column;
        background: #fbfbfa;
    }

    .workspace-header {
        display: flex;
        min-height: 48px;
        flex: none;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
        border-bottom: 1px solid rgba(24, 24, 27, 0.1);
        padding: 0 16px;
    }

    h1 {
        min-width: 0;
        overflow: hidden;
        margin: 0;
        color: #18181b;
        font-size: 0.95rem;
        font-weight: 600;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    button {
        flex: none;
        border: 1px solid rgba(24, 24, 27, 0.14);
        border-radius: 6px;
        background: #ffffff;
        color: #18181b;
        font: inherit;
        font-size: 0.86rem;
        padding: 6px 10px;
    }

    button:hover {
        background: #f4f4f5;
    }

    .editor-shell {
        flex: 1;
        min-height: 0;
    }
</style>
