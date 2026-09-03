import os
import subprocess
from pathlib import Path


def test_missing_file(executable: Path, tmp_path: Path):
    path = tmp_path / "missing"

    result = subprocess.run([executable, path], capture_output=True, check=False)

    assert result.returncode == 1
    assert result.stdout == b""

    assert result.stderr.splitlines() == [
        b"neko: " + os.fsencode(path) + ": そのようなファイルやディレクトリはありません".encode()
    ]


def test_read_error(executable: Path):
    result = subprocess.run(
        [executable, "/proc/self/mem"],
        capture_output=True,
        check=False,
    )

    assert result.returncode == 1
    assert result.stderr.startswith(b"neko: /proc/self/mem: ")


def test_continues_after_recoverable_errors(executable: Path, tmp_path: Path):
    denied = tmp_path / "denied"
    denied.write_bytes(b"secret")
    denied.chmod(0)

    normal = tmp_path / "normal"
    normal.write_bytes(b"normal")

    missing = tmp_path / "missing"

    result = subprocess.run([executable, denied, normal, missing], capture_output=True, check=False)

    assert result.returncode == 1
    assert result.stdout == b"normal"

    assert result.stderr.splitlines() == [
        b"neko: " + os.fsencode(denied) + ": 許可がありません".encode(),
        b"neko: " + os.fsencode(missing) + ": そのようなファイルやディレクトリはありません".encode(),
    ]


def test_same_file_as_stdout(executable: Path, tmp_path: Path):
    path = tmp_path / "file"
    path.write_bytes(b"hello")

    with path.open("wb") as stdout:
        result = subprocess.run([executable, path], stdout=stdout, stderr=subprocess.PIPE, check=False)

    assert result.returncode == 1


def test_same_file_as_stdout_append(executable: Path, tmp_path: Path):
    path = tmp_path / "file"
    path.write_bytes(b"x")

    with path.open("ab") as stdout:
        result = subprocess.run(
            [executable, path],
            stdout=stdout,
            stderr=subprocess.PIPE,
            check=False,
        )

    assert result.returncode == 1
    assert path.read_bytes() == b"x"


def test_same_file_as_stdin_and_stdout(executable: Path, tmp_path: Path):
    path = tmp_path / "file"
    path.write_bytes(b"x")

    with (
        path.open("rb") as stdin,
        path.open("ab") as stdout,
    ):
        result = subprocess.run([executable, "-"], stdin=stdin, stdout=stdout, stderr=subprocess.PIPE, check=False)

    assert result.returncode == 1
    assert path.read_bytes() == b"x"


def test_dev_full(executable: Path):
    with open("/dev/full", "wb") as stdout:
        result = subprocess.run([executable], input=b"x", stdout=stdout, stderr=subprocess.PIPE, check=False)

    assert result.returncode == 1
    assert result.stderr == ("neko: 書き込みエラー: デバイスに空き領域がありません\n".encode())


def test_closed_stdout(executable: Path):
    result = subprocess.run(
        [executable], input=b"x", stderr=subprocess.PIPE, preexec_fn=lambda: os.close(1), check=False
    )

    assert result.returncode == 1
    assert result.stderr == "neko: 標準出力: 不正なファイル記述子です\n".encode()
