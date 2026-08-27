/// File descriptor for the standard input stream.
///
/// See [`stdin(3)`].
///
/// [`stdin(3)`]: https://man7.org/linux/man-pages/man3/stdin.3.html
pub const STDIN_FILENO: i32 = 0;

/// File descriptor for the standard output stream.
///
/// See [`stdout(3)`].
///
/// [`stdout(3)`]: https://man7.org/linux/man-pages/man3/stdout.3.html
pub const STDOUT_FILENO: i32 = 1;

/// File descriptor for the standard error stream.
///
/// See ['stdout(3)'].
///
/// ['stdout(3)']: https://man7.org/linux/man-pages/man3/stderr.3.html
pub const STDERR_FILENO: i32 = 2;
