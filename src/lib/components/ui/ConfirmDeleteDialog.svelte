<script lang="ts">
    import { AlertDialog } from "bits-ui";

    type Props = {
        open: boolean;
        title: string;
        description: string;
        busy?: boolean;
        confirmLabel?: string;
        onConfirm: () => void | Promise<void>;
        onOpenChange?: (open: boolean) => void;
    };

    let {
        open = $bindable(false),
        title,
        description,
        busy = false,
        confirmLabel = "Delete",
        onConfirm,
        onOpenChange,
    }: Props = $props();

    function handleOpenChange(next: boolean) {
        open = next;
        onOpenChange?.(next);
    }
</script>

<AlertDialog.Root {open} onOpenChange={handleOpenChange}>
    <AlertDialog.Portal>
        <AlertDialog.Overlay class="delete-overlay" />
        <AlertDialog.Content class="delete-dialog">
            <AlertDialog.Title class="delete-title">{title}</AlertDialog.Title>
            <AlertDialog.Description class="delete-description">
                {description}
            </AlertDialog.Description>
            <div class="delete-actions">
                <AlertDialog.Cancel class="cancel-button" disabled={busy}>
                    Cancel
                </AlertDialog.Cancel>
                <AlertDialog.Action
                    class="delete-button"
                    disabled={busy}
                    onclick={() => void onConfirm()}
                >
                    {confirmLabel}
                </AlertDialog.Action>
            </div>
        </AlertDialog.Content>
    </AlertDialog.Portal>
</AlertDialog.Root>

<style>
    :global(.delete-overlay) {
        position: fixed;
        inset: 0;
        z-index: 50;
        background: rgba(20, 19, 17, 0.32);
        backdrop-filter: blur(6px);
    }

    :global(.delete-dialog) {
        position: fixed;
        top: 50%;
        left: 50%;
        z-index: 51;
        width: min(360px, calc(100vw - 32px));
        transform: translate(-50%, -50%);
        border-radius: var(--r-lg);
        background: var(--bg-elev);
        box-shadow: var(--shadow-lg);
        outline: none;
        padding: 18px;
    }

    :global(.delete-title) {
        margin: 0;
        color: var(--fg-1);
        font-size: 15px;
        font-weight: 600;
    }

    :global(.delete-description) {
        margin: 8px 0 0;
        color: var(--fg-2);
        font-size: 12.5px;
        line-height: 1.45;
    }

    :global(.delete-dialog .delete-actions) {
        display: flex;
        justify-content: flex-end;
        gap: 8px;
        margin-top: 18px;
    }

    :global(.cancel-button),
    :global(.delete-button) {
        height: 30px;
        border-radius: var(--r-sm);
        padding: 0 11px;
        font: inherit;
        font-size: 12px;
        cursor: pointer;
    }

    :global(.cancel-button) {
        border: 0;
        background: var(--bg-tint);
        color: var(--fg-2);
    }

    :global(.cancel-button:hover) {
        color: var(--fg-1);
        background: var(--bg-tint-2);
    }

    :global(.delete-button) {
        border: 0;
        background: var(--pdf-red);
        color: white;
        font-weight: 500;
    }

    :global(.cancel-button:disabled),
    :global(.delete-button:disabled) {
        cursor: default;
        opacity: 0.55;
    }
</style>
