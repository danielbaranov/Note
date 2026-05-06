<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    import Editor from "../editor/editor.svelte";
    import Hotkey from "../Hotkey.svelte";

    let markdownText = $state("New Note");

    function handleEditorChange(value: string) {
        markdownText = value;
    }

    async function saveHandler() {
        await invoke("save_file", {
            path: "../example.md",
            contents: markdownText,
        });

        console.log("File Saved!");
    }
</script>

<Hotkey onSave={saveHandler} />

<Editor
    initialText={markdownText}
    onChange={handleEditorChange}
    onSave={saveHandler}
/>

<pre>{markdownText}</pre>

<style>
    :root {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: #0f0f0f;
        background-color: #f6f6f6;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }
</style>
