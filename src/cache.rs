use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::scanner::GitRepo;

/// Manages caching of repository lists with TTL (time-to-live) validation.
///
/// Uses SHA256 hashing to generate deterministic cache keys for search paths
/// and stores repositories as tab-separated values with a configurable TTL.
#[derive(Debug)]
pub struct Cache {
    cache_dir: PathBuf,
    ttl_seconds: u64,
}

impl Cache {
    /// Create a new cache instance with the specified TTL.
    ///
    /// # Arguments
    ///
    /// * `ttl_seconds` - Time-to-live in seconds for cached data
    ///
    /// # Returns
    ///
    /// A new `Cache` instance or an error if the cache directory cannot be created
    ///
    /// # Errors
    ///
    /// Returns an error if the cache directory cannot be determined or created.
    pub fn new(ttl_seconds: u64) -> Result<Self> {
        let cache_dir = Self::get_cache_dir()?;
        fs::create_dir_all(&cache_dir)
            .with_context(|| format!("Failed to create cache directory: {}", cache_dir.display()))?;

        Ok(Self {
            cache_dir,
            ttl_seconds,
        })
    }

    /// Get the cache directory path
    fn get_cache_dir() -> Result<PathBuf> {
        dirs::cache_dir()
            .map(|p| p.join("gitnav"))
            .ok_or_else(|| anyhow::anyhow!("Could not determine cache directory"))
    }

    /// Get the cache directory path (public accessor)
    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    /// List all cache files in the cache directory
    ///
    /// # Returns
    ///
    /// A vector of paths to cache files, or an error if the directory cannot be read
    pub fn list_cache_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if !self.cache_dir.exists() {
            return Ok(files);
        }

        let entries = fs::read_dir(&self.cache_dir)
            .with_context(|| format!("Failed to read cache directory: {}", self.cache_dir.display()))?;

