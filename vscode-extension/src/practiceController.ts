import type * as vscode from "vscode";
import { PRACTICE_MODES } from "./models";
import type { PracticeMode, PracticeRange, PracticeSnippet, TextEditorLike } from "./models";
import { PracticeDecorations } from "./practiceDecorations";
import { PracticeDocumentProvider, PRACTICE_SCHEME } from "./practiceDocumentProvider";
import { PracticeSession } from "./practiceSession";
import { SelectionSnippetProvider } from "./selectionSnippetProvider";

type ActiveSession = {
  readonly uri: string;
  readonly session: PracticeSession;
};

export class PracticeController implements vscode.Disposable {
  private readonly disposables: vscode.Disposable[] = [];
  private readonly documentProvider: PracticeDocumentProvider;
  private readonly selectionProvider = new SelectionSnippetProvider();
  private readonly decorations: PracticeDecorations;
  private readonly sessions = new Map<string, ActiveSession>();
  private readonly completedNotifications = new Set<string>();
  private readonly statusBar: vscode.StatusBarItem;

  public constructor(private readonly vscodeApi: typeof vscode) {
    this.documentProvider = new PracticeDocumentProvider(vscodeApi);
    this.decorations = new PracticeDecorations(vscodeApi);
    this.statusBar = vscodeApi.window.createStatusBarItem(vscodeApi.StatusBarAlignment.Left, 100);
    this.statusBar.command = "gittype.togglePracticeMode";
  }

  public register(context: vscode.ExtensionContext): void {
    this.disposables.push(
      this.vscodeApi.workspace.registerTextDocumentContentProvider(PRACTICE_SCHEME, this.documentProvider),
      this.vscodeApi.commands.registerCommand("gittype.practiceSelection", () => this.practiceSelection()),
      this.vscodeApi.commands.registerCommand("gittype.restartPractice", () => this.restartPractice()),
      this.vscodeApi.commands.registerCommand("gittype.togglePracticeMode", () => this.togglePracticeMode()),
      this.vscodeApi.commands.registerCommand("type", (args: { readonly text?: string }) => this.handleType(args)),
      this.vscodeApi.commands.registerCommand("deleteLeft", () => this.handleBackspace()),
      this.vscodeApi.window.onDidChangeActiveTextEditor((editor) => this.refreshForEditor(editor)),
      this.decorations,
      this.statusBar,
    );
    context.subscriptions.push(this);
  }

  public dispose(): void {
    for (const disposable of this.disposables) {
      disposable.dispose();
    }
  }

  private async practiceSelection(): Promise<void> {
    const activeEditor = this.createEditorAdapter(this.vscodeApi.window.activeTextEditor);
    const snippets = await this.selectionProvider.listSnippets(
      activeEditor === undefined ? {} : { activeEditor },
    );
    const snippet = snippets[0];
    if (snippet === undefined) {
      this.vscodeApi.window.showErrorMessage("Select code before starting GitType practice.");
      return;
    }

    await this.openSnippet(snippet);
  }

  private async openSnippet(snippet: PracticeSnippet): Promise<void> {
    const uri = this.documentProvider.createUri(snippet);
    const document = await this.vscodeApi.workspace.openTextDocument(uri);
    const editor = await this.vscodeApi.window.showTextDocument(document, { preview: false });
    const session = new PracticeSession(snippet.content, this.readMode());
    this.sessions.set(uri.toString(), { uri: uri.toString(), session });
    this.completedNotifications.delete(uri.toString());
    await this.vscodeApi.languages.setTextDocumentLanguage(document, snippet.languageId || "plaintext");
    this.applySession(editor, session);
  }

  private async restartPractice(): Promise<void> {
    const active = this.activeSession();
    if (active === undefined) {
      this.vscodeApi.window.showErrorMessage("Open a GitType practice document first.");
      return;
    }

    active.session.restart();
    this.completedNotifications.delete(active.uri);
    this.applySession(this.vscodeApi.window.activeTextEditor, active.session);
  }

