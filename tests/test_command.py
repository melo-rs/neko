import os
import subprocess
import threading
from pathlib import Path

import pytest
from utils import close_stdout


def is_elf(path: Path) -> bool:
    if not path.is_file():
        return False

    try:
        with path.open("rb") as file:
            return file.read(4) == b"\x7fELF"
    except OSError:
        return False

@pytest.fixture(scope="session")
def command() -> Path:
    try:
        path = Path(os.environ["RELEASE_BIN"])
    except KeyError:
        pytest.fail("RELEASE_BIN environment variable should be set")
    
    if not is_elf(path):
        pytest.fail(f"{path} is not an ELF file")

    if not os.access(path, os.X_OK):
        pytest.fail(f"{path} is not executable")

    return path


def test_stdin(command: Path):
    contents = b"hello from stdin"

    result = subprocess.run(
        [command],
        input=contents,
        capture_output=True,
        check=False
    )

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == contents


def test_file(command: Path, tmp_path: Path):
    file = tmp_path / "file"
    file.write_bytes(b"contents\n")

    result = subprocess.run(
        [command, file],
        capture_output=True,
        check=False,
    )

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"contents\n"

def test_multiple_files(command: Path, tmp_path: Path):
    first = tmp_path / "first"
    first.write_bytes(b"first\n")

    second = tmp_path / "second"
    second.write_bytes(b"second\n")

    result = subprocess.run([command, first, second], capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"first\nsecond\n"

def test_stdin_no_args(command: Path):
    result = subprocess.run([command], input=b"hello from stdin!", capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"hello from stdin!"


def test_stdin_dash(command: Path):
    result = subprocess.run([command, "-"], input=b"hello from stdin!", capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"hello from stdin!"


def test_mixed_files_and_stdin(command: Path, tmp_path: Path):
    file_a = tmp_path / "a"
    file_a.write_bytes(b"a\n")

    file_b = tmp_path / "b"
    file_b.write_bytes(b"b\n")

    result = subprocess.run([command, file_a, "-", file_b], input=b"stdin\n", capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"a\nstdin\nb\n"

def test_empty_stdin(command: Path):
    result = subprocess.run(
        [command], 
        capture_output=True, 
        check=False
    )

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b""

def test_empty_file(command: Path, tmp_path: Path):
    path = tmp_path / "file"
    path.write_bytes(b"")

    result = subprocess.run(
        [command, path], 
        capture_output=True, 
        check=False
    )

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b""

def test_same_file_as_stdout(command: Path, tmp_path: Path):
    path = tmp_path / "file"
    path.write_bytes(b"hello")

    with path.open("wb") as stdout:
        result = subprocess.run(
            [command, path],
            stdout=stdout,
            stderr=subprocess.PIPE,
            check=False
        )

    assert result.returncode == 1

def test_same_file_as_stdout_append(command: Path, tmp_path: Path):
    path = tmp_path / "file"
    path.write_bytes(b"hello")

    with path.open("ab") as stdout:
        result = subprocess.run(
            [command, path],
            stdout=stdout,
            stderr=subprocess.PIPE,
            check=False,
        )

    assert result.returncode == 1
    assert path.read_bytes() == b"hello"

def test_same_file_as_stdin_and_stdout(command: Path, tmp_path: Path):
    path = tmp_path / "file"
    path.write_bytes(b"hello")

    with (
        path.open("rb") as stdin,
        path.open("ab") as stdout,
    ):
        result = subprocess.run(
            [command, "-"],
            stdin=stdin,
            stdout=stdout,
            stderr=subprocess.PIPE,
            check=False
        )

    assert result.returncode == 1
    assert path.read_bytes() == b"hello"

def test_dev_full(command: Path):
    with open("/dev/full", "wb") as stdout:
        result = subprocess.run(
            [command],
            input=b"hello",
            stdout=stdout,
            stderr=subprocess.PIPE,
            check=False
        )

    assert result.returncode == 1
    assert result.stderr == ("neko: 書き込みエラー: デバイスに空き領域がありません\n".encode())

def test_closed_stdout(command: Path):
    result = subprocess.run(
        [command],
        input=b"x",
        stderr=subprocess.PIPE,
        preexec_fn=close_stdout,
        check=False
    )

    assert result.returncode == 1
    assert result.stderr == "neko: 標準出力: 不正なファイル記述子です\n".encode()

def test_read_error(command: Path):
    result = subprocess.run(
        [command, "/proc/self/mem"],
        capture_output=True,
        check=False,
    )

    assert result.returncode == 1
    assert result.stderr.startswith(b"neko: /proc/self/mem: ")

def test_proc_file(command: Path):
    result = subprocess.run(
        [command, "/proc/cpuinfo"],
        capture_output=True,
        check=False,
    )

    assert result.returncode == 0
    assert result.stdout
    assert result.stderr == b""

def test_immediate_streaming(command: Path, tmp_path: Path):
    fifo = tmp_path / "fifo"
    os.mkfifo(fifo)

    received = bytearray()
    output_ready = threading.Event()

    def read_fifo():
        with fifo.open("rb") as stdout:
            received.extend(stdout.read(2))
            output_ready.set()

    reader = threading.Thread(target=read_fifo)
    reader.start()

    with fifo.open("wb") as stdout:
        process = subprocess.Popen(
            [command],
            stdin=subprocess.PIPE,
            stdout=stdout,
            stderr=subprocess.PIPE,
        )

        assert process.stdin is not None

        # Give neko some input, but deliberately DON'T close stdin.
        process.stdin.write(b"1\n")
        process.stdin.flush()

        # Neko must make those bytes available before seeing EOF.
        assert output_ready.wait(timeout=1)
        assert received == b"1\n"

        process.stdin.close()
        assert process.wait() == 0

    reader.join()