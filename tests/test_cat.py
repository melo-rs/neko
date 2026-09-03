import subprocess
from pathlib import Path

import pytest


def test_stdin(executable: Path):
    result = subprocess.run([executable], input=b"x", capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"x"


def test_stdin_dash(executable: Path):
    result = subprocess.run([executable, "-"], input=b"x", capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"x"


def test_file(executable: Path, tmp_path: Path):
    path = tmp_path / "file"
    path.write_bytes(b"x")

    result = subprocess.run(
        [executable, path],
        capture_output=True,
        check=False,
    )

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"x"


def test_multiple_files(executable: Path, tmp_path: Path):
    first = tmp_path / "first"
    first.write_bytes(b"first\n")

    second = tmp_path / "second"
    second.write_bytes(b"second\n")

    result = subprocess.run([executable, first, second], capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"first\nsecond\n"


def test_mixed_files_and_stdin(executable: Path, tmp_path: Path):
    file_a = tmp_path / "a"
    file_a.write_bytes(b"a\n")

    file_b = tmp_path / "b"
    file_b.write_bytes(b"b\n")

    result = subprocess.run([executable, file_a, "-", file_b], input=b"stdin\n", capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b"a\nstdin\nb\n"


def test_empty_stdin(executable: Path):
    result = subprocess.run([executable], capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b""


def test_empty_file(executable: Path, tmp_path: Path):
    path = tmp_path / "file"
    path.write_bytes(b"")

    result = subprocess.run([executable, path], capture_output=True, check=False)

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == b""


@pytest.mark.parametrize("size", [0, 1, 4095, 4096, 4097, 8192, 8193])
def test_file_sizes(executable: Path, tmp_path: Path, size: int):
    path = tmp_path / "file"

    contents = bytes(i % 256 for i in range(size))
    path.write_bytes(contents)

    result = subprocess.run(
        [executable, path],
        capture_output=True,
        check=False,
    )

    assert result.returncode == 0
    assert result.stderr == b""
    assert result.stdout == contents


def test_proc_file(executable: Path):
    result = subprocess.run(
        [executable, "/proc/cpuinfo"],
        capture_output=True,
        check=False,
    )

    assert result.returncode == 0
    assert result.stdout
    assert result.stderr == b""
