mod cache;
mod config;
mod fzf;
mod preview;
mod scanner;
mod shell;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gitnav")]
#[command(author, version, about = "Fast git repository navigator with fuzzy finding", long_about = None)]
struct Cli {
    /// Force refresh (bypass cache)
    #[arg(short, long)]
    force: bool,

    /// Override base search path
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Override max search depth
    #[arg(short = 'd', long)]
    max_depth: Option<usize>,

    /// Path to custom config file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Generate shell preview for a repository path (internal use by fzf)
    #[arg(long, hide = true)]
    preview: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell integration script
    Init {
        /// Shell type (zsh, bash, fish)
        shell: String,
    },
    /// Print example config file
    Config,
    /// Clear all cache files
    ClearCache,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        return handle_subcommand(command);
    }

    // Handle preview mode (called by fzf)
    if let Some(repo_path) = cli.preview {
        return handle_preview(&repo_path);
    }

    // Main navigation mode
    run_navigation(&cli)
}

fn handle_subcommand(command: Commands) -> Result<()> {
    match command {
        Commands::Init { shell } => {
            if let Some(script) = shell::generate_init_script(&shell) {
                print!("{}", script);
                Ok(())
            } else {
                anyhow::bail!(
                    "Unsupported shell: {}. Supported shells: zsh, bash, fish",
                    shell
                );
            }
        }
        Commands::Config => {
            println!("{}", config::Config::example_toml());
            Ok(())
        }
        Commands::ClearCache => {
            let config = config::Config::load(None)?;
            let cache = cache::Cache::new(config.cache.ttl_seconds)?;
            cache.clear()?;
            eprintln!("Cache cleared successfully");
            Ok(())
        }
    }
}

fn handle_preview(repo_path: &PathBuf) -> Result<()> {
    let config = config::Config::load(None)?;
    let preview_text = preview::generate_preview(repo_path, &config.preview)?;
    println!("{}", preview_text);
    Ok(())
}

fn run_navigation(cli: &Cli) -> Result<()> {
    // Check if fzf is available
    if !fzf::is_fzf_available() {
        anyhow::bail!("fzf is not installed or not in PATH. Please install fzf first.");
    }

    // Load configuration
    let config = config::Config::load(cli.config.clone())?;

    // Determine search path and depth
    let search_path = cli
        .path
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| config.search.base_path.clone());

    let search_path = shellexpand::tilde(&search_path).to_string();
    let max_depth = cli.max_depth.unwrap_or(config.search.max_depth);

    // Get repos (from cache or fresh scan)
    let repos = if config.cache.enabled && !cli.force {
        let cache = cache::Cache::new(config.cache.ttl_seconds)?;
        
        if cache.is_valid(&search_path) {
            cache.load(&search_path)?
        } else {
            let repos = scanner::scan_repos(&search_path, max_depth)?;
            cache.save(&search_path, &repos)?;
            repos
        }
    } else {
        scanner::scan_repos(&search_path, max_depth)?
    };

    if repos.is_empty() {
        eprintln!("No git repositories found in {}", search_path);
        std::process::exit(1);
    }

    // Get path to current binary for preview
    let current_exe = std::env::current_exe()
        .context("Failed to get current executable path")?;
    let binary_path = current_exe.to_string_lossy();

    // Run fzf and get selection
    match fzf::select_repo(&repos, &config, &binary_path)? {
        Some(selected_path) => {
            // Output selected path to stdout (shell wrapper will cd to it)
            println!("{}", selected_path);
            Ok(())
        }
        None => {
            // User cancelled
            std::process::exit(130);
        }
    }
}
