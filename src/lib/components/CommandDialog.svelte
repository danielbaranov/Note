<script lang="ts">
    import { tick } from "svelte";
    import { Dialog } from "bits-ui";

    import ObjectCommand from "$lib/components/ObjectCommand.svelte";
    import type { ObjectRecord } from "$lib/objects";

    export let open = false;
    export let query = "";
    export let objects: ObjectRecord[] = [];
    export let loading = false;
    export let currentObjectId: string | null = null;
    export let onCreateObject: (title: string) => void | Promise<void> =
        () => {};
    export let onOpenObject: (id: string) => void | Promise<void> = () => {};
    export let onQueryChange: (query: string) => void | Promise<void> =
        () => {};

    let command: ObjectCommand | undefined;

    $: if (open) {
        void focusCommand();
    }

    async function focusCommand() {
        await tick();
        await command?.focusInput();
    }

    function handleOpenChange(nextOpen: boolean) {
        open = nextOpen;

        if (nextOpen) return;

        query = "";
        void onQueryChange("");
    }

    function handleOpenAutoFocus(event: Event) {
        event.preventDefault();
        void focusCommand();
    }
</script>

<Dialog.Root {open} onOpenChange={handleOpenChange}>
    <Dialog.Portal>
        <Dialog.Overlay class="dialog-overlay" />
        <Dialog.Content
            class="dialog-content"
            onOpenAutoFocus={handleOpenAutoFocus}
        >
            <Dialog.Title class="sr-only">Open object</Dialog.Title>
            <ObjectCommand
                bind:this={command}
                bind:query
                mode="modal"
                {objects}
                {loading}
                {currentObjectId}
                {onCreateObject}
                {onOpenObject}
                {onQueryChange}
            />
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>

<style>
    :global(.dialog-overlay) {
        position: fixed;
        inset: 0;
        z-index: 10;
        background: rgba(15, 23, 42, 0.22);
        backdrop-filter: blur(4px);
    }

    :global(.dialog-content) {
        position: fixed;
        top: 14vh;
        left: 50%;
        z-index: 11;
        outline: none;
        transform: translateX(-50%);
    }

    :global(.sr-only) {
        position: absolute;
        width: 1px;
        height: 1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        white-space: nowrap;
        clip-path: inset(50%);
    }
</style>
