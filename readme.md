Here is the complete `README.md` content, ready to save as `README.md`:

Écriture

# Orgaz

A simple command-line file organizer written in Rust to organize files and analyze directory contents.

## Features

- Organize files inside a directory.
- Preview changes with dry-run mode.
- Include hidden files.
- Display directory statistics.
- Export statistics as JSON.
- Optionally scan subdirectories.

## Installation

bash

```bash
git clone <repository-url>
cd orgaz
cargo build --release
```

## Usage

### Organize files

bash

```bash
orgaz run ./documents
```

Preview changes without modifying files:

bash

```bash
orgaz run ./documents --dry
```

Include hidden files:

bash

```bash
orgaz run ./documents --hidden
```

### Display statistics

bash

```bash
orgaz stats ./documents
```

Export statistics as JSON:

bash

```bash
orgaz stats ./documents --json
```

Include subdirectories:

bash

```bash
orgaz stats ./documents --recursive
```

### Configuration

bash

```bash
orgaz config --show
```

## Development

Run tests:

bash

```bash
cargo test
```

Format code:

bash

```bash
cargo fmt
```

Check code quality:

bash

```bash
cargo clippy
```

## Built With

- [Rust](<https://www.rust-lang.org/>)
- [Clap](<https://docs.rs/clap/latest/clap/>) — CLI argument parsing
- [Serde](<https://serde.rs/>) — Serialization
- [Thiserror](<https://docs.rs/thiserror/latest/thiserror/>) — Error handling

## License

MIT