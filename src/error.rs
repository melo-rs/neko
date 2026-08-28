/// A linux error number
///
/// See [`errno(3)`] for the list of error codes and their meanings.
///
/// [`errno(3)`]: https://man7.org/linux/man-pages/man3/errno.3.html
#[derive(PartialEq, Eq, Debug)]
pub struct Errno(pub u16);

impl Errno {
    pub const EPERM: Errno = Errno(1);
    pub const ENOENT: Errno = Errno(2);
    pub const EINTR: Errno = Errno(4);
    pub const EIO: Errno = Errno(5);
    pub const EBADF: Errno = Errno(9);
    pub const EAGAIN: Errno = Errno(11);
    pub const EACCES: Errno = Errno(13);
    pub const EFAULT: Errno = Errno(14);
    pub const EBUSY: Errno = Errno(16);
    pub const EEXIST: Errno = Errno(17);
    pub const ENOTDIR: Errno = Errno(20);
    pub const EISDIR: Errno = Errno(21);
    pub const EINVAL: Errno = Errno(22);
    pub const ENFILE: Errno = Errno(23);
    pub const EMFILE: Errno = Errno(24);
    pub const ENOSPC: Errno = Errno(28);
    pub const EPIPE: Errno = Errno(32);

    pub fn description(&self) -> Option<&'static [u8]> {
        match self.0 {
            1 => Some(b"Operation not permitted"),
            2 => Some(b"No such file or directory"),
            4 => Some(b"Interrupted system call"),
            5 => Some(b"Input/output error"),
            9 => Some(b"Bad file descriptor"),
            11 => Some(b"Resource temporarily unavailable"),
            13 => Some(b"Permission denied"),
            14 => Some(b"Bad address"),
            16 => Some(b"Device or resource busy"),
            17 => Some(b"File exists"),
            20 => Some(b"Not a directory"),
            21 => Some(b"Is a directory"),
            22 => Some(b"Invalid argument"),
            23 => Some(b"Too many open files in system"),
            24 => Some(b"Too many open files"),
            28 => Some(b"No space left on device"),
            32 => Some(b"Broken pipe"),
            _ => None,
        }
    }
}
