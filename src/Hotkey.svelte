<script lang="ts">
    export let onSave: () => void = () => {};

    function handleKeydown(event: KeyboardEvent): void {
        const isSaveShortcut =
            event.key.toLowerCase() === "s" && (event.metaKey || event.ctrlKey);

        if (isSaveShortcut) {
            event.preventDefault();
            onSave();
        }
    }
</script>

<svelte:window onkeydown={handleKeydown} />
