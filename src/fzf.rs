use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::config::{Config, UiConfig};
use crate::scanner::GitRepo;

/// Run fzf with the given repos and return the selected path
pub fn select_repo(repos: &[GitRepo], config: &Config, preview_binary: &str) -> Result<Option<String>> {
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
    let path = selected
        .trim()
        .split('\t')
        .nth(1)
        .map(|s| s.to_string());

    Ok(path)
}

/// Apply UI configuration to fzf command
fn apply_ui_config(cmd: &mut Command, ui: &UiConfig) {
    cmd.arg("--prompt").arg(&ui.prompt);
    cmd.arg("--header").arg(&ui.header);
    cmd.arg("--delimiter").arg("\t");
    cmd.arg("--with-nth").arg("1"); // Show only name column
    
    // Preview window configuration
    let preview_window = format!(
        "right:{}%:wrap",
        ui.preview_width_percent
    );
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

/// Check if fzf is available in PATH
pub fn is_fzf_available() -> bool {
    Command::new("fzf")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}
