# GitType

**GitType** is a CLI typing game that turns your source code into typing challenges.  
Instead of random text or docs, you practice by typing actual functions and classes from your repository.

---

## Concept

- **Type your source**: Extract functions, classes, and code blocks from your repo and turn them into typing challenges.  
- **Practical training**: Practice with real syntax, keywords, and identifiers.  
- **Fun alternative to 写経**: Typing code feels less like copying work and more like a game.

---

## Features (Planned)

- Extract **functions and classes** (via [tree-sitter](https://tree-sitter.github.io/tree-sitter/))  
- Adjustable chunk size (e.g., max 40 lines per challenge)  
- Scoring system:
  - Accuracy (%)
  - Speed (WPM)
  - Mistakes per challenge
- **Local history**: track your past scores, mistakes, and progress  
- **Online ranking (optional)**: compete with other developers on shared leaderboards  

---

## Usage

```bash
gittype [path/to/repo] [options]
```

### Options

```text
--langs ts,rs,py              Filter by language
--unit function,class          Extraction unit
--max-lines 40                 Max lines per challenge
--include "src/**"             Glob include filter
--exclude "node_modules/**"    Glob exclude filter
```

---

## Example Challenge

```text
[src/lib.rs:42-68] (Rust function)

fn calculate_metrics(deps: &Dependencies) -> MetricsResult {
>
```

You type until the snippet is complete. Score is calculated from accuracy and speed. Mistakes are logged for review.

---

## Why?

Typing practice with **real code** builds fluency with actual syntax and project vocabulary.  
It turns repetition into **productive fun** while helping you notice details in codebases.

---

## Roadmap

```text
[ ] Source extraction (functions/classes)
[ ] Local scoring + history
[ ] Local analytics (mistakes, accuracy trends)
[ ] Online ranking / leaderboard
[ ] Multi-language support (Rust, TypeScript, Python, etc.)
```

---

## License

MIT
