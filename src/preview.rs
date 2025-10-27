use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use git2::Repository;
use std::path::Path;

use crate::config::PreviewConfig;

/// Generate preview text for a git repository
pub fn generate_preview<P: AsRef<Path>>(repo_path: P, config: &PreviewConfig) -> Result<String> {
    let repo_path = repo_path.as_ref();
    let repo = Repository::open(repo_path)
        .with_context(|| format!("Failed to open repository: {}", repo_path.display()))?;

    let mut output = Vec::new();

    // Repository name and location
    let name = repo_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    output.push(format!("\x1b[1;36mRepository:\x1b[0m {}", name));
    output.push(format!("\x1b[1;36mLocation:\x1b[0m {}", repo_path.display()));
    output.push(String::new());

    // Branch information
    if config.show_branch {
        if let Ok(head) = repo.head() {
            let branch_name = if head.is_branch() {
                head.shorthand().unwrap_or("unknown")
            } else {
                "(detached HEAD)"
            };
            output.push(format!("\x1b[1;33mBranch:\x1b[0m {}", branch_name));
        }
    }

    // Last activity (most recent commit)
    if config.show_last_activity {
        if let Ok(head) = repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                let time = commit.time();
                let dt = DateTime::<Local>::from(
                    std::time::UNIX_EPOCH + std::time::Duration::from_secs(time.seconds() as u64)
                );
                
                // Relative time
                let now = Local::now();
                let duration = now.signed_duration_since(dt);
                let relative = format_duration(duration);
                
                // Absolute time
                let absolute = dt.format(&config.date_format).to_string();
                
                output.push(format!(
                    "\x1b[1;35mLast Activity:\x1b[0m {} ({})",
                    relative, absolute
                ));
            }
        }
        output.push(String::new());
    }

    // Status information
    if config.show_status {
        if let Ok(statuses) = repo.statuses(None) {
            let mut staged = 0;
            let mut unstaged = 0;
            let mut untracked = 0;

            for entry in statuses.iter() {
                let status = entry.status();
                if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() {
                    staged += 1;
                }
                if status.is_wt_modified() || status.is_wt_deleted() {
                    unstaged += 1;
                }
                if status.is_wt_new() {
                    untracked += 1;
                }
            }

            output.push("\x1b[1;35mStatus:\x1b[0m".to_string());
            if staged > 0 || unstaged > 0 || untracked > 0 {
                if staged > 0 {
                    output.push(format!("  \x1b[32m+{} staged\x1b[0m", staged));
                }
                if unstaged > 0 {
                    output.push(format!("  \x1b[33m~{} unstaged\x1b[0m", unstaged));
                }
                if untracked > 0 {
                    output.push(format!("  \x1b[31m?{} untracked\x1b[0m", untracked));
                }
            } else {
                output.push("  Clean working tree".to_string());
            }
            output.push(String::new());
        }
    }

    // Recent commits
    if config.recent_commits > 0 {
        output.push("\x1b[1;32mRecent commits:\x1b[0m".to_string());
        if let Ok(mut revwalk) = repo.revwalk() {
            revwalk.push_head().ok();
            let commits: Vec<_> = revwalk
                .take(config.recent_commits)
                .filter_map(|oid| oid.ok())
                .filter_map(|oid| repo.find_commit(oid).ok())
                .collect();

            for commit in commits {
                let short_id = &commit.id().to_string()[..7];
                let message = commit
                    .message()
                    .unwrap_or("")
                    .lines()
                    .next()
                    .unwrap_or("");
                output.push(format!("  \x1b[33m{}\x1b[0m {}", short_id, message));
            }
        }
    }

    Ok(output.join("\n"))
}

/// Format a duration into human-readable relative time
fn format_duration(duration: chrono::Duration) -> String {
    let seconds = duration.num_seconds().abs();
    
    if seconds < 60 {
        format!("{} seconds ago", seconds)
    } else if seconds < 3600 {
        format!("{} minutes ago", seconds / 60)
    } else if seconds < 86400 {
        format!("{} hours ago", seconds / 3600)
    } else if seconds < 604800 {
        format!("{} days ago", seconds / 86400)
    } else if seconds < 2592000 {
        format!("{} weeks ago", seconds / 604800)
    } else if seconds < 31536000 {
        format!("{} months ago", seconds / 2592000)
    } else {
        format!("{} years ago", seconds / 31536000)
    }
}
