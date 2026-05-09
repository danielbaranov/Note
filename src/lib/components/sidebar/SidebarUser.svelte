<script lang="ts">
    type Props = {
        name?: string;
        initials?: string;
    };

    let { name = "Local", initials = "L" }: Props = $props();
</script>

<footer class="user">
    <span class="avatar" aria-hidden="true">{initials}</span>
    <span class="name">{name}</span>
</footer>

<style>
    .user {
        padding: 10px 14px;
        border-top: 1px solid var(--line-1);
        font-size: 11px;
        color: var(--fg-3);
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .avatar {
        width: 22px;
        height: 22px;
        border-radius: 50%;
        background: linear-gradient(135deg, #c9c4ba, #8a8884);
        display: inline-flex;
        align-items: center;
        justify-content: center;
        color: #fff;
        font-size: 10px;
        font-weight: 600;
        flex: 0 0 22px;
    }

    .name {
        color: var(--fg-2);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
</style>
