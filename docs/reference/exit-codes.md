# Exit Codes

This document describes all exit codes used by gitnav. Exit codes are used to indicate the success or failure of the application when it terminates.

## Exit Code Reference

### 0 - EXIT_SUCCESS

**Meaning:** Successful execution

The command completed successfully without errors.

**Example:**

```bash
gitnav --list
echo $?  # Output: 0
```

### 1 - EXIT_GENERAL_ERROR

**Meaning:** General error

A generic error occurred during execution. This is returned for:

- Unsupported shell in `gitnav init <shell>`
- No git repositories found in the search path
- Configuration validation errors
- Cache operation failures

**Examples:**

```bash
gitnav init unsupported_shell
echo $?  # Output: 1

gitnav --path /nonexistent/path
echo $?  # Output: 1
```

### 2 - EXIT_USAGE_ERROR

**Meaning:** Command-line argument error

Invalid command-line arguments or flags were provided. This is typically handled by clap and is not explicitly used in the current implementation.

**Example:**

```bash
gitnav --invalid-flag
echo $?  # Output: 2
```

### 65 - EXIT_DATA_ERROR

**Meaning:** Data format error

The input data is invalid or corrupted (e.g., corrupted cache files).

### 69 - EXIT_UNAVAILABLE

**Meaning:** Unavailable resource

A required resource is not available. Currently used for:

- fzf is not installed or not in PATH

**Example:**

```bash
# When fzf is not installed and running in interactive mode
gitnav
echo $?  # Output: 69
```

**Workaround:**

```bash
# Use non-interactive mode instead
gitnav --list

# Or install fzf
# macOS: brew install fzf
# Linux: apt install fzf or pacman -S fzf
# Windows: scoop install fzf
```

### 74 - EXIT_IO_ERROR

**Meaning:** Input/output error

An I/O error occurred while reading or writing files (e.g., permission denied, disk full).

### 130 - EXIT_INTERRUPTED

**Meaning:** User interrupt (SIGINT)

The user interrupted the program with Ctrl+C. This is the standard exit code for SIGINT (signal 2).

The value 130 is derived from: `128 + SIGINT(2) = 130`

**Example:**

```bash
gitnav
# Press Ctrl+C
echo $?  # Output: 130
```

## Exit Code Standards

The exit codes used by gitnav follow standards from:

- **BSD sysexits.h** - Provides standardized exit codes for common error conditions
- **POSIX** - Standard convention that 0 = success, non-zero = error
- **Linux conventions** - Standard use of exit code 130 for user interrupts

## Using Exit Codes in Scripts

Exit codes can be used in shell scripts to handle errors:

```bash
#!/bin/bash

# Check if the command succeeded
if gitnav --list > repos.txt; then
    echo "Successfully listed repositories"
else
    exit_code=$?
    case $exit_code in
        1)
            echo "General error occurred"
            ;;
        69)
            echo "fzf is not installed. Please install it first."
            ;;
        130)
            echo "User interrupted the operation"
            ;;
        *)
            echo "Unknown error: $exit_code"
            ;;
    esac
    exit $exit_code
fi
```

## Troubleshooting Common Exit Codes

### Exit code 1 (ENOREPOS)

No git repositories found in the search path.

**Fix:**

- Verify the base path exists: `ls -la $GITNAV_BASE_PATH`
- Ensure it contains git repositories: `find /path -name .git -type d`
- Try a different path: `gitnav --path ~/projects`

### Exit code 69 (ENOFZF)

fzf is not installed or not in PATH.

**Fix:**

- Install fzf for your system:
  - macOS: `brew install fzf`
  - Ubuntu/Debian: `sudo apt install fzf`
  - Arch Linux: `sudo pacman -S fzf`
  - Windows: `scoop install fzf`
- Use non-interactive mode instead: `gitnav --list`

### Exit code 130 (User interrupt)

User pressed Ctrl+C to cancel the operation.

This is expected behavior. No action needed unless you want to prevent interrupts in a script.
