import { invoke } from "@tauri-apps/api/core";

export type ObjectConnectionRecord = {
    id: string;
    sourceObjectId: string;
    targetObjectId: string;
    contentText: string;
    createdAtMs: number;
    updatedAtMs: number;
};

export function listObjectConnections(
    objectId: string,
): Promise<ObjectConnectionRecord[]> {
    return invoke<ObjectConnectionRecord[]>("list_object_connections", {
        objectId,
    });
}

export function listAllConnections(): Promise<ObjectConnectionRecord[]> {
    return invoke<ObjectConnectionRecord[]>("list_all_connections");
}

export function createObjectConnection(
    sourceObjectId: string,
    targetObjectId: string,
    contentText: string,
): Promise<ObjectConnectionRecord> {
    return invoke<ObjectConnectionRecord>("create_object_connection", {
        sourceObjectId,
        targetObjectId,
        contentText,
    });
}

export function deleteObjectConnection(id: string): Promise<void> {
    return invoke<void>("delete_object_connection", { id });
}

export function updateObjectConnectionContent(
    id: string,
    contentText: string,
): Promise<ObjectConnectionRecord> {
    return invoke<ObjectConnectionRecord>("update_object_connection_content", {
        id,
        contentText,
    });
}

export type ConnectionDirection = "outgoing" | "incoming";

export function otherObjectIdOf(
    connection: ObjectConnectionRecord,
    currentObjectId: string,
): string {
    return connection.sourceObjectId === currentObjectId
        ? connection.targetObjectId
        : connection.sourceObjectId;
}

export function connectionDirectionOf(
    connection: ObjectConnectionRecord,
    currentObjectId: string,
): ConnectionDirection {
    return connection.sourceObjectId === currentObjectId
        ? "outgoing"
        : "incoming";
}
