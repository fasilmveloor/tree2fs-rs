# tree2fs-rs 🦀

[![Crates.io](https://img.shields.io/crates/v/tree2fs-rs.svg)](https://crates.io/crates/tree2fs-rs)
[![Build Status](https://github.com/fasilmveloor/tree2fs-rs/actions/workflows/release.yml/badge.svg)](https://github.com/fasilmveloor/tree2fs-rs/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**tree2fs-rs** is a blazing fast command-line tool written in Rust that converts tree-formatted text into actual filesystem structures (directories and files).

It is the Rust port of the original Python `tree2fs` tool, offering improved performance, safety, and a single-binary distribution.

## 🚀 Features

- **Simple Syntax**: Uses standard tree output format (like the `tree` command).
- **Safe Preview**: Dry-run mode (`--dry-run`) lets you see exactly what will be created.
- **Flexible**: Create structures in any target directory.
- **Smart Parsing**: Handles Unicode tree characters (`│`, `├`, `└`, `─`) and indentation automatically.
- **Cross-Platform**: Works on Linux, macOS, and Windows.
- **Zero Dependencies**: Distributed as a single static binary.

## 📦 Installation

### Automated Install (Linux & macOS)
The easiest way to install is via our shell script:
```bash
curl -fsSL https://raw.githubusercontent.com/fasilmveloor/tree2fs-rs/main/install.sh | bash
```

### From Crates.io
If you have Rust installed:
```bash
cargo install tree2fs-rs
```

### From Source
```bash
git clone https://github.com/fasilmveloor/tree2fs-rs.git
cd tree2fs-rs
cargo install --path .
```

## 🛠️ Usage

### 1. Create a Tree File
Create a text file (e.g., `structure.txt`) describing your directory layout:

```text
my-project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── Cargo.toml
└── README.md
```

### 2. Generate Filesystem
Run the tool to create the structure:

```bash
tree2fs-rs structure.txt
```

### Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Preview changes without creating any files. |
| `-v`, `--verbose` | Enable detailed logging of operations. |
| `-b`, `--base-dir <DIR>` | Specify the target directory (default: current dir). |
| `--no-skip-root` | Create the root directory itself (e.g., `my-project/`) instead of just its contents. |

**Example:**
```bash
# Preview creation in a specific folder
tree2fs-rs structure.txt --base-dir ./output --dry-run --verbose
```

## 📝 Input Format Rules

- **Directories**: Must end with a forward slash `/`.
- **Files**: Anything without a trailing slash.
- **Comments**: Lines starting with `#` or text after `#` are treated as comments.
- **Indentation**: Uses standard tree characters or spaces.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/amazing-feature`).
3. Commit your changes (`git commit -m 'Add some amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.