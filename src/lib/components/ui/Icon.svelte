<script lang="ts" module>
    export type IconName =
        | "search"
        | "graph"
        | "link"
        | "plus"
        | "more"
        | "doc"
        | "pdf"
        | "web"
        | "folder"
        | "star"
        | "arrow"
        | "pin"
        | "expand"
        | "close"
        | "comment"
        | "send"
        | "hash";
</script>

<script lang="ts">
    type Props = {
        name: IconName;
        size?: number;
    };

    let { name, size = 16 }: Props = $props();
</script>

<svg
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.6"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
>
    {#if name === "search"}
        <circle cx="11" cy="11" r="6.5" />
        <path d="m20 20-4.3-4.3" />
    {:else if name === "graph"}
        <circle cx="6" cy="6" r="2.4" />
        <circle cx="18" cy="7" r="2.4" />
        <circle cx="12" cy="17" r="2.4" />
        <path d="M7.7 7.4 10.6 15M16.6 8.6 13.5 15.5M8 6.7h7.7" />
    {:else if name === "link"}
        <path d="M10 14a4 4 0 0 1 0-5.7l3-3a4 4 0 0 1 5.7 5.7l-1.5 1.5" />
        <path d="M14 10a4 4 0 0 1 0 5.7l-3 3a4 4 0 1 1-5.7-5.7L6.8 11.5" />
    {:else if name === "plus"}
        <path d="M12 5v14M5 12h14" />
    {:else if name === "more"}
        <g fill="currentColor" stroke="none">
            <circle cx="6" cy="12" r="1.6" />
            <circle cx="12" cy="12" r="1.6" />
            <circle cx="18" cy="12" r="1.6" />
        </g>
    {:else if name === "doc"}
        <path d="M7 4h7l4 4v12a1 1 0 0 1-1 1H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1z" />
        <path d="M14 4v4h4" />
    {:else if name === "pdf"}
        <path d="M7 4h7l4 4v12a1 1 0 0 1-1 1H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1z" />
        <path d="M14 4v4h4" />
        <path
            d="M9 13h1.2a1.4 1.4 0 0 1 0 2.8H9V13zm0 2.8V18M14 13v5h.8a1.6 1.6 0 0 0 1.6-1.6v-1.8a1.6 1.6 0 0 0-1.6-1.6H14z"
            stroke-width="1.2"
        />
    {:else if name === "web"}
        <circle cx="12" cy="12" r="8" />
        <path d="M4 12h16M12 4a12 12 0 0 1 0 16M12 4a12 12 0 0 0 0 16" />
    {:else if name === "folder"}
        <path
            d="M4 7a1 1 0 0 1 1-1h4l2 2h8a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7z"
        />
    {:else if name === "star"}
        <path
            d="M12 5l2.4 4.9 5.4.8-3.9 3.8.9 5.4L12 17.4 7.2 19.9l.9-5.4-3.9-3.8 5.4-.8z"
        />
    {:else if name === "arrow"}
        <path d="M7 17 17 7M9 7h8v8" />
    {:else if name === "pin"}
        <path d="M9 4h6l-1 5 3 3v2h-5v6l-1 1-1-1v-6H5v-2l3-3z" />
    {:else if name === "expand"}
        <path d="M5 9V5h4M19 9V5h-4M5 15v4h4M19 15v4h-4" />
    {:else if name === "close"}
        <path d="M6 6l12 12M18 6 6 18" />
    {:else if name === "comment"}
        <path
            d="M5 5h14a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1h-7l-4 3v-3H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z"
        />
    {:else if name === "send"}
        <path d="M5 12 19 5l-3 14-4-6-7-1z" />
    {:else if name === "hash"}
        <path d="M5 9h14M5 15h14M10 4 8 20M16 4l-2 16" />
    {/if}
</svg>
