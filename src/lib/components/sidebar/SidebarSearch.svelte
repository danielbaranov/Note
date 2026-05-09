<script lang="ts">
    import Icon from "$lib/components/ui/Icon.svelte";

    type Props = {
        onclick?: () => void;
    };

    let { onclick }: Props = $props();
</script>

<button type="button" class="search" onclick={() => onclick?.()}>
    <span class="icon"><Icon name="search" size={13} /></span>
    <span class="label">Search</span>
    <span class="spacer"></span>
    <span class="kbd">⌘K</span>
</button>

<style>
    .search {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 5px 10px;
        font: inherit;
        font-size: 12.5px;
        color: var(--fg-2);
        cursor: pointer;
        border: 0;
        border-radius: 6px;
        background: var(--bg-tint);
        text-align: left;
        width: 100%;
    }

    .search:hover {
        background: var(--bg-tint-2);
    }

    .icon {
        color: var(--fg-3);
        display: inline-flex;
        width: 13px;
        height: 13px;
    }

    .label {
        color: var(--fg-3);
    }

    .spacer {
        flex: 1;
    }

    .kbd {
        font-family: var(--font-mono);
        font-size: 10px;
        color: var(--fg-4);
    }
</style>
