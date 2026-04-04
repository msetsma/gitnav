# Changelog

Changes to this project are documented in this file.

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/). This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Custom cache location override
- FZF flag passthrough (`fzf_extra_flags` in config)
- Keybindings for editor, lazygit, and browser (`ctrl-o`, `ctrl-g`, `ctrl-b`)
- Frecency-based sorting
- Pinned/favorite repositories

---

## [0.2.0] - 2026-04-04

### Added

- **Multiple search paths** — `paths = ["~/dev", "~/work"]` in `[search]` config; env var `GITNAV_SEARCH_PATHS` (colon-separated). Cache key accounts for all paths.
- **Ignore patterns** — `ignore_patterns = ["node_modules", "vendor"]` in `[search]` config; env var `GITNAV_IGNORE_PATTERNS`.
- **Inline branch + dirty indicator** — fzf list shows `gitnav  main *` style. Controlled by `show_inline_meta` in `[ui]` config; env var `GITNAV_UI_INLINE_META`.
- **Project type badges** — detects Rust, Node, Go, Python, Ruby, Java, C# by marker files. Shown in list and preview. Controlled by `badge_style = "text"/"icon"/"none"` in `[ui]`; env var `GITNAV_UI_BADGE_STYLE`.
- **Initial fzf query** — `gn react` opens fzf pre-filtered to "react". `--query` flag available for direct use.
- **PowerShell shell integration** — `gitnav init powershell` generates a `gn` function for PowerShell.
- **Scoop package** — Windows users can install via `scoop install gitnav` from the `msetsma/scoop-gitnav` bucket.
- **Non-interactive mode** (`--list`) — list repositories without fzf for scripting and piping.
- **JSON output** (`--json`) — machine-readable output for automation.
- **Environment variable configuration** — `GITNAV_*` variables override all config file settings.
- **Color/TTY detection** — automatic color handling with `NO_COLOR` and `TERM=dumb` support. Preview pane forces color on (fzf subprocess has no TTY).
- **New CLI flags** — `--quiet (-q)`, `--verbose (-v)`, `--no-color`, `--debug`.
- **Cache dry-run** — `gitnav clear-cache --dry-run` previews deletion without removing files.
- **Exit codes** — standardized codes following BSD sysexits.h (`EX_OK`, `EX_UNAVAILABLE`, `EX_INTERRUPTED`).
- **Version subcommand** — `gitnav version --verbose` shows OS, architecture, and build profile.
- **Structured error messages** — errors include code, description, suggested fix, and documentation link.

### Changed

- Shell wrapper functions (zsh, bash, fish, nushell, powershell) detect first non-flag positional argument and pass it as `--query`.
- `EnrichedRepo` wrapper carries runtime git metadata (branch, dirty state, project type) without touching the serialized `GitRepo` cache format.
- Configuration precedence: environment variables > custom config > default config > built-in defaults.
- Output: success messages to stdout, errors to stderr. Quiet mode suppresses non-error output.

### Documentation

- Full usage guide (`docs/guides/usage-guide.md`)
- Scripting examples (`docs/guides/scripting.md`)
- Environment variable reference (`docs/configuration/environment-variables.md`)
- Exit code reference (`docs/reference/exit-codes.md`)
- Performance guide (`docs/reference/performance.md`)
- GitHub workflows documentation (`docs/development/github-workflows.md`)
- Roadmap (`docs/development/roadmap.md`)

---

## [0.1.1] - 2025-11-04

### Fixed

- Homebrew update script: corrected checksum file paths in release workflow.

---

## [0.1.0] - 2025-10-27

### Added

- Initial release of gitnav.
- Fast git repository scanning using the `ignore` crate (ripgrep engine).
- Interactive fuzzy finding powered by fzf.
- Rich git preview: branch, last activity, working tree status, recent commits.
- Smart caching with SHA256-based cache keys and configurable TTL (default: 5 minutes).
- TOML configuration with sensible defaults.
- Shell integration for zsh, bash, and fish.
- CLI: `gn` (interactive), `gn -f` (force refresh), `gn --path`, `gn --max-depth`, `gitnav init`, `gitnav config`, `gitnav clear-cache`.
- Native git operations via git2 crate (no subprocess overhead for git info).
- Cross-platform: macOS, Linux, Windows.

---

## Release Links

- [Unreleased]: https://github.com/msetsma/gitnav/compare/0.2.0...HEAD
- [0.2.0]: https://github.com/msetsma/gitnav/releases/tag/0.2.0
- [0.1.1]: https://github.com/msetsma/gitnav/releases/tag/0.1.1
- [0.1.0]: https://github.com/msetsma/gitnav/releases/tag/0.1.0
