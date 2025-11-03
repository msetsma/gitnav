# Quick Start Guide

Get started with gitnav in 5 minutes.

## Installation

### macOS

```bash
brew install gitnav
```

### Linux

Build from source:

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/msetsma/gitnav
cd gitnav
cargo install --path .
```

### Windows

Build from source (see Linux instructions) or use:

```powershell
scoop install gitnav  # If available
```

## Setup Shell Integration (2 minutes)

Add gitnav to your shell so you can use the `gn` command:

### Zsh (~/.zshrc)

Add this line to the end of your `~/.zshrc`:

```bash
eval "$(gitnav init zsh)"
```

Then reload:

```bash
source ~/.zshrc
```

### Bash (~/.bashrc)

Add this line to the end of your `~/.bashrc`:

```bash
eval "$(gitnav init bash)"
```

Then reload:

```bash
source ~/.bashrc
```

### Fish (~/.config/fish/config.fish)

Add this line:

```bash
eval (gitnav init fish)
```

Then reload:

```bash
source ~/.config/fish/config.fish
```

### Nushell

Run this once:

```bash
gitnav init nu | save --raw ~/nushell_gitnav_init.nu
```

Then add this to your `env.nu`:

```bash
source ~/nushell_gitnav_init.nu
```

## Try It Out (1 minute)

### Navigate to a Repository

```bash
gn
```

This opens an interactive fuzzy finder. Type to search for a repository, then press Enter to navigate to it.

**Keys:**

- Type to search
- Arrow keys to navigate
- Enter to select
- Esc to cancel

### List Repositories (Non-Interactive)

```bash
gn --list
```

Shows all found repositories without opening fzf. Great for scripts.

## Common Tasks (1 minute)

### Search in a Specific Directory

```bash
gn --path ~/projects
```

### Search Deeper

```bash
gn --max-depth 10
```

### Force Refresh Cache

```bash
gn --force
```

### Get JSON Output

```bash
gn --list --json
```

Perfect for piping to tools like `jq`.

## Help and Documentation

### View Help

```bash
gn --help              # Interactive mode help
gitnav --help          # Full command help
gitnav init --help     # Subcommand specific help
```

### View Version

```bash
gitnav version
gitnav version --verbose  # Detailed build info
```

### Common Issues

#### "fzf not found" Error

Install fzf:

```bash
# macOS
brew install fzf

# Ubuntu/Debian
sudo apt install fzf

# Arch
sudo pacman -S fzf

# Fedora
sudo dnf install fzf
```

Or use non-interactive mode instead:

```bash
gn --list
```

#### No Repositories Found

1. Check you have git repositories in your home directory:

```bash
find ~ -maxdepth 3 -name .git -type d 2>/dev/null | head -5
```

2. Try a different search path:

```bash
gn --path ~/projects
```

3. Search deeper:

```bash
gn --max-depth 10
```

## Next Steps

### Learn More

- [Full Usage Guide](../guides/usage-guide.md) - Complete feature documentation
- [Configuration Guide](../guides/usage-guide.md#configuration) - Customize gitnav
- [Performance Guide](../reference/performance.md) - Optimize for your setup
- [Scripting Examples](../guides/scripting.md) - Use gitnav in scripts

### Configure (Optional)

gitnav works great without configuration, but you can customize it:

```bash
# Print example configuration
gitnav config > ~/.config/gitnav/config.toml

# Edit to customize
nano ~/.config/gitnav/config.toml
```

Common customizations:

- Change search path to specific projects directory
- Adjust maximum search depth
- Customize fzf appearance
- Modify preview settings

### Start Using

You're ready to go! Start using `gn` to navigate your repositories:

```bash
gn
```

## Tips

- **Create aliases** for frequently used paths:

  ```bash
  alias gnwork='gn --path ~/work'
  alias gnpersonal='gn --path ~/personal'
  ```

- **Use in scripts**:

  ```bash
  # Navigate to a repo matching a pattern
  cd "$(gn --list | grep "pattern")"
  ```

- **Combine with other tools**:

  ```bash
  # Open in editor
  gn --list | fzf | xargs code

  # Show git status
  gn --list | xargs -I {} sh -c 'cd {} && git status'
  ```

## What's Next?

1. **Explore interactive mode** - Use `gn` to navigate your repos
2. **Try non-interactive mode** - Use `gn --list` for scripting
3. **Check the full guide** - Read [usage-guide.md](../guides/usage-guide.md) for all features
4. **Customize** - Set up [configuration](../guides/usage-guide.md#configuration) if needed

Enjoy faster repository navigation!
