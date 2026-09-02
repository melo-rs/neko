#[macro_export]
macro_rules! eprintln {
    () => {
        {
            use $crate::io::ToWriteVector as _;

            $crate::io::write_vectored(
                $crate::io::STDERR_FILENO,
                &mut [b"\n".to_write_vector()],
            )
        }
    };

    ($($arg:expr),+ $(,)?) => {
        {
            use $crate::io::ToWriteVector as _;

            $crate::io::write_vectored(
                $crate::io::STDERR_FILENO,
                &mut [
                    $(
                        $arg.to_write_vector(),
                    )+
                    b"\n".to_write_vector(),
                ],
            )
        }
    };
}
