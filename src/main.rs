#![no_std]
#![no_main]

use core::{arch::global_asm, mem::MaybeUninit, panic::PanicInfo};
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

#[unsafe(no_mangle)]
pub extern "C" fn do_start(rsp_ptr: *const usize) -> ! {
    unsafe {
        let argc = *rsp_ptr;

        let mut buffer = MaybeUninit::<[u8; 4096]>::uninit();
        let mut had_error = false;

        if argc == 1 {
            let result = stream_to_stdout(STDIN_FILENO, &mut buffer);

            match result {
                Ok(()) => {}
                Err(StreamError::Read(errno)) => {
                    let _ = write_stdin_error(false, errno);
                    had_error = true
                }
                Err(StreamError::Write(error)) => {
                    let _ = write_stdout_error(error);
                    exit(1);
                }
            }
        } else {
            for operand in 1..argc {
                let pathname_ptr = *rsp_ptr.add(operand + 1) as *const u8;
                let is_stdin = *pathname_ptr == b'-' && *pathname_ptr.add(1) == 0;

                let fd = if is_stdin {
                    STDIN_FILENO
                } else {
                    let openat_result = loop {
                        match openat(AT_FDCWD, pathname_ptr, 0, 0) {
                            Err(errno) if errno == Errno::EINTR => continue,
                            result => break result,
                        }
                    };

                    match openat_result {
                        Ok(fd) => fd,
                        Err(errno) => {
                            let _ = write_operand_error(pathname_ptr, errno);
                            had_error = true;

                            continue;
                        }
                    }
                };

                let result = stream_to_stdout(fd, &mut buffer);

                match result {
                    Ok(()) => {}
                    Err(StreamError::Read(errno)) => {
                        let _ = write_operand_error(pathname_ptr, errno);
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
                        let _ = write_operand_error(pathname_ptr, errno);
                        had_error = true;
                    }
                }
            }
        }

        exit(if had_error { 1 } else { 0 });
    }
}

enum StreamError {
    Read(Errno),
    Write(WriteError),
}

fn stream_to_stdout(fd: i32, buffer: &mut MaybeUninit<[u8; 4096]>) -> Result<(), StreamError> {
    loop {
        let _read = match read(fd, buffer.as_mut_ptr() as *mut u8, 4096) {
            Ok(_read) => _read,
            Err(errno) if errno == Errno::EINTR => continue,
            Err(errno) => break Err(StreamError::Read(errno)),
        };

        if _read == 0 {
            break Ok(());
        }

        let initialized =
            unsafe { core::slice::from_raw_parts(buffer.as_ptr() as *const u8, _read) };

        let result = write_all(STDOUT_FILENO, &initialized);

        if let Err(error) = result {
            break Err(StreamError::Write(error));
        }
    }
}

const UNKNOWN_ERROR_MESSAGE: &[u8] = b"unknown error";
const LINE_FEED: &[u8] = b"\n";

fn write_stdin_error(as_dash: bool, errno: Errno) -> Result<(), WriteError> {
    write_vectored(
        STDERR_FILENO,
        &mut [
            if as_dash {
                WriteVector::from_slice(b"neko: -: ")
            } else {
                WriteVector::from_slice(b"neko: stdin: ")
            },
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
        ],
    )
}

fn write_operand_error(
    pathname_ptr: *const u8,
    errno: Errno,
) -> Result<(), neko_rs::io::WriteError> {
    const PROGRAM: &[u8] = b"neko: ";
    const EMPTY_STR: &[u8] = b"'': ";
    const SEPARATOR: &[u8] = b": ";

    let is_empty_str = unsafe { *pathname_ptr == 0 };
    let description = errno.description().unwrap_or(UNKNOWN_ERROR_MESSAGE);

    if is_empty_str {
        write_vectored(
            STDERR_FILENO,
            &mut [
                WriteVector::from_slice(PROGRAM),
                WriteVector::from_slice(EMPTY_STR),
                WriteVector::from_slice(description),
                WriteVector::from_slice(LINE_FEED),
            ],
        )
    } else {
        write_vectored(
            STDERR_FILENO,
            &mut [
                WriteVector::from_slice(PROGRAM),
                unsafe { WriteVector::from_c_str(pathname_ptr) },
                WriteVector::from_slice(SEPARATOR),
                WriteVector::from_slice(description),
                WriteVector::from_slice(LINE_FEED),
            ],
        )
    }
}
