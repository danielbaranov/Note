import { RangeSetBuilder } from "@codemirror/state";
import { syntaxTree } from "@codemirror/language";
import {
  Decoration,
  EditorView,
  ViewPlugin,
  type DecorationSet,
  type ViewUpdate,
} from "@codemirror/view";

const h1Line = Decoration.line({ class: "cm-md-h1" });
const h2Line = Decoration.line({ class: "cm-md-h2" });
const boldMark = Decoration.mark({ class: "cm-md-bold" });
const italicMark = Decoration.mark({ class: "cm-md-italic" });
const hiddenSyntax = Decoration.replace({});

type PendingDecoration = {
  from: number;
  to: number;
  decoration: Decoration;
};

function cursorIsInside(view: EditorView, from: number, to: number): boolean {
  return view.state.selection.ranges.some((range) => {
    return range.from >= from && range.from <= to;
  });
}

function buildMarkdownDecorations(view: EditorView): DecorationSet {
  const pending: PendingDecoration[] = [];
  const tree = syntaxTree(view.state);

  tree.iterate({
    enter(node) {
      const cursorInsideNode = cursorIsInside(view, node.from, node.to);

      if (node.name === "ATXHeading1") {
        pending.push({
          from: node.from,
          to: node.from,
          decoration: h1Line,
        });
      }

      if (node.name === "ATXHeading2") {
        pending.push({
          from: node.from,
          to: node.from,
          decoration: h2Line,
        });
      }

      if (node.name === "StrongEmphasis") {
        pending.push({
          from: node.from,
          to: node.to,
          decoration: boldMark,
        });
      }

      if (node.name === "Emphasis") {
        pending.push({
          from: node.from,
          to: node.to,
          decoration: italicMark,
        });
      }

      if (node.name === "HeaderMark" && !cursorInsideNode) {
        pending.push({
          from: node.from,
          to: node.to,
          decoration: hiddenSyntax,
        });
      }

      if (node.name === "EmphasisMark" && !cursorInsideNode) {
        pending.push({
          from: node.from,
          to: node.to,
          decoration: hiddenSyntax,
        });
      }
    },
  });

  pending.sort((a, b) => {
    if (a.from !== b.from) return a.from - b.from;
    return a.to - b.to;
  });

  const builder = new RangeSetBuilder<Decoration>();

  for (const item of pending) {
    builder.add(item.from, item.to, item.decoration);
  }

  return builder.finish();
}

export const markdownDecorations = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
      this.decorations = buildMarkdownDecorations(view);
    }

    update(update: ViewUpdate) {
      if (update.docChanged || update.viewportChanged || update.selectionSet) {
        this.decorations = buildMarkdownDecorations(update.view);
      }
    }
  },
  {
    decorations: (plugin) => plugin.decorations,
  },
);
