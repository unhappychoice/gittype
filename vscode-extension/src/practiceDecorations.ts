import type * as vscode from "vscode";
import type { PracticePosition, PracticeRange, PracticeSnapshot, TypedResult } from "./models";

export type PracticeDecorationRanges = {
  readonly correct: readonly PracticeRange[];
  readonly current: readonly PracticeRange[];
  readonly wrong: readonly PracticeRange[];
  readonly pending: readonly PracticeRange[];
};

export function buildDecorationRanges(snapshot: PracticeSnapshot): PracticeDecorationRanges {
  const mapper = new OffsetMapper(snapshot.target);
  const correct = snapshot.typed
    .filter((result) => result.correct)
    .map((result) => mapper.rangeForIndex(result.index));
  const wrong = snapshot.mistakes.map((result) => mapper.rangeForIndex(result.index));
  const current = snapshot.completed ? [] : [mapper.rangeForIndex(snapshot.currentIndex)];
  const pendingStart = snapshot.completed ? snapshot.target.length : snapshot.currentIndex + 1;
  const pending = pendingStart < snapshot.target.length ? [mapper.rangeForOffsets(pendingStart, snapshot.target.length)] : [];

  return { correct, current, wrong, pending };
}

export class PracticeDecorations implements vscode.Disposable {
  private readonly correctType: vscode.TextEditorDecorationType;
  private readonly currentType: vscode.TextEditorDecorationType;
  private readonly wrongType: vscode.TextEditorDecorationType;
  private readonly pendingType: vscode.TextEditorDecorationType;

  public constructor(private readonly vscodeApi: typeof vscode) {
    this.correctType = vscodeApi.window.createTextEditorDecorationType({
      backgroundColor: "rgba(46, 160, 67, 0.28)",
      color: new vscodeApi.ThemeColor("editor.foreground"),
    });
    this.currentType = vscodeApi.window.createTextEditorDecorationType({
      border: "1px solid",
      borderColor: new vscodeApi.ThemeColor("editorCursor.foreground"),
      backgroundColor: "rgba(255, 214, 10, 0.18)",
    });
    this.wrongType = vscodeApi.window.createTextEditorDecorationType({
      backgroundColor: "rgba(248, 81, 73, 0.35)",
      color: new vscodeApi.ThemeColor("editorError.foreground"),
    });
    this.pendingType = vscodeApi.window.createTextEditorDecorationType({
      opacity: "0.55",
    });
  }

  public apply(editor: vscode.TextEditor, snapshot: PracticeSnapshot): void {
    const ranges = buildDecorationRanges(snapshot);
    editor.setDecorations(this.correctType, this.toVsCodeRanges(ranges.correct));
    editor.setDecorations(this.currentType, this.toVsCodeRanges(ranges.current));
    editor.setDecorations(this.wrongType, this.toVsCodeRanges(ranges.wrong));
    editor.setDecorations(this.pendingType, this.toVsCodeRanges(ranges.pending));
  }

  public dispose(): void {
    this.correctType.dispose();
    this.currentType.dispose();
    this.wrongType.dispose();
    this.pendingType.dispose();
  }

  private toVsCodeRanges(ranges: readonly PracticeRange[]): vscode.Range[] {
    return ranges.map((range) => new this.vscodeApi.Range(
      range.start.line,
      range.start.character,
      range.end.line,
      range.end.character,
    ));
  }
}

class OffsetMapper {
  private readonly lineStarts: readonly number[];

  public constructor(private readonly text: string) {
    this.lineStarts = this.buildLineStarts(text);
  }

  public rangeForIndex(index: number): PracticeRange {
    return this.rangeForOffsets(index, Math.min(index + 1, this.text.length));
  }

  public rangeForOffsets(start: number, end: number): PracticeRange {
    return {
      start: this.positionForOffset(start),
      end: this.positionForOffset(end),
    };
  }

  private positionForOffset(offset: number): PracticePosition {
    const normalizedOffset = Math.max(0, Math.min(offset, this.text.length));
    const line = this.findLine(normalizedOffset);
    const lineStart = this.lineStarts[line] ?? 0;
    return {
      line,
      character: normalizedOffset - lineStart,
    };
  }

  private findLine(offset: number): number {
    let line = 0;
    for (const start of this.lineStarts) {
      if (start > offset) {
        return Math.max(0, line - 1);
      }
      line += 1;
    }
    return Math.max(0, this.lineStarts.length - 1);
  }

  private buildLineStarts(text: string): readonly number[] {
    const starts = [0];
    for (let index = 0; index < text.length; index += 1) {
      if (text[index] === "\n") {
        starts.push(index + 1);
      }
    }
    return starts;
  }
}
