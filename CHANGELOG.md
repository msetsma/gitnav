# Changelog

Changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned (Phase 5+)

- Multiple search paths support
- Ignore patterns for scanning
- Custom cache location override
- FZF flag passthrough
- Keybindings for opening repos in editor/lazygit/browser
- Subcommand restructuring with colon syntax (cache:clear, shell:init)
- Stdin support for scripting workflows
- Additional shell integration (PowerShell, Elvish)

See [ROADMAP.md](ROADMAP.md) for detailed future plans.

---

## [0.2.0] - 2025-11-01

### Added (Phase 1: Critical Fixes)

- **Non-interactive mode (`--list`)** - List repositories without launching fzf for scripting and piping
- **JSON output (`--json`)** - Machine-readable JSON output for automation and tool integration
- **Environment variable configuration** - Support for `GITNAV_*` environment variables to override config
  - GITNAV_BASE_PATH, GITNAV_MAX_DEPTH
  - GITNAV_CACHE_ENABLED, GITNAV_CACHE_TTL
  - `GITNAV_UI_*` and `GITNAV_PREVIEW_*` settings
- **Color/TTY detection** - Automatic color handling with NO_COLOR and TERM=dumb support
- **New CLI flags:**
  - `--quiet (-q)` - Suppress non-error output
  - `--verbose (-v)` - Show debug information
  - `--no-color` - Disable colored output
  - `--debug` - Enable debug mode with detailed logging
- **Help text with examples** - Enhanced `--help` with usage examples for common tasks

### Added (Phase 2: High Priority)

- **Cache management improvements:**
  - `--dry-run` flag for `clear-cache` to preview deletion
  - Cache file listing and size reporting
- **Exit codes with standards compliance:**
  - Standardized exit codes following BSD sysexits.h
  - EXIT_SUCCESS (0), EXIT_GENERAL_ERROR (1), EXIT_UNAVAILABLE (69), EXIT_INTERRUPTED (130)
  - Consistent usage throughout application
- **Enhanced `version` command:**
  - New `gitnav version` subcommand
  - `--verbose` flag showing OS, architecture, and build profile
  - Build information display (version, authors, license, repository)
- **Comprehensive documentation:**
  - [docs/exit-codes.md](docs/exit-codes.md) - Exit code reference guide
  - [docs/environment-variables.md](docs/environment-variables.md) - Complete environment variable documentation

### Added (Phase 3: Medium Priority)

- **Structured error messages:**
  - ErrorInfo struct for consistent error formatting
  - Error format: code, title, description, fix suggestion, documentation URL
  - Helpful error messages for: ENOSUPPORT, ENOREPOS, ENOFZF
- **OutputFormatter enhancements:**
  - `error(&ErrorInfo)` - Structured error display
  - `error_simple(code, message)` - Simple error format
  - `success(msg)` - Success messages with quiet mode support
  - `warn(msg)` - Warning message support
- **Enhanced help text:**
  - Detailed examples for all commands
  - Interactive mode examples
  - Non-interactive scripting examples
  - Cache management examples
  - Configuration and environment variable examples
- **Subcommand documentation with examples:**
  - `gitnav init` - Shell integration setup
  - `gitnav config` - Configuration management
  - `gitnav clear-cache` - Cache operations
  - `gitnav version` - Version information
- **Comprehensive usage guide:**
  - [docs/usage-guide.md](docs/usage-guide.md) - 500+ line user guide covering:
    - Quick start and installation
    - Interactive and non-interactive modes
    - Configuration (file and environment)
    - Advanced usage and scripting
    - Batch operations with other tools
    - Troubleshooting common issues
    - Performance optimization tips

### Changed

- **Output handling:**
  - Success messages now go to stdout (previously stderr)
  - Error messages properly directed to stderr
  - Respects quiet mode and verbose flags
- **Error messages:**
  - Now follow professional structured format
  - Include actionable fix suggestions
  - Link to documentation for context
- **Configuration loading:**
  - Environment variables now override all other config sources
  - Configuration precedence: env > custom > default > built-in

### Technical Improvements

- Added `atty` crate for TTY detection
- Added `serde_json` for JSON serialization
- Enhanced scanner module with serialization support
- Created output module for centralized output formatting
- New exit_codes module for consistent exit codes

### Documentation

- Added [docs/usage-guide.md](docs/usage-guide.md) - Comprehensive user guide
- Added [docs/exit-codes.md](docs/exit-codes.md) - Exit code reference
- Added [docs/environment-variables.md](docs/environment-variables.md) - Environment variable guide
- Enhanced help text in CLI with examples

### Bug Fixes

- Fixed stdout/stderr usage for better Unix philosophy compliance
- Fixed error messages to be more helpful and actionable
- Fixed color output to respect NO_COLOR and TERM environment variables

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
