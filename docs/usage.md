# Usage Guide

## Quick Start

1. **Navigate to any code repository:**
   ```bash
   cd /path/to/your/project
   ```

2. **Start typing practice (uses current directory by default):**
   ```bash
   gittype
   ```

3. **Or specify a specific repository:**
   ```bash
   gittype /path/to/another/repo
   ```

4. **Or clone and use a GitHub repository:**
   ```bash
   gittype --repo unhappychoice/gittype
   ```

## Command Line Options

```bash
gittype [OPTIONS] [REPO_PATH] [COMMAND]
```

**Note:** `REPO_PATH` is optional and defaults to the current directory (`.`) if not specified.

### Basic Options

| Option | Description | Default |
|---|---|---|
| `--repo` | GitHub repository URL or path to clone and use | None |
| `--langs` | Filter by programming languages (comma-separated) | All supported |
| `--config` | Path to a custom configuration file | None |

### Examples

```bash
# Practice with Rust and TypeScript files only
gittype --langs rust,typescript
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
gittype export [OPTIONS]
```
Exports session data to a specified format.

| Option | Description | Default |
|---|---|---|
| `--format` | Export format (e.g., `json`) | `json` |
| `--output` | Output file path | stdout |

**Example:**
```bash
# Export history to a JSON file
gittype export --output history.json
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
