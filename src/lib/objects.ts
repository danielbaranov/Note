import { invoke } from "@tauri-apps/api/core";

export type ObjectRecord = {
    id: string;
    objectType: string;
    title: string;
    contentText: string;
    metadataJson: string;
    sourceUri: string | null;
    createdAtMs: number;
    updatedAtMs: number;
};

export type ObjectUpdate = {
    id: string;
    objectType?: string;
    title?: string;
    contentText?: string;
    metadataJson?: string;
    sourceUri?: string | null;
    createdAtMs?: number;
    updatedAtMs?: number;
};

export function searchObjects(query: string): Promise<ObjectRecord[]> {
    return invoke<ObjectRecord[]>("search_objects", { query });
}

export function loadObject(id: string): Promise<ObjectRecord> {
    return invoke<ObjectRecord>("load_object", { id });
}

export function createMarkdownObject(
    contentText: string,
): Promise<ObjectRecord> {
    return invoke<ObjectRecord>("create_markdown_object", { contentText });
}

export function updateObject(update: ObjectUpdate): Promise<ObjectRecord> {
    return invoke<ObjectRecord>("update_object", { update });
}

export function deleteObject(id: string): Promise<void> {
    return invoke<void>("delete_object", { id });
}

export function saveObjectContent(
    id: string,
    contentText: string,
): Promise<ObjectRecord> {
    return updateObject({ id, contentText });
}

export function displayTitleForObject(object: ObjectRecord): string {
    return object.title || "Untitled";
}

export function markdownTitleOrDefault(markdown: string): string {
    return markdownTitle(markdown) ?? "Untitled";
}

export function markdownTitle(markdown: string): string | null {
    for (const line of markdown.split(/\r?\n/)) {
        const title = markdownHeadingTitle(line);

        if (title) {
            return title;
        }
    }

    return null;
}

function markdownHeadingTitle(line: string): string | null {
    const trimmedStart = line.trimStart();

    if (!trimmedStart.startsWith("#") || trimmedStart.startsWith("##")) {
        return null;
    }

    const rest = trimmedStart.slice(1);
    if (!rest[0] || !/\s/.test(rest[0])) {
        return null;
    }

    const title = stripClosingHeadingMarker(rest.trim()).trim();

    return title ? title : null;
}

function stripClosingHeadingMarker(title: string): string {
    const trimmed = title.trimEnd();
    const withoutHashes = trimmed.replace(/#+$/, "");

    if (withoutHashes.length !== trimmed.length && /\s$/.test(withoutHashes)) {
        return withoutHashes.trimEnd();
    }

    return trimmed;
}
