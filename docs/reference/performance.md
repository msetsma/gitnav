# Performance Guide

A comprehensive guide to understanding and optimizing gitnav's performance characteristics.

## Overview

gitnav is designed for speed and efficiency. The application targets:

- **Cached startup time:** < 100ms
- **Cold startup (first scan):** < 500ms for typical home directory
- **Memory footprint:** < 20MB
- **Cache efficiency:** > 95% hit rate for repeated searches

## Performance Characteristics

### Cached Performance (Typical)

When results are cached, gitnav can navigate repositories in under 100ms:

```bash
$ time gn --list > /dev/null
real    0m0.050s
user    0m0.020s
sys     0m0.010s
```

This includes:

- Loading configuration
- Validating cache
- Loading cached repository list
- Serialization for output

### Cold Performance (First Scan)

First scan without cache typically takes 100-500ms depending on system and directory structure:

```bash
$ time gn --force --list > /dev/null
real    0m0.150s
user    0m0.080s
sys     0m0.050s
```

This includes:

- Configuration loading
- Filesystem scanning (checking all directories up to max_depth)
- Git metadata extraction for each repository
- Cache serialization and storage

### Factors Affecting Performance

1. **Search depth** - More directories = more time
   - `--max-depth 3`: ~50-100ms (typical)
   - `--max-depth 5`: ~100-200ms (normal)
   - `--max-depth 10`: ~300-500ms (deep scans)

2. **Search path location** - Affects filesystem scan time
   - Home directory: ~100-200ms
   - Specific project folder: ~20-50ms
   - Root directory: > 1s (not recommended)

3. **Repository count** - Linear relationship
   - 10 repos: < 50ms
   - 100 repos: ~100ms
   - 1000 repos: ~300-400ms

4. **Filesystem speed** - SSD vs HDD
   - SSD: 50-200ms typical
   - HDD: 200-800ms typical
   - Network mounted: 500-2000ms+ (use cache)

## Cache Management

### Understanding Cache

gitnav uses persistent caching to avoid repeated filesystem scans:

``` text
Cache location: ~/.cache/gitnav/
Cache format: JSON files with SHA256-based keys
Default TTL: 300 seconds (5 minutes)
Cache hit rate: > 95% in typical usage
```

### Cache Effectiveness

The cache provides dramatic performance improvement:

```bash
# First run (cache miss)
$ time gn --list > /dev/null
real    0m0.180s

# Second run (cache hit)
$ time gn --list > /dev/null
real    0m0.045s

# Performance improvement: ~4x faster
```

### Cache Invalidation

Cache is automatically invalidated when:

1. **TTL expires** - Default 5 minutes
2. **Configuration changes** - Different search path or depth
3. **Forced refresh** - Using `--force` flag

View cache status:

```bash
# See cache files
gitnav clear-cache --dry-run

# Output example:
# Cache directory: /home/user/.cache/gitnav
# Cache files: 3
# Total size: 2048 bytes
```

## Optimization Techniques

### 1. Use Caching Effectively

Cache is enabled by default and provides the most significant performance improvement:

```bash
# Verify cache is enabled
echo $GITNAV_CACHE_ENABLED

# Manually enable cache
export GITNAV_CACHE_ENABLED=true
```

### 2. Limit Search Depth

Searching fewer directory levels is significantly faster:

```bash
# Fast (shallow search)
gn --max-depth 3
time: ~50-80ms

# Medium (typical)
gn --max-depth 5
time: ~100-150ms

# Slow (deep search)
gn --max-depth 10
time: ~300-500ms
```

Add to shell config for consistent behavior:

```bash
export GITNAV_MAX_DEPTH=5  # Typical project structure
```

### 3. Use Specific Search Paths

Searching a subset of your filesystem is faster than searching everything:

```bash
# Fast (specific path)
gn --path ~/projects
time: ~50-100ms

# Slower (home directory)
gn --path ~
time: ~100-200ms

# Much slower (entire system)
gn --path /
time: > 1s (not recommended)
```

