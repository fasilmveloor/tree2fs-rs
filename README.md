# tree2fs

Convert tree-formatted text into filesystem structures.

Now rewritten in Rust 🦀 for blazing speed and safety!

## Installation

### From Source
```bash
git clone https://github.com/ABDELLAH-Hallou/tree2fs.git
cd tree2fs
cargo install --path .
```

## Usage

```bash
# Create structure from tree file
tree2fs tree.txt

# Preview without creating (dry run)
tree2fs tree.txt --dry-run --verbose

# Create in specific directory
tree2fs tree.txt --base-dir /path/to/project

# Include root directory in creation
tree2fs tree.txt --no-skip-root
```

## Tree File Format

```
project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── Cargo.toml
└── README.md
```

- Directories end with `/`
- Comments start with `#`
- Supports standard tree drawing characters: `│`, `├`, `└`, `─`

## Features

- ✅ Parse tree-formatted text files
- ✅ Create directories and files
- ✅ Dry-run mode for preview
- ✅ Verbose output
- ✅ Skip root directory option (default)
- ✅ Cross-platform (Linux, macOS, Windows)

## Development

```bash
# Run tests
cargo test

# Run locally
cargo run -- tree.txt --dry-run
```

## License

MIT License - see LICENSE file for details.