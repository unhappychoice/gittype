import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { PRACTICE_MODES } from "../models";
import { PracticeSession } from "../practiceSession";

class FakeClock {
  private currentMs: number;

  public constructor(initialMs: number) {
    this.currentMs = initialMs;
  }

  public now = (): number => this.currentMs;

  public advance(ms: number): void {
    this.currentMs += ms;
  }
}

describe("PracticeSession", () => {
  it("advances through correct strict input when characters match", () => {
    const clock = new FakeClock(1_000);
    const session = new PracticeSession("abc", PRACTICE_MODES.strict, clock);

    const snapshot = session.handleTextInput("ab");

    assert.equal(snapshot.currentIndex, 2);
    assert.equal(snapshot.correctCount, 2);
    assert.equal(snapshot.errorCount, 0);
    assert.equal(snapshot.completed, false);
  });

  it("blocks strict progress when a character is incorrect", () => {
    const session = new PracticeSession("abc", PRACTICE_MODES.strict, new FakeClock(1_000));

    const snapshot = session.handleTextInput("ax");

    assert.equal(snapshot.currentIndex, 1);
    assert.equal(snapshot.correctCount, 1);
    assert.equal(snapshot.errorCount, 1);
    assert.deepEqual(snapshot.mistakes.map((mistake) => mistake.index), [1]);
  });

  it("clears visible strict mistake when the current character is corrected", () => {
    const session = new PracticeSession("abc", PRACTICE_MODES.strict, new FakeClock(1_000));

    session.handleTextInput("ax");
    const snapshot = session.handleTextInput("b");

    assert.equal(snapshot.currentIndex, 2);
    assert.equal(snapshot.errorCount, 1);
    assert.deepEqual(snapshot.mistakes, []);
  });

  it("continues flow progress when a character is incorrect", () => {
    const session = new PracticeSession("abc", PRACTICE_MODES.flow, new FakeClock(1_000));

    const snapshot = session.handleTextInput("axc");

    assert.equal(snapshot.currentIndex, 3);
    assert.equal(snapshot.completed, true);
    assert.equal(snapshot.correctCount, 2);
    assert.equal(snapshot.errorCount, 1);
    assert.equal(snapshot.accuracy, 67);
  });

  it("moves back through typed history when backspace is pressed", () => {
    const session = new PracticeSession("abc", PRACTICE_MODES.flow, new FakeClock(1_000));
    session.handleTextInput("ax");

    const snapshot = session.handleBackspace();

    assert.equal(snapshot.currentIndex, 1);
    assert.equal(snapshot.typed.length, 1);
    assert.equal(snapshot.errorCount, 0);
  });

  it("resets progress when restarted", () => {
    const session = new PracticeSession("abc", PRACTICE_MODES.strict, new FakeClock(1_000));
    session.handleTextInput("ab");

    const snapshot = session.restart();

    assert.equal(snapshot.currentIndex, 0);
    assert.equal(snapshot.typed.length, 0);
    assert.equal(snapshot.startedAtMs, undefined);
  });

  it("reports completion and metrics when target is finished", () => {
    const clock = new FakeClock(1_000);
    const session = new PracticeSession("abcdefghij", PRACTICE_MODES.strict, clock);

    session.handleTextInput("abcde");
    clock.advance(60_000);
    const snapshot = session.handleTextInput("fghij");

    assert.equal(snapshot.completed, true);
    assert.equal(snapshot.elapsedMs, 60_000);
    assert.equal(snapshot.wpm, 2);
    assert.equal(snapshot.accuracy, 100);
  });

  it("toggles between strict and flow modes", () => {
    const session = new PracticeSession("abc", PRACTICE_MODES.strict, new FakeClock(1_000));

    const flowSnapshot = session.toggleMode();
    const strictSnapshot = session.toggleMode();

    assert.equal(flowSnapshot.mode, PRACTICE_MODES.flow);
    assert.equal(strictSnapshot.mode, PRACTICE_MODES.strict);
  });
});
