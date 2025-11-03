use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::config::{Config, UiConfig};
use crate::scanner::GitRepo;

/// Run fzf to let the user select a repository.
///
/// Spawns an fzf process with the given repositories and waits for user selection.
/// The preview pane calls the binary again with `--preview` flag to generate previews.
///
/// # Arguments
///
/// * `repos` - The repositories to present to the user
/// * `config` - Configuration for UI and preview settings
/// * `preview_binary` - Path to the gitnav binary (for preview commands)
///
/// # Returns
///
/// - `Ok(Some(path))` if the user selected a repository
/// - `Ok(None)` if the user cancelled (ESC or Ctrl-C)
/// - `Err(...)` if fzf cannot be spawned or communication fails
pub fn select_repo(
    repos: &[GitRepo],
    config: &Config,
    preview_binary: &str,
) -> Result<Option<String>> {
    // Format repos for fzf input
    let input = repos
        .iter()
        .map(|repo| format!("{}\t{}", repo.name, repo.path.display()))
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

    // Parse selected line (format: name\tpath)
    let selected = String::from_utf8_lossy(&output.stdout);
    let path = selected.trim().split('\t').nth(1).map(|s| s.to_string());

    Ok(path)
}

/// Apply UI configuration to an fzf command.
///
/// Configures fzf with settings from the UI config including prompt,
/// layout, preview window size, and border visibility.
///
/// # Arguments
///
/// * `cmd` - Mutable reference to the fzf Command to configure
/// * `ui` - UI configuration to apply
fn apply_ui_config(cmd: &mut Command, ui: &UiConfig) {
    cmd.arg("--prompt").arg(&ui.prompt);
    cmd.arg("--header").arg(&ui.header);
    cmd.arg("--delimiter").arg("\t");
    cmd.arg("--with-nth").arg("1"); // Show only name column

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
}

/// Check if fzf is available and executable in the system PATH.
///
/// # Returns
///
/// `true` if fzf can be found and executed, `false` otherwise
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

    #[test]
    fn test_apply_ui_config_adds_arguments() {
        let ui_config = UiConfig {
            prompt: "Test > ".to_string(),
            header: "Test Header".to_string(),
            preview_width_percent: 60,
            layout: "reverse".to_string(),
            height_percent: 90,
            show_border: true,
        };

        let mut cmd = Command::new("fzf");
        apply_ui_config(&mut cmd, &ui_config);

        // The actual command arguments would be applied,
        // but we can't inspect them directly on the Command object.
        // This test mainly ensures the function doesn't panic.
    }

    #[test]
    fn test_apply_ui_config_without_border() {
        let ui_config = UiConfig {
            prompt: "Test > ".to_string(),
            header: "Test Header".to_string(),
            preview_width_percent: 50,
            layout: "default".to_string(),
            height_percent: 80,
            show_border: false,
        };

        let mut cmd = Command::new("fzf");
        apply_ui_config(&mut cmd, &ui_config);

        // Test ensures the function doesn't panic with border disabled
    }

    #[test]
    fn test_ui_config_width_boundary_values() {
        let test_cases = vec![0, 1, 50, 99, 100];

        for width in test_cases {
            let ui_config = UiConfig {
                prompt: "Test > ".to_string(),
                header: "Test".to_string(),
                preview_width_percent: width,
                layout: "default".to_string(),
                height_percent: 90,
                show_border: true,
            };

            let mut cmd = Command::new("fzf");
            apply_ui_config(&mut cmd, &ui_config);
            // Ensure no panic on valid values
        }
    }

    #[test]
    fn test_ui_config_height_boundary_values() {
        let test_cases = vec![1, 50, 90, 100];

        for height in test_cases {
            let ui_config = UiConfig {
                prompt: "Test > ".to_string(),
                header: "Test".to_string(),
                preview_width_percent: 60,
                layout: "default".to_string(),
                height_percent: height,
                show_border: true,
            };

            let mut cmd = Command::new("fzf");
            apply_ui_config(&mut cmd, &ui_config);
            // Ensure no panic on valid values
        }
    }

    #[test]
    fn test_ui_config_different_layouts() {
        let layouts = vec!["default", "reverse", "reverse-list"];

        for layout in layouts {
            let ui_config = UiConfig {
                prompt: "Test > ".to_string(),
                header: "Test".to_string(),
                preview_width_percent: 60,
                layout: layout.to_string(),
                height_percent: 90,
                show_border: true,
            };

            let mut cmd = Command::new("fzf");
            apply_ui_config(&mut cmd, &ui_config);
            // Ensure no panic on different layout values
        }
    }
}
