# neko

A minimal `cat(1)` implementation for Linux, written in Rust without libc,
with Japanese diagnostics.

neko was built to explore low-level Rust, with as little between the program and the Linux kernel as possible. The program avoids Rust's [libstd](https://doc.rust-lang.org/std/) entirely with `#![no_std]`, provides its own process entry point, and relies directly on the Linux syscall interface.

> neko is named after 猫 — the Japanese word for 'cat'. Badum tss!

## Platforms

neko currently requires x86-64 Linux. Support for other Linux architectures is planned.

## Distribution

Download the latest release for your platform from [GitHub Releases](https://github.com/melo-rs/neko/releases/), extract the archive, and install the binary somewhere in your `PATH`:

```sh
sha256sum -c neko-1.0.1-x86_64-unknown-linux-none.sha256 # checksum
tar -xf neko-1.0.1-x86_64-unknown-linux-none.tar.gz      # extract the release archive
sudo install -m 755 neko /usr/local/bin/neko             # install
```

You can then summon the feline from anywhere:

```sh
neko file.txt
```

## Build & Test

Build neko with Cargo:

```sh
cargo build --release 
```

The test suite can then be run with:

```sh
tests/run.sh
```

Tests are written in POSIX shell and run against the release binary in `target/x86_64-unknown-linux-none/release/`.

## License

Licensed under the [MIT License](https://mit-license.org/).
