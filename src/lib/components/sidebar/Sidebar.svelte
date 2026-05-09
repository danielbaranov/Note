<script lang="ts">
    import Wordmark from "$lib/components/brand/Wordmark.svelte";
    import Icon from "$lib/components/ui/Icon.svelte";
    import IconButton from "$lib/components/ui/IconButton.svelte";
    import SidebarSection from "./SidebarSection.svelte";
    import SidebarItem from "./SidebarItem.svelte";
    import SidebarSearch from "./SidebarSearch.svelte";
    import SidebarUser from "./SidebarUser.svelte";
    import { displayTitleForObject, type ObjectRecord } from "$lib/objects";

    type Props = {
        objects: ObjectRecord[];
        currentObjectId: string | null;
        onSelectObject: (id: string) => void;
        onOpenCommand: () => void;
        onCreateObject: () => void;
    };

    let {
        objects,
        currentObjectId,
        onSelectObject,
        onOpenCommand,
        onCreateObject,
    }: Props = $props();
</script>

<div class="sidebar">
    <div class="sidebar-head">
        <Wordmark size={14} />
        <IconButton title="New note" size={22} onclick={onCreateObject}>
            <Icon name="plus" size={14} />
        </IconButton>
    </div>

    <div class="sidebar-search">
        <SidebarSearch onclick={onOpenCommand} />
    </div>

    <div class="sidebar-scroll">
        <SidebarSection title="Recent">
            {#if objects.length === 0}
                <p class="empty">No notes yet</p>
            {:else}
                {#each objects as object (object.id)}
                    <SidebarItem
                        name={displayTitleForObject(object)}
                        kind="note"
                        active={object.id === currentObjectId}
                        onSelect={() => onSelectObject(object.id)}
                    />
                {/each}
            {/if}
        </SidebarSection>
    </div>

    <SidebarUser />
</div>

<style>
    .sidebar {
        display: flex;
        flex-direction: column;
        flex: 1;
        min-height: 0;
    }

    .sidebar-head {
        padding: 16px 16px 12px;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .sidebar-search {
        padding: 0 8px 4px;
    }

    .sidebar-scroll {
        padding: 10px 8px 0;
        flex: 1;
        min-height: 0;
        overflow-y: auto;
    }

    .empty {
        margin: 6px 10px;
        font-size: 12px;
        color: var(--fg-4);
    }
</style>
