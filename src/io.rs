use crate::{
    error::Errno,
    x86_64::{SYS_READ, SYS_WRITE, SYS_WRITEV},
};
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
pub fn read(fd: i32, buf: *mut u8, count: usize) -> Result<usize, Errno> {
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

    if result.is_negative() {
        Err(Errno(result.unsigned_abs() as u16))
    } else {
        Ok(result as usize)
    }
}

/// Writes up to `count` bytes from `buf` to the given file descriptor
/// using the Linux [`write(2)`] system call.
///
/// [`write(2)`]: https://man7.org/linux/man-pages/man2/write.2.html
pub fn write(fd: i32, buf: *const u8, count: usize) -> Result<usize, Errno> {
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

    if result.is_negative() {
        Err(Errno(result.unsigned_abs() as u16))
    } else {
        Ok(result as usize)
    }
}

#[repr(C)]
pub struct WriteVector {
    base: *const u8,
    len: usize,
}

impl WriteVector {
    pub fn from_slice(slice: &[u8]) -> Self {
        Self {
            base: slice.as_ptr(),
            len: slice.len(),
        }
    }

    pub unsafe fn from_c_str(ptr: *const u8) -> Self {
        let mut len = 0usize;

        loop {
            let byte = unsafe { *ptr.add(len) };

            if byte == 0 {
                break;
            }

            len += 1;
        }

        Self { base: ptr, len }
    }
}

pub fn writev(fd: i32, vec: *const WriteVector, veccnt: usize) -> Result<usize, Errno> {
    let result: isize;

    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_WRITEV,
            in("rdi") fd,
            in("rsi") vec,
            in("rdx") veccnt,
            lateout("rax") result,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }

    if result.is_negative() {
        Err(Errno(result.unsigned_abs() as u16))
    } else {
        Ok(result as usize)
    }
}

pub enum WriteError {
    Errno(Errno),
    WriteZero,
}

pub fn write_all(fd: i32, bytes: &[u8]) -> Result<(), WriteError> {
    let mut offset = 0usize;

    while offset < bytes.len() {
        let remaining = &bytes[offset..];

        match write(fd, remaining.as_ptr(), remaining.len()) {
            Err(errno) if errno == Errno::EINTR => continue,
            Err(errno) => return Err(WriteError::Errno(errno)),
            Ok(0) => return Err(WriteError::WriteZero),
            Ok(written) => offset += written as usize,
        }
    }

    Ok(())
}

/// Writes all vectors to `fd`, retrying interrupted and partial writes.
///
/// # Empty vectors
///
/// All vectors passed to this function must have a non-zero length.
///
/// `write_vectored` treats a zero-byte write as [`WriteError::WriteZero`].
/// Therefore, a trailing empty vector may cause `WriteZero` even after all
/// non-empty data has been written successfully.
///
/// Callers handling user-provided content must replace empty values with an
/// appropriate non-empty representation before constructing the vectors.
/// For example, an empty file operand may be displayed as `''`.
pub fn write_vectored(fd: i32, vector: &mut [WriteVector]) -> Result<(), WriteError> {
    let mut offset = 0usize;

    while offset < vector.len() {
        let vec = &mut vector[offset..];

        match writev(fd, vec.as_ptr(), vec.len()) {
            Err(errno) if errno == Errno::EINTR => continue,
            Err(errno) => return Err(WriteError::Errno(errno)),
            Ok(0) => return Err(WriteError::WriteZero),
            Ok(mut written) => {
                for item in vec.iter_mut() {
                    if item.len < written {
                        offset += 1;
                        written -= item.len;

                        continue;
                    }

                    if item.len == written {
                        offset += 1;
                        break;
                    }

                    item.len -= written;
                    item.base = unsafe { item.base.add(written) };

                    break;
                }
            }
        }
    }

    Ok(())
}
