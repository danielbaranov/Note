<script lang="ts">
    import { Dialog } from "bits-ui";

    import Icon from "$lib/components/ui/Icon.svelte";

    type Props = {
        open: boolean;
        title: string;
    };

    let { open = $bindable(false), title }: Props = $props();
</script>

<Dialog.Root bind:open>
    <Dialog.Portal>
        <Dialog.Overlay class="graph-overlay" />
        <Dialog.Content class="graph-content">
            <header class="head">
                <div class="title-group">
                    <span class="dot" aria-hidden="true"></span>
                    <div>
                        <Dialog.Title class="title">{title}</Dialog.Title>
                        <Dialog.Description class="subtitle"
                            >Local view</Dialog.Description
                        >
                    </div>
                </div>

                <div class="head-spacer"></div>

                <div class="scope-toggle" role="tablist" aria-label="Scope">
                    {#each ["Local", "Global", "Tags"] as scope, i (scope)}
                        <span
                            class="scope-option"
                            class:active={i === 0}
                            role="tab"
                            aria-selected={i === 0}>{scope}</span
                        >
                    {/each}
                </div>

                <Dialog.Close class="close-btn" aria-label="Close">
                    <Icon name="close" />
                </Dialog.Close>
            </header>

            <div class="canvas">
                <div class="placeholder">
                    <Icon name="graph" size={36} />
                    <h3>Graph view coming soon</h3>
                    <p>
                        A spatial map of this note's connections will live
                        here.
                    </p>
                </div>
            </div>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>

<style>
    :global(.graph-overlay) {
        position: fixed;
        inset: 0;
        z-index: 30;
        background: rgba(20, 19, 17, 0.32);
        backdrop-filter: blur(6px);
    }

    :global(.graph-content) {
        position: fixed;
        top: 50%;
        left: 50%;
        z-index: 31;
        transform: translate(-50%, -50%);
        width: min(920px, calc(100vw - 64px));
        height: min(680px, calc(100vh - 96px));
        background: var(--bg-elev);
        border-radius: var(--r-xl);
        box-shadow: var(--shadow-lg);
        display: flex;
        flex-direction: column;
        overflow: hidden;
        outline: none;
    }

    .head {
        padding: 14px 20px;
        border-bottom: 1px solid var(--line-1);
        display: flex;
        align-items: center;
        gap: 14px;
        background: var(--bg-elev);
    }

    .title-group {
        display: flex;
        align-items: center;
        gap: 10px;
    }

    .dot {
        width: 8px;
        height: 8px;
        border-radius: 50%;
        background: var(--accent);
        display: inline-block;
        flex: 0 0 8px;
    }

    :global(.graph-content .title) {
        font-size: 13px;
        font-weight: 600;
        letter-spacing: -0.005em;
        color: var(--fg-1);
        margin: 0;
    }

    :global(.graph-content .subtitle) {
        font-size: 11px;
        color: var(--fg-3);
        margin: 0;
    }

    .head-spacer {
        flex: 1;
    }

    .scope-toggle {
        display: flex;
        background: var(--bg-tint);
        border-radius: 7px;
        padding: 2px;
        gap: 2px;
    }

    .scope-option {
        padding: 4px 10px;
        font-size: 11.5px;
        font-weight: 500;
        color: var(--fg-3);
        border-radius: 5px;
        cursor: pointer;
    }

    .scope-option.active {
        background: var(--bg-elev);
        color: var(--fg-1);
        box-shadow: var(--shadow-sm);
    }

    :global(.close-btn) {
        width: 28px;
        height: 28px;
        border: 0;
        padding: 0;
        border-radius: 7px;
        background: transparent;
        color: var(--fg-2);
        cursor: pointer;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        transition:
            background 0.12s,
            color 0.12s;
    }

    :global(.close-btn):hover {
        background: var(--bg-tint);
        color: var(--fg-1);
    }

    .canvas {
        flex: 1;
        position: relative;
        background: radial-gradient(
            ellipse at center,
            #fdfcfa 0%,
            #f4f3f0 100%
        );
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .placeholder {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
        color: var(--fg-3);
        text-align: center;
        padding: 40px;
    }

    .placeholder h3 {
        margin: 6px 0 0;
        font-size: 16px;
        font-weight: 600;
        color: var(--fg-2);
        letter-spacing: -0.01em;
    }

    .placeholder p {
        margin: 0;
        max-width: 320px;
        font-size: 13px;
    }
</style>
