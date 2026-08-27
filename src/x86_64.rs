/// Special `dirfd` value that resolves relative paths against the
/// current working directory.
///
/// See [`openat(2)`].
///
/// [`openat(2)`]: https://man7.org/linux/man-pages/man2/open.2.html
pub const AT_FDCWD: i32 = -100;

/// x86-64 Linux syscall number for ['openat(2)'].
///
/// ['openat(2)']: https://man7.org/linux/man-pages/man2/open.2.html
pub const SYS_OPENAT: usize = 257;

/// x86-64 Linux syscall number for [`exit(2)`].
///
/// [`exit(2)`]: https://man7.org/linux/man-pages/man2/_exit.2.html
pub const SYS_EXIT: usize = 60;

/// x86-64 Linux syscall number for ['read(2)'].
///
/// ['read(2)']: https://man7.org/linux/man-pages/man2/read.2.html
pub const SYS_READ: usize = 0;

/// x86-64 Linux syscall number for ['write(2)'].
///
/// ['write(2)']: https://man7.org/linux/man-pages/man2/write.2.html
pub const SYS_WRITE: usize = 1;

/// x86-64 Linux syscall number for ['close(2)'].
///
/// ['close(2)']: https://man7.org/linux/man-pages/man2/close.2.html
pub const SYS_CLOSE: usize = 3;
