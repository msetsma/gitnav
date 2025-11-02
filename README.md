# gitnav

Fast git repository navigator with fuzzy finding.

`gitnav` (command: `gn`) scans for git repositories, caches results, and provides an interactive fuzzy finder (powered by fzf) with rich git information.

![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
![Rust](https://img.shields.io/badge/rust-1.81%2B-orange)
![Version](https://img.shields.io/badge/version-0.1.0-green)

**[Changelog](CHANGELOG.md)** | **[Roadmap](docs/development/roadmap.md)**

## Features

- **[*] Fast**: Written in Rust with native git operations (no subprocess overhead)
- **[?] Fuzzy Finding**: Interactive selection powered by [fzf](https://github.com/junegunn/fzf)
- **[#] Rich Preview**: See branch, last activity, status, and recent commits
- **[$] Smart Caching**: Results cached with configurable TTL (default: 5 minutes)
- **[+] Configurable**: Optional TOML config file with sensible defaults
- **[>] Shell Integration**: Works seamlessly with zsh, bash, and fish
- **[!] Zero Config**: Works out-of-the-box, configure only if you want to

## Preview

``` text
Select repo > my-project

┌──────────────────────────────────────┐
│ Repository: my-project               │
│ Location: /Users/you/dev/my-project  │
│                                      │
│ Branch: main                         │
│ Last Activity: 2 hours ago           │
│                (2025-10-27 14:30)    │
│                                      │
│ Status:                              │
│   +2 staged                          │
│   ~1 unstaged                        │
│                                      │
│ Recent commits:                      │
│   a1b2c3d Add new feature            │
│   e4f5g6h Fix bug in parser          │
│   ...                                │
└──────────────────────────────────────┘
```

## Installation

### Homebrew (macOS and Linux)

```bash
brew install msetsma/gitnav/gitnav
```

### Cargo

```bash
cargo install gitnav
```

### Pre-compiled Binaries

Download pre-compiled binaries from the [releases page](https://github.com/msetsma/gitnav/releases).

Available for:

- Linux (x86_64, aarch64, musl)
- macOS (x86_64, Apple Silicon)
- Windows (x86_64)

## Quick Start

### 1. Set up shell integration

Add to your shell configuration file:

**Zsh** (~/.zshrc):

```bash
eval "$(gitnav --init zsh)"
```

**Bash** (~/.bashrc):

```bash
eval "$(gitnav --init bash)"
```

**Fish** (~/.config/fish/config.fish):

```fish
gitnav --init fish | source
```

**Nushell** (~/.config/nushell/config.nu):

```nushell
gitnav --init nu | save --force ~/.cache/gitnav/init.nu
source ~/.cache/gitnav/init.nu
```

### 2. Use it

```bash
gn                    # Navigate to a repo
gn -f                 # Force refresh (bypass cache)
gn --path ~/work      # Search specific path
gn --max-depth 3      # Custom search depth

gitnav config         # Print example config
gitnav clear-cache    # Clear cache
```

## Configuration

Zero configuration required. Optional config file at `~/.config/gitnav/config.toml`:

```bash
mkdir -p ~/.config/gitnav
gitnav config > ~/.config/gitnav/config.toml
```

### Example

```toml
[search]
base_path = "~"              # Where to search for repos
max_depth = 5                # How deep to search

[cache]
enabled = true
ttl_seconds = 300            # 5 minutes

[ui]
prompt = "Select repo > "
header = "Repository (↑/↓, ⏎, Esc)"
preview_width_percent = 60
layout = "reverse"
height_percent = 90
show_border = true

[preview]
show_branch = true
show_last_activity = true
show_status = true
recent_commits = 5
date_format = "%Y-%m-%d %H:%M"
```

See [config/config.example.toml](config/config.example.toml) for a full example with comments.

## How It Works

1. **Scan**: Fast filesystem traversal using [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) crate (ripgrep engine)
2. **Cache**: SHA256-keyed cache with configurable TTL
3. **Select**: Interactive fuzzy finder powered by fzf
4. **Preview**: Native git operations via [git2](https://github.com/rust-lang/git2-rs) (no subprocess overhead)
5. **Navigate**: Shell wrapper handles `cd` to selected path

## Requirements

- **fzf**: Must be installed and in PATH
  - macOS: `brew install fzf`
  - Linux: `apt install fzf` or `pacman -S fzf`
  - Windows: `scoop install fzf`
- **Rust 1.70+**: For building from source

## Performance

Compared to shell-based alternatives:

| Operation | Shell Script | gitnav (Rust) |
|-----------|--------------|---------------|
| Startup | ~50ms | <5ms |
| Scan (cold) | ~200ms | ~100ms |
| Git info | ~50ms/repo | ~10ms/repo |
| **Total (cold)** | **~500ms** | **~200ms** |
| **Total (cached)** | **~100ms** | **~20ms** |

## Roadmap

See [ROADMAP.md](docs/development/roadmap.md) for detailed future plans and release targets.

**Upcoming Highlights:**

- **v0.2**: Multiple search paths, ignore patterns, custom cache location
- **v0.3**: Keybindings for editor/lazygit/browser integration
- **v0.4**: Frecency-based sorting, workspace detection
- **v0.5**: Plugin system, worktree support

## Comparison to Alternatives

| Tool | Language | Speed | Git Info | Caching | Config |
|------|----------|-------|----------|---------|--------|
| **gitnav** | Rust | *** | [x] Rich | [x] Smart | [x] TOML |
| zoxide | Rust | *** | [ ] | [x] | [x] |
| z | Shell | * | [ ] | [x] | [ ] |
| autojump | Python | ** | [ ] | [x] | [ ] |

`gitnav` is git-specific with rich repository previews. Alternatives handle all directories but lack git context.

## Contributing

Contributions welcome! Submit a Pull Request.

```bash
git clone https://github.com/msetsma/gitnav.git
cd gitnav
cargo build
cargo test
cargo run -- -f
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Acknowledgments

Built with [fzf](https://github.com/junegunn/fzf), [ripgrep](https://github.com/BurntSushi/ripgrep), and [git2-rs](https://github.com/rust-lang/git2-rs). Inspired by [zoxide](https://github.com/ajeetdsouza/zoxide) and [z](https://github.com/rupa/z).

---

**Author**: [@msetsma](https://github.com/msetsma) | **License**: MIT OR Apache-2.0
