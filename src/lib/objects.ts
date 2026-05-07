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

export function searchObjects(query: string): Promise<ObjectRecord[]> {
    return invoke<ObjectRecord[]>("search_objects", { query });
}

export function loadObject(id: string): Promise<ObjectRecord> {
    return invoke<ObjectRecord>("load_object", { id });
}

export function createMarkdownObject(title: string): Promise<ObjectRecord> {
    return invoke<ObjectRecord>("create_markdown_object", { title });
}

export function saveObjectContent(
    id: string,
    contentText: string,
): Promise<void> {
    return invoke("save_object_content", { id, contentText });
}

export function displayTitleForObject(object: ObjectRecord): string {
    return (
        firstMarkdownHeading(object.contentText) ??
        (object.title || "Untitled")
    );
}

export function firstMarkdownHeading(markdown: string): string | null {
    const match = markdown.match(/^#(?!#)\s+(.+?)\s*#*\s*$/m);
    const title = match?.[1]?.trim();

    return title ? title : null;
}
