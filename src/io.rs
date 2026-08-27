use crate::x86_64::{SYS_READ, SYS_WRITE};
use core::arch::asm;

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

/// Reads up to `count` bytes from the given file descriptor into `buf`
/// using the Linux [`read(2)`] system call.
///
/// ['read(2)']: https://man7.org/linux/man-pages/man2/read.2.html
pub fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    let result: isize;

    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_READ,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") result,
            lateout("rcx") _,
            lateout("r11") _,
        )
    }

    result
}

/// Writes up to `count` bytes from `buf` to the given file descriptor
/// using the Linux [`write(2)`] system call.
///
/// [`write(2)`]: https://man7.org/linux/man-pages/man2/write.2.html
pub fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    let result: isize;

    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_WRITE,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") result,
            lateout("rcx") _,
            lateout("r11") _,
        )
    }

    result
}
