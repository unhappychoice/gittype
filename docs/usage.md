# Usage Guide

## Quick Start

1. **Navigate to any code repository:**
   ```bash
   cd /path/to/your/project
   ```

2. **Start typing practice:**
   ```bash
   gittype
   ```

3. **Or specify a specific repository:**
   ```bash
   gittype /path/to/another/repo
   ```

## Command Line Options

```bash
gittype [OPTIONS] [REPO_PATH] [COMMAND]
```

### Basic Options

| Option | Description | Default |
|--------|-------------|---------|
| `--langs` | Filter by programming languages (comma-separated) | All supported |
| `--include` | Glob patterns for files to include | All files |
| `--exclude` | Glob patterns for files to exclude | None |

### Examples

```bash
# Practice with Rust and TypeScript files only
gittype --langs rust,typescript

# Include only source files, exclude tests
gittype --include "src/**" --exclude "**/tests/**"

# Exclude multiple patterns
gittype --exclude "**/tests/**" --exclude "**/node_modules/**"
```

## Commands

### View Session History
```bash
gittype history
```

### Show Analytics
```bash
gittype stats
```

### Export Session Data
```bash
gittype export
```

## Game Interface

### Controls

- **Type**: Simply start typing to begin
- **Ctrl+C**: Exit current session
- **Tab**: Skip current challenge (if available)
- **Enter**: Confirm completion

### Scoring

- **Accuracy**: Percentage of correct characters
- **WPM**: Words per minute (based on 5 characters per word)
- **CPM**: Characters per minute
- **Mistakes**: Number of incorrect keystrokes

### Challenge Flow

1. **Title Screen**: Welcome and instructions
2. **Loading Screen**: Extracting code chunks
3. **Countdown**: 3-2-1 start
4. **Typing Challenge**: Type the displayed code
5. **Results**: View performance metrics
6. **Next Challenge**: Continue to next stage