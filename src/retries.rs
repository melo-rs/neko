use crate::errno::Errno;

/// Executes `f`, retrying it whenever it returns [`Errno::EINTR`].
///
/// Linux system calls may be interrupted by signal delivery before they
/// complete. In that case, the syscall can return `EINTR` to indicate that
/// the operation was interrupted rather than permanently failed.
///
/// For operations where retrying after `EINTR` is valid, the usual response
/// is to invoke the syscall again. This function centralizes that behavior
/// by repeatedly calling `f` until it succeeds or returns a different error.
///
/// Errors other than `EINTR` are returned unchanged.
pub fn retry_on_eintr<F, R>(mut f: F) -> Result<R, Errno>
where
    F: FnMut() -> Result<R, Errno>,
{
    loop {
        match f() {
            Err(errno) if errno == Errno::EINTR => continue,
            result => break result,
        }
    }
}
