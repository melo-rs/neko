use core::arch::asm;
use crate::x86_64::{SYS_OPENAT, SYS_CLOSE};

/// Opens the file specified by `pathname` relative to `dirfd` using the
/// Linux [`openat(2)`] system call.
/// 
/// [`openat(2)`]: https://man7.org/linux/man-pages/man2/open.2.html
pub fn openat(dirfd: i32, pathname: *const u8, flags: i32, mode: u32) -> i32 {
    let fd: i32;

    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_OPENAT,
            in("rdi") dirfd,
            in("rsi") pathname,
            in("rdx") flags,
            in("r10") mode,
            lateout("rax") fd,
            lateout("rcx") _,
            lateout("r11") _,
        )
    };

    fd
}

/// Closes the given file descriptor using the Linux [`close(2)`] system
/// call.
///
/// [`close(2)`]: https://man7.org/linux/man-pages/man2/close.2.html
pub fn close(fd: i32) -> isize {
    let result: isize;

    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_CLOSE,
            in("rdi") fd,
            lateout("rax") result,
            lateout("rcx") _,
            lateout("r11") _,
        )
    }

    result
}
