# Installation Guide

## Prerequisites

- Rust 1.70 or later
- Git (for repository access)

## Install from Source

```bash
# Clone the repository
git clone https://github.com/unhappychoice/gittype.git
cd gittype

# Build and install
cargo build --release
cargo install --path .
```

## Install from Cargo

```bash
cargo install gittype
```

## Verify Installation

```bash
gittype --version
```

## Local Data Storage

`gittype` stores its data, including the local database and log files, in the `~/.gittype/` directory.

- **Database**: `~/.gittype/gittype.db` - Contains your typing history, scores, and other session data.
- **Logs**: `~/.gittype/gittype.log` - Used for debugging and monitoring.

You can safely remove this directory if you want to reset all your data.

## Troubleshooting

### Common Issues

1. **Rust version too old**
   ```bash
   rustup update stable
   ```

2. **Permission denied**
   ```bash
   # On macOS/Linux, ensure cargo bin is in PATH
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Build failures**
   ```bash
   # Clean and rebuild
   cargo clean
   cargo build --release
   ```