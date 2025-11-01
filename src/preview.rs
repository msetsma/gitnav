use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use git2::Repository;
use std::path::Path;

use crate::config::PreviewConfig;

/// Generate a colored preview of a git repository.
///
/// Displays repository information including branch, status, recent commits,
/// and last activity. Output is ANSI color-coded for terminal display.
///
/// # Arguments
///
/// * `repo_path` - Path to the git repository
/// * `config` - Preview configuration controlling what information to display
///
/// # Returns
///
/// A formatted string with ANSI color codes suitable for terminal display
///
/// # Errors
///
/// Returns an error if the repository cannot be opened or accessed
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

/// Format a duration into human-readable relative time.
///
/// Converts a duration into an English phrase like "3 days ago" or "5 minutes ago".
/// Uses absolute value to handle both past and future durations.
///
/// # Arguments
///
/// * `duration` - The duration to format
///
/// # Returns
///
/// A formatted string describing the duration in human-readable terms
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_seconds() {
        let duration = chrono::Duration::seconds(30);
        assert_eq!(format_duration(duration), "30 seconds ago");
    }

    #[test]
    fn test_format_duration_one_second() {
        let duration = chrono::Duration::seconds(1);
        assert_eq!(format_duration(duration), "1 seconds ago");
    }

    #[test]
    fn test_format_duration_minutes() {
        let duration = chrono::Duration::minutes(45);
        assert_eq!(format_duration(duration), "45 minutes ago");
    }

    #[test]
    fn test_format_duration_hours() {
        let duration = chrono::Duration::hours(5);
        assert_eq!(format_duration(duration), "5 hours ago");
    }

    #[test]
    fn test_format_duration_days() {
        let duration = chrono::Duration::days(3);
        assert_eq!(format_duration(duration), "3 days ago");
    }

    #[test]
    fn test_format_duration_weeks() {
        let duration = chrono::Duration::weeks(3);
        assert_eq!(format_duration(duration), "3 weeks ago");
    }

    #[test]
    fn test_format_duration_months() {
        let duration = chrono::Duration::days(60);
        assert_eq!(format_duration(duration), "2 months ago");
    }

    #[test]
    fn test_format_duration_years() {
        let duration = chrono::Duration::days(400);
        assert_eq!(format_duration(duration), "1 years ago");
    }

    #[test]
    fn test_format_duration_negative() {
        // Test that we handle negative durations (future dates) by taking absolute value
        let duration = chrono::Duration::seconds(-30);
        assert_eq!(format_duration(duration), "30 seconds ago");
    }

    #[test]
    fn test_preview_config_default() {
        let config = PreviewConfig {
            show_branch: true,
            show_last_activity: true,
            show_status: true,
            recent_commits: 5,
            date_format: "%Y-%m-%d %H:%M".to_string(),
        };

        assert!(config.show_branch);
        assert!(config.show_last_activity);
        assert!(config.show_status);
        assert_eq!(config.recent_commits, 5);
    }

    #[test]
    fn test_preview_config_all_disabled() {
        let config = PreviewConfig {
            show_branch: false,
            show_last_activity: false,
            show_status: false,
            recent_commits: 0,
            date_format: "%Y-%m-%d".to_string(),
        };

        assert!(!config.show_branch);
        assert!(!config.show_last_activity);
        assert!(!config.show_status);
        assert_eq!(config.recent_commits, 0);
    }

    #[test]
    fn test_format_duration_boundary_seconds_to_minutes() {
        let duration = chrono::Duration::seconds(59);
        assert_eq!(format_duration(duration), "59 seconds ago");
    }

    #[test]
    fn test_format_duration_boundary_minutes_to_hours() {
        let duration = chrono::Duration::minutes(59);
        assert_eq!(format_duration(duration), "59 minutes ago");
    }

    #[test]
    fn test_format_duration_boundary_hours_to_days() {
        let duration = chrono::Duration::hours(23);
        assert_eq!(format_duration(duration), "23 hours ago");
    }

    #[test]
    fn test_format_duration_boundary_days_to_weeks() {
        let duration = chrono::Duration::days(6);
        assert_eq!(format_duration(duration), "6 days ago");
    }

    #[test]
    fn test_format_duration_boundary_weeks_to_months() {
        let duration = chrono::Duration::days(29);
        assert_eq!(format_duration(duration), "4 weeks ago");
    }

    #[test]
    fn test_format_duration_boundary_months_to_years() {
        let duration = chrono::Duration::days(364);
        assert_eq!(format_duration(duration), "12 months ago");
    }

    #[test]
    fn test_format_duration_zero() {
        let duration = chrono::Duration::seconds(0);
        assert_eq!(format_duration(duration), "0 seconds ago");
    }

    #[test]
    fn test_format_duration_multiple_months() {
        let duration = chrono::Duration::days(100);
        assert_eq!(format_duration(duration), "3 months ago");
    }

    #[test]
    fn test_format_duration_multiple_years() {
        let duration = chrono::Duration::days(1000);
        assert_eq!(format_duration(duration), "2 years ago");
    }

    #[test]
    fn test_format_duration_large_duration() {
        let duration = chrono::Duration::days(10000);
        assert_eq!(format_duration(duration), "27 years ago");
    }

    #[test]
    fn test_format_duration_one_minute() {
        let duration = chrono::Duration::minutes(1);
        assert_eq!(format_duration(duration), "1 minutes ago");
    }

    #[test]
    fn test_format_duration_boundary_59_seconds() {
        let duration = chrono::Duration::seconds(59);
        assert_eq!(format_duration(duration), "59 seconds ago");
    }

    #[test]
    fn test_format_duration_boundary_60_seconds() {
        let duration = chrono::Duration::seconds(60);
        assert_eq!(format_duration(duration), "1 minutes ago");
    }

    #[test]
    fn test_preview_config_serialization() {
        let config = PreviewConfig {
            show_branch: true,
            show_last_activity: false,
            show_status: true,
            recent_commits: 10,
            date_format: "%Y-%m-%d".to_string(),
        };

        // Verify all fields are accessible
        assert!(config.show_branch);
        assert!(!config.show_last_activity);
        assert!(config.show_status);
        assert_eq!(config.recent_commits, 10);
        assert_eq!(config.date_format, "%Y-%m-%d");
    }

    #[test]
    fn test_preview_config_clone() {
        let config1 = PreviewConfig {
            show_branch: true,
            show_last_activity: true,
            show_status: false,
            recent_commits: 5,
            date_format: "%Y-%m-%d %H:%M".to_string(),
        };

        let config2 = config1.clone();
        assert_eq!(config1.show_branch, config2.show_branch);
        assert_eq!(config1.show_last_activity, config2.show_last_activity);
        assert_eq!(config1.show_status, config2.show_status);
        assert_eq!(config1.recent_commits, config2.recent_commits);
        assert_eq!(config1.date_format, config2.date_format);
    }

    #[test]
    fn test_format_duration_with_large_negative_value() {
        let duration = chrono::Duration::days(-500);
        assert_eq!(format_duration(duration), "1 years ago");
    }

    #[test]
    fn test_format_duration_very_large_years() {
        let duration = chrono::Duration::days(100000);
        assert_eq!(format_duration(duration), "273 years ago");
    }

    #[test]
    fn test_format_duration_boundary_hour_transitions() {
        // Test boundaries around hour transitions
        let duration_3599 = chrono::Duration::seconds(3599);
        assert_eq!(format_duration(duration_3599), "59 minutes ago");

        let duration_3600 = chrono::Duration::seconds(3600);
        assert_eq!(format_duration(duration_3600), "1 hours ago");

        let duration_3601 = chrono::Duration::seconds(3601);
        assert_eq!(format_duration(duration_3601), "1 hours ago");
    }

    #[test]
    fn test_format_duration_boundary_day_transitions() {
        // Test boundaries around day transitions
        let duration_86399 = chrono::Duration::seconds(86399);
        assert_eq!(format_duration(duration_86399), "23 hours ago");

        let duration_86400 = chrono::Duration::seconds(86400);
        assert_eq!(format_duration(duration_86400), "1 days ago");

        let duration_86401 = chrono::Duration::seconds(86401);
        assert_eq!(format_duration(duration_86401), "1 days ago");
    }

    #[test]
    fn test_format_duration_boundary_week_transitions() {
        // Test boundaries around week transitions
        let duration_604799 = chrono::Duration::seconds(604799);
        assert_eq!(format_duration(duration_604799), "6 days ago");

        let duration_604800 = chrono::Duration::seconds(604800);
        assert_eq!(format_duration(duration_604800), "1 weeks ago");
    }

    #[test]
    fn test_format_duration_boundary_month_transitions() {
        // 30 days in seconds = 2592000, but 6 days = 604800 is threshold to weeks
        let duration_604799 = chrono::Duration::seconds(604799); // Just under 7 days
        assert_eq!(format_duration(duration_604799), "6 days ago");

        let duration_2592000 = chrono::Duration::seconds(2592000); // 30 days
        assert_eq!(format_duration(duration_2592000), "1 months ago");
    }

    #[test]
    fn test_format_duration_boundary_year_transitions() {
        // 365 days in seconds = 31536000
        let duration_31535999 = chrono::Duration::seconds(31535999);
        assert_eq!(format_duration(duration_31535999), "12 months ago");

        let duration_31536000 = chrono::Duration::seconds(31536000);
        assert_eq!(format_duration(duration_31536000), "1 years ago");
    }

    #[test]
    fn test_preview_config_with_zero_commits() {
        let config = PreviewConfig {
            show_branch: true,
            show_last_activity: true,
            show_status: true,
            recent_commits: 0,
            date_format: "%Y-%m-%d".to_string(),
        };

        assert_eq!(config.recent_commits, 0);
    }

    #[test]
    fn test_preview_config_with_many_commits() {
        let config = PreviewConfig {
            show_branch: true,
            show_last_activity: true,
            show_status: true,
            recent_commits: 1000,
            date_format: "%Y-%m-%d".to_string(),
        };

        assert_eq!(config.recent_commits, 1000);
    }

    #[test]
    fn test_preview_config_custom_date_format() {
        let formats = vec![
            "%Y-%m-%d",
            "%d/%m/%Y",
            "%Y-%m-%d %H:%M:%S",
            "%A, %B %d, %Y",
        ];

        for format in formats {
            let config = PreviewConfig {
                show_branch: true,
                show_last_activity: true,
                show_status: true,
                recent_commits: 5,
                date_format: format.to_string(),
            };

            assert_eq!(config.date_format, format);
        }
    }
}
