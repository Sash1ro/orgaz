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
git clone https://github.com/Sash1ro/orgaz.git
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