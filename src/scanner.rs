use anyhow::Result;
use git2::{Repository, StatusOptions};
use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::config::BadgeStyle;

/// Represents a git repository found during scanning.
///
/// Contains the repository name (directory name) and its full path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GitRepo {
    pub name: String,
    pub path: PathBuf,
}

impl GitRepo {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Self { name, path }
    }
}

/// Project type detected from marker files in the repository root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Rust,
    Node,
    Go,
    Python,
    Ruby,
    Java,
    CSharp,
    Unknown,
}

impl ProjectType {
    pub fn badge_text(&self) -> &str {
        match self {
            ProjectType::Rust => "rust",
            ProjectType::Node => "node",
            ProjectType::Go => "go",
            ProjectType::Python => "python",
            ProjectType::Ruby => "ruby",
            ProjectType::Java => "java",
            ProjectType::CSharp => "csharp",
            ProjectType::Unknown => "",
        }
    }

    pub fn badge_icon(&self) -> &str {
        match self {
            ProjectType::Rust => "🦀",
            ProjectType::Node => "⬡",
            ProjectType::Go => "🐹",
            ProjectType::Python => "🐍",
            ProjectType::Ruby => "💎",
            ProjectType::Java => "☕",
            ProjectType::CSharp => "⚙",
            ProjectType::Unknown => "",
        }
    }
}

/// Git and project metadata collected via enrichment pass.
#[derive(Debug, Clone)]
pub struct RepoMeta {
    pub branch: Option<String>,
    pub is_dirty: bool,
    #[allow(dead_code)]
    pub is_detached: bool,
    pub project_type: ProjectType,
}

/// A git repository with enriched metadata for display in the fzf picker.
#[derive(Debug, Clone)]
pub struct EnrichedRepo {
    pub name: String,
    pub path: PathBuf,
    pub meta: RepoMeta,
}

/// Detect the primary project type by checking for marker files.
///
/// Checks are ordered by priority. Returns `Unknown` if no known marker is found.
pub fn detect_project_type(path: &Path) -> ProjectType {
    if path.join("Cargo.toml").exists() {
        return ProjectType::Rust;
    }
    if path.join("package.json").exists() {
        return ProjectType::Node;
    }
    if path.join("go.mod").exists() {
        return ProjectType::Go;
    }
    if path.join("pyproject.toml").exists()
        || path.join("setup.py").exists()
        || path.join("requirements.txt").exists()
    {
        return ProjectType::Python;
    }
    if path.join("Gemfile").exists() {
        return ProjectType::Ruby;
    }
    if path.join("pom.xml").exists() || path.join("build.gradle").exists() {
        return ProjectType::Java;
    }
    // CSharp: look for .sln file in the root (bounded scan)
    if let Ok(entries) = std::fs::read_dir(path) {
        let has_sln = entries
            .take(20)
            .filter_map(|e| e.ok())
            .any(|e| e.path().extension().and_then(|x| x.to_str()) == Some("sln"));
        if has_sln {
            return ProjectType::CSharp;
        }
    }
    ProjectType::Unknown
}

/// Collect git metadata for a single repository path.
fn enrich_single(path: &Path) -> RepoMeta {
    let project_type = detect_project_type(path);

    let git_repo = match Repository::open(path) {
        Ok(r) => r,
        Err(_) => {
            return RepoMeta {
                branch: None,
                is_dirty: false,
                is_detached: false,
                project_type,
            }
        }
    };

    let (branch, is_detached) = match git_repo.head() {
        Ok(head) => {
            let detached = !head.is_branch();
            let name = if head.is_branch() {
                head.shorthand().map(|s| s.to_string())
            } else {
                Some("(detached)".to_string())
            };
            (name, detached)
        }
        Err(_) => (None, false),
    };

    let is_dirty = {
        let mut opts = StatusOptions::new();
        opts.include_untracked(false)
            .include_ignored(false)
            .recurse_untracked_dirs(false);

        match git_repo.statuses(Some(&mut opts)) {
            Ok(statuses) => statuses.iter().any(|s| {
                let flags = s.status();
                flags.is_index_modified()
                    || flags.is_index_new()
                    || flags.is_index_deleted()
                    || flags.is_wt_modified()
                    || flags.is_wt_deleted()
            }),
            Err(_) => false,
        }
    };

    RepoMeta {
        branch,
        is_dirty,
        is_detached,
        project_type,
    }
}

