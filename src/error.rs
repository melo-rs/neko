use crate::{
    errno::Errno,
    io::{STDERR_FILENO, ToWriteVector},
};

pub trait Error {
    fn write_to_stderr(&self) -> Result<(), Errno>;
}

impl<T1, T2> Error for (T1, T2)
where
    T1: ToWriteVector,
    T2: ToWriteVector,
{
    fn write_to_stderr(&self) -> Result<(), Errno> {
        eprintln!(b"neko: ", self.0, b": ", self.1)
    }
}

impl<T> Error for (T, Errno)
where
    T: ToWriteVector,
{
    fn write_to_stderr(&self) -> Result<(), Errno> {
        let (context, errno) = self;

        if errno.is_unknown() {
            let mut buffer = [0u8; 5];
            let mut number = errno.0;
            let mut start = buffer.len();

            loop {
                start -= 1;
                buffer[start] = b'0' + (number % 10) as u8;
                number /= 10;

                if number == 0 {
                    break;
                }
            }

            let number_as_slice: &[u8] = &buffer[start..];

            eprintln!(b"neko: ", context, b": ", errno.cause(), number_as_slice)
        } else {
            eprintln!(b"neko: ", context, b": ", errno.cause(),)
        }
    }
}
