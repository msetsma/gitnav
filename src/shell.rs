/// Generate a shell initialization script for the given shell type.
///
/// Produces shell-specific code that creates a `gn` function to use gitnav.
/// Supports bash, zsh, fish, and nushell.
///
/// # Arguments
///
/// * `shell` - The shell type: "bash", "zsh", "fish", or "nu"/"nushell"
///
/// # Returns
///
/// A string containing the shell function definition, or `None` if shell is unsupported
pub fn generate_init_script(shell: &str) -> Option<String> {
    match shell.to_lowercase().as_str() {
        "zsh" => Some(generate_zsh_script()),
        "bash" => Some(generate_bash_script()),
        "fish" => Some(generate_fish_script()),
        "nu" | "nushell" => Some(generate_nushell_script()),
        "powershell" | "pwsh" => Some(generate_powershell_script()),
        _ => None,
    }
}

fn generate_zsh_script() -> String {
    r#"# gitnav shell integration for zsh
# Add this to your ~/.zshrc:
#   eval "$(gitnav init zsh)"

gn() {
  local result
  # First non-flag argument is treated as an initial fzf query
  if [[ $# -gt 0 ]] && [[ "$1" != -* ]]; then
    result=$(gitnav --query "$1" "${@:2}")
  else
    result=$(gitnav "$@")
  fi

  if [[ -n "$result" ]] && [[ -d "$result" ]]; then
    cd "$result" || return 1

    # Optional: show a quick listing after cd
    if command -v eza &> /dev/null; then
      eza -l
    elif command -v ls &> /dev/null; then
      ls -la
    fi
  fi
}
"#
    .to_string()
}

fn generate_bash_script() -> String {
    r#"# gitnav shell integration for bash
# Add this to your ~/.bashrc:
#   eval "$(gitnav init bash)"

gn() {
  local result
  # First non-flag argument is treated as an initial fzf query
  if [[ $# -gt 0 ]] && [[ "$1" != -* ]]; then
    result=$(gitnav --query "$1" "${@:2}")
  else
    result=$(gitnav "$@")
  fi

  if [[ -n "$result" ]] && [[ -d "$result" ]]; then
    cd "$result" || return 1

    # Optional: show a quick listing after cd
    if command -v eza &> /dev/null; then
      eza -l
    elif command -v ls &> /dev/null; then
      ls -la
    fi
  fi
}
"#
    .to_string()
}

fn generate_fish_script() -> String {
    r#"# gitnav shell integration for fish
# Add this to your ~/.config/fish/config.fish:
#   gitnav init fish | source

function gn
  # First non-flag argument is treated as an initial fzf query
  set result
  if test (count $argv) -gt 0; and not string match -q -- '-*' $argv[1]
    set result (gitnav --query $argv[1] $argv[2..])
  else
    set result (gitnav $argv)
  end

  if test -n "$result" -a -d "$result"
    cd "$result"; or return 1

    # Optional: show a quick listing after cd
    if command -q eza
      eza -l
    else
      ls -la
    end
  end
end
"#
    .to_string()
}

fn generate_nushell_script() -> String {
    r#"# gitnav shell integration for nushell
# Add this to your nushell config (typically ~/.config/nushell/config.nu):
#   gitnav init nu | save --force ~/.cache/gitnav/init.nu
#   source ~/.cache/gitnav/init.nu

def --env gn [...args] {
  # First non-flag argument is treated as an initial fzf query
  let result = if ($args | length) > 0 and not ($args | first | str starts-with '-') {
    (gitnav --query ($args | first) ...($args | skip 1) | str trim)
  } else {
    (gitnav ...$args | str trim)
  }

  if ($result != "") and ($result | path exists) {
    cd $result

    # Optional: show a quick listing after cd
    if (which eza | length) > 0 {
      eza -l
    } else {
      ls
    }
  }
}
"#
    .to_string()
}

fn generate_powershell_script() -> String {
    r#"# gitnav shell integration for PowerShell
# Add this to your PowerShell profile ($PROFILE):
#   Invoke-Expression (& gitnav init powershell)

function gn {
  # First non-flag argument is treated as an initial fzf query
  $result = if ($args.Count -gt 0 -and -not $args[0].StartsWith('-')) {
    & gitnav --query $args[0] @($args | Select-Object -Skip 1)
  } else {
    & gitnav @args
  }

  if ($result -and (Test-Path $result -PathType Container)) {
    Set-Location $result

    # Optional: show a quick listing after cd
    if (Get-Command eza -ErrorAction SilentlyContinue) {
      eza -l
    } else {
      Get-ChildItem
    }
  }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_init_script() {
        assert!(generate_init_script("zsh").is_some());
        assert!(generate_init_script("bash").is_some());
        assert!(generate_init_script("fish").is_some());
        assert!(generate_init_script("nu").is_some());
        assert!(generate_init_script("nushell").is_some());
        assert!(generate_init_script("powershell").is_some());
        assert!(generate_init_script("pwsh").is_some());
        assert!(generate_init_script("unknown").is_none());
    }

    #[test]
    fn test_zsh_script_contains_function() {
        let script = generate_zsh_script();
        assert!(script.contains("gn()"));
        assert!(script.contains("gitnav"));
    }

    #[test]
    fn test_all_shells_contain_query_flag() {
        for shell in &["zsh", "bash", "fish", "nu", "powershell"] {
            let script = generate_init_script(shell).unwrap();
            assert!(
                script.contains("--query"),
                "Shell '{}' script missing --query support",
                shell
            );
        }
    }

    #[test]
    fn test_all_shells_handle_non_flag_positional() {
        // zsh/bash: detect "$1" not starting with -
        let zsh = generate_zsh_script();
        assert!(zsh.contains("\"$1\" != -*") || zsh.contains("[[ \"$1\" != -*"));
        // fish: string match pattern
        let fish = generate_fish_script();
        assert!(fish.contains("not string match") || fish.contains("'-*'"));
        // powershell: StartsWith('-')
        let ps = generate_powershell_script();
        assert!(ps.contains("StartsWith"));
    }

    #[test]
    fn test_powershell_script_contains_function() {
        let script = generate_powershell_script();
        assert!(script.contains("function gn"));
        assert!(script.contains("gitnav"));
    }
}
