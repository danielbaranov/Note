// Connection — a labelled edge between two objects. The backend does not
// persist these yet; the type lives here so the panel components can be
// strongly typed and the wire-up is a small change once the storage lands.

export type ConnectionKind = "note" | "pdf" | "web";

export type Connection = {
    id: string;
    kind: ConnectionKind;
    title: string;
    meta?: string;
    label: string;
};
