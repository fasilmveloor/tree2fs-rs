# tree2fs

Convert tree-formatted text into filesystem structures.

Now rewritten in Rust 🦀 for blazing speed and safety!

## Installation

### From Source
```bash
git clone https://github.com/fasilmveloor/tree2fs-rs.git
cd tree2fs-rs
cargo install --path .
```

### Automated Installation (Linux/macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/fasilmveloor/tree2fs-rs/main/install.sh | bash
```

## Usage

```bash
# Create structure from tree file
tree2fs-rs tree.txt

# Preview without creating (dry run)
tree2fs-rs tree.txt --dry-run --verbose

# Create in specific directory
tree2fs-rs tree.txt --base-dir /path/to/project

# Include root directory in creation
tree2fs-rs tree.txt --no-skip-root
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

### Testing
Run the test suite, including unit tests and integration tests:
```bash
cargo test
```

Run the example API usage:
```bash
cargo run --example api_usage
```

### Release

To release a new version to [crates.io](https://crates.io/):

1. Update the version in `Cargo.toml`.
2. Commit the changes:
   ```bash
   git commit -am "Bump version to x.y.z"
   ```
3. Tag the release:
   ```bash
   git tag vx.y.z
   git push origin vx.y.z
   ```
4. Publish to crates.io:
   ```bash
   cargo publish
   ```

**Note:** You need to be logged in with `cargo login` before publishing.

## License

MIT License - see LICENSE file for details.