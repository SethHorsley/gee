# 🚀 Gee: The Binary Installer

Gee is a blazing-fast, user-friendly tool for installing binaries directly from GitHub releases. Say goodbye to manual downloads and hello to effortless installations! 🎉

## 🌟 Features

- 🔍 Automatically detects your system architecture (including Apple Silicon!)
- 📦 Downloads the latest release of any GitHub binary
- 🔧 Installs binaries to a user-specific location
- 🐚 Automatically updates your shell configuration (Bash, Zsh, and Fish supported)
- 🔁 Supports various naming conventions for macOS and Linux binaries

## 🚀 Quick Start

### Installation

```bash
cargo install gee
```

### Usage

To install a binary from a GitHub repository:

```bash
gee install username/repo/binary_name
```

For example, to install the agg_files binary from the SethHorsley/ai-tools repository:

```bash
gee install SethHorsley/ai-tools/agg_files
```

## 🛠 How It Works

- 🔍 Gee fetches the latest release information from the specified GitHub repository.
- 🧠 It intelligently determines the appropriate binary for your system.
- ⬇️ The binary is downloaded and saved to ~/.local/share/gee/bin/.
- 🔐 Gee sets the correct permissions to make the binary executable.
- 🐚 Your shell configuration is updated to include the Gee bin directory in your PATH.

## 💻 Supported Platforms

- 🍎 macOS (Intel and Apple Silicon)
- 🐧 Linux (various architectures)

## 🤝 Contributing

Contributions are welcome! Feel free to submit a Pull Request.

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🐛 Found a bug?

If you find a bug or have a feature request, please open an issue on GitHub.

Made with ❤️ and Rust 🦀
