---
name: release
description: Build and install claude-statusbar binary to ~/.cargo/bin
allowed-tools: Bash
---

Build and install the `claude-statusbar` binary so it's available on PATH for Claude Code's status line.

Run from the project root (`/Users/juan/Documents/Code/claude-statusbar`):

```bash
cargo install --path .
```

This compiles in release mode and installs to `~/.cargo/bin/claude-statusbar`.

Then copy to `~/.local/bin/` which takes precedence on PATH:

```bash
cp ~/.cargo/bin/claude-statusbar ~/.local/bin/claude-statusbar
```

After installing, confirm it worked:

```bash
which claude-statusbar
```
