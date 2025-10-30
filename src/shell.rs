/// Generate shell initialization script for the given shell
pub fn generate_init_script(shell: &str) -> Option<String> {
    match shell.to_lowercase().as_str() {
        "zsh" => Some(generate_zsh_script()),
        "bash" => Some(generate_bash_script()),
        "fish" => Some(generate_fish_script()),
        "nu" | "nushell" => Some(generate_nushell_script()),
        _ => None,
    }
}

fn generate_zsh_script() -> String {
    r#"# gitnav shell integration for zsh
# Add this to your ~/.zshrc:
#   eval "$(gitnav --init zsh)"

gn() {
  local result
  result=$(gitnav "$@")
  
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
"#.to_string()
}

fn generate_bash_script() -> String {
    r#"# gitnav shell integration for bash
# Add this to your ~/.bashrc:
#   eval "$(gitnav --init bash)"

gn() {
  local result
  result=$(gitnav "$@")
  
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
"#.to_string()
}

fn generate_fish_script() -> String {
    r#"# gitnav shell integration for fish
# Add this to your ~/.config/fish/config.fish:
#   gitnav --init fish | source

function gn
  set result (gitnav $argv)

  if test -n "$result" -a -d "$result"
    cd "$result"; or return 1

    # Optional: show a quick listing after cd
    if command -v eza &> /dev/null
      eza -l
    else if command -v ls &> /dev/null
      ls -la
    end
  end
end
"#.to_string()
}

fn generate_nushell_script() -> String {
    r#"# gitnav shell integration for nushell
# Add this to your nushell config (typically ~/.config/nushell/config.nu):
#   gitnav --init nu | save --force ~/.cache/gitnav/init.nu
#   source ~/.cache/gitnav/init.nu
# Or add directly:
#   source (gitnav --init nu | str trim)

def --env gn [...args] {
  let result = (gitnav ...$args | str trim)

  if ($result != "") and ($result | path exists) {
    cd $result

    # Optional: show a quick listing after cd
    if (which eza | length) > 0 {
      eza -l
    } else if (which ls | length) > 0 {
      ls
    }
  }
}
"#.to_string()
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
        assert!(generate_init_script("unknown").is_none());
    }

    #[test]
    fn test_zsh_script_contains_function() {
        let script = generate_zsh_script();
        assert!(script.contains("gn()"));
        assert!(script.contains("gitnav"));
    }
}
