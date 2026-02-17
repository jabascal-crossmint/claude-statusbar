# Claude Code Status Bar (Rust Implementation)

High-performance Rust rewrite of the Claude Code status bar. Zero runtime dependencies, fast execution, and drop-in compatible with the JavaScript version.

## Features

- ✅ Git repository status (branch, changes, worktrees)
- ✅ Context usage percentage with color coding
- ✅ Session duration tracking
- ✅ Model name display (with z.ai API support)
- ✅ GitHub PR integration with CI status
- ✅ File-based caching for performance
- ✅ Async session summary generation
- ✅ Support for `--short` and `--skip-pr-status` flags

## Performance

- **Binary size**: ~600KB (stripped)
- **Execution time**: < 50ms typical
- **Memory usage**: Minimal
- **Dependencies**: None (statically linked)

## Building

```bash
cargo build --release
```

The binary will be at `target/release/claude-statusbar`.

## Installation

```bash
# Copy binary to a location in your PATH
cp target/release/claude-statusbar ~/.local/bin/

# Or install globally
sudo cp target/release/claude-statusbar /usr/local/bin/
```

## Usage

The statusbar reads JSON input from stdin:

```bash
echo '{"workspace":{"current_dir":"/path/to/repo"},"model":{"display_name":"Claude Sonnet 4.5"},"session_id":"abc123","transcript_path":"/path/to/transcript.jsonl"}' | claude-statusbar
```

### Command-line flags

- `--short`: Short display mode (hide path when in ~/Projects/{repo})
- `--skip-pr-status`: Skip GitHub PR status checks (faster)

## Configuration for Claude Code

To use this statusbar with Claude Code, update your shell profile or Claude Code settings to call this binary instead of the JavaScript version.

## Output Format

```
~/path [branch status] • 70% Sonnet • 1h 5m • session-id • summary • PR-URL CI-checks
```

### Color Coding

- **Path**: Cyan
- **Branch**: Green (normal) / Magenta (worktree)
- **Context %**:
  - Gray: < 50%
  - Yellow: 50-70%
  - Orange: 70-90%
  - Red: ≥ 90%
- **Worktree**: Shows `↟` symbol

## Caching

Cache files are stored in `.git/statusbar/`:
- `pr-{branch}` - PR URL (60s TTL)
- `pr-status-{branch}` - CI checks (30s TTL)
- `session-{id}-summary` - Session summary (persistent)

## Dependencies

Runtime: None (statically compiled)

Build dependencies:
- serde 1.0
- serde_json 1.0
- clap 4.5
- anyhow 1.0
- chrono 0.4

## Testing

```bash
# Run comprehensive tests
./test.sh

# Or test manually
echo '{"workspace":{"current_dir":"'$(pwd)'"}}' | ./target/release/claude-statusbar
```

## Comparison with JavaScript Version

| Metric | JavaScript | Rust |
|--------|-----------|------|
| Binary size | ~50KB (+ Bun runtime) | 622KB (includes everything) |
| Startup time | ~15ms (Bun overhead) | ~1ms |
| Execution time | ~100ms | ~50ms |
| Dependencies | Bun runtime required | None |
| Memory usage | ~30MB (Bun) | ~2MB |

## License

Same as Claude Code.
