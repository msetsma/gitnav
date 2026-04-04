# gitnav

Fast git repository navigator with fuzzy finding.

`gitnav` (command: `gn`) scans for git repositories, caches results, and provides an interactive fuzzy finder (powered by fzf) with rich git information.

![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
![Rust](https://img.shields.io/badge/rust-stable-orange)
![Version](https://img.shields.io/badge/version-0.2.0-green)

**[Changelog](CHANGELOG.md)** | **[Roadmap](docs/development/roadmap.md)**

## Features

- **Fast**: Written in Rust with native git operations via git2 (no subprocess overhead)
- **Fuzzy Finding**: Interactive selection powered by [fzf](https://github.com/junegunn/fzf)
- **Rich Preview**: Branch, last activity, status, recent commits, and project type
- **Inline List Info**: Branch name and dirty indicator shown directly in the fzf list
- **Project Type Badges**: Detects Rust, Node, Go, Python, Ruby, Java, C# projects
- **Smart Caching**: Results cached with configurable TTL (default: 5 minutes)
- **Multiple Search Paths**: Scan across several directories simultaneously
- **Query Filter**: `gn react` opens fzf pre-filtered to "react"
- **Shell Integration**: zsh, bash, fish, nushell, and PowerShell
- **Zero Config**: Works out-of-the-box, configure only what you want

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

### Scoop (Windows)

```powershell
scoop bucket add gitnav https://github.com/msetsma/scoop-gitnav
scoop install gitnav
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
eval "$(gitnav init zsh)"
```

**Bash** (~/.bashrc):

```bash
eval "$(gitnav init bash)"
```

**Fish** (~/.config/fish/config.fish):

```fish
gitnav init fish | source
```

**Nushell** (~/.config/nushell/config.nu):

```nushell
gitnav init nu | save --force ~/.cache/gitnav/init.nu
source ~/.cache/gitnav/init.nu
```

**PowerShell** ($PROFILE):

```powershell
Invoke-Expression (& gitnav init powershell)
```

### 2. Use it

```bash
gn                    # Navigate to a repo
gn react              # Open fzf pre-filtered to "react"
gn -f                 # Force refresh (bypass cache)
gn --path ~/work      # Search a specific path
gn --list             # List all repos (no fzf)

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
base_path = "~"              # Single search root
# paths = ["~/dev", "~/work"] # Multiple roots (overrides base_path)
max_depth = 5
# ignore_patterns = ["node_modules", "vendor", ".tox"]

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
show_inline_meta = true      # Show branch + dirty indicator in list
badge_style = "text"         # "text" ([rust]), "icon" (🦀), or "none"

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

**Upcoming:**

- **v0.3**: Keybindings — open in editor (`ctrl-o`), lazygit (`ctrl-g`), browser (`ctrl-b`)
- **v0.4**: Frecency-based sorting, pinned repositories
- **v0.5**: Worktree support, GitHub CLI integration

## Comparison to Alternatives

| Tool | Language | Speed | Git Info | Caching | Config |
|------|----------|-------|----------|---------|--------|
| **gitnav** | Rust | Fast | Rich | Smart TTL | TOML |
| zoxide | Rust | Fast | None | Yes | Yes |
| z | Shell | Slow | None | Yes | No |
| autojump | Python | Med | None | Yes | No |

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

Licensed under either of Apache License 2.0 or MIT license, at your option. See [LICENSE](LICENSE).

## Acknowledgments

Built with [fzf](https://github.com/junegunn/fzf), [ripgrep](https://github.com/BurntSushi/ripgrep), and [git2-rs](https://github.com/rust-lang/git2-rs). Inspired by [zoxide](https://github.com/ajeetdsouza/zoxide) and [z](https://github.com/rupa/z).

---

**Author**: [@msetsma](https://github.com/msetsma) | **License**: MIT OR Apache-2.0
