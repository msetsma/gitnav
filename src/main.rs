mod cache;
mod config;
mod exit_codes;
mod fzf;
mod output;
mod preview;
mod scanner;
mod shell;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gitnav")]
#[command(author, version)]
#[command(about = "Fast git repository navigator with fuzzy finding")]
#[command(long_about = "gitnav - Fast git repository navigator\n\n\
EXAMPLES:\n  \
Interactive Mode:\n    \
gn                              # Navigate to repository interactively\n    \
gn -f                           # Force cache refresh\n    \
gn --path ~/projects            # Search in specific directory\n    \
gn --path ~/work --max-depth 8  # Search deeper\n\n  \
Non-Interactive (Scripting):\n    \
gn --list                       # List all repositories\n    \
gn --list --json                # Output as JSON\n    \
gn --list > repos.txt           # Save to file\n\n  \
Cache Management:\n    \
gn clear-cache                  # Clear all cached data\n    \
gn clear-cache --dry-run        # Preview what will be deleted\n\n  \
Configuration:\n    \
gitnav config                   # Show example configuration\n    \
gitnav init zsh                 # Generate shell integration\n    \
gitnav version --verbose        # Show detailed version info\n\n\
ENVIRONMENT:\n  \
NO_COLOR=1                      # Disable colored output\n  \
GITNAV_BASE_PATH=~/projects     # Change default search path\n  \
GITNAV_MAX_DEPTH=10             # Change maximum search depth\n\n\
HELP:\n  \
Use 'gitnav <COMMAND> --help' for detailed command information")]
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

    /// List repositories without launching fzf (enables piping)
    #[arg(short, long)]
    list: bool,

    /// Output as JSON (for scripting)
    #[arg(long)]
    json: bool,

    /// Suppress non-error output
    #[arg(short, long)]
    quiet: bool,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Enable debug output
    #[arg(long)]
    debug: bool,

    /// Generate shell preview for a repository path (internal use by fzf)
    #[arg(long, hide = true)]
    preview: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell integration script for interactive navigation
    ///
    /// Creates a shell function that allows you to use gitnav from your shell.
    /// After running this, you can use 'gn' as a shortcut to navigate.
    ///
    /// EXAMPLE:
    ///   eval "$(gitnav init zsh)" # For Zsh
    ///   eval "$(gitnav init bash)"  # For Bash
    Init {
        /// Shell type: zsh, bash, fish, nu, or nushell
        shell: String,
    },

    /// Print example configuration file to stdout
    ///
    /// Outputs the default configuration in TOML format.
    /// Save this to ~/.config/gitnav/config.toml to customize gitnav.
    ///
    /// EXAMPLE:
    ///   gitnav config > ~/.config/gitnav/config.toml
    Config,

    /// Clear all cached repository data
    ///
    /// Removes cached repository lists. Use --dry-run to preview what will be deleted.
    /// Cache is automatically recreated the next time you run gitnav.
    ///
    /// EXAMPLE:
    ///   gitnav clear-cache          # Delete all cache
    ///   gitnav clear-cache --dry-run # Preview deletion
    ClearCache {
        /// Show what would be deleted without deleting
        #[arg(long)]
        dry_run: bool,
    },

    /// Show version information
    ///
    /// Display the installed version. Use --verbose for detailed build information.
    ///
    /// EXAMPLE:
    ///   gitnav version              # Show version
    ///   gitnav version --verbose    # Show build details
    Version {
        /// Show detailed version information (OS, arch, build profile)
        #[arg(short, long)]
        verbose: bool,
    },
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
                let formatter = output::OutputFormatter::new(false, false, false);
                let error = output::ErrorInfo::new(
                    "ENOSUPPORT",
                    "Unsupported shell",
                    format!("The shell '{}' is not supported by gitnav.", shell),
                    "Use one of the supported shells: zsh, bash, fish, nu, or nushell.\n  Examples:\n    gitnav init zsh\n    gitnav init bash\n    gitnav init fish\n    gitnav init nu",
                    "https://github.com/msetsma/gitnav#shell-integration"
                );
                formatter.error(&error);
                std::process::exit(exit_codes::EXIT_GENERAL_ERROR);
            }
        }
        Commands::Config => {
            println!("{}", config::Config::example_toml());
            Ok(())
        }
        Commands::ClearCache { dry_run } => {
            let formatter = output::OutputFormatter::new(false, false, false);
            let config = config::Config::load(None)?;
            let cache = cache::Cache::new(config.cache.ttl_seconds)?;

            let cache_files = cache.list_cache_files()?;
            let cache_size = cache.get_cache_size()?;

            if dry_run {
                println!("Cache directory: {}", cache.cache_dir().display());
                println!("Cache files: {}", cache_files.len());
                println!("Total size: {} bytes\n", cache_size);

                if !cache_files.is_empty() {
                    println!("Files to be deleted:");
                    for file in &cache_files {
                        if let Ok(metadata) = std::fs::metadata(&file) {
                            println!("  {} ({} bytes)", file.display(), metadata.len());
                        } else {
                            println!("  {}", file.display());
                        }
                    }
                } else {
                    println!("No cache files to delete");
                }
            } else {
                cache.clear()?;
                formatter.success("Cache cleared successfully");
                if !cache_files.is_empty() {
                    println!("Deleted {} cache files ({} bytes)", cache_files.len(), cache_size);
                }
            }
            Ok(())
        }
        Commands::Version { verbose } => {
            println!("gitnav {}", env!("CARGO_PKG_VERSION"));

            if verbose {
                println!("\nBuild Information:");
                println!("  Version: {}", env!("CARGO_PKG_VERSION"));
                println!("  Authors: {}", env!("CARGO_PKG_AUTHORS"));
                println!("  License: {}", env!("CARGO_PKG_LICENSE"));
                println!("  Repository: {}", env!("CARGO_PKG_REPOSITORY"));
                println!("\nSystem Information:");
                println!("  OS: {}", std::env::consts::OS);
                println!("  Architecture: {}", std::env::consts::ARCH);
                #[cfg(debug_assertions)]
                println!("  Build Profile: debug");
                #[cfg(not(debug_assertions))]
                println!("  Build Profile: release");

                println!("\nFeatures:");
                println!("  Colors: {}", if output::should_use_color() { "enabled" } else { "disabled" });
                println!("  Interactive Mode: enabled");

                println!("\nDependencies:");
                println!("  clap: 4.5");
                println!("  git2: 0.19");
                println!("  serde: 1.0");
                println!("  chrono: 0.4");
            }
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
    let _formatter = output::OutputFormatter::new(cli.quiet, cli.verbose, cli.no_color);

    // Load configuration
    let config = config::Config::load(cli.config.clone())?;

    // Validate configuration
    config.validate()?;

    // Determine search path and depth
    let search_path = cli
        .path
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| config.search.base_path.clone());

    let search_path = shellexpand::tilde(&search_path).to_string();
    let max_depth = cli.max_depth.unwrap_or(config.search.max_depth);

    if cli.debug {
        eprintln!("DEBUG: Search path: {}", search_path);
        eprintln!("DEBUG: Max depth: {}", max_depth);
        eprintln!("DEBUG: Cache enabled: {}", config.cache.enabled);
        eprintln!("DEBUG: Force refresh: {}", cli.force);
    }

    // Get repos (from cache or fresh scan)
    let repos = if config.cache.enabled && !cli.force {
        let cache = cache::Cache::new(config.cache.ttl_seconds)?;

        if cache.is_valid(&search_path) {
            if cli.verbose {
                eprintln!("DEBUG: Loading from cache");
            }
            cache.load(&search_path)?
        } else {
            if cli.verbose {
                eprintln!("DEBUG: Cache miss, scanning repositories");
            }
            let repos = scanner::scan_repos(&search_path, max_depth)?;
            cache.save(&search_path, &repos)?;
            repos
        }
    } else {
        if cli.verbose {
            eprintln!("DEBUG: Scanning repositories (cache disabled or force refresh)");
        }
        scanner::scan_repos(&search_path, max_depth)?
    };

    if repos.is_empty() {
        let formatter = output::OutputFormatter::new(cli.quiet, cli.verbose, cli.no_color);
        let error = output::ErrorInfo::new(
            "ENOREPOS",
            "No repositories found",
            format!("No git repositories found in: {}", search_path),
            format!("Verify the path exists and contains git repositories.\nYou can also try:\n  gitnav --path <different_path>\n  gitnav --max-depth <higher_number>"),
            "https://github.com/msetsma/gitnav#usage"
        );
        formatter.error(&error);
        std::process::exit(exit_codes::EXIT_GENERAL_ERROR);
    }

    if cli.verbose {
        eprintln!("DEBUG: Found {} repositories", repos.len());
    }

    // Handle --list mode (non-interactive, pipe-friendly)
    if cli.list {
        if cli.json {
            // Output as JSON
            let json_output = serde_json::to_string_pretty(&repos)
                .context("Failed to serialize repositories as JSON")?;
            println!("{}", json_output);
        } else {
            // Plain text output (one path per line)
            for repo in &repos {
                println!("{}", repo.path.display());
            }
        }
        return Ok(());
    }

    // Interactive mode requires fzf
    if !fzf::is_fzf_available() {
        let formatter = output::OutputFormatter::new(cli.quiet, cli.verbose, cli.no_color);
        let error = output::ErrorInfo::new(
            "ENOFZF",
            "fzf not found",
            "fzf is required for interactive mode but was not found in your PATH.",
            "Install fzf for your system:\n  macOS:   brew install fzf\n  Linux:   apt install fzf  or  pacman -S fzf\n  Windows: scoop install fzf\n\nAlternatively, use non-interactive mode:\n  gitnav --list",
            "https://github.com/msetsma/gitnav#requirements"
        );
        formatter.error(&error);
        std::process::exit(exit_codes::EXIT_UNAVAILABLE);
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
            // User cancelled (SIGINT)
            std::process::exit(exit_codes::EXIT_INTERRUPTED);
        }
    }
}
