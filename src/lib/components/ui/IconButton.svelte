<script lang="ts">
    import type { Snippet } from "svelte";

    type Props = {
        title?: string;
        size?: number;
        pressed?: boolean;
        onclick?: (event: MouseEvent) => void;
        children: Snippet;
    };

    let { title, size = 28, pressed, onclick, children }: Props = $props();
</script>

<button
    type="button"
    class="mp-iconbtn"
    style="width: {size}px; height: {size}px"
    {title}
    aria-label={title}
    aria-pressed={pressed ?? undefined}
    data-active={pressed ? "true" : undefined}
    {onclick}
>
    {@render children()}
</button>
