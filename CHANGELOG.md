# Changelog

Changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Multiple search paths support
- Ignore patterns for scanning
- Custom cache location override
- FZF flag passthrough
- Keybindings for opening repos in editor/lazygit/browser

See [ROADMAP.md](ROADMAP.md) for detailed future plans.

---

## [0.1.0] - 2025-10-27

### Added

- Initial release of gitnav
- Fast git repository scanning using `ignore` crate
- Interactive fuzzy finding powered by fzf
- Rich git preview showing:
  - Current branch
  - Last activity timestamp
  - Working tree status (staged/unstaged/untracked)
  - Recent commits
- Smart caching with SHA256-based cache keys
- Configurable TTL for cache (default: 5 minutes)
- TOML configuration support with sensible defaults
- Shell integration for zsh, bash, and fish
- CLI commands:
  - `gn` - Interactive repository selection
  - `gn -f` - Force refresh (bypass cache)
  - `gn --path <PATH>` - Search from specific directory
  - `gn --max-depth <N>` - Custom search depth
  - `gn --config <FILE>` - Use custom config file
  - `gitnav --init <shell>` - Generate shell wrapper
  - `gitnav config` - Print example config
  - `gitnav clear-cache` - Clear cached results
- Native git operations using `git2` crate (no subprocess overhead)
- Cross-platform support (macOS, Linux, Windows)
- Comprehensive documentation and examples

### Technical Details

- Written in Rust (2021 edition)
- Zero-config operation with optional TOML configuration
- XDG-compliant cache directory (`~/.cache/gitnav/`)
- Performance: <20ms for cached results, ~100ms for cold scan

---

## Release Links

- [Unreleased]: https://github.com/msetsma/gitnav/compare/v0.1.0...HEAD
- [0.1.0]: https://github.com/msetsma/gitnav/releases/tag/v0.1.0
