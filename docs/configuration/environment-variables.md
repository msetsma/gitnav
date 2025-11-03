# Environment Variables

This document describes all environment variables supported by gitnav. Environment variables allow you to configure gitnav behavior without using command-line flags or config files.

## Configuration Precedence

Environment variables have the **highest precedence** in the configuration hierarchy:

1. **Environment variables** (highest priority - overrides everything)
2. Custom config file (`--config` flag)
3. Default config files (in order of priority):
   - `~/.config/gitnav/config.toml` (cross-platform, checked first)
   - Platform-specific location (second):
     - Linux: `~/.config/gitnav/config.toml` (respects `$XDG_CONFIG_HOME`)
     - macOS: `~/Library/Application Support/gitnav/config.toml`
     - Windows: `%APPDATA%\gitnav\config.toml`
4. Built-in defaults (lowest priority)

This means environment variables will override settings in config files. Users on Windows and macOS can place their config in `~/.config/gitnav/config.toml` for cross-platform compatibility, and it will be checked before the platform-specific location.

## Search Configuration

### GITNAV_BASE_PATH

**Type:** String
**Default:** User's home directory (`~`)
**Description:** The base directory to start searching for git repositories.

**Example:**

```bash
export GITNAV_BASE_PATH="$HOME/projects"
gitnav
```

### GITNAV_MAX_DEPTH

**Type:** Integer
**Default:** `5`
**Description:** Maximum directory depth to traverse when searching for repositories.

**Example:**

```bash
# Search deeper directory structure
export GITNAV_MAX_DEPTH=10
gitnav

# Shallow search (faster, but might miss repos)
export GITNAV_MAX_DEPTH=2
gitnav
```

## Cache Configuration

### GITNAV_CACHE_ENABLED

**Type:** Boolean (`true`, `false`, `1`, `0`, `yes`, `no`)
**Default:** `true`
**Description:** Enable or disable caching of repository lists.

**Example:**

```bash
# Disable caching (always scan fresh)
export GITNAV_CACHE_ENABLED=false
gitnav

# Enable caching (default)
export GITNAV_CACHE_ENABLED=true
gitnav
```

### GITNAV_CACHE_TTL

**Type:** Integer (seconds)
**Default:** `300` (5 minutes)
**Description:** Time-to-live for cached repository data in seconds.

**Example:**

```bash
# Cache for 30 minutes
export GITNAV_CACHE_TTL=1800
gitnav

# Cache for 1 hour
export GITNAV_CACHE_TTL=3600
gitnav
```

## UI Configuration

### GITNAV_UI_PROMPT

**Type:** String
**Default:** `"Select repo > "`
**Description:** Prompt text displayed to the user in fzf's search field.

**Example:**

```bash
export GITNAV_UI_PROMPT="Pick a repo: "
gitnav
```

### GITNAV_UI_HEADER

**Type:** String
**Default:** `"Repository (↑/↓, ⏎, Esc)"`
**Description:** Header text shown above the repository list in fzf.

**Example:**

```bash
export GITNAV_UI_HEADER="Choose repository (use arrow keys)"
gitnav
```

### GITNAV_UI_PREVIEW_WIDTH

**Type:** Integer (0-100, percentage)
**Default:** `60`
**Description:** Width of the preview pane as a percentage of the terminal width.

**Example:**

```bash
# Larger preview (70% of width)
export GITNAV_UI_PREVIEW_WIDTH=70
gitnav

# No preview, just list
export GITNAV_UI_PREVIEW_WIDTH=0
gitnav
```

### GITNAV_UI_LAYOUT

**Type:** String (`reverse`, `default`)
**Default:** `"reverse"`
**Description:** Layout style for the fzf interface.

- `reverse`: Fzf search at the top, list below
- `default`: Fzf search at the bottom, list above

**Example:**

```bash
# Traditional layout (search at bottom)
export GITNAV_UI_LAYOUT=default
gitnav

# Reverse layout (search at top)
export GITNAV_UI_LAYOUT=reverse
gitnav
```

### GITNAV_UI_HEIGHT

**Type:** Integer (1-100, percentage)
**Default:** `90`
**Description:** Height of the fzf window as a percentage of terminal height.

**Example:**

```bash
# Use 80% of terminal height
export GITNAV_UI_HEIGHT=80
gitnav

# Use most of the terminal
export GITNAV_UI_HEIGHT=95
gitnav
```

### GITNAV_UI_BORDER

**Type:** Boolean (`true`, `false`, `1`, `0`, `yes`, `no`)
**Default:** `true`
**Description:** Show a border around the fzf window.

**Example:**

```bash
export GITNAV_UI_BORDER=false
gitnav
```

## Preview Configuration

### GITNAV_PREVIEW_SHOW_BRANCH

