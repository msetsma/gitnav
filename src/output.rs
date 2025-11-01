use std::io::{self, Write};

/// Error code and metadata for structured error messages
#[derive(Debug, Clone)]
pub struct ErrorInfo {
    /// Error code identifier (e.g., "ENOFZF", "ENOREPOS")
    pub code: String,
    /// Short error title
    pub title: String,
    /// Detailed error description
    pub description: String,
    /// Suggested fix or workaround
    pub fix: String,
    /// URL to documentation
    pub url: String,
}

impl ErrorInfo {
    /// Create a new error info
    pub fn new(
        code: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        fix: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            title: title.into(),
            description: description.into(),
            fix: fix.into(),
            url: url.into(),
        }
    }
}

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
    #[allow(dead_code)]
    verbose: bool,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn info(&self, msg: &str) {
        if !self.quiet {
            println!("{}", msg);
        }
    }

    /// Print success message to stdout (only if not quiet).
    ///
    /// Used for successful operation confirmations.
    pub fn success(&self, msg: &str) {
        if !self.quiet {
            println!("{}", msg);
        }
    }

    /// Print verbose message to stdout (only if verbose flag is set).
    ///
    /// Used for detailed operational information.
    #[allow(dead_code)]
    pub fn verbose(&self, msg: &str) {
        if self.verbose && !self.quiet {
            println!("{}", msg);
        }
    }

    /// Print structured error message to stderr.
    ///
    /// Formats error with code, title, description, fix, and documentation URL.
    ///
    /// # Arguments
    ///
    /// * `error_info` - ErrorInfo struct containing all error details
    ///
    /// # Example
    ///
    /// ```ignore
    /// let error = ErrorInfo::new(
    ///     "ENOFZF",
    ///     "fzf not found",
    ///     "fzf is required for interactive mode.",
    ///     "Install fzf: brew install fzf",
    ///     "https://github.com/msetsma/gitnav#requirements"
    /// );
    /// formatter.error(&error);
    /// ```
    pub fn error(&self, error_info: &ErrorInfo) {
        let _ = writeln!(stderr(), "Error: {} - {}\n", error_info.code, error_info.title);
        let _ = writeln!(stderr(), "{}\n", error_info.description);
        let _ = writeln!(stderr(), "Fix: {}\n", error_info.fix);
        let _ = writeln!(stderr(), "Documentation: {}", error_info.url);
    }

    /// Print error message to stderr with code and title only (simple format).
    ///
    /// # Arguments
    ///
    /// * `code` - Error code identifier (e.g., "ENOFZF", "ENOSUPPORT")
    /// * `message` - Error message
    #[allow(dead_code)]
    pub fn error_simple(&self, code: &str, message: &str) {
        let _ = writeln!(stderr(), "Error: {} - {}", code, message);
    }

    /// Print warning message to stderr.
    #[allow(dead_code)]
    pub fn warn(&self, msg: &str) {
        let _ = writeln!(stderr(), "Warning: {}", msg);
    }

    /// Format text with color if colors are enabled.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to format
    /// * `color_code` - ANSI color code (e.g., "\x1b[1;36m" for bright cyan)
    #[allow(dead_code)]
    pub fn colorize(&self, text: &str, color_code: &str) -> String {
        if self.use_color {
            format!("{}{}\x1b[0m", color_code, text)
        } else {
            text.to_string()
        }
    }

    /// Format cyan/bright cyan text.
    #[allow(dead_code)]
    pub fn cyan(&self, text: &str) -> String {
        self.colorize(text, "\x1b[1;36m")
    }

    /// Format yellow text.
    #[allow(dead_code)]
    pub fn yellow(&self, text: &str) -> String {
        self.colorize(text, "\x1b[1;33m")
    }

    /// Format green text.
    #[allow(dead_code)]
    pub fn green(&self, text: &str) -> String {
        self.colorize(text, "\x1b[32m")
    }

    /// Format red text.
    #[allow(dead_code)]
    pub fn red(&self, text: &str) -> String {
        self.colorize(text, "\x1b[31m")
    }

    /// Format magenta text.
    #[allow(dead_code)]
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
