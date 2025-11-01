# CLI Design Guide Summary (clig.dev)

## Core Philosophy

1. **Human-First Design**: CLIs are primarily used by humans, not scripts
2. **Composable**: Small tools that work together (Unix philosophy)
3. **Consistent**: Follow established patterns and conventions
4. **Conversational**: Design for trial-and-error learning
5. **Robust**: Handle errors gracefully, feel solid and reliable

## The Basics (Must-Follow)

- Use argument parsing library
- Exit code 0 for success, non-zero for failure
- Output to stdout, errors/logs to stderr
- Display help with -h/--help
- Show concise help by default when args missing

## Help & Documentation

- Lead with examples in help text
- Show most common flags first
- Use formatting (bold headers) to improve scannability
- Provide web documentation + terminal docs
- Suggest corrections when user makes typos
- Link to web docs in terminal help

## Output

- Human-readable by default (check if TTY with isatty())
- Provide --json flag for machine-readable output
- Provide --plain for simplified tabular output
- Use color intentionally, disable if: not TTY, NO_COLOR set, TERM=dumb, --no-color
- Show progress bars for long operations
- Display brief success messages, verbose state changes
- Suggest next commands to run

## Errors

- Catch and rewrite errors for humans
- High signal-to-noise ratio (don't overwhelm)
- Put most important info at end (where eye goes)
- Provide debug info + bug report instructions for unexpected errors
- Make it effortless to submit bug reports

## Arguments & Flags

- Prefer flags over positional args (clearer, more flexible)
- Provide both short (-h) and long (--help) versions
- Use standard flag names: -f/--force, -v/--verbose, -q/--quiet, -n/--dry-run, -o/--output, --json, --debug
- Multiple args OK for simple file operations (rm file1 file2)
- Avoid 2+ different positional args (use flags instead)
- Prompt for input if missing (but never require prompts)
- Support - for stdin/stdout
- Don't read secrets from flags (use files or stdin)

## Interactivity

- Only prompt if stdin is TTY
- Respect --no-input flag to disable all prompts
- Hide password input (disable echo)
- Always allow Ctrl-C to work
- Confirm dangerous operations (y/yes or --force)

## Subcommands

- Be consistent across subcommands
- Use consistent verb names (noun verb ordering: docker container create)
- Avoid ambiguous names

## Robustness

- Validate user input early
- Print something within 100ms (responsive > fast)
- Show progress for long operations
- Make operations idempotent/recoverable
- Support timeouts (with reasonable defaults)
- Make it crash-only (defer cleanup to next run)

## Configuration

Precedence order (highest to lowest):

1. Flags
2. Environment variables
3. Project config (.env)
4. User config
5. System config

**Config locations (XDG spec):**

- Config: $XDG_CONFIG_HOME/appname (default: ~/.config/appname)
- Data: $XDG_DATA_HOME/appname (default: ~/.local/share/appname)
- Cache: $XDG_CACHE_HOME/appname (default: ~/.cache/appname)
- State: $XDG_STATE_HOME/appname (default: ~/.local/state/appname)

## Environment Variables

- Use UPPERCASE_WITH_UNDERSCORES
- Prefer single-line values
- Read from .env for project-specific config
- Check standard vars: NO_COLOR, DEBUG, EDITOR, HTTP_PROXY, TMPDIR, HOME, PAGER
- **Never store secrets in env vars** (too easy to leak)

## Naming

- Simple, memorable, lowercase
- Keep it short but not too short
- Use dashes if needed (no underscores/caps)
- Easy to type (good keyboard flow)

## Future-Proofing

- Keep changes additive
- Warn before breaking changes
- Changing human output is usually OK
- Don't allow arbitrary abbreviations
- Don't have catch-all subcommands
- Consider semantic versioning

## Distribution

- Single binary if possible
- Make uninstall easy
- Don't phone home without explicit consent

## Standard Flags Reference

| Flag | Meaning |
|------|---------|
| -h, --help | Help text |
| -V, --version | Version |
| -v, --verbose | Verbose output |
| -q, --quiet | Suppress output |
| -n, --dry-run | Show what would happen |
| -f, --force | Skip confirmations |
| -y, --yes | Assume yes to prompts |
| -o, --output FILE | Output to file |
| --json | Machine-readable JSON |
| --plain | Plain tabular text |
| --debug | Debug information |
| --no-input | Disable all prompts |
| --no-color | Disable colors |

## Exit Codes

- 0: Success
- 1: General errors
- 2: Misuse of shell command
- 126: Command cannot execute
- 127: Command not found
- 128+N: Fatal error signal N
