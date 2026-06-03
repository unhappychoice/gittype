import type * as vscode from "vscode";
import type { PracticeSnippet } from "./models";

export const PRACTICE_SCHEME = "gittype-practice";

export class PracticeDocumentProvider implements vscode.TextDocumentContentProvider {
  private readonly snippets = new Map<string, PracticeSnippet>();

  public constructor(private readonly vscodeApi: typeof vscode) {}

  public createUri(snippet: PracticeSnippet): vscode.Uri {
    const safeTitle = snippet.title.replace(/[^a-zA-Z0-9._-]/g, "-");
    const uri = this.vscodeApi.Uri.from({
      scheme: PRACTICE_SCHEME,
      path: `/${safeTitle}-${snippet.id}`,
    });
    this.snippets.set(uri.toString(), snippet);
    return uri;
  }

  public provideTextDocumentContent(uri: vscode.Uri): string {
    return this.snippets.get(uri.toString())?.content ?? "";
  }

  public getSnippet(uri: vscode.Uri): PracticeSnippet | undefined {
    return this.snippets.get(uri.toString());
  }
}
