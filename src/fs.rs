use crate::{
    error::Errno,
    x86_64::{SYS_CLOSE, SYS_OPENAT},
};
use core::{arch::asm, ffi::CStr};

/// Opens the file specified by `pathname` relative to `dirfd` using the Linux
/// [`openat(2)`] system call.
///
/// [`openat(2)`]: https://man7.org/linux/man-pages/man2/open.2.html
pub fn openat(dirfd: i32, pathname: &CStr, flags: i32, mode: u32) -> Result<i32, Errno> {
    let result: i32;

    // SAFETY: `pathname` points to a valid NUL-terminated C string, and the
    // registers follow the Linux x86-64 syscall ABI.
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_OPENAT,
            in("rdi") dirfd,
            in("rsi") pathname.as_ptr(),
            in("rdx") flags,
            in("r10") mode,
            lateout("rax") result,
            lateout("rcx") _,
            lateout("r11") _,
        )
    };

    if result.is_negative() {
        Err(Errno(result.unsigned_abs() as u16))
    } else {
        Ok(result)
    }
}

/// Closes the given file descriptor using the Linux [`close(2)`] system call.
///
/// [`close(2)`]: https://man7.org/linux/man-pages/man2/close.2.html
pub fn close(fd: i32) -> Result<isize, Errno> {
    let result: isize;

    // SAFETY: the registers follow the Linux x86-64 syscall ABI.
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

    if result.is_negative() {
        Err(Errno(result.unsigned_abs() as u16))
    } else {
        Ok(result)
    }
}
