<script lang="ts">
    import type { Snippet } from "svelte";

    type ResizableSidebar = "sidebar" | "connections";

    type Props = {
        titleBar?: Snippet;
        sidebar?: Snippet;
        editor?: Snippet;
        connections?: Snippet;
        connectionsOpen?: boolean;
    };

    const MIN_SIDEBAR_WIDTH = 220;
    const MAX_SIDEBAR_WIDTH = 420;
    const MIN_CONNECTIONS_WIDTH = 240;
    const MAX_CONNECTIONS_WIDTH = 520;
    const MIN_CONTENT_WIDTH = 360;
    const KEYBOARD_RESIZE_STEP = 12;

    let {
        titleBar,
        sidebar,
        editor,
        connections,
        connectionsOpen = true,
    }: Props = $props();

    let mainLayout: HTMLDivElement;
    let sidebarWidth = $state(280);
    let connectionsWidth = $state(320);
    let resizing = $state<ResizableSidebar | null>(null);

    const showConnections = $derived(Boolean(connections && connectionsOpen));
    const gridTemplateColumns = $derived(
        [
            sidebar ? `${sidebarWidth}px` : undefined,
            "minmax(0, 1fr)",
            showConnections ? `${connectionsWidth}px` : undefined,
        ]
            .filter(Boolean)
            .join(" "),
    );

    function clamp(value: number, min: number, max: number) {
        return Math.min(Math.max(value, min), max);
    }

    function maxWidthFor(kind: ResizableSidebar, rect: DOMRect) {
        if (kind === "sidebar") {
            const connectionsSpace = showConnections ? connectionsWidth : 0;
            return Math.max(
                MIN_SIDEBAR_WIDTH,
                Math.min(
                    MAX_SIDEBAR_WIDTH,
                    rect.width - connectionsSpace - MIN_CONTENT_WIDTH,
                ),
            );
        }

        const sidebarSpace = sidebar ? sidebarWidth : 0;
        return Math.max(
            MIN_CONNECTIONS_WIDTH,
            Math.min(
                MAX_CONNECTIONS_WIDTH,
                rect.width - sidebarSpace - MIN_CONTENT_WIDTH,
            ),
        );
    }

    function resizeSidebar(kind: ResizableSidebar, clientX: number) {
        if (!mainLayout) return;

        const rect = mainLayout.getBoundingClientRect();

        if (kind === "sidebar") {
            sidebarWidth = clamp(
                clientX - rect.left,
                MIN_SIDEBAR_WIDTH,
                maxWidthFor(kind, rect),
            );
            return;
        }

        connectionsWidth = clamp(
            rect.right - clientX,
            MIN_CONNECTIONS_WIDTH,
            maxWidthFor(kind, rect),
        );
    }

    function startResize(kind: ResizableSidebar, event: PointerEvent) {
        if (event.button !== 0) return;

        resizing = kind;
        event.preventDefault();
        resizeSidebar(kind, event.clientX);
    }

    function handlePointerMove(event: PointerEvent) {
        if (!resizing) return;

        event.preventDefault();
        resizeSidebar(resizing, event.clientX);
    }

    function stopResize() {
        resizing = null;
    }

    function handleResizeKeydown(
        kind: ResizableSidebar,
        event: KeyboardEvent,
    ) {
        if (event.key !== "ArrowLeft" && event.key !== "ArrowRight") return;
        event.preventDefault();

        const rect = mainLayout?.getBoundingClientRect();
        if (!rect) return;

        if (kind === "sidebar") {
            const delta =
                event.key === "ArrowRight"
                    ? KEYBOARD_RESIZE_STEP
                    : -KEYBOARD_RESIZE_STEP;
            sidebarWidth = clamp(
                sidebarWidth + delta,
                MIN_SIDEBAR_WIDTH,
                maxWidthFor(kind, rect),
            );
            return;
        }

        const delta =
            event.key === "ArrowLeft"
                ? KEYBOARD_RESIZE_STEP
                : -KEYBOARD_RESIZE_STEP;
        connectionsWidth = clamp(
            connectionsWidth + delta,
            MIN_CONNECTIONS_WIDTH,
            maxWidthFor(kind, rect),
        );
    }
</script>

<svelte:window
    onpointermove={handlePointerMove}
    onpointerup={stopResize}
    onpointercancel={stopResize}
/>

<div class="app-shell">
    {#if titleBar}
        <div class="app-shell-header">{@render titleBar()}</div>
    {/if}

    <div
        class="main-layout"
        bind:this={mainLayout}
        data-resizing={resizing ?? undefined}
        style:grid-template-columns={gridTemplateColumns}
    >
        {#if sidebar}
            <aside class="sidebar-shell">
                {@render sidebar()}
                <button
                    type="button"
                    class="resize-handle resize-handle--sidebar"
                    aria-label="Resize left sidebar"
                    onpointerdown={(event) => startResize("sidebar", event)}
                    onkeydown={(event) =>
                        handleResizeKeydown("sidebar", event)}
                ></button>
            </aside>
        {/if}

        <main class="content-shell">
            {@render editor?.()}
        </main>

        {#if connections && connectionsOpen}
            <aside class="connections-shell">
                <button
                    type="button"
                    class="resize-handle resize-handle--connections"
                    aria-label="Resize connections sidebar"
                    onpointerdown={(event) =>
                        startResize("connections", event)}
                    onkeydown={(event) =>
                        handleResizeKeydown("connections", event)}
                ></button>
                {@render connections()}
            </aside>
        {/if}
    </div>
</div>

<style>
    .app-shell {
        height: 100vh;
        min-height: 0;
        display: grid;
        grid-template-rows: var(--header-height) 1fr;
        background: var(--app-bg);
    }

    .app-shell-header {
        min-width: 0;
    }

    .main-layout {
        min-height: 0;
        display: grid;
        background: var(--app-bg);
    }

    .main-layout[data-resizing] {
        cursor: col-resize;
    }

    .main-layout[data-resizing] * {
        cursor: col-resize !important;
        user-select: none;
    }

    .sidebar-shell {
        position: relative;
        min-height: 0;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        background: var(--sidebar-bg);
        border-right: 1px solid var(--border-color);
    }

    .content-shell {
        min-width: 0;
        min-height: 0;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        background: var(--app-bg);
    }

    .connections-shell {
        position: relative;
        min-height: 0;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        background: var(--app-bg);
        border-left: 1px solid var(--border-color);
    }

    .resize-handle {
        position: absolute;
        top: 0;
        bottom: 0;
        z-index: 5;
        width: 10px;
        border: 0;
        padding: 0;
        background: transparent;
        cursor: col-resize;
        touch-action: none;
    }

    .resize-handle--sidebar {
        right: -5px;
    }

    .resize-handle--connections {
        left: -5px;
    }

    .resize-handle::after {
        content: "";
        position: absolute;
        top: 0;
        bottom: 0;
        left: 4px;
        width: 2px;
        background: transparent;
        transition: background 0.12s;
    }

    .resize-handle:hover::after,
    .resize-handle:focus-visible::after,
    .main-layout[data-resizing="sidebar"]
        .resize-handle--sidebar::after,
    .main-layout[data-resizing="connections"]
        .resize-handle--connections::after {
        background: var(--accent-line);
    }

    .resize-handle:focus-visible {
        outline: none;
    }
</style>
