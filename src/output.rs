use std::io::{self, Write};

/// Determines if colored output should be used based on environment and TTY status.
///
/// Checks for:
/// - NO_COLOR environment variable (disables color)
/// - TERM=dumb environment variable (disables color)
/// - Whether stdout is a TTY (only use colors when outputting to terminal)
///
/// # Returns
///
/// `true` if colors should be used, `false` otherwise
pub fn should_use_color() -> bool {
    // Check NO_COLOR env var (https://no-color.org/)
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }

    // Check TERM environment variable
    if let Ok(term) = std::env::var("TERM") {
        if term == "dumb" {
            return false;
        }
    }

    // Check if stdout is connected to a TTY
    atty::is(atty::Stream::Stdout)
}

/// Centralized output formatter for consistent CLI output.
///
/// Handles:
/// - Selective quiet/verbose output
/// - Color management based on TTY detection
/// - Structured error messages with codes and metadata
/// - Stream separation (stdout for data, stderr for errors/info)
pub struct OutputFormatter {
    quiet: bool,
    verbose: bool,
    use_color: bool,
}

impl OutputFormatter {
    /// Create a new output formatter.
    ///
    /// # Arguments
    ///
    /// * `quiet` - If true, suppress non-error output
    /// * `verbose` - If true, show verbose output
    /// * `no_color` - If true, disable colored output (overrides TTY detection)
    pub fn new(quiet: bool, verbose: bool, no_color: bool) -> Self {
        let use_color = !no_color && should_use_color();
        Self {
            quiet,
            verbose,
            use_color,
        }
    }

    /// Print informational message to stdout (only if not quiet).
    ///
    /// Used for normal operational output like success messages.
    pub fn info(&self, msg: &str) {
        if !self.quiet {
            println!("{}", msg);
        }
    }

    /// Print verbose message to stdout (only if verbose flag is set).
    ///
    /// Used for detailed operational information.
    pub fn verbose(&self, msg: &str) {
        if self.verbose && !self.quiet {
            println!("{}", msg);
        }
    }

    /// Print error message to stderr with structured format.
    ///
    /// # Arguments
    ///
    /// * `code` - Error code identifier (e.g., "ENOFZF", "ENOSUPPORT")
    /// * `title` - Short error title
    /// * `description` - Detailed description of what went wrong
    /// * `fix` - Suggested fix or workaround
    /// * `url` - URL to documentation for more information
    pub fn error(
        &self,
        code: &str,
        title: &str,
        description: &str,
        fix: &str,
        url: &str,
    ) {
        let _ = writeln!(stderr(), "Error: {} - {}\n", code, title);
        let _ = writeln!(stderr(), "{}\n", description);
        let _ = writeln!(stderr(), "Fix: {}\n", fix);
        let _ = writeln!(stderr(), "Documentation: {}", url);
    }

    /// Print warning message to stderr.
    pub fn warn(&self, msg: &str) {
        let _ = writeln!(stderr(), "Warning: {}", msg);
    }

    /// Format text with color if colors are enabled.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to format
    /// * `color_code` - ANSI color code (e.g., "\x1b[1;36m" for bright cyan)
    pub fn colorize(&self, text: &str, color_code: &str) -> String {
        if self.use_color {
            format!("{}{}\x1b[0m", color_code, text)
        } else {
            text.to_string()
        }
    }

    /// Format cyan/bright cyan text.
    pub fn cyan(&self, text: &str) -> String {
        self.colorize(text, "\x1b[1;36m")
    }

    /// Format yellow text.
    pub fn yellow(&self, text: &str) -> String {
        self.colorize(text, "\x1b[1;33m")
    }

    /// Format green text.
    pub fn green(&self, text: &str) -> String {
        self.colorize(text, "\x1b[32m")
    }

    /// Format red text.
    pub fn red(&self, text: &str) -> String {
        self.colorize(text, "\x1b[31m")
    }

    /// Format magenta text.
    pub fn magenta(&self, text: &str) -> String {
        self.colorize(text, "\x1b[1;35m")
    }
}

/// Get stderr writer for error output.
fn stderr() -> io::Stderr {
    io::stderr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_formatter_new() {
        let formatter = OutputFormatter::new(false, false, false);
        assert!(!formatter.quiet);
        assert!(!formatter.verbose);
    }

    #[test]
    fn test_output_formatter_quiet() {
        let formatter = OutputFormatter::new(true, false, false);
        assert!(formatter.quiet);
    }

    #[test]
    fn test_output_formatter_verbose() {
        let formatter = OutputFormatter::new(false, true, false);
        assert!(formatter.verbose);
    }

    #[test]
    fn test_output_formatter_no_color() {
        let formatter = OutputFormatter::new(false, false, true);
        assert!(!formatter.use_color);
    }

    #[test]
    fn test_should_use_color_with_no_color_env() {
        // Save original env var
        let original = std::env::var("NO_COLOR").ok();

        // Test with NO_COLOR set
        std::env::set_var("NO_COLOR", "1");
        let should_color = should_use_color();
        assert!(!should_color);

        // Restore original
        if let Some(val) = original {
            std::env::set_var("NO_COLOR", val);
        } else {
            std::env::remove_var("NO_COLOR");
        }
    }

    #[test]
    fn test_colorize_disabled() {
        let formatter = OutputFormatter::new(false, false, true);
        let result = formatter.cyan("test");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_color_methods() {
        let formatter = OutputFormatter::new(false, false, true);
        assert_eq!(formatter.cyan("test"), "test");
        assert_eq!(formatter.yellow("test"), "test");
        assert_eq!(formatter.green("test"), "test");
        assert_eq!(formatter.red("test"), "test");
        assert_eq!(formatter.magenta("test"), "test");
    }
}
