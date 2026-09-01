use crate::{
    error::Errno,
    x86_64::{SYS_READ, SYS_WRITE, SYS_WRITEV},
};
use core::{arch::asm, ffi::CStr, marker::PhantomData, mem::MaybeUninit};

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
/// See [`stderr(3)`].
///
/// [`stderr(3)`]: https://man7.org/linux/man-pages/man3/stderr.3.html
pub const STDERR_FILENO: i32 = 2;

/// Reads data from the given file descriptor into `buffer` using the Linux
/// [`read(2)`] system call.
///
/// Returns the number of bytes read. A successful call initializes the first
/// returned number of elements in `buffer`. Partial reads are possible.
///
/// [`read(2)`]: https://man7.org/linux/man-pages/man2/read.2.html
pub fn read(fd: i32, buffer: &mut [MaybeUninit<u8>]) -> Result<usize, Errno> {
    let result: isize;

    // SAFETY: `buffer` provides valid writable memory for `buffer.len()`
    // bytes, and the registers follow the Linux x86-64 syscall ABI.
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_READ,
            in("rdi") fd,
            in("rsi") buffer.as_mut_ptr(),
            in("rdx") buffer.len(),
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

/// Writes data from `buffer` to the given file descriptor using the Linux
/// [`write(2)`] system call.
///
/// Returns the number of bytes written. Partial writes are possible. Use
/// [`write_all`] to ensure the entire buffer is written.
///
/// [`write(2)`]: https://man7.org/linux/man-pages/man2/write.2.html
pub fn write(fd: i32, buffer: &[u8]) -> Result<usize, Errno> {
    let result: isize;

    // SAFETY: `buffer` provides valid readable memory for `buffer.len()`
    // bytes, and the registers follow the Linux x86-64 syscall ABI.
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_WRITE,
            in("rdi") fd,
            in("rsi") buffer.as_ptr(),
            in("rdx") buffer.len(),
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
pub struct WriteVector<'a> {
    base: *const u8,
    len: usize,
    _lifetime: PhantomData<&'a [u8]>,
}

pub const trait Writable {
    fn to_write_vector(&self) -> WriteVector<'_>;
}

const impl<const N: usize> Writable for [u8; N] {
    fn to_write_vector(&'_ self) -> WriteVector<'_> {
        WriteVector {
            base: self.as_ptr(),
            len: self.len(),
            _lifetime: PhantomData,
        }
    }
}

const impl Writable for &[u8] {
    fn to_write_vector(&'_ self) -> WriteVector<'_> {
        WriteVector {
            base: self.as_ptr(),
            len: self.len(),
            _lifetime: PhantomData,
        }
    }
}

const impl Writable for &CStr {
    fn to_write_vector(&'_ self) -> WriteVector<'_> {
        WriteVector {
            base: self.as_ptr().cast(),
            len: self.count_bytes(),
            _lifetime: PhantomData,
        }
    }
}

impl<'a> WriteVector<'a> {
    pub const fn from_slice(slice: &'a [u8]) -> Self {
        Self {
            base: slice.as_ptr(),
            len: slice.len(),
            _lifetime: PhantomData,
        }
    }

    pub const fn from_c_str(c_str: &'a CStr) -> Self {
        Self {
            base: c_str.as_ptr().cast(),
            len: c_str.count_bytes(),
            _lifetime: PhantomData,
        }
    }
}

pub fn writev(fd: i32, vectors: &[WriteVector]) -> Result<usize, Errno> {
    let result: isize;

    // SAFETY: `vectors` provides a valid contiguous array of `vectors.len()`
    // iovecs, and the registers follow the Linux x86-64 syscall ABI.
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_WRITEV,
            in("rdi") fd,
            in("rsi") vectors.as_ptr(),
            in("rdx") vectors.len(),
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

/// Error used when a non-empty write reports zero bytes written.
///
/// Following GNU coreutils, zero progress is treated as `ENOSPC`.
/// Historically, some buggy device drivers, such as Linux 1.2.13's `/dev/fd0`,
/// returned zero when attempting to write beyond the end of a device instead
/// of reporting an error.
///
/// Treating this as an error also prevents write loops from retrying forever
/// when the underlying descriptor makes no progress.
pub const ZERO_PROGRESS_WRITE_ERRNO: Errno = Errno::ENOSPC;

/// Writes the entire `buffer` to the given file descriptor.
///
/// This function will continuously calls [`write()`] until there is no more data to
/// be written or an error other than [`Errno::EINTR`] is returned. The first error
/// that is not [`Errno::EINTR`] generated from this function will be returned.
///
/// This function will never call [`write()`] if the buffer contains no data.
pub fn write_all(fd: i32, buffer: &[u8]) -> Result<(), Errno> {
    let mut offset = 0usize;

    while offset < buffer.len() {
        match write(fd, &buffer[offset..]) {
            Err(errno) if errno == Errno::EINTR => continue,
            Err(errno) => return Err(errno),
            Ok(0) => return Err(ZERO_PROGRESS_WRITE_ERRNO),
            Ok(written) => offset += written,
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
pub fn write_vectored(fd: i32, vectors: &mut [WriteVector]) -> Result<(), Errno> {
    let mut offset = 0usize;

    while offset < vectors.len() {
        let remaining = &mut vectors[offset..];

        match writev(fd, remaining) {
            Err(errno) if errno == Errno::EINTR => continue,
            Err(errno) => return Err(errno),
            Ok(0) => return Err(ZERO_PROGRESS_WRITE_ERRNO),
            Ok(mut written) => {
                for item in remaining.iter_mut() {
                    if item.len < written {
                        offset += 1;
                        written -= item.len;

                        continue;
                    }

                    if item.len == written {
                        offset += 1;
                        break;
                    }

                    // SAFETY: This branch is reached only when
                    // `written < item.len()`. `base` points to the start of
                    // `item.len()` valid bytes, so advancing it by `written`
                    // keeps it within that allocation.
                    item.base = unsafe { item.base.add(written) };
                    item.len -= written;

                    break;
                }
            }
        }
    }

    Ok(())
}
