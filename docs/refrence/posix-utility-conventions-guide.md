# POSIX Utility Conventions Reference

## Overview

The POSIX Utility Conventions (IEEE Std 1003.1, Chapter 12) define standard syntax and behavior for command-line utilities to ensure consistency and portability across Unix-like systems.

## Argument Terminology

**Option**: A flag preceded by a hyphen, like `-a` or `-v`

- May be single character: `-h`
- Take option-arguments: `-o file.txt`

**Option-argument**: A value that follows an option, like `file.txt` in `-o file.txt`

**Operand**: Arguments that are not options or option-arguments, typically file names or other data

- Example: `cp source.txt dest.txt` (both are operands)

**Delimiter**: The `--` argument that signals the end of options

## Synopsis Notation

``` text
utility_name [-a] [-b] [-c option_argument] [-d|-e] [-f[option_argument]] [operand...]
```

**Notation meanings:**

- `[-a]` - Optional option
- `[-c option_argument]` - Option with mandatory option-argument
- `[-d|-e]` - Mutually exclusive options (use one or the other)
- `[-f[option_argument]]` - Option with optional option-argument
- `[operand...]` - Zero or more operands
- `-f option_argument [-f option_argument]...` - Option that can repeat

## The 14 Guidelines

### Guideline 1: Utility Name Length

**Utility names should be between 2 and 9 characters, inclusive.**

Good: `ls`, `grep`, `find`, `cat`
Bad: `x`, `verylongutilityname`

### Guideline 2: Utility Name Characters

**Utility names should include lowercase letters and digits only from the portable character set.**

Good: `ls`, `ps`, `gcc`, `python3`
Bad: `myUtil`, `my-tool`, `My_App`

### Guideline 3: Single Character Options

**Each option name should be a single alphanumeric character. The `-W` option is reserved for vendor extensions.**

Good: `-a`, `-v`, `-1`, `-h`
Bad: `-verbose`, `-help`, `-aa`

Multi-digit options should not be allowed.

### Guideline 4: Hyphen Delimiter

**All options should be preceded by the '-' delimiter character.**

Good: `-a -b -c`
Bad: `/a`, `+a`, `a`

### Guideline 5: Option Grouping

**Multiple options without option-arguments can be grouped behind a single '-' delimiter, followed by at most one option that takes an option-argument.**

Good:

- `-abc` (equivalent to `-a -b -c`)
- `-abcf file.txt` (equivalent to `-a -b -c -f file.txt`)

Invalid:

- `-af file1.txt -g file2.txt` (two options with arguments grouped)

### Guideline 6: Separate Arguments

**Each option and option-argument should be a separate argument.**

Good: `-o file.txt`
Allowed (for compatibility): `-ofile.txt`

Exception: For optional option-arguments, they must be in the same argument string:

- Good: `-f` or `-fvalue`
- Bad: `-f value` (value would be treated as operand, not option-argument)

### Guideline 7: No Optional Option-Arguments

**Option-arguments should not be optional.**

This is a guideline for new utilities. Some historical utilities violate this for backward compatibility.

### Guideline 8: Multiple Option-Arguments

**When multiple option-arguments are specified for a single option, they should be presented as a single argument, separated by commas or spaces.**

Good:

- `-f file1,file2,file3`
- `-f "file1 file2 file3"`

Bad:

- `-f file1 -f file2 -f file3` (unless explicitly allowed)

### Guideline 9: Options Before Operands

**All options should precede operands on the command line.**

Good: `grep -i -r pattern file1 file2`
Bad: `grep file1 -i -r pattern file2`

Note: GNU utilities often relax this with `getopt_long`, but POSIX requires strict ordering.

### Guideline 10: Double-Dash Delimiter

**The first `--` argument that is not an option-argument should be accepted as a delimiter indicating the end of options. Any following arguments should be treated as operands, even if they begin with '-'.**

Example:

```bash
rm -- -filename.txt    # Deletes file named "-filename.txt"
grep -- --pattern file.txt  # Searches for literal "--pattern"
```

### Guideline 11: Option Order Independence

**The order of different options relative to one another should not matter, unless options are mutually-exclusive.**

Good (both equivalent):

- `ls -la`
- `ls -al`

Exception: If an option overrides another, later options win:

- `cmd --verbose --quiet` (quiet wins)

If an option with arguments is repeated, process them in order:

- `cmd -I dir1 -I dir2` (search dir1 first, then dir2)

### Guideline 12: Operand Order May Matter

**The order of operands may matter and position-related interpretations should be determined on a utility-specific basis.**

Examples:

- `cp source dest` (order matters: source first, destination second)
- `cat file1 file2` (order determines concatenation order)

### Guideline 13: Dash as stdin/stdout

