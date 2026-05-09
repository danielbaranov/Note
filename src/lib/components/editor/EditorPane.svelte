<script lang="ts">
    import Editor from "../../../editor/editor.svelte";
    import EditorBreadcrumb from "./EditorBreadcrumb.svelte";
    import type { ObjectRecord } from "$lib/objects";

    type Props = {
        object: ObjectRecord;
        markdownText: string;
        onChange: (value: string) => void;
        onSave: () => void | Promise<void>;
        onOpenCommand: () => void;
    };

    let {
        object,
        markdownText,
        onChange,
        onSave,
        onOpenCommand,
    }: Props = $props();

    const wordCount = $derived(countWords(markdownText));

    function countWords(markdown: string): number {
        const stripped = markdown
            .replace(/```[\s\S]*?```/g, " ")
            .replace(/`[^`]+`/g, " ")
            .replace(/[#>*_\-\[\]\(\)!]/g, " ");
        const tokens = stripped.split(/\s+/).filter(Boolean);
        return tokens.length;
    }
</script>

<section class="pane" aria-label={object.title || "Untitled"}>
    <EditorBreadcrumb updatedAtMs={object.updatedAtMs} {wordCount} />

    <div class="prose-shell">
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
    .pane {
        flex: 1;
        min-height: 0;
        display: flex;
        flex-direction: column;
        background: var(--bg-surface);
    }

    .prose-shell {
        flex: 1;
        min-height: 0;
        overflow: auto;
    }

    /* Constrain CodeMirror to the same column as the mockup's prose. */
    .prose-shell :global(.cm-editor) {
        height: 100%;
    }

    .prose-shell :global(.cm-scroller) {
        font-family: var(--font-sans);
        font-size: 16px;
        line-height: 1.65;
        letter-spacing: -0.005em;
        color: var(--fg-1);
    }

    .prose-shell :global(.cm-content) {
        max-width: 720px;
        margin: 0 auto;
        padding: 30px 56px 60px;
        caret-color: var(--accent);
    }

    .prose-shell :global(.cm-line) {
        padding: 0;
    }

    .prose-shell :global(.cm-md-h1) {
        font-size: 32px;
        font-weight: 600;
        line-height: 1.15;
        letter-spacing: -0.02em;
    }

    .prose-shell :global(.cm-md-h2) {
        font-size: 19px;
        font-weight: 600;
        letter-spacing: -0.01em;
    }

    .prose-shell :global(.cm-md-bold) {
        font-weight: 600;
        color: var(--fg-1);
    }

    .prose-shell :global(.cm-md-italic) {
        font-style: italic;
    }

    .prose-shell :global(.cm-cursor) {
        border-left-color: var(--accent);
        border-left-width: 2px;
    }

    .prose-shell :global(.cm-focused) {
        outline: none;
    }
</style>
