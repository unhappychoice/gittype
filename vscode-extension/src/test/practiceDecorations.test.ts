import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { PRACTICE_MODES } from "../models";
import type { PracticeSnapshot } from "../models";
import { buildDecorationRanges } from "../practiceDecorations";

function makeSnapshot(overrides: Partial<PracticeSnapshot>): PracticeSnapshot {
  return {
    target: "ab\ncd",
    mode: PRACTICE_MODES.strict,
    currentIndex: 1,
    typed: [{ index: 0, expected: "a", actual: "a", correct: true }],
    mistakes: [],
    completed: false,
    startedAtMs: 1_000,
    completedAtMs: undefined,
    elapsedMs: 0,
    wpm: 0,
    accuracy: 100,
    correctCount: 1,
    errorCount: 0,
    currentLine: 1,
    totalLines: 2,
    ...overrides,
  };
}

describe("buildDecorationRanges", () => {
  it("maps typed and current offsets to editor ranges", () => {
    const ranges = buildDecorationRanges(makeSnapshot({}));

    assert.deepEqual(ranges.correct, [{ start: { line: 0, character: 0 }, end: { line: 0, character: 1 } }]);
    assert.deepEqual(ranges.current, [{ start: { line: 0, character: 1 }, end: { line: 0, character: 2 } }]);
    assert.deepEqual(ranges.pending, [{ start: { line: 0, character: 2 }, end: { line: 1, character: 2 } }]);
  });

  it("maps wrong offsets across lines", () => {
    const ranges = buildDecorationRanges(makeSnapshot({
      currentIndex: 4,
      typed: [
        { index: 0, expected: "a", actual: "a", correct: true },
        { index: 3, expected: "c", actual: "x", correct: false },
      ],
      mistakes: [{ index: 3, expected: "c", actual: "x", correct: false }],
      correctCount: 1,
      errorCount: 1,
      accuracy: 50,
    }));

    assert.deepEqual(ranges.wrong, [{ start: { line: 1, character: 0 }, end: { line: 1, character: 1 } }]);
    assert.deepEqual(ranges.current, [{ start: { line: 1, character: 1 }, end: { line: 1, character: 2 } }]);
  });

  it("does not create current or pending ranges when completed", () => {
    const ranges = buildDecorationRanges(makeSnapshot({
      currentIndex: 5,
      completed: true,
    }));

    assert.deepEqual(ranges.current, []);
    assert.deepEqual(ranges.pending, []);
  });
});