/// Enrich a list of repos with git metadata and project type.
///
/// Opens each repo with git2 to read branch, dirty status, and detached HEAD state.
/// Repos that cannot be opened are still included with empty metadata.
pub fn enrich_repos(repos: Vec<GitRepo>) -> Vec<EnrichedRepo> {
    repos
        .into_iter()
        .map(|repo| {
            let meta = enrich_single(&repo.path);
            EnrichedRepo {
                name: repo.name,
                path: repo.path,
                meta,
            }
        })
        .collect()
}

/// Format a single enriched repo's display string for the fzf list.
///
/// The name is padded to `name_width` for alignment. Branch and dirty indicator
/// are appended when present. Project badge is appended based on `badge_style`.
pub fn format_display(
    repo: &EnrichedRepo,
    name_width: usize,
    use_color: bool,
    badge_style: &BadgeStyle,
) -> String {
    let padded_name = format!("{:<width$}", repo.name, width = name_width);
    let mut parts: Vec<String> = vec![padded_name];

    if let Some(ref branch) = repo.meta.branch {
        let branch_str = if use_color {
            format!("\x1b[0;36m{}\x1b[0m", branch)
        } else {
            branch.clone()
        };
        parts.push(branch_str);

        if repo.meta.is_dirty {
            let dot = if use_color {
                "\x1b[0;33m●\x1b[0m".to_string()
            } else {
                "●".to_string()
            };
            parts.push(dot);
        }
    }

    let badge = match badge_style {
        BadgeStyle::None => String::new(),
        BadgeStyle::Text => {
            let text = repo.meta.project_type.badge_text();
            if text.is_empty() {
                String::new()
            } else {
                format!("[{}]", text)
            }
        }
        BadgeStyle::Icon => repo.meta.project_type.badge_icon().to_string(),
    };

    if !badge.is_empty() {
        parts.push(badge);
    }

    parts.join("  ")
}

/// Check if a path contains a component matching any of the ignore patterns.
fn should_ignore_path(path: &Path, ignore_patterns: &[String]) -> bool {
    if ignore_patterns.is_empty() {
        return false;
    }
    path.components().any(|component| {
        if let std::path::Component::Normal(os_str) = component {
            if let Some(s) = os_str.to_str() {
                return ignore_patterns.iter().any(|p| p == s);
            }
        }
        false
    })
}

/// Internal scanner implementation used by both `scan_repos` and `scan_repos_multi`.
fn scan_repos_inner(
    base_path: &Path,
    max_depth: usize,
    ignore_patterns: &[String],
) -> Result<Vec<GitRepo>> {
    if !base_path.exists() {
        anyhow::bail!("Base path does not exist: {}", base_path.display());
    }

    let mut repos = Vec::new();

    let walker = WalkBuilder::new(base_path)
        .max_depth(Some(max_depth))
        .hidden(false)
        .follow_links(false)
        .build();

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.file_name().and_then(|n| n.to_str()) == Some(".git") && path.is_dir() {
            if let Some(repo_path) = path.parent() {
                if !should_ignore_path(repo_path, ignore_patterns) {
                    repos.push(GitRepo::new(repo_path.to_path_buf()));
                }
            }
        }
    }

    Ok(repos)
}

/// Scan for git repositories starting from a base path up to a maximum depth.
///
/// Searches for `.git` directories and returns their parent directories as repositories.
/// Uses efficient directory traversal and respects `.gitignore` files.
///
/// # Arguments
///
/// * `base_path` - The starting directory to scan from
/// * `max_depth` - Maximum directory depth to traverse
///
/// # Returns
///
/// A vector of `GitRepo` instances found, sorted by name
///
/// # Errors
///
/// Returns an error if the base path does not exist or cannot be accessed
#[allow(dead_code)]
pub fn scan_repos<P: AsRef<Path>>(base_path: P, max_depth: usize) -> Result<Vec<GitRepo>> {
    let mut repos = scan_repos_inner(base_path.as_ref(), max_depth, &[])?;
    repos.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(repos)
}

