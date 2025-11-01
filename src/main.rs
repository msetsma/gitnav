mod cache;
mod config;
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
EXAMPLES:\n    \
gn                        # Interactive repository selection\n    \
gn -f                     # Force cache refresh\n    \
gn --path ~/work          # Search in specific directory\n    \
gn --list                 # List all repos (no interactive mode)\n    \
gn --list --json          # Machine-readable output\n    \
gitnav init zsh          # Generate shell integration\n    \
gitnav config            # Show example config\n    \
gitnav clear-cache       # Clear cache")]
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
    /// Generate shell integration script
    Init {
        /// Shell type (zsh, bash, fish, nu/nushell)
        shell: String,
    },
    /// Print example config to stdout
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
                eprintln!("Error: ENOSUPPORT - Unsupported shell\n");
                eprintln!("The shell '{}' is not supported by gitnav.\n", shell);
                eprintln!("Supported shells: zsh, bash, fish, nu, nushell\n");
                eprintln!("Fix: Use one of the supported shells:\n");
                eprintln!("  gitnav init zsh");
                eprintln!("  gitnav init bash");
                eprintln!("  gitnav init fish");
                eprintln!("  gitnav init nu\n");
                eprintln!("Documentation: https://github.com/msetsma/gitnav#shell-integration");
                std::process::exit(1);
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
            println!("Cache cleared successfully");
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
        eprintln!("Error: ENOREPOS - No repositories found\n");
        eprintln!("No git repositories found in: {}\n", search_path);
        eprintln!("Fix: Verify the path exists and contains git repositories");
        std::process::exit(1);
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
        eprintln!("Error: ENOFZF - fzf not found\n");
        eprintln!("fzf is required for interactive mode.\n");
        eprintln!("Installation:\n");
        eprintln!("  macOS:   brew install fzf");
        eprintln!("  Linux:   apt install fzf  or  pacman -S fzf");
        eprintln!("  Windows: scoop install fzf\n");
        eprintln!("Alternatively, use non-interactive mode:\n");
        eprintln!("  gitnav --list\n");
        eprintln!("Documentation: https://github.com/msetsma/gitnav#requirements");
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
            // User cancelled (SIGINT)
            std::process::exit(130);
        }
    }
}
