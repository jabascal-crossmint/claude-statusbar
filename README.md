# claude-statusbar

Custom status line binary for [Claude Code](https://code.claude.com/docs/en/statusline).

## Install

```bash
cargo install --path .
cp ~/.cargo/bin/claude-statusbar ~/.local/bin/claude-statusbar
```

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "command": "claude-statusbar"
  }
}
```

## Output

```
[Sonnet] 42% | my-project
🌿 main
```
