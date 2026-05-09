<script lang="ts">
    type Props = {
        folder?: string;
        updatedAtMs?: number;
        wordCount?: number;
    };

    let { folder = "Notes", updatedAtMs, wordCount = 0 }: Props = $props();

    let now = $state(Date.now());

    $effect(() => {
        const id = setInterval(() => (now = Date.now()), 30_000);
        return () => clearInterval(id);
    });

    const updatedLabel = $derived(
        updatedAtMs ? `Edited ${formatRelative(now - updatedAtMs)}` : "",
    );

    function formatRelative(deltaMs: number): string {
        const seconds = Math.max(0, Math.floor(deltaMs / 1000));
        if (seconds < 60) return "just now";
        const minutes = Math.floor(seconds / 60);
        if (minutes < 60) return `${minutes}m ago`;
        const hours = Math.floor(minutes / 60);
        if (hours < 24) return `${hours}h ago`;
        const days = Math.floor(hours / 24);
        if (days < 30) return `${days}d ago`;
        const months = Math.floor(days / 30);
        if (months < 12) return `${months}mo ago`;
        return `${Math.floor(months / 12)}y ago`;
    }
</script>

<div class="breadcrumb">
    <div class="folder">
        <span>{folder}</span>
    </div>
    <div class="meta">
        {#if updatedLabel}
            <span>{updatedLabel}</span>
            <span aria-hidden="true">·</span>
        {/if}
        <span>{wordCount} words</span>
    </div>
</div>

<style>
    .breadcrumb {
        padding: 14px 56px 0;
        display: flex;
        align-items: center;
        justify-content: space-between;
        color: var(--fg-3);
        font-size: 11.5px;
    }

    .folder {
        display: flex;
        align-items: center;
        gap: 6px;
    }

    .meta {
        display: flex;
        align-items: center;
        gap: 14px;
    }
</style>
