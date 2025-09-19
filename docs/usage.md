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

5. **Play with cached repositories interactively:**
   ```bash
   gittype repo play
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
Show session history.

### Show Analytics
```bash
gittype stats
```
Show analytics.

### Export Session Data
```bash
gittype export [OPTIONS]
```
Export session data.

| Option | Description | Default |
|---|---|---|
| `--format` | Export format | `json` |
| `--output` | Output file path | stdout |

**Example:**
```bash
# Export history to a JSON file
gittype export --output history.json
```

### Manage Challenge Cache
```bash
gittype cache <COMMAND>
```

#### Cache Commands:
- `gittype cache stats` - Show cache statistics
- `gittype cache clear` - Clear all cached challenges
- `gittype cache list` - List cached repository keys

### Manage Repositories
```bash
gittype repo <COMMAND>
```

#### Repository Commands:
- `gittype repo list` - List all cached repositories
- `gittype repo clear [--force]` - Clear all cached repositories
- `gittype repo play` - Play a cached repository interactively
