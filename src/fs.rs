use crate::{
    errno::Errno,
    x86_64::{SYS_CLOSE, SYS_FSTAT, SYS_OPENAT},
};
use core::{arch::asm, ffi::CStr, mem::MaybeUninit};

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

/// File metadata returned by the Linux [`fstat(2)`] system call.
///
/// This structure represents the Linux x86-64 `stat` ABI.
///
/// [`fstat(2)`]: https://man7.org/linux/man-pages/man2/stat.2.html
#[repr(C)]
pub struct Metadata {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,

    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    __pad0: u32,

    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,

    pub st_atime: u64,
    pub st_atime_nsec: u64,
    pub st_mtime: u64,
    pub st_mtime_nsec: u64,
    pub st_ctime: u64,
    pub st_ctime_nsec: u64,

    __unused: [i64; 3],
}

impl Metadata {
    pub const fn is_file(&self) -> bool {
        const S_IFMT: u32 = 0o170000;
const S_IFREG: u32 = 0o100000;

        self.st_mode & S_IFMT == S_IFREG
    }
}

/// Retrieves metadata about the file referred to by `fd` using the Linux
/// [`fstat(2)`] system call.
///
/// [`fstat(2)`]: https://man7.org/linux/man-pages/man2/stat.2.html
pub fn fstat(fd: i32) -> Result<Metadata, Errno> {
    let mut buffer = MaybeUninit::<Metadata>::uninit();
    let result: isize;

    // SAFETY: `buffer` provides valid writable memory for one `Stat`, and the
    // registers follow the Linux x86-64 syscall ABI.
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_FSTAT,
            in("rdi") fd,
            in("rsi") buffer.as_mut_ptr(),
            lateout("rax") result,
            lateout("rcx") _,
            lateout("r11") _
        )
    }

    if result.is_negative() {
        Err(Errno(result.unsigned_abs() as u16))
    } else {
        // SAFETY: This branch is reached only after `fstat(2)` succeeds, which
        // guarantees that `buffer` contains a fully initialized `Stat`.
        Ok(unsafe { buffer.assume_init() })
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
