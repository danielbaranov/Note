<script lang="ts">
    import MagpieMark from "./MagpieMark.svelte";

    type Props = {
        size?: number;
    };

    let { size = 16 }: Props = $props();
</script>

<span class="wordmark" style="font-size: {size}px">
    <MagpieMark size={size + 6} label="" />
    <span class="primary">Magpie</span>
    <span class="dot">·</span>
    <span class="secondary">notes</span>
</span>

<style>
    .wordmark {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        font-weight: 600;
        letter-spacing: -0.01em;
        color: var(--fg-1);
    }

    .primary {
        color: var(--fg-1);
    }

    .dot {
        color: var(--accent);
        font-weight: 500;
    }

    .secondary {
        color: var(--fg-3);
        font-weight: 400;
    }
</style>
