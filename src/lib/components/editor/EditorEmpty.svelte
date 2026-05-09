<script lang="ts">
    import MagpieMark from "$lib/components/brand/MagpieMark.svelte";

    type Props = {
        onOpenCommand: () => void;
    };

    let { onOpenCommand }: Props = $props();
</script>

<div class="empty">
    <MagpieMark size={48} />
    <h2>No note open</h2>
    <p>Open a note from the sidebar, or search to find or create one.</p>
    <button type="button" class="cta" onclick={onOpenCommand}>
        <span>Open search</span>
        <span class="kbd">⌘K</span>
    </button>
</div>

<style>
    .empty {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 12px;
        color: var(--fg-3);
        padding: 40px;
    }

    h2 {
        margin: 8px 0 0;
        font-size: 18px;
        font-weight: 600;
        color: var(--fg-2);
        letter-spacing: -0.01em;
    }

    p {
        margin: 0;
        font-size: 13.5px;
        text-align: center;
        max-width: 320px;
    }

    .cta {
        margin-top: 6px;
        display: inline-flex;
        align-items: center;
        gap: 10px;
        padding: 8px 14px;
        border: 1px solid var(--line-2);
        border-radius: var(--r-md);
        background: var(--bg-elev);
        color: var(--fg-2);
        font: inherit;
        font-size: 12.5px;
        cursor: pointer;
        transition: background 0.12s;
    }

    .cta:hover {
        background: var(--bg-tint);
    }

    .kbd {
        font-family: var(--font-mono);
        font-size: 10.5px;
        padding: 1px 5px;
        border: 1px solid var(--line-2);
        border-radius: 4px;
        color: var(--fg-3);
    }
</style>
