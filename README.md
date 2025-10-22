![GitType Banner](docs/images/gittype-banner.png)

# GitType ⌨️💻

> *"Show your AI who's boss: just you, your keyboard, and your coding sins"*

**GitType** turns your own source code into typing challenges. Because why practice with boring lorem ipsum when you can type your beautiful `fn main()` implementations?

## Demo 🎬

![GitType Demo](docs/images/demo.gif)

## Features ✨

- 🦀🐍⚡🐹💎🍎🎯☕🐘#️⃣🔧➕🎭🎯⚡ **Multi-language**: Rust, TypeScript, JavaScript, Python, Go, Ruby, Swift, Kotlin, Java, PHP, C#, C, C++, Haskell, Dart, Scala, Zig (more languages incoming!)  
- 📊 **Real-time metrics**: Live WPM, accuracy, and consistency tracking as you type
- 🏆 **Ranking system**: Unlock developer titles from "Hello World Newbie" to "Quantum Computer" with ASCII art
- 🎮 **Multiple game modes**: Normal, Time Attack, and custom difficulty levels (Easy to Zen)
- ⏸️ **Pause/resume**: Take breaks without ruining your stats
- 🎯 **Your own code**: Type functions from your actual projects, not boring examples
- 🔥 **Trending repositories**: Practice with hot GitHub repositories updated daily
- 🎨 **15+ Themes**: Built-in themes with Dark/Light modes + custom theme support

## Installation 📦

### Quick Install (Recommended)
#### One-liner installation (Linux/macOS/Windows)
```bash
curl -sSL https://raw.githubusercontent.com/unhappychoice/gittype/main/install.sh | bash
```

#### Or with specific version
```bash
curl -sSL https://raw.githubusercontent.com/unhappychoice/gittype/main/install.sh | bash -s -- --version v0.5.0
```

### Homebrew (macOS/Linux)
```bash
brew install unhappychoice/tap/gittype
```


### Cargo (Universal)
```bash
cargo install gittype
```

### Nix (NixOS/Nix)
If you have [Nix](https://nixos.org/) installed, you can run GitType directly:

```bash
# Stable version (recommended)
nix run github:unhappychoice/gittype

# Development version (latest from main branch)
nix run github:unhappychoice/gittype#unstable
```

### Binary Downloads
Get pre-compiled binaries for your platform from our [releases page](https://github.com/unhappychoice/gittype/releases/latest).

Available platforms:
- `x86_64-apple-darwin` (Intel Mac)
- `aarch64-apple-darwin` (Apple Silicon Mac)
- `x86_64-unknown-linux-gnu` (Linux x64)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `x86_64-pc-windows-msvc` (Windows)

## Quick Start 🚀

```bash
# cd into your messy codebase
cd ~/that-project-you-never-finished

# Start typing your own spaghetti code (uses current directory by default)
gittype

# Or specify a specific repository path
gittype /path/to/another/repo

# Clone and play with any GitHub repository
gittype --repo clap-rs/clap
gittype --repo https://github.com/ratatui-org/ratatui
gittype --repo git@github.com:dtolnay/anyhow.git

# Discover and practice with trending GitHub repositories
gittype trending                    # Browse trending repos interactively
gittype trending rust               # Filter by language (Rust)

# Play with cached repositories interactively
gittype repo play
```

## Why GitType? 🤔

- **Look busy at work** → "I'm studying the codebase" (technically true!)
- **Beat the AI overlords** → Type faster than ChatGPT can generate
- **Stop typing boring stuff** → Your own bugs are way more interesting than lorem ipsum
- **Discover forgotten treasures** → That elegant function you wrote at 3am last year
- **Procrastinate like a pro** → It's code review, but gamified!
- **Embrace your legacy code** → Finally face those variable names you're not proud of
- **Debug your typing skills** → Because `pubic static void main` isn't a typo anymore
- **Therapeutic code reliving** → Type through your programming journey, tears included
- **Climb the dev ladder** → From "Code Monkey" to "Quantum Computer" - each rank comes with fancy ASCII art

*"Basically, you need an excuse to avoid real work, and this one's pretty good."*

## Documentation 📚

Perfect for when the game gets too addictive:

- **[Installation](docs/installation.md)** - `cargo install` and chill
- **[Usage](docs/usage.md)** - All the CLI flags your heart desires
- **[Playing Guide](docs/playing-guide.md)** - Game modes, scoring, and ranks
- **[Themes](docs/themes.md)** - 15+ built-in themes and custom theme creation
- **[Languages](docs/supported-languages.md)** - What we extract and how
- **[Contributing](docs/CONTRIBUTING.md)** - Join the keyboard warriors
- **[Architecture](docs/ARCHITECTURE.md)** - For the curious minds

## Screenshots 📸

![GitType Title Screen](docs/images/title.png)

![GitType Gaming](docs/images/gaming.png)

![GitType Result](docs/images/result.png)

![GitType Result](docs/images/stage-result.png)

![GitType Records](docs/images/records.png)

![GitType Records Detail](docs/images/records-detail.png)

![GitType Analytics Overview](docs/images/analytics-overview.png)

![GitType Analytics Trends](docs/images/analytics-trends.png)

![GitType Analytics Languages](docs/images/analytics-languages.png)

![GitType Analytics Repositories](docs/images/analytics-repositories.png)

![GitType Settings](docs/images/settings-theme.png)

## License 📄

[MIT](LICENSE) - Because sharing is caring (and legal requirements)

---

*Built with ❤️ and way too much caffeine by developers who got tired of typing "hello world"*
