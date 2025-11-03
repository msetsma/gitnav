# Scripting and Integration Guide

Real-world examples for using gitnav in scripts, automation, and tool integration.

## Table of Contents

- [Shell Scripting Basics](#shell-scripting-basics)
- [Common Patterns](#common-patterns)
- [Git Integration](#git-integration)
- [Tool Integration](#tool-integration)
- [CI/CD Automation](#cicd-automation)
- [Advanced Patterns](#advanced-patterns)

## Shell Scripting Basics

### Basic Loop Through Repositories

Iterate over all repositories and perform an operation:

```bash
#!/bin/bash

# Simple loop
gn --list | while read repo; do
    echo "Processing: $repo"
    cd "$repo"
    # Do something with the repo
done
```

### Filtering Repositories

Find repositories matching criteria:

```bash
#!/bin/bash

# Find repos with uncommitted changes
gn --list | while read repo; do
    if [ -n "$(cd "$repo" && git status --porcelain)" ]; then
        echo "$repo"
    fi
done

# Find repos on specific branch
gn --list | while read repo; do
    branch=$(cd "$repo" && git rev-parse --abbrev-ref HEAD 2>/dev/null)
    if [ "$branch" = "main" ]; then
        echo "$repo"
    fi
done

# Find large repos (by commit count)
gn --list | while read repo; do
    count=$(cd "$repo" && git rev-list --count HEAD 2>/dev/null)
    echo "$count $repo"
done | sort -rn | head -10
```

### Error Handling

Robust script with error checking:

```bash
#!/bin/bash

set -e  # Exit on error

gn --list | while read repo; do
    if [ -z "$repo" ]; then
        continue  # Skip empty lines
    fi

    if [ ! -d "$repo" ]; then
        echo "Error: Repository not found: $repo" >&2
        continue
    fi

    echo "Processing: $repo"
    cd "$repo" || exit 1

    # Your commands here
    git status
done

echo "Done!"
```

## Common Patterns

### Update All Repositories

```bash
#!/bin/bash

gn --list | while read repo; do
    echo "Updating: $repo"
    cd "$repo"

    # Fetch latest
    git fetch --all --quiet

    # Check for new commits
    local_commits=$(git rev-list --count HEAD)
    remote_commits=$(git rev-list --count origin/main)

    if [ "$local_commits" -lt "$remote_commits" ]; then
        echo "  ↑ Pulling updates..."
        git pull origin main --quiet
    else
        echo "  ✓ Up to date"
    fi
done
```

### Backup All Repositories

```bash
#!/bin/bash

BACKUP_DIR="$HOME/backups"
mkdir -p "$BACKUP_DIR"

gn --list | while read repo; do
    repo_name=$(basename "$repo")
    backup_path="$BACKUP_DIR/$repo_name"

    echo "Backing up: $repo_name"

    if [ -d "$backup_path" ]; then
        cd "$backup_path"
        git fetch --all --quiet
    else
        git clone --bare "$repo" "$backup_path"
    fi
done

echo "Backup complete: $BACKUP_DIR"
```

### Count Statistics Across Repos

```bash
#!/bin/bash

total_repos=0
total_commits=0
total_branches=0
repos_with_changes=0

gn --list | while read repo; do
    ((total_repos++))

    cd "$repo"
    commits=$(git rev-list --count HEAD)
    branches=$(git branch | wc -l)

    ((total_commits += commits))
    ((total_branches += branches))

    if [ -n "$(git status --porcelain)" ]; then
        ((repos_with_changes++))
    fi
done

echo "Repository Statistics:"
echo "  Total repos: $total_repos"
echo "  Total commits: $total_commits"
echo "  Total branches: $total_branches"
echo "  Repos with changes: $repos_with_changes"
```

### Parallel Processing

Process multiple repositories in parallel for faster execution:

```bash
#!/bin/bash

# Process 4 repos in parallel
gn --list | xargs -P 4 -I {} sh -c '
    echo "Processing: {}"
    cd "{}"
    git fetch --all --quiet
    echo "Done: {}"
'

# Process with custom command
gn --list | xargs -P 4 -I {} git -C {} status
```

## Git Integration

### Sync Fork with Upstream

```bash
#!/bin/bash

# Sync all forks with their upstream
gn --list | while read repo; do
    cd "$repo"

    # Check if upstream exists
    if ! git remote get-url upstream > /dev/null 2>&1; then
        continue
    fi

    echo "Syncing: $(basename $PWD)"

    # Fetch upstream
    git fetch upstream

    # Merge into main
    git checkout main
    git merge upstream/main --quiet

    # Push to origin
    git push origin main --quiet
done
```

### Bulk Branch Management

```bash
#!/bin/bash

# Delete merged branches in all repos
gn --list | while read repo; do
    cd "$repo"

    echo "Cleaning branches in: $(basename $PWD)"

    # Delete local merged branches
    git branch --merged | grep -v "\*" | xargs -r git branch -d

    # Prune remote branches
    git remote prune origin
done

# List stale branches (not updated in 3 months)
gn --list | while read repo; do
    cd "$repo"
    cutoff_date=$(date -d "3 months ago" +%s)

    for branch in $(git branch -r); do
        branch="${branch#origin/}"
        timestamp=$(git log -1 --format=%at origin/"$branch" 2>/dev/null || echo 0)

        if [ "$timestamp" -lt "$cutoff_date" ]; then
            echo "Stale: $(basename $repo)/$branch"
        fi
    done
done
```

### Find Recent Changes

```bash
#!/bin/bash

# Find all repos with commits in the last week
gn --list | while read repo; do
    cd "$repo"

    # Check for recent commits
    recent=$(git log --since="1 week ago" --pretty=format:"%h" | wc -l)

    if [ "$recent" -gt 0 ]; then
        echo "$(basename $repo): $recent commits in last week"
    fi
done
```

## Tool Integration

### Using with ripgrep (rg)

Search code across all repositories:

```bash
#!/bin/bash

pattern="${1:?Please provide a search pattern}"

echo "Searching for: $pattern"
echo "---"

gn --list | while read repo; do
    results=$(rg "$pattern" "$repo" --count-matches 2>/dev/null)

    if [ ! -z "$results" ]; then
        echo "$(basename $repo): $results matches"
        rg "$pattern" "$repo" --color never -n | head -3
        echo ""
    fi
done
```

### Using with jq

Process JSON output:

```bash
#!/bin/bash

# Get all repo names
gn --list --json | jq -r '.[].name'

# Get repos in specific directory
gn --list --json | jq -r '.[] | select(.path | contains("work")) | .path'

# Count repos by directory
gn --list --json | jq -r '.[].path | sub("/[^/]*$"; "")' | sort | uniq -c

# Export to CSV
gn --list --json | jq -r '["Name", "Path"] | @csv' > repos.csv
gn --list --json | jq -r '.[] | [.name, .path] | @csv' >> repos.csv
```

### Using with fzf

Advanced fuzzy selection:

```bash
#!/bin/bash

# Select repo with custom preview
cd "$(gn --list | fzf --preview 'git -C {} log --oneline -5' --header 'Select repository')"

# Multi-select repos
gn --list | fzf --multi | xargs -I {} bash -c 'cd {}; git status'

# Preview with status
gn --list | fzf --preview '
  echo "Branch: $(git -C {} rev-parse --abbrev-ref HEAD)"
  echo "Status: $(git -C {} status --short | wc -l) changes"
  git -C {} log --oneline -5
' --header 'Select repository'
```

### Integration with Other Tools

```bash
#!/bin/bash

# With GNU Parallel
gn --list | parallel 'cd {}; git fetch --all'

# With entr (file watcher)
gn --list | entr -c 'echo "Repository changed"'

# With find
find $(gn --list | tr '\n' ' ') -name '*.TODO' -type f

# With grep
gn --list | xargs -I {} grep -r "TODO" {} --include="*.rs"

# With sed (batch rename)
gn --list | xargs -I {} bash -c 'cd {}; git mv old_file new_file'
```

## CI/CD Automation

### GitHub Actions Example

```yaml
name: Repository Operations

on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly

jobs:
  sync-repos:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v2

      - name: Install gitnav
        run: cargo install --path .

      - name: Update repositories
        run: |
          gn --list | while read repo; do
            cd "$repo"
            git fetch --all
            git pull origin main || echo "Pull failed for $repo"
          done

      - name: Commit changes
        run: |
          git add -A
          git commit -m "Auto: Update repositories" || echo "No changes"
          git push
```

### GitLab CI Example

```yaml
repository_maintenance:
  script:
    - gn --list | while read repo; do
        echo "Processing $repo"
        cd "$repo"
        git fetch origin
        git log --oneline -1
      done
  only:
    - schedules
```

### Cron Job Example

```bash
#!/bin/bash
# /usr/local/bin/sync-repos.sh

LOG_FILE="/var/log/gitnav-sync.log"

{
    echo "Repository sync started at $(date)"

    gn --list | while read repo; do
        cd "$repo"
        echo "Syncing: $(basename $repo)"

        if git fetch origin 2>&1 | grep -q "error"; then
            echo "ERROR: Failed to fetch $(basename $repo)" >> "$LOG_FILE"
        else
            echo "OK: $(basename $repo)" >> "$LOG_FILE"
        fi
    done

    echo "Repository sync completed at $(date)"
} >> "$LOG_FILE" 2>&1
```

Add to crontab:

```bash
# Run every Sunday at 2 AM
0 2 * * 0 /usr/local/bin/sync-repos.sh
```

## Advanced Patterns

### Repository Analysis

```bash
#!/bin/bash

echo "Repository Analysis Report"
echo "=========================="
echo ""

# Find oldest repositories
echo "Oldest Repositories (by last commit):"
gn --list | while read repo; do
    cd "$repo"
    date=$(git log -1 --format=%ad --date=short 2>/dev/null)
    echo "$date $repo"
done | sort | head -5

echo ""

# Find largest repositories
echo "Largest Repositories (by size):"
gn --list | while read repo; do
    size=$(du -sh "$repo" 2>/dev/null | cut -f1)
    echo "$size $repo"
done | sort -h | tail -5

echo ""

# Find most active repositories
echo "Most Active (commits in last 30 days):"
gn --list | while read repo; do
    count=$(cd "$repo" && git log --since="30 days ago" --pretty=format:"%h" | wc -l)
    echo "$count commits: $repo"
done | sort -rn | head -5
```

### Dependency Analysis

```bash
#!/bin/bash

# Find all Cargo.toml files
echo "Rust Projects:"
gn --list | while read repo; do
    if [ -f "$repo/Cargo.toml" ]; then
        name=$(grep "^name" "$repo/Cargo.toml" | head -1)
        echo "  $repo: $name"
    fi
done

# Find all package.json files
echo "Node Projects:"
gn --list | while read repo; do
    if [ -f "$repo/package.json" ]; then
        version=$(jq .version "$repo/package.json")
        echo "  $repo: v$version"
    fi
done
```

### Monitoring and Health Checks

```bash
#!/bin/bash

# Check health of all repos
gn --list | while read repo; do
    cd "$repo"

    status_count=$(git status --porcelain | wc -l)

    if [ "$status_count" -gt 0 ]; then
        echo "⚠️  $(basename $repo): $status_count uncommitted changes"
    else
        echo "✓ $(basename $repo)"
    fi
done
```

### Mass Commit Operations

```bash
#!/bin/bash

# Apply changes across all repos
message="$1"

gn --list | while read repo; do
    cd "$repo"

    if [ -n "$(git status --porcelain)" ]; then
        echo "Committing changes in: $(basename $repo)"
        git add -A
        git commit -m "$message"
        git push
    fi
done
```

## Error Handling Best Practices

### Robust Script Template

```bash
#!/bin/bash
set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_FILE="${SCRIPT_DIR}/gitnav-script.log"
ERROR_COUNT=0

# Cleanup on exit
cleanup() {
    echo "Script completed with $ERROR_COUNT errors"
    exit $((ERROR_COUNT > 0 ? 1 : 0))
}
trap cleanup EXIT

# Process repositories
gn --list | while read -r repo || [ -n "$repo" ]; do
    if [ -z "$repo" ]; then
        continue
    fi

    # Verify repo exists
    if [ ! -d "$repo" ]; then
        echo "ERROR: Repository not found: $repo" | tee -a "$LOG_FILE"
        ((ERROR_COUNT++))
        continue
    fi

    # Execute with error handling
    if ! (cd "$repo" && git fetch --all); then
        echo "ERROR: Failed to fetch in $repo" | tee -a "$LOG_FILE"
        ((ERROR_COUNT++))
    fi
done
```

## Tips and Tricks

### Batch Processing Performance

Use parallel processing for faster execution:

```bash
# Using GNU Parallel
gn --list | parallel --max-procs 4 'cd {} && git fetch'

# Using xargs (more portable)
gn --list | xargs -P 4 -I {} sh -c 'cd {} && git fetch'

# Using bash background jobs
gn --list | while read repo; do
    (cd "$repo" && git fetch) &
    if [ $(jobs -r -p | wc -l) -ge 4 ]; then
        wait -n
    fi
done
wait
```

### Performance Optimization

Cache results for repeated operations:

```bash
#!/bin/bash

CACHE_FILE="/tmp/gitnav-cache.txt"
CACHE_TTL=3600  # 1 hour

if [ -f "$CACHE_FILE" ] && [ $(($(date +%s) - $(stat -f%m "$CACHE_FILE"))) -lt $CACHE_TTL ]; then
    cat "$CACHE_FILE"
else
    gn --list > "$CACHE_FILE"
    cat "$CACHE_FILE"
fi
```

## See Also

- [Usage Guide](usage-guide.md) - General usage patterns
- [Performance Guide](../reference/performance.md) - Optimization techniques
- [Quick Start](../getting-started/quick-start.md) - Getting started
