# Edit - A Vim-like Text Editor

A minimalist, modal text editor written in Rust with vim-like keybindings and a modular architecture.

## Features

- Modal editing (Normal, Insert, Command modes)
- Vim-style navigation (hjkl)
- Status bar showing:
  - Current mode
  - Current time
  - Current file name
- Command bar for executing commands
- Buffer-based text handling
- Cross-platform support

## Getting Started

### Prerequisites

- Rust toolchain (cargo, rustc)
- Terminal with UTF-8 support

### Installation

```bash
git clone https://github.com/andyrudovv/edit
cd edit
cargo build --release
```

The compiled binary will be available in `target/release/edit`

### Usage

```bash
edit [filename]
```

## Keyboard Shortcuts

### Normal Mode
- `h` - Move left
- `j` - Move down
- `k` - Move up
- `l` - Move right
- `i` - Enter Insert mode
- `:` - Enter Command mode

### Insert Mode
- `ESC` - Return to Normal mode
- `Enter` - New line
- `Tab` - Insert 4 spaces
- `Backspace` - Delete character

### Command Mode
- `:q` - Quit editor
- `:w` - Save current file
- `:w <filename>` - Save to specific file
- `ESC` - Return to Normal mode

## Development

The editor is built with a modular architecture:

- `core/` - Core editor functionality
  - `buffer/` - Text buffer handling
  - `editor/` - Main editor implementation
  - `time/` - Timer utilities