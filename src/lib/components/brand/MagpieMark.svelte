<script lang="ts">
    type Props = {
        size?: number;
        label?: string;
    };

    let { size = 22, label = "Magpie" }: Props = $props();
</script>

<img
    src="/magpie.svg"
    alt={label}
    width={size}
    height={size}
    class="mark"
/>

<style>
    .mark {
        display: block;
        object-fit: contain;
    }
</style>
