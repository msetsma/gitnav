# gitnav Roadmap

This document outlines the planned features and enhancements for gitnav. The project follows semantic versioning and aims to maintain backward compatibility.

**Current Version**: v0.1.0 (MVP - Feature Complete)

## Vision

gitnav aims to be the fastest and most intelligent git repository navigator, combining the speed of Rust with smart features like frecency-based sorting, rich git information, and extensible actions. The goal is to make repository navigation effortless for developers working across multiple projects.

## Release Schedule

Releases follow a **when-ready** approach prioritizing stability over fixed timelines. Minor versions target 2-3 month intervals, with patch releases as needed.

---

## v0.2 - Enhanced Configuration

**Theme**: Flexibility & Customization
**Status**: Planned

### Features - v0.2

#### Multiple Search Paths

- **Goal**: Search across different directory trees simultaneously
- **Config Example**:

  ```toml
  [search]
  paths = ["~/dev", "~/work", "~/projects"]
  max_depth = 5
  ```

- **Implementation**: Parallel scanning across paths, merge results
- **Challenge**: Cache key needs to account for multiple paths

#### Ignore Patterns

- **Goal**: Skip directories that rarely contain useful repos
- **Config Example**:

  ```toml
  [search]
  ignore_patterns = ["node_modules", ".tox", "venv", "target"]
  ```

- **Implementation**: Integrate with `ignore` crate's gitignore patterns
- **Benefit**: Faster scanning, fewer false positives

#### Custom Cache Location

- **Goal**: Allow users to override default cache directory
- **Config Example**:

  ```toml
  [cache]
  location = "~/custom/cache/path"
  ```

- **Use Case**: Network drives, custom XDG setups
- **Implementation**: Path validation, fallback to default on error

#### Full FZF Flag Passthrough

- **Goal**: Expose all fzf customization to power users
- **Config Example**:

  ```toml
  [ui]
  fzf_extra_flags = ["--exact", "--no-sort", "--tac"]
  ```

- **Implementation**: Append flags to fzf command
- **Benefit**: Flexibility without hardcoding every option

### Technical Considerations

- **Breaking Changes**: None - all features are additive
- **Migration**: Existing configs continue to work
- **Testing**: Need comprehensive config validation tests

---

## v0.3 - Keybindings & Actions

**Theme**: Interactivity & Workflows
**Target**: Q2 2026
**Status**: Planned

### Features - v0.3

#### Open in Editor (`ctrl-o`)

- **Goal**: Launch $EDITOR in the selected repository
- **Implementation**:
  - Detect $EDITOR or fallback to common editors
  - Use fzf's `--bind` for keybinding
  - Execute command via shell wrapper
- **UX**: Press `ctrl-o` to open, exit fzf automatically

#### Open in lazygit (`ctrl-g`)

- **Goal**: Launch lazygit TUI for selected repo
- **Implementation**:
  - Check if `lazygit` is in PATH
  - Execute in interactive mode
  - Alternative: Support custom TUI git clients via config
- **Config Example**:

  ```toml
  [actions]
  git_tui = "lazygit"  # or "tig", "gitui"
  ```

#### Open Remote URL (`ctrl-b`)

- **Goal**: Open GitHub/GitLab/Bitbucket URL in browser
- **Implementation**:
  - Parse remote URL from git config
  - Detect remote type (github.com, gitlab.com, etc.)
  - Use platform-specific open command (xdg-open, open, start)
- **Challenge**: Handle SSH vs HTTPS remotes, custom domains

#### Remove from Recent (`ctrl-d`)

- **Goal**: Remove repo from frecency list (future feature)
- **Status**: Depends on v0.4 frecency tracking
- **Implementation**: Mark repo as excluded in frecency database

### Technical Considerations - v0.3

- **FZF Integration**: Use `--bind 'ctrl-o:execute(...)'` pattern
- **Shell Wrapper**: May need to enhance wrapper function
- **Error Handling**: Graceful fallback if action commands not found
- **Testing**: Manual testing required (hard to automate fzf interactions)

---

## v0.4 - Intelligence

**Theme**: Smart Sorting & Context Awareness
**Status**: Planned

### Features - v0.4

#### Frecency Tracking

