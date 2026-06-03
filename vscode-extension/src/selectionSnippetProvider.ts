import type { PracticeSnippet, SnippetContext, SnippetProvider, TextEditorLike } from "./models";

export class SelectionSnippetProvider implements SnippetProvider {
  public async listSnippets(context: SnippetContext): Promise<readonly PracticeSnippet[]> {
    const editor = context.activeEditor;
    if (editor === undefined || editor.selection.isEmpty) {
      return [];
    }

    const content = editor.document.getText(editor.selection);
    if (content.trim().length === 0) {
      return [];
    }

    return [this.createSnippet(editor, content)];
  }

  private createSnippet(editor: TextEditorLike, content: string): PracticeSnippet {
    return {
      id: this.createId(editor, content),
      title: this.createTitle(editor),
      languageId: editor.document.languageId || "plaintext",
      sourceUri: editor.document.uri.toString(),
      sourceRange: editor.selection,
      content,
    };
  }

  private createId(editor: TextEditorLike, content: string): string {
    const source = `${editor.document.uri.toString()}:${editor.selection.start.line}:${editor.selection.start.character}:${content}`;
    return Buffer.from(source).toString("base64url");
  }

  private createTitle(editor: TextEditorLike): string {
    const filename = editor.document.fileName.split(/[\\/]/).at(-1) ?? "selection";
    return `selected-${filename}`;
  }
}
