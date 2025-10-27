use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::scanner::GitRepo;

pub struct Cache {
    cache_dir: PathBuf,
    ttl_seconds: u64,
}

impl Cache {
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

    /// Generate cache file path for a given search path
    fn cache_file_path<P: AsRef<Path>>(&self, search_path: P) -> PathBuf {
        let mut hasher = Sha256::new();
        hasher.update(search_path.as_ref().to_string_lossy().as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        
        self.cache_dir.join(format!("repos_{}.cache", &hash[..16]))
    }

    /// Check if cache file exists and is within TTL
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

    /// Load repos from cache
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

    /// Save repos to cache
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

    /// Clear all cache files
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
}
