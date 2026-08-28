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
            1 => Some("許可されていない操作です".as_bytes()),
            2 => Some("そのようなファイルやディレクトリはありません".as_bytes()),
            4 => Some("システムコール割り込み".as_bytes()),
            5 => Some("入力/出力エラーです".as_bytes()),
            9 => Some("不正なファイル記述子です".as_bytes()),
            11 => Some("リソースが一時的に利用できません".as_bytes()),
            13 => Some("許可がありません".as_bytes()),
            14 => Some("不正なアドレスです".as_bytes()),
            16 => Some("デバイスもしくはリソースがビジー状態です".as_bytes()),
            17 => Some("ファイルが存在します".as_bytes()),
            20 => Some("ディレクトリではありません".as_bytes()),
            21 => Some("ディレクトリです".as_bytes()),
            22 => Some("無効な引数です".as_bytes()),
            23 => Some("システム中のファイルを開きすぎです".as_bytes()),
            24 => Some("ファイルを開きすぎです".as_bytes()),
            28 => Some("デバイスに空き領域がありません".as_bytes()),
            32 => Some("壊れたパイプです".as_bytes()),
            _ => None,
        }
    }
}
