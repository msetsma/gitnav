/// Standard exit codes used by gitnav.
///
/// These exit codes follow the conventions from:
/// - sysexits.h (BSD)
/// - Linux documentation
/// - POSIX standards
///
/// The application uses:
/// - 0: Success
/// - 1: General error
/// - 130: User interrupt (SIGINT/Ctrl+C)
///
/// # Exit Codes
///
/// Exit code for successful execution
#[allow(dead_code)]
pub const EXIT_SUCCESS: i32 = 0;

/// Exit code for general errors
pub const EXIT_GENERAL_ERROR: i32 = 1;

/// Exit code for command-line argument errors
#[allow(dead_code)]
pub const EXIT_USAGE_ERROR: i32 = 2;

/// Exit code for data errors (e.g., invalid input format)
#[allow(dead_code)]
pub const EXIT_DATA_ERROR: i32 = 65;

/// Exit code for unavailable resources (e.g., missing dependencies)
pub const EXIT_UNAVAILABLE: i32 = 69;

/// Exit code for input/output errors
#[allow(dead_code)]
pub const EXIT_IO_ERROR: i32 = 74;

/// Exit code for user interrupt (SIGINT/Ctrl+C)
///
/// This is the standard exit code when a user interrupts the program
/// with Ctrl+C (SIGINT). The value 130 is derived from 128 + SIGINT (2).
pub const EXIT_INTERRUPTED: i32 = 130;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_code_success() {
        assert_eq!(EXIT_SUCCESS, 0);
    }

    #[test]
    fn test_exit_code_general_error() {
        assert_eq!(EXIT_GENERAL_ERROR, 1);
    }

    #[test]
    fn test_exit_code_interrupted() {
        assert_eq!(EXIT_INTERRUPTED, 130);
    }

    #[test]
    fn test_exit_codes_are_distinct() {
        let codes = [
            EXIT_SUCCESS,
            EXIT_GENERAL_ERROR,
            EXIT_USAGE_ERROR,
            EXIT_DATA_ERROR,
            EXIT_UNAVAILABLE,
            EXIT_IO_ERROR,
            EXIT_INTERRUPTED,
        ];

        for (i, &code1) in codes.iter().enumerate() {
            for &code2 in codes.iter().skip(i + 1) {
                assert_ne!(
                    code1, code2,
                    "Exit code {} and {} should be distinct",
                    code1, code2
                );
            }
        }
    }
}
