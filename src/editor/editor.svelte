<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { EditorState } from "@codemirror/state";
    import { EditorView, keymap } from "@codemirror/view";
    import { defaultKeymap } from "@codemirror/commands";
    import { markdown } from "@codemirror/lang-markdown";

    import { markdownDecorations } from "./markdownDecorators";

    // to be bound to the editor div and displayed
    let editorElement: HTMLDivElement;
    let view: EditorView;

    export let initialText = "Hello Word";
    export let onSave: () => void = () => {};

    const saveKeymap = keymap.of([
        {
            key: "Mod-s",
            preventDefault: true,
            run: () => {
                onSave();
                return true;
            },
        },
    ]);

    // To propagate further
    export let onChange: (value: string) => void = () => {};

    let startState: EditorState = EditorState.create({
        doc: initialText,
        extensions: [
            markdown(),
            markdownDecorations,

            saveKeymap,
            keymap.of(defaultKeymap),

            EditorView.updateListener.of((update) => {
                if (update.docChanged) {
                    const markdown = update.state.doc.toString();
                    onChange(markdown);
                }
            }),
        ],
    });
    onMount(() => {
        view = new EditorView({
            state: startState,
            parent: editorElement,
        });
    });

    onDestroy(() => {
        view?.destroy();
    });
</script>

<div class="editor" bind:this={editorElement}></div>

<style>
    .editor {
        height: 100%;
    }

    .editor :global(.cm-editor) {
        height: 100%;
    }

    .editor :global(.cm-scroller) {
        font-family: system-ui, sans-serif;
        line-height: 1.6;
    }

    .editor :global(.cm-md-h1) {
        font-size: 2rem;
        font-weight: 700;
        line-height: 2.5rem;
    }

    .editor :global(.cm-md-h2) {
        font-size: 1.5rem;
        font-weight: 700;
        line-height: 2rem;
    }

    .editor :global(.cm-md-bold) {
        font-weight: 700;
    }

    .editor :global(.cm-md-italic) {
        font-style: italic;
    }
</style>
