# gitnav Usage Guide

A comprehensive guide to using gitnav for fast git repository navigation.

## Table of Contents

- [Quick Start](#quick-start)
- [Interactive Mode](#interactive-mode)
- [Non-Interactive Mode](#non-interactive-mode)
- [Configuration](#configuration)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)

## Quick Start

### Installation

```bash
# macOS
brew install gitnav

# Or build from source
cargo install --path .
```

### Setup Shell Integration

Add gitnav to your shell's configuration file:

```bash
# For Zsh (~/.zshrc)
eval "$(gitnav init zsh)"

# For Bash (~/.bashrc)
eval "$(gitnav init bash)"

# For Fish (~/.config/fish/config.fish)
eval (gitnav init fish)

# For Nushell
gitnav init nu | save --raw ~/nushell_gitnav_init.nu
source ~/nushell_gitnav_init.nu
```

After adding to your shell, reload the configuration:

```bash
source ~/.zshrc  # or ~/.bashrc, etc.
```

### Basic Usage

Once installed and configured, use the `gn` shortcut to navigate:

```bash
gn  # Start interactive navigation
```

## Interactive Mode

The default mode that launches an interactive fzf interface for selecting repositories.

### Basic Commands

**Navigate to a repository:**

```bash
gn
# Use arrow keys to move up/down
# Type to search/filter
# Press Enter to select and navigate
# Press Esc to cancel
```

### Options

**Search in a specific directory:**

```bash
gn --path ~/projects
# or the shorthand
gn -p ~/projects
```

**Force cache refresh:**

```bash
gn --force
# or
gn -f
```

This rescans all repositories even if cache is valid.

**Search with custom depth:**

```bash
gn --max-depth 8
# or
gn -d 8
```

**Show verbose output:**

```bash
gn --verbose
gn -v
```

Shows cache operations and debug information to stderr.

**Disable colors:**

```bash
gn --no-color
NO_COLOR=1 gn  # Also works with environment variable
```

### Configuration

Create `~/.config/gitnav/config.toml`:

```bash
gitnav config > ~/.config/gitnav/config.toml
# Edit the file to customize
```

Then edit with your preferred settings.

## Non-Interactive Mode

For scripting and automation, use `--list` mode:

### List All Repositories

```bash
gn --list
# Output: one repository path per line
```

### Save to File

```bash
gn --list > repositories.txt
```

### Count Repositories

```bash
gn --list | wc -l
```

### JSON Output

```bash
gn --list --json
# Output: JSON array of repository objects
```

Perfect for parsing with `jq` or other JSON tools:

```bash
gn --list --json | jq '.[].name'
# Get all repository names
```

### Pipe to Other Tools

```bash
# Find repositories with specific names
gn --list | grep "react"

# Open in editor
gn --list | fzf | xargs -I {} code {}

# Copy to clipboard
gn --list | head -1 | xclip -selection clipboard
```

### Suppress Output

```bash
gn --quiet
gn -q
```

No non-error output is printed. Useful in scripts.

## Configuration

### Configuration File

Create `~/.config/gitnav/config.toml`:

```toml
[search]
# Base path to search from
base_path = "~"
# Maximum directory depth
max_depth = 5

[cache]
# Enable or disable caching
enabled = true
# Cache TTL in seconds (5 minutes = 300)
ttl_seconds = 300

[ui]
# Search prompt in fzf
prompt = "Select repo > "
# Header text above list
header = "Repository (↑/↓, ⏎, Esc)"
# Preview pane width (0-100%)
preview_width_percent = 60
# fzf layout: "reverse" or "default"
layout = "reverse"
# fzf window height (1-100%)
height_percent = 90
# Show border around fzf window
show_border = true

[preview]
# Show current branch in preview
show_branch = true
# Show last activity (recent commit)
show_last_activity = true
# Show working tree status
show_status = true
# Number of recent commits to display
recent_commits = 5
# Date format (strftime syntax)
date_format = "%Y-%m-%d %H:%M"
```

### Environment Variables

Override configuration with environment variables:

```bash
# Search configuration
export GITNAV_BASE_PATH="$HOME/projects"
export GITNAV_MAX_DEPTH=10

# Cache configuration
export GITNAV_CACHE_ENABLED=true
export GITNAV_CACHE_TTL=600

# UI configuration
export GITNAV_UI_PROMPT="Pick: "
export GITNAV_UI_PREVIEW_WIDTH=50
export GITNAV_UI_HEIGHT=100

# Preview configuration
export GITNAV_PREVIEW_RECENT_COMMITS=10
export GITNAV_PREVIEW_DATE_FORMAT="%Y-%m-%d %H:%M:%S"

# Global options
export NO_COLOR=1  # Disable colors
```

## Advanced Usage

### Working with Multiple Projects

If you have multiple project directories:

```bash
# Create shell aliases for each directory
alias gn-work='gn --path ~/work'
alias gn-personal='gn --path ~/personal'
alias gn-open='gn --path ~/opensource'

# Use them
gn-work    # Only search ~/work
gn-personal  # Only search ~/personal
```

### Scripting with gitnav

**Example: Clone all repositories from a list**

```bash
#!/bin/bash
gn --list --json | jq -r '.[].path' | while read repo; do
  echo "Working on: $repo"
  cd "$repo"
  # Do something with each repo
done
```

**Example: Update all repositories**

```bash
#!/bin/bash
gn --list | while read repo; do
  echo "Updating $repo..."
  cd "$repo"
  git pull origin main
done
```

**Example: Search for repositories with uncommitted changes**

```bash
#!/bin/bash
gn --list | while read repo; do
  if [ -n "$(cd "$repo" && git status --porcelain)" ]; then
    echo "$repo has uncommitted changes"
  fi
done
```

### Using with Other Tools

**With `fzf` in non-interactive mode:**

```bash
gn --list | fzf --preview 'git -C {} log --oneline -n 5'
```

**With `ripgrep` to find code across all repos:**

```bash
# Search for a pattern in all repositories
gn --list | xargs -I {} rg "pattern" {}

# Search with context
gn --list | xargs -I {} rg "pattern" {} -A 5 -B 5
```

**With `git` to perform batch operations:**

```bash
# Check status of all repos
gn --list | xargs -I {} sh -c 'echo "=== {} ===" && cd {} && git status --short'

# List all branches
gn --list | xargs -I {} sh -c 'echo "=== {} ===" && cd {} && git branch'
```

## Cache Management

### Clear Cache

Remove all cached repository data:

```bash
gitnav clear-cache
```

The cache will be automatically recreated the next time you run gitnav.

### Preview Cache Deletion

See what will be deleted without actually deleting:

```bash
gitnav clear-cache --dry-run
```

### Force Refresh

Refresh cache without clearing:

```bash
gn --force
gn -f
```

## Troubleshooting

### Issue: "fzf not found"

**Error:** `Error: ENOFZF - fzf not found`

**Solution:** Install fzf:

```bash
# macOS
brew install fzf

# Linux - Ubuntu/Debian
sudo apt install fzf

# Linux - Arch
sudo pacman -S fzf

# Windows
scoop install fzf
```

Alternatively, use non-interactive mode:

```bash
gn --list
```

### Issue: No repositories found

**Error:** `Error: ENOREPOS - No repositories found`

**Solution:**

1. Check the search path:

```bash
ls -la ~/  # Check your home directory contains repos
```

2. Try a different path:

```bash
gn --path /path/to/projects
```

3. Increase search depth:

```bash
gn --max-depth 10
```

### Issue: Slow initial search

**Solution:** The first search scans all directories. Subsequent searches use the cache.

To check cache status:

```bash
gitnav clear-cache --dry-run
```

To disable cache:

```bash
export GITNAV_CACHE_ENABLED=false
gn
```

### Issue: Colors not working properly

**Solution:**

1. Check if colors are being disabled:

```bash
# If colors are disabled:
unset NO_COLOR

# If terminal is set to dumb:
export TERM=xterm-256color
```

2. Force enable/disable colors:

```bash
gn --no-color      # Disable colors
gn                 # Use auto-detection
```

### Issue: Shell integration not working

**Solution:**

1. Verify the command works:

```bash
gitnav init zsh  # For Zsh
```

2. Make sure it's added to your shell config:

```bash
# Check if it's in your shell config
grep "gitnav" ~/.zshrc

# If not, add it
echo 'eval "$(gitnav init zsh)"' >> ~/.zshrc

# Reload shell config
source ~/.zshrc
```

3. Test the `gn` command:

```bash
which gn
gn --help
```

## Tips and Tricks

### Set up fuzzy search in shell

If you want more control over the search:

```bash
# Use gitnav with custom fzf options
gn --list | fzf --multi --preview 'git -C {} log --oneline -n 3'
```

### Search with custom preview

```bash
gn --list | fzf --preview '
  git -C {} \
    log --oneline -n 10 && \
  echo "---" && \
  git -C {} status --short
'
```

### Create project-specific shortcuts

```bash
# In your shell config (~/.zshrc or ~/.bashrc)

# Work projects
gnwork() {
  cd "$(gn --path ~/work --list | fzf)"
}

# Open source projects
gnoss() {
  cd "$(gn --path ~/opensource --list | fzf)"
}

# Use them
gnwork     # Navigate work projects
gnoss      # Navigate open source projects
```

### Statistics

```bash
# Count all repositories
gn --list | wc -l

# Show repository distribution by directory
gn --list | sed 's|/[^/]*$||' | sort | uniq -c | sort -rn

# Find largest repositories (by commit count)
gn --list | while read repo; do
  count=$(cd "$repo" && git rev-list --count HEAD)
  echo "$count $repo"
done | sort -rn | head -10
```

## Performance

### Startup Time

gitnav aims for sub-500ms startup time:

```bash
# Measure startup time
time gn --list > /dev/null

# Should be very fast (< 500ms) with cache
```

### Optimization Tips

1. **Use cache:** Cache is enabled by default and significantly speeds up repeated searches

2. **Limit search depth:**

```bash
gn --max-depth 5  # Shallower search is faster
```

3. **Use specific paths:**

```bash
gn --path ~/projects  # Faster than searching entire home
```

4. **Disable preview if not needed:**

Update `~/.config/gitnav/config.toml`:

```toml
[ui]
preview_width_percent = 0  # Disable preview pane
```

## Getting Help

```bash
# Show main help
gn --help
gitnav --help

# Show command-specific help
gitnav init --help
gitnav clear-cache --help
gitnav version --help

# Check version
gitnav version
gitnav version --verbose

# Check environment variables
env | grep GITNAV
```

## See Also

- [Exit Codes Reference](exit-codes.md)
- [Environment Variables Reference](environment-variables.md)
- [Configuration Guide](https://github.com/msetsma/gitnav#configuration)