  private async togglePracticeMode(): Promise<void> {
    const active = this.activeSession();
    if (active === undefined) {
      this.vscodeApi.window.showErrorMessage("Open a GitType practice document first.");
      return;
    }

    active.session.toggleMode();
    this.applySession(this.vscodeApi.window.activeTextEditor, active.session);
  }

  private async handleType(args: { readonly text?: string }): Promise<void> {
    const active = this.activeSession();
    if (active === undefined) {
      await this.vscodeApi.commands.executeCommand("default:type", args);
      return;
    }

    active.session.handleTextInput(args.text ?? "");
    this.applySession(this.vscodeApi.window.activeTextEditor, active.session);
    this.showCompletionIfNeeded(active);
  }

  private async handleBackspace(): Promise<void> {
    const active = this.activeSession();
    if (active === undefined) {
      await this.vscodeApi.commands.executeCommand("default:deleteLeft");
      return;
    }

    active.session.handleBackspace();
    this.applySession(this.vscodeApi.window.activeTextEditor, active.session);
  }

  private applySession(editor: vscode.TextEditor | undefined, session: PracticeSession): void {
    if (editor === undefined || editor.document.uri.scheme !== PRACTICE_SCHEME) {
      this.statusBar.hide();
      return;
    }

    const snapshot = session.snapshot();
    this.decorations.apply(editor, snapshot);
    if (this.showStatusBar()) {
      this.statusBar.text = `GitType ${snapshot.mode} | ${snapshot.wpm} WPM | ${snapshot.accuracy}% | ${snapshot.currentLine}/${snapshot.totalLines}`;
      this.statusBar.show();
    }
  }

  private refreshForEditor(editor: vscode.TextEditor | undefined): void {
    const session = this.sessionForEditor(editor);
    if (session === undefined) {
      this.statusBar.hide();
      return;
    }
    this.applySession(editor, session);
  }

  private showCompletionIfNeeded(active: ActiveSession): void {
    const snapshot = active.session.snapshot();
    if (!snapshot.completed || this.completedNotifications.has(active.uri)) {
      return;
    }

    this.completedNotifications.add(active.uri);
    this.vscodeApi.window.showInformationMessage(`GitType complete: ${snapshot.wpm} WPM, ${snapshot.accuracy}% accuracy.`);
  }

  private activeSession(): ActiveSession | undefined {
    const editor = this.vscodeApi.window.activeTextEditor;
    if (editor === undefined || editor.document.uri.scheme !== PRACTICE_SCHEME) {
      return undefined;
    }
    return this.sessions.get(editor.document.uri.toString());
  }

  private sessionForEditor(editor: vscode.TextEditor | undefined): PracticeSession | undefined {
    if (editor === undefined || editor.document.uri.scheme !== PRACTICE_SCHEME) {
      return undefined;
    }
    return this.sessions.get(editor.document.uri.toString())?.session;
  }

  private createEditorAdapter(editor: vscode.TextEditor | undefined): TextEditorLike | undefined {
    if (editor === undefined) {
      return undefined;
    }

    return {
      document: {
        uri: { toString: () => editor.document.uri.toString() },
        fileName: editor.document.fileName,
        languageId: editor.document.languageId,
        getText: () => editor.document.getText(editor.selection),
      },
      selection: this.toPracticeRange(editor.selection),
    };
  }

  private toPracticeRange(range: vscode.Range & { readonly isEmpty: boolean }): PracticeRange & { readonly isEmpty: boolean } {
    return {
      start: { line: range.start.line, character: range.start.character },
      end: { line: range.end.line, character: range.end.character },
      isEmpty: range.isEmpty,
    };
  }

  private readMode(): PracticeMode {
    const configured = this.vscodeApi.workspace.getConfiguration("gittype").get<PracticeMode>("practice.mode");
    return configured === PRACTICE_MODES.flow ? PRACTICE_MODES.flow : PRACTICE_MODES.strict;
  }

  private showStatusBar(): boolean {
    return this.vscodeApi.workspace.getConfiguration("gittype").get<boolean>("practice.showStatusBar", true);
  }
}