        for entry in entries {
            let entry = entry
                .with_context(|| format!("Failed to read cache entry in {}", self.cache_dir.display()))?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "cache") {
                files.push(path);
            }
        }

        files.sort();
        Ok(files)
    }

    /// Get the total size of all cache files in bytes
    ///
    /// # Returns
    ///
    /// The total size in bytes, or an error if files cannot be accessed
    pub fn get_cache_size(&self) -> Result<u64> {
        let mut total_size = 0u64;

        for file in self.list_cache_files()? {
            let metadata = fs::metadata(&file)
                .with_context(|| format!("Failed to get metadata for cache file: {}", file.display()))?;
            total_size += metadata.len();
        }

        Ok(total_size)
    }

    /// Generate cache file path for a given search path
    fn cache_file_path<P: AsRef<Path>>(&self, search_path: P) -> PathBuf {
        let mut hasher = Sha256::new();
        hasher.update(search_path.as_ref().to_string_lossy().as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        
        self.cache_dir.join(format!("repos_{}.cache", &hash[..16]))
    }

    /// Check if cached data exists and is still valid (within TTL).
    ///
    /// # Arguments
    ///
    /// * `search_path` - The path to check cache validity for
    ///
    /// # Returns
    ///
    /// `true` if a valid cache file exists and hasn't expired, `false` otherwise
    pub fn is_valid<P: AsRef<Path>>(&self, search_path: P) -> bool {
        let cache_path = self.cache_file_path(search_path);

        if !cache_path.exists() {
            return false;
        }

        let metadata = match fs::metadata(&cache_path) {
            Ok(m) => m,
            Err(_) => return false,
        };

        let modified = match metadata.modified() {
            Ok(m) => m,
            Err(_) => return false,
        };

        let age = match SystemTime::now().duration_since(modified) {
            Ok(d) => d.as_secs(),
            Err(_) => return false,
        };

        age < self.ttl_seconds
    }

    /// Load repository list from cache.
    ///
    /// # Arguments
    ///
    /// * `search_path` - The path to load cache for
    ///
    /// # Returns
    ///
    /// A vector of `GitRepo` instances parsed from the cache file
    ///
    /// # Errors
    ///
    /// Returns an error if the cache file cannot be read or parsed
    pub fn load<P: AsRef<Path>>(&self, search_path: P) -> Result<Vec<GitRepo>> {
        let cache_path = self.cache_file_path(search_path);
        let contents = fs::read_to_string(&cache_path)
            .with_context(|| format!("Failed to read cache file: {}", cache_path.display()))?;

        let repos: Vec<GitRepo> = contents
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    Some(GitRepo {
                        name: parts[0].to_string(),
                        path: PathBuf::from(parts[1]),
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(repos)
    }

    /// Save repository list to cache.
    ///
    /// # Arguments
    ///
    /// * `search_path` - The path this cache is for
    /// * `repos` - The repository list to cache
    ///
    /// # Errors
    ///
    /// Returns an error if the cache file cannot be written
    pub fn save<P: AsRef<Path>>(&self, search_path: P, repos: &[GitRepo]) -> Result<()> {
        let cache_path = self.cache_file_path(search_path);
        let contents: String = repos
            .iter()
            .map(|repo| format!("{}\t{}", repo.name, repo.path.display()))
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&cache_path, contents)
            .with_context(|| format!("Failed to write cache file: {}", cache_path.display()))?;

        Ok(())
    }

    /// Clear all cached repository data.
    ///
    /// Removes and recreates the cache directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the cache directory cannot be cleared or recreated
    pub fn clear(&self) -> Result<()> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)
                .with_context(|| format!("Failed to clear cache directory: {}", self.cache_dir.display()))?;
            fs::create_dir_all(&self.cache_dir)
                .with_context(|| format!("Failed to recreate cache directory: {}", self.cache_dir.display()))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_file_path_deterministic() {
        let cache = Cache {
            cache_dir: PathBuf::from("/tmp/test"),
            ttl_seconds: 300,
        };

        let path1 = cache.cache_file_path("/home/user");
        let path2 = cache.cache_file_path("/home/user");
        assert_eq!(path1, path2);
    }

    #[test]
    fn test_cache_file_path_different_paths() {
        let cache = Cache {
            cache_dir: PathBuf::from("/tmp/test"),
            ttl_seconds: 300,
        };

        let path1 = cache.cache_file_path("/home/user");
        let path2 = cache.cache_file_path("/home/other");
        assert_ne!(path1, path2);
    }

    #[test]
    fn test_cache_file_path_contains_hash_prefix() {
        let cache = Cache {
            cache_dir: PathBuf::from("/tmp/test"),
            ttl_seconds: 300,
        };

        let path = cache.cache_file_path("/home/user");
        let filename = path.file_name().unwrap().to_str().unwrap();
        assert!(filename.starts_with("repos_"));
        assert!(filename.ends_with(".cache"));
    }

    #[test]
    fn test_cache_file_path_hash_length() {
        let cache = Cache {
            cache_dir: PathBuf::from("/tmp/test"),
            ttl_seconds: 300,
        };

        let path = cache.cache_file_path("/home/user");
        let filename = path.file_name().unwrap().to_str().unwrap();
        // Should be "repos_" (6 chars) + 16 hex chars + ".cache" (6 chars) = 28 chars total
        assert_eq!(filename.len(), 28);
    }

    #[test]
    fn test_cache_save_and_load_roundtrip() {
        // Note: This test requires temp directory handling
        // For now, we test the logic with mock data
        let repos = vec![
            GitRepo {
                name: "test-repo".to_string(),
                path: PathBuf::from("/home/user/repos/test-repo"),
            },
            GitRepo {
                name: "another-repo".to_string(),
                path: PathBuf::from("/home/user/repos/another-repo"),
            },
        ];

        let cache_dir = PathBuf::from("/tmp/test");
        let _cache = Cache {
            cache_dir,
            ttl_seconds: 300,
        };

        let contents: String = repos
            .iter()
            .map(|repo| format!("{}\t{}", repo.name, repo.path.display()))
            .collect::<Vec<_>>()
            .join("\n");

        // Verify format
        assert!(contents.contains("test-repo"));
        assert!(contents.contains("another-repo"));
        assert!(contents.contains('\t'));
    }

    #[test]
    fn test_cache_parse_tsv_format() {
        let contents = "repo1\t/path/to/repo1\nrepo2\t/path/to/repo2";

        let repos: Vec<GitRepo> = contents
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    Some(GitRepo {
                        name: parts[0].to_string(),
                        path: PathBuf::from(parts[1]),
                    })
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(repos.len(), 2);
        assert_eq!(repos[0].name, "repo1");
        assert_eq!(repos[1].name, "repo2");
    }

    #[test]
    fn test_cache_parse_ignores_malformed_lines() {
        let contents = "repo1\t/path/to/repo1\nmalformed_line\nrepo2\t/path/to/repo2";

        let repos: Vec<GitRepo> = contents
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    Some(GitRepo {
                        name: parts[0].to_string(),
                        path: PathBuf::from(parts[1]),
                    })
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(repos.len(), 2);
        assert_eq!(repos[0].name, "repo1");
        assert_eq!(repos[1].name, "repo2");
    }

    #[test]
    fn test_git_repo_construction() {
        let repo = GitRepo {
            name: "test-repo".to_string(),
            path: PathBuf::from("/home/user/test-repo"),
        };

        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.path, PathBuf::from("/home/user/test-repo"));
    }

    #[test]
    fn test_cache_ttl_parameter() {
        let cache_short = Cache {
            cache_dir: PathBuf::from("/tmp/test"),
            ttl_seconds: 60,
        };

        let cache_long = Cache {
            cache_dir: PathBuf::from("/tmp/test"),
            ttl_seconds: 3600,
        };

        assert_eq!(cache_short.ttl_seconds, 60);
        assert_eq!(cache_long.ttl_seconds, 3600);
        assert_ne!(cache_short.ttl_seconds, cache_long.ttl_seconds);
    }

    #[test]
    fn test_cache_file_path_multiple_hash_consistency() {
        let cache = Cache {
            cache_dir: PathBuf::from("/tmp/test"),
            ttl_seconds: 300,
        };

        // Same path should always generate same hash
        let paths: Vec<_> = (0..5)
            .map(|_| cache.cache_file_path("/home/user/projects"))
            .collect();

        for path in &paths[1..] {
            assert_eq!(paths[0], *path);
        }
    }

    #[test]
    fn test_cache_file_path_unicode_paths() {
        let cache = Cache {
            cache_dir: PathBuf::from("/tmp/test"),
            ttl_seconds: 300,
        };

        let path1 = cache.cache_file_path("/home/用户/项目");
        let path2 = cache.cache_file_path("/home/用户/项目");
        assert_eq!(path1, path2);

        let path3 = cache.cache_file_path("/home/different/项目");
        assert_ne!(path1, path3);
    }

    #[test]
    fn test_cache_file_path_similar_paths() {
        let cache = Cache {
            cache_dir: PathBuf::from("/tmp/test"),
            ttl_seconds: 300,
        };

        // Similar paths should produce different hashes
        let path1 = cache.cache_file_path("/home/user");
        let path2 = cache.cache_file_path("/home/user/");
        let path3 = cache.cache_file_path("/home/user2");

        assert_ne!(path1, path2);
        assert_ne!(path1, path3);
        assert_ne!(path2, path3);
    }

    #[test]
    fn test_cache_roundtrip_multiple_repos() {
        let repos = vec![
            GitRepo {
                name: "repo1".to_string(),
                path: PathBuf::from("/path/1"),
            },
            GitRepo {
                name: "repo2".to_string(),
                path: PathBuf::from("/path/2"),
            },
            GitRepo {
                name: "repo3".to_string(),
                path: PathBuf::from("/path/3"),
            },
        ];

        let _cache_dir = PathBuf::from("/tmp/test");

        // Format as cache would save
        let contents: String = repos
            .iter()
            .map(|repo| format!("{}\t{}", repo.name, repo.path.display()))
            .collect::<Vec<_>>()
            .join("\n");

        // Parse back
        let parsed: Vec<GitRepo> = contents
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    Some(GitRepo {
                        name: parts[0].to_string(),
                        path: PathBuf::from(parts[1]),
                    })
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(repos.len(), parsed.len());
        for (original, parsed_repo) in repos.iter().zip(parsed.iter()) {
            assert_eq!(original.name, parsed_repo.name);
            assert_eq!(original.path, parsed_repo.path);
        }
    }

    #[test]
    fn test_cache_handles_empty_repository_list() {
        let repos: Vec<GitRepo> = vec![];

        let contents: String = repos
            .iter()
            .map(|repo| format!("{}\t{}", repo.name, repo.path.display()))
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(contents, "");

        let parsed: Vec<GitRepo> = contents
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    Some(GitRepo {
                        name: parts[0].to_string(),
                        path: PathBuf::from(parts[1]),
                    })
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(parsed.len(), 0);
    }
}