/// Scan multiple base paths and merge results, deduplicating by path.
///
/// Paths are expanded (tilde expansion must be done before calling).
/// Results are merged, deduplicated by path, and sorted alphabetically by name.
///
/// # Arguments
///
/// * `paths` - The directories to scan
/// * `max_depth` - Maximum directory depth to traverse in each path
/// * `ignore_patterns` - Directory names to skip (e.g. `["node_modules", "vendor"]`)
///
/// # Errors
///
/// Returns an error if any path cannot be accessed
pub fn scan_repos_multi(
    paths: &[String],
    max_depth: usize,
    ignore_patterns: &[String],
) -> Result<Vec<GitRepo>> {
    let mut all_repos: Vec<GitRepo> = Vec::new();

    for path_str in paths {
        let path = Path::new(path_str);
        match scan_repos_inner(path, max_depth, ignore_patterns) {
            Ok(mut repos) => all_repos.append(&mut repos),
            Err(e) => {
                // Log warning but continue with other paths
                eprintln!("Warning: skipping path '{}': {}", path_str, e);
            }
        }
    }

    // Deduplicate by canonical path
    all_repos.sort_by(|a, b| a.path.cmp(&b.path));
    all_repos.dedup_by(|a, b| a.path == b.path);

    // Sort by name for display
    all_repos.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(all_repos)
}

