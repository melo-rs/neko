#![no_std]
#![no_main]

use core::{
    arch::global_asm,
    ffi::{CStr, c_char},
    mem::MaybeUninit,
    panic::PanicInfo,
};
use neko_rs::{
    error::Errno,
    fs::{close, openat},
    io::{
        STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, WriteError, WriteVector, read, write_all,
        write_vectored,
    },
    process::exit,
    x86_64::AT_FDCWD,
};

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    exit(1);
}

global_asm!(
    r#"
.global _start
_start:
    mov rdi, rsp
    call do_start
"#
);

/// # Safety
///
/// `rsp_ptr` must point to a valid Linux x86-64 initial process stack
/// as provided to `_start`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_start(rsp_ptr: *const usize) -> ! {
    // SAFETY: `rsp_ptr` is valid and points to `argc` as guaranteed by the
    // function's safety contract
    let argc = unsafe { *rsp_ptr };

    let mut buffer = [MaybeUninit::<u8>::uninit(); 4096];
    let mut had_error = false;

    if argc == 1 {
        let result = stream_to_stdout(STDIN_FILENO, &mut buffer);

        match result {
            Ok(()) => {}
            Err(StreamError::Read(errno)) => {
                let _ = write_stdin_error(errno);
                had_error = true
            }
            Err(StreamError::Write(error)) => {
                let _ = write_stdout_error(error);
                exit(1);
            }
        }
    } else {
        for operand_index in 1..argc {
            // SAFETY: `_start` passes the initial stack pointer to this function.
            // Its leading words are laid out as:
            //
            //     [ argc ][ argv[0] ][ argv[1] ] ... [ argv[argc - 1] ][ NULL ]
            //
            // Thus `rsp_ptr.add(operand_index + 1)` points to
            // `argv[operand_index]`. Every non-null argv entry points to a valid
            // NUL-terminated string for the lifetime of the process image.
            let operand = unsafe {
                let operand_ptr = *rsp_ptr.add(operand_index + 1) as *const c_char;
                CStr::from_ptr(operand_ptr)
            };

            let is_stdin = operand == c"-";

            let fd = if is_stdin {
                STDIN_FILENO
            } else {
                let openat_result = loop {
                    match openat(AT_FDCWD, operand, 0, 0) {
                        Err(errno) if errno == Errno::EINTR => continue,
                        result => break result,
                    }
                };

                match openat_result {
                    Ok(fd) => fd,
                    Err(errno) => {
                        let _ = write_operand_error(operand, errno);
                        had_error = true;

                        continue;
                    }
                }
            };

            let result = stream_to_stdout(fd, &mut buffer);

            match result {
                Ok(()) => {}
                Err(StreamError::Read(errno)) => {
                    let _ = write_operand_error(operand, errno);
                    had_error = true
                }
                Err(StreamError::Write(error)) => {
                    let _ = write_stdout_error(error);
                    exit(1);
                }
            }

            if !is_stdin {
                let close_result = close(fd);

                if let Err(errno) = close_result {
                    let _ = write_operand_error(operand, errno);
                    had_error = true;
                }
            }
        }
    }

    exit(if had_error { 1 } else { 0 });
}

enum StreamError {
    Read(Errno),
    Write(WriteError),
}

fn stream_to_stdout(fd: i32, buffer: &mut [MaybeUninit<u8>]) -> Result<(), StreamError> {
    loop {
        let _read = match read(fd, buffer) {
            Ok(_read) => _read,
            Err(errno) if errno == Errno::EINTR => continue,
            Err(errno) => break Err(StreamError::Read(errno)),
        };

        if _read == 0 {
            break Ok(());
        }

        let initialized: &[u8] =
            // SAFETY: `read` guarantees that the first `_read` elements of 
            // `buffer` are initialized, with `_read <= buffer.len()`. 
            // Therefore this range is valid to view as an initialized `[u8]`.
            unsafe { core::slice::from_raw_parts(buffer.as_ptr().cast(), _read) };

        let result = write_all(STDOUT_FILENO, initialized);

        if let Err(error) = result {
            break Err(StreamError::Write(error));
        }
    }
}

const UNKNOWN_ERROR_MESSAGE: &[u8] = b"unknown error";
const LINE_FEED: &[u8] = b"\n";

fn write_stdin_error(errno: Errno) -> Result<(), WriteError> {
    write_vectored(
        STDERR_FILENO,
        &mut [
            WriteVector::from_slice(b"neko: -: "),
            WriteVector::from_slice(errno.description().unwrap_or(UNKNOWN_ERROR_MESSAGE)),
            WriteVector::from_slice(LINE_FEED),
        ],
    )
}

fn write_stdout_error(error: WriteError) -> Result<(), WriteError> {
    write_vectored(
        STDERR_FILENO,
        &mut [
            WriteVector::from_slice(b"neko: stdout: "),
            match error {
                WriteError::WriteZero => {
                    WriteVector::from_slice("書き込みに失敗しました".as_bytes())
                }
                WriteError::Errno(errno) => {
                    WriteVector::from_slice(errno.description().unwrap_or(UNKNOWN_ERROR_MESSAGE))
                }
            },
            WriteVector::from_slice(LINE_FEED),
        ],
    )
}

fn write_operand_error(operand: &CStr, errno: Errno) -> Result<(), neko_rs::io::WriteError> {
    let description = errno.description().unwrap_or(UNKNOWN_ERROR_MESSAGE);

    if operand.is_empty() {
        write_vectored(
            STDERR_FILENO,
            &mut [
                WriteVector::from_slice(b"neko: '': "),
                WriteVector::from_slice(description),
                WriteVector::from_slice(LINE_FEED),
            ],
        )
    } else {
        write_vectored(
            STDERR_FILENO,
            &mut [
                WriteVector::from_slice(b"neko: "),
                WriteVector::from_c_str(operand),
                WriteVector::from_slice(b": "),
                WriteVector::from_slice(description),
                WriteVector::from_slice(LINE_FEED),
            ],
        )
    }
}
