import { PRACTICE_MODES } from "./models";
import type { PracticeMode, PracticeSnapshot, TypedResult } from "./models";

export type PracticeClock = {
  readonly now: () => number;
};

const MILLIS_PER_MINUTE = 60_000;
const WORD_SIZE = 5;

export class PracticeSession {
  private readonly target: string;
  private readonly clock: PracticeClock;
  private mode: PracticeMode;
  private currentIndex = 0;
  private readonly typed: TypedResult[] = [];
  private readonly mistakes: TypedResult[] = [];
  private startedAtMs: number | undefined;
  private completedAtMs: number | undefined;

  public constructor(target: string, mode: PracticeMode, clock: PracticeClock = { now: Date.now }) {
    this.target = target;
    this.mode = mode;
    this.clock = clock;
  }

  public handleTextInput(text: string): PracticeSnapshot {
    for (const char of text) {
      this.handleCharacter(char);
    }

    return this.snapshot();
  }

  public handleBackspace(): PracticeSnapshot {
    if (this.typed.length === 0) {
      return this.snapshot();
    }

    const latest = this.typed.pop();
    if (latest === undefined) {
      return this.snapshot();
    }

    this.removeLatestMistake(latest);
    this.currentIndex = latest.index;
    this.completedAtMs = undefined;
    return this.snapshot();
  }

  public restart(): PracticeSnapshot {
    this.currentIndex = 0;
    this.typed.length = 0;
    this.mistakes.length = 0;
    this.startedAtMs = undefined;
    this.completedAtMs = undefined;
    return this.snapshot();
  }

  public toggleMode(): PracticeSnapshot {
    this.mode = this.mode === PRACTICE_MODES.strict ? PRACTICE_MODES.flow : PRACTICE_MODES.strict;
    return this.snapshot();
  }

  public setMode(mode: PracticeMode): PracticeSnapshot {
    this.mode = mode;
    return this.snapshot();
  }

  public snapshot(): PracticeSnapshot {
    const now = this.completedAtMs ?? this.clock.now();
    const elapsedMs = this.startedAtMs === undefined ? 0 : Math.max(0, now - this.startedAtMs);
    const correctCount = this.typed.filter((result) => result.correct).length;
    const errorCount = this.typed.filter((result) => !result.correct).length;
    const typedCount = this.typed.length;
    const accuracy = typedCount === 0 ? 100 : Math.round((correctCount / typedCount) * 100);

    return {
      target: this.target,
      mode: this.mode,
      currentIndex: this.currentIndex,
      typed: [...this.typed],
      mistakes: [...this.mistakes],
      completed: this.currentIndex >= this.target.length && this.target.length > 0,
      startedAtMs: this.startedAtMs,
      completedAtMs: this.completedAtMs,
      elapsedMs,
      wpm: this.calculateWpm(correctCount, elapsedMs),
      accuracy,
      correctCount,
      errorCount,
      currentLine: this.calculateCurrentLine(),
      totalLines: this.calculateTotalLines(),
    };
  }

  private handleCharacter(actual: string): void {
    if (this.currentIndex >= this.target.length) {
      return;
    }

    this.startedAtMs ??= this.clock.now();

    const expected = this.target[this.currentIndex];
    if (expected === undefined) {
      return;
    }

    const result: TypedResult = {
      index: this.currentIndex,
      expected,
      actual,
      correct: actual === expected,
    };

    this.typed.push(result);
    this.removeVisibleMistakeAt(result.index);
    if (!result.correct) {
      this.mistakes.push(result);
    }

    if (result.correct || this.mode === PRACTICE_MODES.flow) {
      this.currentIndex += 1;
    }

    if (this.currentIndex >= this.target.length) {
      this.completedAtMs = this.clock.now();
    }
  }

  private removeLatestMistake(result: TypedResult): void {
    if (result.correct) {
      return;
    }

    const latestMistake = this.mistakes.at(-1);
    if (latestMistake?.index === result.index) {
      this.mistakes.pop();
    }
  }

  private removeVisibleMistakeAt(index: number): void {
    const remaining = this.mistakes.filter((mistake) => mistake.index !== index);
    this.mistakes.length = 0;
    this.mistakes.push(...remaining);
  }

  private calculateWpm(correctCount: number, elapsedMs: number): number {
    if (elapsedMs === 0) {
      return 0;
    }

    return Math.round((correctCount / WORD_SIZE / elapsedMs) * MILLIS_PER_MINUTE);
  }

  private calculateCurrentLine(): number {
    return this.target.slice(0, this.currentIndex).split("\n").length;
  }

  private calculateTotalLines(): number {
    return this.target.length === 0 ? 0 : this.target.split("\n").length;
  }
}