Create aliases for common paths:

```bash
alias gnwork='gn --path ~/work'
alias gnoss='gn --path ~/opensource'
alias gnpersonal='gn --path ~/personal'
```

### 4. Disable Preview (if not needed)

The preview pane does minimal processing, but can be disabled:

```toml
[ui]
# Disable preview to save resources
preview_width_percent = 0
```

### 5. Reduce Recent Commits in Preview

Decrease the number of commits shown:

```toml
[preview]
# Show fewer commits for faster preview rendering
recent_commits = 3  # Default: 5
```

### 6. Use Non-Interactive Mode for Scripts

Non-interactive mode is faster for automated tasks:

```bash
# Interactive (with fzf overhead)
gn

# Non-interactive (minimal overhead)
gn --list
gn --list --json

# Time savings: 50-100ms per invocation
```

## Profiling and Measurement

### Measuring Startup Time

Use `time` command to measure performance:

```bash
# Measure with cache (typical usage)
time gn --list > /dev/null

# Measure without cache (force refresh)
time gn --force --list > /dev/null

# Measure with verbose output to see operations
time gn --verbose --list 2>&1 | tee perf.log
```

### Detailed Timing with Verbose Mode

Use `--verbose` flag to see cache operation timing:

```bash
$ gn --verbose --list 2>&1
DEBUG: Loading from cache
DEBUG: Found 42 repositories
```

### Benchmarking Different Configurations

Compare performance across settings:

```bash
#!/bin/bash

echo "=== Cache Performance ==="
echo -n "With cache:    "; time gn --list > /dev/null 2>&1
echo -n "Without cache: "; time gn --force --list > /dev/null 2>&1

echo -e "\n=== Depth Performance ==="
for depth in 3 5 8 10; do
    echo -n "Depth $depth: "
    time gn --force --max-depth $depth --list > /dev/null 2>&1
done

echo -e "\n=== Path Performance ==="
echo -n "Work path:     "; time gn --path ~/work --list > /dev/null 2>&1
echo -n "Home path:     "; time gn --path ~ --list > /dev/null 2>&1
```

### Checking System Information

View build and feature information:

```bash
$ gitnav version --verbose

gitnav 0.2.0

Build Information:
  Version: 0.2.0
  Authors: Author Name <email>
  License: MIT
  Repository: https://github.com/msetsma/gitnav

System Information:
  OS: linux
  Architecture: x86_64
  Build Profile: release

Features:
  Colors: enabled
  Interactive Mode: enabled
```

## Performance Tuning

### Recommended Configuration

For optimal performance in typical usage:

```toml
[search]
# Search from home directory
base_path = "~"
# Most projects are within 5 levels
max_depth = 5

[cache]
# Enable caching for dramatic speedup
enabled = true
# 5 minutes is a good balance
ttl_seconds = 300

[ui]
# Interactive settings (minimal performance impact)
preview_width_percent = 60
height_percent = 90

[preview]
# Balanced preview settings
show_branch = true
show_last_activity = true
show_status = true
recent_commits = 5
date_format = "%Y-%m-%d %H:%M"
```

### For Performance-Critical Environments

If every millisecond counts:

```toml
[search]
base_path = "~/projects"  # Specific path, not home
max_depth = 3             # Shallow search

[cache]
enabled = true
ttl_seconds = 600        # Longer TTL = fewer scans

[ui]
preview_width_percent = 0  # Disable preview
height_percent = 50

[preview]
show_branch = true
show_last_activity = false  # Skip git operations
show_status = false         # Skip status check
recent_commits = 1          # Minimal data
```

### Environment Variables for Performance

Quick performance tuning via environment variables:

```bash
# Use shallow search for speed
export GITNAV_MAX_DEPTH=3

# Search specific path instead of home
export GITNAV_BASE_PATH="$HOME/projects"

# Disable preview for non-interactive use
export GITNAV_UI_PREVIEW_WIDTH=0

# Longer cache TTL
export GITNAV_CACHE_TTL=600
```

