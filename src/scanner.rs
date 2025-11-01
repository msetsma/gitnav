use anyhow::Result;
use ignore::WalkBuilder;
use serde::Serialize;
use std::path::{Path, PathBuf};

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
pub fn scan_repos<P: AsRef<Path>>(base_path: P, max_depth: usize) -> Result<Vec<GitRepo>> {
    let base_path = base_path.as_ref();

    if !base_path.exists() {
        anyhow::bail!("Base path does not exist: {}", base_path.display());
    }

    let mut repos = Vec::new();

    let walker = WalkBuilder::new(base_path)
        .max_depth(Some(max_depth))
        .hidden(false) // Show hidden directories (needed for .git)
        .follow_links(false)
        .build();

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // Skip inaccessible paths
        };

        let path = entry.path();

        // Check if this is a .git directory
        if path.file_name().and_then(|n| n.to_str()) == Some(".git") && path.is_dir() {
            // Parent directory is the repo
            if let Some(repo_path) = path.parent() {
                repos.push(GitRepo::new(repo_path.to_path_buf()));
            }
        }
    }

    // Sort by repo name for consistent ordering
    repos.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(repos)
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

        // Check first and last
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
        assert_eq!(tab_count, 2); // One tab per repo
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

        // All should have same name but different paths
        for line in lines {
            assert!(line.starts_with("project\t"));
        }
    }
}
