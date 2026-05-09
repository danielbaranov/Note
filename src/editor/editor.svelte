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
    export let onOpenCommand: () => void = () => {};

    const appKeymap = keymap.of([
        {
            key: "Mod-s",
            preventDefault: true,
            run: () => {
                onSave();
                return true;
            },
        },
        {
            key: "Mod-/",
            preventDefault: true,
            run: () => {
                onOpenCommand();
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

            appKeymap,
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
    /* Visual styling (typography, h1/h2, etc.) lives with whichever component
       hosts this editor — see EditorPane.svelte for the prose styles. */
    .editor {
        height: 100%;
    }

    .editor :global(.cm-editor) {
        height: 100%;
    }
</style>
