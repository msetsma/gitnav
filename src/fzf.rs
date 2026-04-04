use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::config::{Config, UiConfig};
use crate::scanner::{format_display, EnrichedRepo};

/// Run fzf to let the user select a repository.
///
/// Spawns an fzf process with the given repositories and waits for user selection.
/// The preview pane calls the binary again with `--preview` flag to generate previews.
///
/// # Arguments
///
/// * `repos` - The enriched repositories to present to the user
/// * `config` - Configuration for UI and preview settings
/// * `preview_binary` - Path to the gitnav binary (for preview commands)
/// * `initial_query` - Optional query string to pre-fill in fzf
///
/// # Returns
///
/// - `Ok(Some(path))` if the user selected a repository
/// - `Ok(None)` if the user cancelled (ESC or Ctrl-C)
/// - `Err(...)` if fzf cannot be spawned or communication fails
pub fn select_repo(
    repos: &[EnrichedRepo],
    config: &Config,
    preview_binary: &str,
    initial_query: Option<&str>,
) -> Result<Option<String>> {
    // fzf always renders ANSI in its list, so force color on
    let use_color = std::env::var("NO_COLOR").is_err();
    let name_width = repos.iter().map(|r| r.name.len()).max().unwrap_or(0);

    let input = repos
        .iter()
        .map(|repo| {
            let display = format_display(repo, name_width, use_color, &config.ui.badge_style);
            format!("{}\t{}", display, repo.path.display())
        })
        .collect::<Vec<_>>()
        .join("\n");

    if input.is_empty() {
        return Ok(None);
    }

    // Build fzf command
    let mut cmd = Command::new("fzf");

    apply_ui_config(&mut cmd, &config.ui);

    // Add preview command that calls gitnav --preview
    let preview_cmd = format!("{} --preview {{2}}", preview_binary);
    cmd.arg("--preview").arg(&preview_cmd);

    // Pre-fill query if provided
    if let Some(query) = initial_query {
        cmd.arg("--query").arg(query);
    }

    // Configure input/output
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    let mut child = cmd.spawn().context("Failed to spawn fzf process")?;

    // Write input to fzf
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .context("Failed to write to fzf stdin")?;
    }

    // Wait for fzf to complete and capture output
    let output = child.wait_with_output().context("Failed to wait for fzf")?;

    if !output.status.success() {
        // User cancelled (ESC or Ctrl-C)
        return Ok(None);
    }

    // Parse selected line (format: display\tpath) — path is always the last tab-separated field
    let selected = String::from_utf8_lossy(&output.stdout);
    let path = selected.trim().split('\t').last().map(|s| s.to_string());

    Ok(path)
}

/// Apply UI configuration to an fzf command.
fn apply_ui_config(cmd: &mut Command, ui: &UiConfig) {
    cmd.arg("--prompt").arg(&ui.prompt);
    cmd.arg("--header").arg(&ui.header);
    cmd.arg("--delimiter").arg("\t");
    cmd.arg("--with-nth").arg("1"); // Show only display column

    // Preview window configuration
    let preview_window = format!("right:{}%:wrap", ui.preview_width_percent);
    cmd.arg("--preview-window").arg(preview_window);

    // Layout
    cmd.arg("--layout").arg(&ui.layout);

    // Height
    let height = format!("{}%", ui.height_percent);
    cmd.arg("--height").arg(height);

    // Border
    if ui.show_border {
        cmd.arg("--border");
    }

    // Don't sort (keep alphabetical order from scanner)
    cmd.arg("--no-sort");

    // Use ANSI color rendering
    cmd.arg("--ansi");
}

/// Check if fzf is available and executable in the system PATH.
pub fn is_fzf_available() -> bool {
    Command::new("fzf")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BadgeStyle;

    fn make_ui_config() -> UiConfig {
        UiConfig {
            prompt: "Test > ".to_string(),
            header: "Test Header".to_string(),
            preview_width_percent: 60,
            layout: "reverse".to_string(),
            height_percent: 90,
            show_border: true,
            show_inline_meta: true,
            badge_style: BadgeStyle::Text,
        }
    }

    #[test]
    fn test_apply_ui_config_adds_arguments() {
        let mut cmd = Command::new("fzf");
        apply_ui_config(&mut cmd, &make_ui_config());
        // Ensures the function doesn't panic
    }

    #[test]
    fn test_apply_ui_config_without_border() {
        let mut ui = make_ui_config();
        ui.show_border = false;
        let mut cmd = Command::new("fzf");
        apply_ui_config(&mut cmd, &ui);
    }

    #[test]
    fn test_ui_config_width_boundary_values() {
        for width in [0u8, 1, 50, 99, 100] {
            let mut ui = make_ui_config();
            ui.preview_width_percent = width;
            let mut cmd = Command::new("fzf");
            apply_ui_config(&mut cmd, &ui);
        }
    }

    #[test]
    fn test_ui_config_height_boundary_values() {
        for height in [1u8, 50, 90, 100] {
            let mut ui = make_ui_config();
            ui.height_percent = height;
            let mut cmd = Command::new("fzf");
            apply_ui_config(&mut cmd, &ui);
        }
    }

    #[test]
    fn test_ui_config_different_layouts() {
        for layout in ["default", "reverse", "reverse-list"] {
            let mut ui = make_ui_config();
            ui.layout = layout.to_string();
            let mut cmd = Command::new("fzf");
            apply_ui_config(&mut cmd, &ui);
        }
    }
}
