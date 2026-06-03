import assert from "node:assert/strict";
import { describe, it } from "node:test";
import type { PracticeRange, TextEditorLike } from "../models";
import { SelectionSnippetProvider } from "../selectionSnippetProvider";

const selectedRange: PracticeRange & { readonly isEmpty: boolean } = {
  start: { line: 2, character: 4 },
  end: { line: 4, character: 1 },
  isEmpty: false,
};

function makeEditor(content: string, selection = selectedRange): TextEditorLike {
  return {
    document: {
      uri: { toString: () => "file:///repo/src/main.rs" },
      fileName: "D:\\repo\\src\\main.rs",
      languageId: "rust",
      getText: () => content,
    },
    selection,
  };
}

describe("SelectionSnippetProvider", () => {
  it("returns no snippets when there is no active editor", async () => {
    const provider = new SelectionSnippetProvider();

    const snippets = await provider.listSnippets({});

    assert.deepEqual(snippets, []);
  });

  it("returns no snippets when the selection is empty", async () => {
    const provider = new SelectionSnippetProvider();
    const editor = makeEditor("fn main() {}", {
      start: { line: 0, character: 0 },
      end: { line: 0, character: 0 },
      isEmpty: true,
    });

    const snippets = await provider.listSnippets({ activeEditor: editor });

    assert.deepEqual(snippets, []);
  });

  it("maps selected text into a practice snippet", async () => {
    const provider = new SelectionSnippetProvider();

    const snippets = await provider.listSnippets({ activeEditor: makeEditor("fn main() {}") });

    assert.equal(snippets.length, 1);
    const snippet = snippets[0];
    assert.equal(snippet?.title, "selected-main.rs");
    assert.equal(snippet?.languageId, "rust");
    assert.equal(snippet?.sourceUri, "file:///repo/src/main.rs");
    assert.deepEqual(snippet?.sourceRange, selectedRange);
    assert.equal(snippet?.content, "fn main() {}");
  });
});