/// Format repositories as tab-separated values for fzf input.
///
/// Each line contains: `name\tpath`
///
/// # Arguments
///
/// * `repos` - Slice of repositories to format
///
/// # Returns
///
/// A string with repositories formatted as TSV, one per line
#[allow(dead_code)]
pub fn format_for_fzf(repos: &[GitRepo]) -> String {
    repos
        .iter()
        .map(|repo| format!("{}\t{}", repo.name, repo.path.display()))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_for_fzf() {
        let repos = vec![
            GitRepo {
                name: "repo1".to_string(),
                path: PathBuf::from("/home/user/repo1"),
            },
            GitRepo {
                name: "repo2".to_string(),
                path: PathBuf::from("/home/user/repo2"),
            },
        ];

        let output = format_for_fzf(&repos);
        assert!(output.contains("repo1\t/home/user/repo1"));
        assert!(output.contains("repo2\t/home/user/repo2"));
    }

    #[test]
    fn test_format_for_fzf_empty_repos() {
        let repos: Vec<GitRepo> = vec![];
        let output = format_for_fzf(&repos);
        assert_eq!(output, "");
    }

    #[test]
    fn test_format_for_fzf_single_repo() {
        let repos = vec![GitRepo {
            name: "single-repo".to_string(),
            path: PathBuf::from("/home/user/single-repo"),
        }];

        let output = format_for_fzf(&repos);
        assert_eq!(output, "single-repo\t/home/user/single-repo");
    }

    #[test]
    fn test_format_for_fzf_tab_separated() {
        let repos = vec![GitRepo {
            name: "test".to_string(),
            path: PathBuf::from("/path/to/test"),
        }];

        let output = format_for_fzf(&repos);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains('\t'));

        let parts: Vec<&str> = lines[0].split('\t').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "test");
        assert_eq!(parts[1], "/path/to/test");
    }

    #[test]
    fn test_format_for_fzf_newline_separated() {
        let repos = vec![
            GitRepo {
                name: "repo1".to_string(),
                path: PathBuf::from("/path/to/repo1"),
            },
            GitRepo {
                name: "repo2".to_string(),
                path: PathBuf::from("/path/to/repo2"),
            },
            GitRepo {
                name: "repo3".to_string(),
                path: PathBuf::from("/path/to/repo3"),
            },
        ];

        let output = format_for_fzf(&repos);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_git_repo_new_extracts_name() {
        let repo = GitRepo::new(PathBuf::from("/home/user/my-awesome-repo"));
        assert_eq!(repo.name, "my-awesome-repo");
        assert_eq!(repo.path, PathBuf::from("/home/user/my-awesome-repo"));
    }

    #[test]
    fn test_git_repo_new_with_single_component_path() {
        let repo = GitRepo::new(PathBuf::from("repo"));
        assert_eq!(repo.name, "repo");
    }

    #[test]
    fn test_git_repo_new_handles_unicode_names() {
        let repo = GitRepo::new(PathBuf::from("/home/user/мой-репозиторий"));
        assert_eq!(repo.name, "мой-репозиторий");
    }

    #[test]
    fn test_git_repo_clone() {
        let repo1 = GitRepo {
            name: "test".to_string(),
            path: PathBuf::from("/path/to/test"),
        };

        let repo2 = repo1.clone();
        assert_eq!(repo1.name, repo2.name);
        assert_eq!(repo1.path, repo2.path);
    }

    #[test]
    fn test_format_for_fzf_preserves_order() {
        let repos = vec![
            GitRepo {
                name: "zebra".to_string(),
                path: PathBuf::from("/path/to/zebra"),
            },
            GitRepo {
                name: "apple".to_string(),
                path: PathBuf::from("/path/to/apple"),
            },
            GitRepo {
                name: "middle".to_string(),
                path: PathBuf::from("/path/to/middle"),
            },
        ];

        let output = format_for_fzf(&repos);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[0], "zebra\t/path/to/zebra");
        assert_eq!(lines[1], "apple\t/path/to/apple");
        assert_eq!(lines[2], "middle\t/path/to/middle");
    }

    #[test]
    fn test_git_repo_path_with_spaces() {
        let repo = GitRepo {
            name: "my repo".to_string(),
            path: PathBuf::from("/path/with spaces/my repo"),
        };

        let output = format_for_fzf(&[repo]);
        assert!(output.contains("my repo\t/path/with spaces/my repo"));
    }

    #[test]
    fn test_git_repo_path_with_special_chars() {
        let repo = GitRepo {
            name: "repo-name_123".to_string(),
            path: PathBuf::from("/path/to/repo-name_123"),
        };

        let output = format_for_fzf(&[repo]);
        assert!(output.contains("repo-name_123\t/path/to/repo-name_123"));
    }

    #[test]
    fn test_git_repo_debug_trait() {
        let repo = GitRepo {
            name: "test".to_string(),
            path: PathBuf::from("/test"),
        };

        let debug_str = format!("{:?}", repo);
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_git_repo_equality() {
        let repo1 = GitRepo {
            name: "test".to_string(),
            path: PathBuf::from("/test"),
        };

        let repo2 = GitRepo {
            name: "test".to_string(),
            path: PathBuf::from("/test"),
        };

        let repo3 = GitRepo {
            name: "different".to_string(),
            path: PathBuf::from("/test"),
        };

        assert_eq!(repo1, repo2);
        assert_ne!(repo1, repo3);
    }

    #[test]
    fn test_format_for_fzf_with_long_paths() {
        let long_path =
            "/very/long/path/to/some/deeply/nested/repository/directory/with/many/levels";
        let repo = GitRepo {
            name: "deep-repo".to_string(),
            path: PathBuf::from(long_path),
        };

        let output = format_for_fzf(&[repo]);
        assert!(output.contains("deep-repo"));
        assert!(output.contains(long_path));
    }

    #[test]
    fn test_git_repo_new_with_path_ending_in_slash() {
        let repo = GitRepo::new(PathBuf::from("/home/user/repo/"));
        assert_eq!(repo.name, "repo");
    }

    #[test]
    fn test_format_for_fzf_many_repos() {
        let repos: Vec<GitRepo> = (0..100)
            .map(|i| GitRepo {
                name: format!("repo{}", i),
                path: PathBuf::from(format!("/path/to/repo{}", i)),
            })
            .collect();

        let output = format_for_fzf(&repos);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 100);

        assert!(lines[0].contains("repo0"));
        assert!(lines[99].contains("repo99"));
    }

    #[test]
    fn test_git_repo_with_dot_names() {
        let repo = GitRepo {
            name: ".config".to_string(),
            path: PathBuf::from("/home/user/.config"),
        };

        let output = format_for_fzf(&[repo]);
        assert!(output.contains(".config\t/home/user/.config"));
    }

    #[test]
    fn test_git_repo_with_numeric_names() {
        let repo = GitRepo {
            name: "12345".to_string(),
            path: PathBuf::from("/path/12345"),
        };

        let output = format_for_fzf(&[repo]);
        assert!(output.contains("12345\t/path/12345"));
    }

    #[test]
    fn test_format_for_fzf_tab_character_in_output() {
        let repos = vec![
            GitRepo {
                name: "repo1".to_string(),
                path: PathBuf::from("/path/1"),
            },
            GitRepo {
                name: "repo2".to_string(),
                path: PathBuf::from("/path/2"),
            },
        ];

        let output = format_for_fzf(&repos);
        let tab_count = output.matches('\t').count();
        assert_eq!(tab_count, 2);
    }

    #[test]
    fn test_git_repo_new_extracts_correct_name_from_nested_path() {
        let paths = vec![
            ("/a/b/c/d/repo", "repo"),
            ("/deep/nested/path/my-project", "my-project"),
            ("/x/y/z/test_repo", "test_repo"),
        ];

        for (path, expected_name) in paths {
            let repo = GitRepo::new(PathBuf::from(path));
            assert_eq!(repo.name, expected_name);
        }
    }

    #[test]
    fn test_git_repo_new_handles_non_ascii_gracefully() {
        let repo = GitRepo::new(PathBuf::from("/home/user/🚀-project"));
        assert_eq!(repo.name, "🚀-project");
    }

    #[test]
    fn test_format_for_fzf_with_similar_names() {
        let repos = vec![
            GitRepo {
                name: "project".to_string(),
                path: PathBuf::from("/path/1/project"),
            },
            GitRepo {
                name: "project".to_string(),
                path: PathBuf::from("/path/2/project"),
            },
            GitRepo {
                name: "project".to_string(),
                path: PathBuf::from("/path/3/project"),
            },
        ];

        let output = format_for_fzf(&repos);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3);

        for line in lines {
            assert!(line.starts_with("project\t"));
        }
    }

    #[test]
    fn test_detect_project_type_rust() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Rust);
    }

    #[test]
    fn test_detect_project_type_node() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("package.json"), "{}").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Node);
    }

    #[test]
    fn test_detect_project_type_go() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("go.mod"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Go);
    }

    #[test]
    fn test_detect_project_type_python() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("requirements.txt"), "").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Python);
    }

    #[test]
    fn test_detect_project_type_unknown() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Unknown);
    }

    #[test]
    fn test_detect_project_type_rust_takes_priority() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "").unwrap();
        std::fs::write(dir.path().join("package.json"), "{}").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Rust);
    }

    #[test]
    fn test_project_type_badge_text() {
        assert_eq!(ProjectType::Rust.badge_text(), "rust");
        assert_eq!(ProjectType::Node.badge_text(), "node");
        assert_eq!(ProjectType::Go.badge_text(), "go");
        assert_eq!(ProjectType::Unknown.badge_text(), "");
    }

    #[test]
    fn test_format_display_no_meta() {
        let repo = EnrichedRepo {
            name: "myrepo".to_string(),
            path: PathBuf::from("/path/myrepo"),
            meta: RepoMeta {
                branch: None,
                is_dirty: false,
                is_detached: false,
                project_type: ProjectType::Unknown,
            },
        };
        let display = format_display(&repo, 6, false, &BadgeStyle::None);
        assert_eq!(display, "myrepo");
    }

    #[test]
    fn test_format_display_with_branch() {
        let repo = EnrichedRepo {
            name: "myrepo".to_string(),
            path: PathBuf::from("/path/myrepo"),
            meta: RepoMeta {
                branch: Some("main".to_string()),
                is_dirty: false,
                is_detached: false,
                project_type: ProjectType::Unknown,
            },
        };
        let display = format_display(&repo, 6, false, &BadgeStyle::None);
        assert!(display.contains("main"));
        assert!(!display.contains('●'));
    }

    #[test]
    fn test_format_display_dirty() {
        let repo = EnrichedRepo {
            name: "myrepo".to_string(),
            path: PathBuf::from("/path/myrepo"),
            meta: RepoMeta {
                branch: Some("main".to_string()),
                is_dirty: true,
                is_detached: false,
                project_type: ProjectType::Unknown,
            },
        };
        let display = format_display(&repo, 6, false, &BadgeStyle::None);
        assert!(display.contains('●'));
    }

    #[test]
    fn test_format_display_badge_text() {
        let repo = EnrichedRepo {
            name: "proj".to_string(),
            path: PathBuf::from("/path/proj"),
            meta: RepoMeta {
                branch: Some("main".to_string()),
                is_dirty: false,
                is_detached: false,
                project_type: ProjectType::Rust,
            },
        };
        let display = format_display(&repo, 4, false, &BadgeStyle::Text);
        assert!(display.contains("[rust]"));
    }

    #[test]
    fn test_should_ignore_path() {
        let path = Path::new("/home/user/node_modules/some-pkg");
        let patterns = vec!["node_modules".to_string()];
        assert!(should_ignore_path(path, &patterns));

        let path2 = Path::new("/home/user/myproject");
        assert!(!should_ignore_path(path2, &patterns));
    }

    #[test]
    fn test_should_ignore_path_empty_patterns() {
        let path = Path::new("/home/user/node_modules/some-pkg");
        assert!(!should_ignore_path(path, &[]));
    }

    #[test]
    fn test_scan_repos_multi_deduplicates() {
        // Two identical paths should yield same repos once
        let tmp = tempfile::tempdir().unwrap();
        let repo_path = tmp.path().join("myrepo");
        std::fs::create_dir_all(repo_path.join(".git")).unwrap();

        let path_str = tmp.path().to_string_lossy().to_string();
        let repos =
            scan_repos_multi(&[path_str.clone(), path_str], 5, &[]).unwrap();

        let names: Vec<&str> = repos.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names.iter().filter(|&&n| n == "myrepo").count(), 1);
    }
}