**For utilities that use operands to represent files, the '-' operand should be used to mean standard input (or standard output when clear from context) or a file named `-`.**

Examples:

```bash
cat file.txt - file2.txt  # Read file.txt, then stdin, then file2.txt
tar czf - dir/            # Write tar output to stdout
diff file.txt -           # Compare file.txt with stdin
```

### Guideline 14: Option Identification

**If an argument can be identified as an option or group of options (per Guidelines 3-10), then it should be treated as such.**

The parser should recognize options even if they appear in unexpected positions, though Guideline 9 says they should precede operands.

## Numeric Arguments

Unless otherwise specified:

- Numbers are interpreted as **decimal integers**
- Range: `0` to `2147483647` (positive)
- Negative numbers: `-2147483647` to `2147483647` (when explicitly supported)
- File size values: `0` to maximum file size supported by implementation

Invalid numeric syntax should produce clear error messages indicating the value is out of range.

## Exit Status

**Required behavior:**

- `0` - Successful completion
- `>0` - Error occurred

**Common conventions:**

- `1` - General errors
- `2` - Misuse of shell command (incorrect arguments)
- `126` - Command cannot execute (permission problem)
- `127` - Command not found
- `128+N` - Fatal error signal N (e.g., `130` = Ctrl-C killed it)

## Standard Options

While not mandated by POSIX, these are strongly recommended:

- `--help` - Display help (not in POSIX, but universal)
- `--version` - Display version (not in POSIX, but universal)

Note: POSIX doesn't specify long options (`--`) at all. This is a GNU extension that became widely adopted.

## File Descriptor Usage

- **stdin (0)**: Standard input for data
- **stdout (1)**: Primary output for results and data
- **stderr (2)**: Error messages, warnings, diagnostics

## Environment Variables

Standard environment variables to respect:

- `PATH` - Search path for executables
- `HOME` - User's home directory
- `TMPDIR` - Directory for temporary files
- `TERM` - Terminal type
- `LANG`, `LC_*` - Locale settings

## Portable Character Set

For maximum portability, utility names and options should use only:

- Uppercase letters: `A-Z`
- Lowercase letters: `a-z`
- Digits: `0-9`
- Underscore: `_`
- Period: `.`
- Hyphen: `-` (for options and names)

## Key Differences: POSIX vs GNU Style

| Aspect | POSIX | GNU Extension |
|--------|-------|---------------|
| Long options | Not specified | `--long-option` |
| Option location | Before operands | Anywhere (with `getopt_long`) |
| Option-argument | Separate or attached | Separate or `--option=value` |
| Multiple occurrences | Guidelines vary | Usually allowed |
| Single-letter grouping | Yes (`-abc`) | Yes, plus long options |

## Common POSIX Utilities

Standard utilities that follow these conventions:

- File: `cat`, `cp`, `ls`, `mkdir`, `rm`, `mv`, `ln`, `chmod`, `chown`
- Text: `grep`, `sed`, `awk`, `sort`, `uniq`, `cut`, `paste`, `tr`, `wc`
- Process: `ps`, `kill`, `nice`, `sleep`
- Shell: `sh`, `echo`, `test`, `expr`
- Archive: `tar`, `cpio`
- Development: `make`, `cc`, `lex`, `yacc`

## Practical Examples

### Good POSIX Style

```bash
# Clear separation of options and operands
grep -i -n pattern file.txt

# Grouped options
ls -la /tmp

# Using -- delimiter
rm -- -weirdfile

# Stdin/stdout via dash
cat file1.txt - file2.txt | sort - > output.txt

# Option with argument
cc -o program program.c
```

### Bad POSIX Style (but may work on GNU systems)

```bash
# Options after operands (GNU extension)
grep file.txt -i -n pattern

# Long options (GNU extension)
ls --all --long /tmp

# Optional option-arguments (violates Guideline 7)
ls -l -color /tmp

# Equals syntax (GNU extension)
cc --output=program program.c
```

## For New Utility Developers

**DO:**

- Follow all 14 guidelines strictly
- Use single-character options only
- Separate options and option-arguments
- Put options before operands
- Support `--` as option terminator
- Use `-` for stdin/stdout
- Return 0 on success, non-zero on error
- Write errors to stderr, results to stdout

**DON'T:**

- Create multi-character options (use `-W` for vendor extensions)
- Make option-arguments optional
- Require options in a specific order (except mutually-exclusive)
- Treat `-` as anything other than stdin/stdout or actual filename
- Use non-portable characters in utility names
- Make utilities longer than 9 characters

*Note: While POSIX defines the minimum standard, GNU utilities often extend it with `--long-options`, relaxed ordering, and other conveniences. For maximum portability, stick to POSIX. For usability on modern systems, consider GNU extensions.*