- **Goal**: Sort repos by frequency + recency of access
- **Algorithm**: Similar to Firefox's frecency algorithm
  - Recent visits weighted higher
  - Frequency smoothed over time windows
  - Decay function for old visits
- **Storage**: SQLite database or JSON file in cache directory
- **Schema**:

  ```json
  {
    "repo_path": "/Users/you/dev/project",
    "visits": [
      {"timestamp": 1234567890, "weight": 1.0},
      ...
    ],
    "last_visited": 1234567890,
    "frecency_score": 87.3
  }
  ```

#### Smart Sorting

- **Goal**: Multi-factor ranking algorithm
- **Factors**:
  1. Frecency score (if available)
  2. Last git activity (commit timestamp)
  3. Alphabetical (fallback)
- **Config Example**:

  ```toml
  [sorting]
  method = "frecency"  # "frecency", "activity", "alpha"
  boost_recent_activity = true
  ```

#### Workspace Detection

- **Goal**: Identify project type and show relevant info
- **Detection**:
  - Rust: `Cargo.toml` presence
  - Node.js: `package.json`
  - Go: `go.mod`
  - Python: `pyproject.toml`, `setup.py`
- **Preview Enhancement**: Show workspace type icon/label
- **Use Case**: Future actions could be workspace-aware (e.g., `ctrl-t` runs tests)

#### Favorite/Pin Repositories

- **Goal**: Pin important repos to always appear at the top of results
- **UX**:
  - Press `ctrl-f` to toggle favorite/pin status
  - Pinned repos show a `[*]` indicator in the list
  - Pinned repos always sorted first, then by frecency/activity
- **Storage**: Stored in frecency database alongside visit tracking
- **Schema**:

  ```json
  {
    "repo_path": "/Users/you/dev/important-project",
    "pinned": true,
    "pinned_at": 1234567890,
    "frecency_score": 87.3
  }
  ```

- **Config Example**:

  ```toml
  [sorting]
  method = "frecency"
  pinned_first = true  # default: true
  ```

- **Implementation**:
  - Add keybinding in fzf with `--bind 'ctrl-f:...'`
  - Store pin state in frecency database
  - Sort algorithm: pinned repos → frecency → alphabetical
- **Benefit**: Quick access to critical projects without relying on visit history

### Technical Considerations - v0.4

- **Performance**: Frecency calculation must be fast (< 5ms)
- **Privacy**: Frecency data stored locally, never transmitted
- **Migration**: Smooth transition from non-frecency to frecency
- **Testing**: Generate synthetic visit data for benchmarks

---

## v0.5 - Advanced Features

**Theme**: Extensibility & Power Features
**Target**: 2027
**Status**: Exploration

### Features

#### Git Worktree Support

- **Goal**: Show all worktrees for a repository
- **Implementation**: Parse `.git/worktrees/`, list all checkouts
- **Preview**: Show worktree paths + branches
- **Navigation**: Option to select specific worktree

#### Submodule Awareness

- **Goal**: Optionally treat submodules as separate repos
- **Config Example**:

  ```toml
  [search]
  scan_submodules = true
  ```

- **Challenge**: Avoid duplicate parent/child listings

#### GitHub CLI Integration

- **Goal**: Show PR count, issue count, CI status in preview
- **Requirements**: `gh` CLI installed and authenticated
- **Implementation**: Call `gh` commands, cache results
- **Challenge**: Rate limiting, authentication failures

---

## Non-Goals

Features explicitly out of scope:

- **Non-git directory navigation**: Use zoxide/z/autojump instead
- **Remote repository browsing**: Use GitHub/GitLab web UI
- **Git operations**: Use git CLI or lazygit/tig
- **File-level navigation**: Use fzf/fd/ripgrep directly

gitnav focuses on **repository navigation** and stays in its lane.

---

## Feedback

Have ideas not on this roadmap? Open an issue with the `enhancement` label!

- GitHub Issues: [github.com/msetsma/gitnav/issues](https://github.com/msetsma/gitnav/issues)
- Discussions: [github.com/msetsma/gitnav/discussions](https://github.com/msetsma/gitnav/discussions)

---

**Last Updated**: 2025-11-01
**Maintained by**: [@msetsma](https://github.com/msetsma)