## Common Performance Issues

### Issue: "First run is slow"

**Cause:** Cache miss on first invocation

**Solution:**

```bash
# This is normal - cache is warming up
gn --list
# Subsequent invocations will be fast (< 100ms)
```

### Issue: "Performance degrades on large searches"

**Cause:** Searching too many directories or repositories

**Solution:**

```bash
# 1. Use specific path instead of home directory
gn --path ~/projects

# 2. Reduce search depth
gn --max-depth 5

# 3. Use non-interactive mode if possible
gn --list
```

### Issue: "Cache seems stale"

**Cause:** TTL is too long or cache wasn't invalidated

**Solution:**

```bash
# Clear cache manually
gitnav clear-cache

# Or adjust TTL
export GITNAV_CACHE_TTL=300

# Or force refresh
gn --force
```

### Issue: "Slow on network filesystem"

**Cause:** Network latency on NFS/SMB mounts

**Solution:**

```bash
# 1. Ensure cache is enabled (default)
export GITNAV_CACHE_ENABLED=true

# 2. Use local path instead of network path
gn --path ~/local_projects

# 3. Increase TTL to avoid frequent scans
export GITNAV_CACHE_TTL=1200  # 20 minutes
```

## Performance Benchmarks

### Typical System (Laptop, SSD, ~50 repos)

```
Operation              Time      Notes
Cache hit              45ms      Most common operation
Cache miss (refresh)   120ms     First run, periodic refresh
--force refresh        150ms     Manual cache clear
--list (cached)        50ms      Pipe-friendly output
--json (cached)        55ms      JSON serialization minimal overhead
Interactive selection  60-200ms  Depends on fzf rendering
```

### Large System (Workstation, SSD, ~500 repos)

```
Operation              Time      Notes
Cache hit              95ms      Larger result set
Cache miss (refresh)   380ms     More repositories
--force refresh        400ms     Full filesystem scan
--list (cached)        110ms     Larger output
--json (cached)        120ms     More data to serialize
Interactive selection  200-500ms Fzf rendering time
```

### Optimization Results

Typical optimizations can achieve:

- **4-5x speedup** with caching enabled vs. disabled
- **3-4x speedup** with reduced search depth (5 vs. 10 levels)
- **2-3x speedup** with specific path vs. home directory
- **2x speedup** with cache TTL extension (300s → 600s)

## Advanced Topics

### Cache Behavior

Cache key is determined by:

1. Search base path
2. Maximum search depth
3. Current working directory

This ensures:

- Different paths have separate caches
- Cache is used when searching same path again
- Different depths create different cache entries

### Memory Usage

gitnav uses minimal memory:

- **Base application:** ~2-3MB
- **Cached results (100 repos):** ~1-2MB
- **Peak during scan:** ~5-10MB
- **Typical usage:** < 20MB

### Parallel Operations

For batch operations on multiple repositories:

```bash
# Process each repo in parallel
gn --list | xargs -P 4 -I {} sh -c 'cd {} && git status'

# The -P 4 flag uses 4 parallel workers
# Adjust based on your CPU cores for optimal performance
```

## Performance Monitoring

### Regular Performance Checks

Periodically verify performance:

```bash
# Weekly performance baseline
date >> perf_baseline.txt
time gn --list > /dev/null 2>&1 >> perf_baseline.txt

# Monitor trends
tail -20 perf_baseline.txt
```

### Signs of Performance Degradation

Watch for:

- Cache hit time increasing (> 150ms) - disk may be failing
- Cache miss time increasing (> 800ms) - filesystem issue or more repos
- Memory usage > 50MB - unusual, may indicate memory leak

## See Also

- [Usage Guide](../guides/usage-guide.md) - General usage patterns
- [Exit Codes](exit-codes.md) - Understanding error conditions
- [Environment Variables](../configuration/environment-variables.md) - Configuration options
