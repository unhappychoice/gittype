export const PRACTICE_MODES = {
  strict: "strict",
  flow: "flow",
} as const;

export type PracticeMode = (typeof PRACTICE_MODES)[keyof typeof PRACTICE_MODES];

export type PracticePosition = {
  readonly line: number;
  readonly character: number;
};

export type PracticeRange = {
  readonly start: PracticePosition;
  readonly end: PracticePosition;
};

export type TextDocumentLike = {
  readonly uri: { readonly toString: () => string };
  readonly fileName: string;
  readonly languageId: string;
  readonly getText: (range?: PracticeRange) => string;
};

export type TextEditorLike = {
  readonly document: TextDocumentLike;
  readonly selection: PracticeRange & { readonly isEmpty: boolean };
};

export type SnippetContext = {
  readonly activeEditor?: TextEditorLike;
};

export type PracticeSnippet = {
  readonly id: string;
  readonly title: string;
  readonly languageId: string;
  readonly sourceUri?: string;
  readonly sourceRange?: PracticeRange;
  readonly content: string;
  readonly metadata?: Readonly<Record<string, string | number | boolean>>;
};

export interface SnippetProvider {
  listSnippets(context: SnippetContext): Promise<readonly PracticeSnippet[]>;
}

export type TypedResult = {
  readonly index: number;
  readonly expected: string;
  readonly actual: string;
  readonly correct: boolean;
};

export type PracticeSnapshot = {
  readonly target: string;
  readonly mode: PracticeMode;
  readonly currentIndex: number;
  readonly typed: readonly TypedResult[];
  readonly mistakes: readonly TypedResult[];
  readonly completed: boolean;
  readonly startedAtMs: number | undefined;
  readonly completedAtMs: number | undefined;
  readonly elapsedMs: number;
  readonly wpm: number;
  readonly accuracy: number;
  readonly correctCount: number;
  readonly errorCount: number;
  readonly currentLine: number;
  readonly totalLines: number;
};
