import os
from pathlib import Path

import pytest


def is_elf(path: Path) -> bool:
    if not path.is_file():
        return False

    try:
        with path.open("rb") as file:
            return file.read(4) == b"\x7fELF"
    except OSError:
        return False


@pytest.fixture(scope="session")
def executable() -> Path:
    try:
        path = Path(os.environ["ELF_PATH"])
    except KeyError:
        pytest.fail("ELF_PATH environment variable should be set")

    if not is_elf(path):
        pytest.fail(f"{path} is not an ELF file")

    if not os.access(path, os.X_OK):
        pytest.fail(f"{path} is not executable")

    return path
