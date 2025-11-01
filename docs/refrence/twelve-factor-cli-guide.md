# 12 Factor CLI Apps Reference

Source: [12 Factor CLI Apps by Jeff Dickey](https://medium.com/@jdxcode/12-factor-cli-apps-dd3c227a0e46)

## Philosophy

CLIs are powerful tools that take less time to build than web apps and allow users to compose multiple tools together for advanced tasks. These 12 factors create CLI UX that users will love.

---

## 1. Great Help is Essential

**Help is far more important for CLIs than web apps** since there's no UI to guide users.

### Provide Multiple Help Access Methods

```bash
# All of these should show help
mycli
mycli --help
mycli help
mycli -h

# Subcommand help
mycli subcommand --help
mycli subcommand -h
```

### What to Include in Help

- Description of the command
- Description of arguments
- Description of all flags
- **Examples of common usage** (most important!)

### Where to Provide Help

- **In-CLI help** (immediate, no need to leave terminal)
- **Web documentation** (READMEs, Google-searchable)
- Shell completion (another form of help)
- Man pages (optional - not used as often, don't work on Windows)

**Key Rule:** `-h` and `--help` should be reserved exclusively for help

---

## 2. Prefer Flags to Args

**Flags are clearer than positional arguments.**

### Bad Example (confusing)

```bash
heroku fork FROMAPP --app TOAPP  # Which is source? Which is destination?
```

### Good Example (clear)

```bash
heroku fork --from FROMAPP --to TOAPP
```

### When Args Are OK

- **1 type of argument:** `rm file_to_remove` ✓
- **Multiple of same type:** `rm file1 file2 file3` ✓
- **2 types of arguments:** Very suspect ⚠️
- **3+ types:** Never good ✗

### Special Case: Pass-through Arguments

Use `--` to stop parsing and pass everything through:

```bash
heroku run -a myapp -- myscript.sh -a arg1
```

---

## 3. What Version Am I On?

**Make version easy to find:**

```bash
mycli version      # multi-command CLIs
mycli --version
mycli -V
mycli -v          # unless -v is used for --verbose
```

### Version Command Tips

- Good place to add extra debugging information
- Send version as User-Agent header for API debugging
- Users will ask for this when reporting bugs

---

## 4. Mind the Streams

**stdout is for output, stderr is for messaging.**

### Correct Usage

```bash
$ myapp > foo.txt
Warning: something went wrong  # This warning is on stderr, not in file
```

### Rules

- **stdout:** Program output, structured data (JSON, binary)
- **stderr:** Errors, warnings, progress bars, status messages
- Always pipe subcommand stderr to user's screen
- Not everything on stderr is an error (e.g., curl progress)

---

## 5. Handle Things Going Wrong

**Things go wrong more often in CLIs than web apps.**

### Anatomy of a Great Error Message

1. **Error code** (e.g., `EPERM`)
2. **Error title**
3. **Error description** (optional)
4. **How to fix it**
5. **URL for more information**

### Example

```bash
$ myapp dump -o myfile.out
Error: EPERM - Invalid permissions on myfile.out
Cannot write to myfile.out, file does not have write permissions.
Fix with: chmod +w myfile.out
https://github.com/jdxcode/myapp
```

### For Unexpected Errors

- Provide full traceback with flag or env var
- Use `DEBUG` environment variable for verbose output
- Error logs should:
  - Have timestamps
  - Be truncated occasionally (don't fill disk)
  - Not contain ANSI color codes

---

## 6. Be Fancy

**Modern CLIs should use visual enhancements thoughtfully.**

### Use

- Colors/dimming to highlight important info
- Spinners for long-running tasks
- Progress bars
- OS notifications for very long tasks

### But Know When to Fall Back

**Disable fancy output when:**

- stdout/stderr is not a TTY (piped to file)
- `TERM=dumb`
- `NO_COLOR` environment variable is set
- `--no-color` flag is passed
- App-specific env var: `MYAPP_NOCOLOR=1`

**Never output ANSI codes to files** - they only work on screens

---

## 7. Prompt If You Can

**If stdin is a TTY, prompt instead of requiring flags.**

### Guidelines

- Never require a prompt (users need to automate)
- Always allow flag override
- Use prompts for:
  - Input collection
  - Confirmation dialogs for dangerous actions
  - Checkboxes/radio buttons for visual option selection

### Example: Destructive Action Confirmation

```bash
$ heroku apps:destroy
Type app name to confirm: myapp-name
```

---

## 8. Use Tables

**Tables are common but must be done right.**

### Rules for Tables

- **Each row = one entry** (essential for piping/parsing)
- **No table borders** (noisy, breaks parsing)
- Show column headers by default
- Truncate rows that exceed screen width (unless `--no-truncate`)

### Useful Flags to Provide

- `--columns <list>`: Show specific columns
- `--no-headers`: Hide headers
- `--filter`: Filter specific columns
- `--sort`: Sort by column
- `--no-truncate`: Don't truncate long values
- `--csv`: Output as CSV
- `--json`: Output as JSON

### Why No Borders?

Without borders, you can pipe to tools:

```bash
ls | wc -l                    # Count files
ls | grep ".js" | wc -l      # Count JS files
```

---

## 9. Be Speedy

**Startup time matters.**

### Benchmarks

```bash
time mycli
```

- **<100ms:** Very fast (not feasible for scripting languages)
- **100ms-500ms:** Fast enough - aim here ✓
- **500ms-2s:** Usable but not impressive
- **2s+:** Languid - users will avoid your CLI

### For Long Operations

- Show progress bar or spinner
- Even a spinner makes it feel faster than it is

---

## 10. Encourage Contributions

**Keep code open source and welcoming.**

### Essentials

- **Pick a license**
- Host on GitHub/GitLab
- Write a good README with overview
- Document how to:
  - Run CLI locally
  - Run test suites
- Provide contribution guidelines
- **Add a Code of Conduct** (important to some people, won't hurt)

### Plugin System

Consider allowing users to extend your CLI with plugins

---

## 11. Be Clear About Subcommands

**Two types of CLIs: single-command and multi-command.**

### Single-Command

- Basic UNIX-style: `cp`, `grep`, `cat`
- Simple, performs one task
- Show help if no arguments

### Multi-Command

- Git-style: `git commit`, `npm install`
- Accepts subcommand as first argument
- List subcommands if no arguments

### Topics (Sub-subcommands)

**Two notation styles:**

**Spaces (Git style):**

```bash
git submodule add repo-url
```

**Colons (Heroku style - preferred):**

```bash
heroku domains:add www.myapp.com
```

**Why colons are better:**

- Clearly delineate command from arguments
- Allow topic-level commands to accept arguments
- Parser can distinguish `mycli topic arg` from `mycli topic:command`

---

## 12. Follow XDG-spec

**Use standard locations for files.**

### Config Files

- **Unix/Linux:** `~/.config/myapp` (or `$XDG_CONFIG_HOME/myapp`)
- Use XDG environment variables when set

### Data Files

- **Unix/Linux:** `~/.local/share/myapp` (or `$XDG_DATA_HOME/myapp`)

### Cache Files

- **Unix/Linux:** `~/.cache/myapp`
- **macOS:** `~/Library/Caches/myapp`
- **Windows:** `%LOCALAPPDATA%\myapp`

**Don't use:** `~/.myapp` (legacy pattern, deprecated)

---

## Quick Reference Checklist

- [ ] Multiple help access points (-h, --help, help)
- [ ] Prefer flags over positional args (especially for 2+ different types)
- [ ] Version accessible via --version, -V
- [ ] stdout for output, stderr for messages
- [ ] Informative error messages with fix instructions
- [ ] Colors/spinners (but respect NO_COLOR and TTY detection)
- [ ] Prompt when interactive (but never require prompts)
- [ ] Tables without borders, with useful flags
- [ ] Startup time < 500ms
- [ ] Open source with contribution docs
- [ ] Clear subcommand structure (prefer colons over spaces)
- [ ] Follow XDG-spec for file locations

---

*Note: The article's author created oclif, a CLI framework for Node.js designed to follow these principles automatically.*
