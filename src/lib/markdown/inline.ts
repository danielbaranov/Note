// Tiny inline-only markdown tokenizer (bold / italic / code). Used by the
// connections panel where labels are short prose annotations. Block markdown
// (headings, lists, code blocks) is intentionally out of scope — for full
// rendering, use a real markdown library.

export type InlineToken =
    | { type: "text"; value: string }
    | { type: "bold"; value: string }
    | { type: "italic"; value: string }
    | { type: "code"; value: string };

const TOKEN_RE = /(\*\*[^*]+\*\*|\*[^*]+\*|`[^`]+`)/g;

export function tokenizeInline(source: string): InlineToken[] {
    const tokens: InlineToken[] = [];
    let lastIndex = 0;
    let match: RegExpExecArray | null;

    TOKEN_RE.lastIndex = 0;
    while ((match = TOKEN_RE.exec(source)) !== null) {
        if (match.index > lastIndex) {
            tokens.push({
                type: "text",
                value: source.slice(lastIndex, match.index),
            });
        }

        const raw = match[0];
        if (raw.startsWith("**")) {
            tokens.push({ type: "bold", value: raw.slice(2, -2) });
        } else if (raw.startsWith("`")) {
            tokens.push({ type: "code", value: raw.slice(1, -1) });
        } else {
            tokens.push({ type: "italic", value: raw.slice(1, -1) });
        }

        lastIndex = match.index + raw.length;
    }

    if (lastIndex < source.length) {
        tokens.push({ type: "text", value: source.slice(lastIndex) });
    }

    return tokens;
}