**Type:** Boolean (`true`, `false`, `1`, `0`, `yes`, `no`)
**Default:** `true`
**Description:** Show current git branch in the preview pane.

**Example:**

```bash
export GITNAV_PREVIEW_SHOW_BRANCH=false
gitnav
```

### GITNAV_PREVIEW_SHOW_ACTIVITY

**Type:** Boolean (`true`, `false`, `1`, `0`, `yes`, `no`)
**Default:** `true`
**Description:** Show last activity (most recent commit) in the preview pane.

**Example:**

```bash
export GITNAV_PREVIEW_SHOW_ACTIVITY=false
gitnav
```

### GITNAV_PREVIEW_SHOW_STATUS

**Type:** Boolean (`true`, `false`, `1`, `0`, `yes`, `no`)
**Default:** `true`
**Description:** Show working tree status in the preview pane.

**Example:**

```bash
export GITNAV_PREVIEW_SHOW_STATUS=false
gitnav
```

### GITNAV_PREVIEW_RECENT_COMMITS

**Type:** Integer
**Default:** `5`
**Description:** Number of recent commits to display in the preview pane.

**Example:**

```bash
# Show more recent commits
export GITNAV_PREVIEW_RECENT_COMMITS=10
gitnav

# Show no commits
export GITNAV_PREVIEW_RECENT_COMMITS=0
gitnav
```

### GITNAV_PREVIEW_DATE_FORMAT

**Type:** String (strftime format)
**Default:** `"%Y-%m-%d %H:%M"`
**Description:** Date format for timestamps in the preview pane.

**Example:**

```bash
# Full timestamp with seconds
export GITNAV_PREVIEW_DATE_FORMAT="%Y-%m-%d %H:%M:%S"
gitnav

# European date format
export GITNAV_PREVIEW_DATE_FORMAT="%d/%m/%Y %H:%M"
gitnav

# ISO 8601 format
export GITNAV_PREVIEW_DATE_FORMAT="%Y-%m-%dT%H:%M:%S"
gitnav
```

## Global Options (Not Config-related)

These environment variables affect gitnav globally:

### NO_COLOR

**Type:** Flag (presence = enabled)
**Description:** Disable colored output in the terminal. Part of the [no-color.org](https://no-color.org) standard.

**Example:**

```bash
# Disable all colors
export NO_COLOR=1
gitnav

# Unset to re-enable colors
unset NO_COLOR
```

### TERM

**Type:** String
**Description:** Terminal type. If set to `dumb`, gitnav disables colors.

**Note:** Usually set automatically by your shell or SSH session.

## Complete Configuration Example

```bash
#!/bin/bash

# Search configuration
export GITNAV_BASE_PATH="$HOME/workspace"
export GITNAV_MAX_DEPTH=8

# Cache configuration
export GITNAV_CACHE_ENABLED=true
export GITNAV_CACHE_TTL=1800

# UI configuration
export GITNAV_UI_PROMPT="Select repository: "
export GITNAV_UI_HEADER="Available Repositories"
export GITNAV_UI_PREVIEW_WIDTH=50
export GITNAV_UI_LAYOUT=reverse
export GITNAV_UI_HEIGHT=100
export GITNAV_UI_BORDER=true

# Preview configuration
export GITNAV_PREVIEW_SHOW_BRANCH=true
export GITNAV_PREVIEW_SHOW_ACTIVITY=true
export GITNAV_PREVIEW_SHOW_STATUS=true
export GITNAV_PREVIEW_RECENT_COMMITS=10
export GITNAV_PREVIEW_DATE_FORMAT="%Y-%m-%d %H:%M:%S"

# Color output
unset NO_COLOR

# Now run gitnav with all configured options
gitnav
```

## Persisting Environment Variables

### In Bash/Zsh

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
# gitnav configuration
export GITNAV_BASE_PATH="$HOME/projects"
export GITNAV_MAX_DEPTH=10
export GITNAV_CACHE_TTL=600
```

### In Fish

Add to your `~/.config/fish/config.fish`:

```fish
# gitnav configuration
set -gx GITNAV_BASE_PATH $HOME/projects
set -gx GITNAV_MAX_DEPTH 10
set -gx GITNAV_CACHE_TTL 600
```

### In Nushell

Add to your `env.nu` or `config.nu`:

```nushell
# gitnav configuration
$env.GITNAV_BASE_PATH = ($env.HOME + "/projects")
$env.GITNAV_MAX_DEPTH = 10
$env.GITNAV_CACHE_TTL = 600
```

## Debugging Environment Variables

To see which environment variables are currently set:

```bash
# View all GITNAV_* variables
env | grep GITNAV

# View all variables that might affect output
env | grep -E "NO_COLOR|TERM"
```

To see what gitnav is actually using:

```bash
# Run with verbose mode to see what config is loaded
gitnav --verbose
```
