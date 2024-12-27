# Notes Sync

![Rust CI](https://github.com/carpe-diem/notes-sync/actions/workflows/rust.yml/badge.svg)
<!-- [![Crates.io](https://img.shields.io/crates/v/notesync.svg)](https://crates.io/crates/notesync) -->
<!-- [![Documentation](https://docs.rs/notesync/badge.svg)](https://docs.rs/notesync) -->
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![dependency status](https://deps.rs/repo/github/carpe-diem/notes-sync/status.svg)](https://deps.rs/repo/github/carpe-diem/notes-sync)
<!-- [![codecov](https://codecov.io/gh/carpe-diem/notes-sync/branch/main/graph/badge.svg)](https://codecov.io/gh/carpe-diem/notes-sync) -->

A simple application to synchronize your notes with GitHub. Keep your notes backed up and accessible across multiple devices using Git as the synchronization mechanism.

## Features
- ...

## TODO
- Automatic synchronization with GitHub repositories.
- Support for multiple GitHub repositories.
- Two-way sync between local files and remote repository.

## Description

Notes Sync is a tool designed to help you maintain your notes organized and synchronized using GitHub as a backend. It watches for changes in your local notes directory and automatically commits and pushes changes to your configured GitHub repository.

## Usage
```bash
# Initial setup
notesync setup

# Sync your notes
notesync sync
```

## Installation
1. Install Rust if you haven't already:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone and build the project:
   ```bash
   git clone [repository-url]
   cd notes-sync
   cargo build --release
   ```

3. The binary will be available at `target/release/notesync`

## Configuration
Run the setup command and follow the prompts:
```bash
notesync setup
```

You'll need to provide:
- GitHub Personal Access Token (with repo permissions)
- GitHub repository (format: username/repo)
- Path to your Notes folder

## Project Structure

```
notes-sync
├── .github              // GitHub Actions configuration
├── src
│   ├── main.rs          // Application entry point
│   ├── config.rs        // Project configuration (GitHub token, paths, etc.)
├── Cargo.toml           // Project dependencies and configuration
├── README.md            // Project documentation
├── .gitignore           // Ignore files and directories
├── LICENSE              // Project license
```

## Development

### Pre-commit Hooks
This project uses pre-commit hooks to ensure code quality. The hooks run:
- `cargo test`: Run all tests
- `cargo fmt`: Check code formatting
- `cargo clippy`: Run the Rust linter

To set up pre-commit:

1. Install pre-commit:
```bash
# macOS
brew install pre-commit

# Linux
pip install pre-commit

# Windows
pip install pre-commit
```

2. Install the hooks:
```bash
pre-commit install
```

The hooks will now run automatically on every commit. You can also run them manually:
```bash
pre-commit run --all-files
```

### Building
```bash
cargo build        # Debug build
cargo build --release  # Release build
```

### Testing
```bash
cargo test              # Run all tests
cargo test config      # Run tests for config module only
cargo test -- --nocapture  # Run tests and show println output
```

### Running in Development
```bash
cargo run -- setup  # Run setup command
cargo run -- sync   # Run sync command
```

Tests are located within each module file. For example, configuration tests can be found in `src/config.rs` under the `tests` module.