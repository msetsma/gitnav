use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
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

/// Scan for git repositories starting from base_path up to max_depth
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

/// Format repos as TSV for fzf (name\tpath)
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
}
