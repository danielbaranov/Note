<script lang="ts">
    import type { Snippet } from "svelte";

    type Props = {
        title: string;
        action?: Snippet;
        children: Snippet;
    };

    let { title, action, children }: Props = $props();
</script>

<section class="section">
    <header class="section-head">
        <span class="section-title">{title}</span>
        {#if action}
            <span class="section-action">{@render action()}</span>
        {/if}
    </header>
    <div class="section-body">
        {@render children()}
    </div>
</section>

<style>
    .section {
        margin-bottom: 14px;
    }

    .section-head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 12px 4px;
    }

    .section-title {
        font-size: 10.5px;
        color: var(--fg-4);
        letter-spacing: 0.06em;
        text-transform: uppercase;
        font-weight: 500;
    }

    .section-action {
        display: inline-flex;
        align-items: center;
        color: var(--fg-4);
    }

    .section-body {
        display: flex;
        flex-direction: column;
    }
</style>
