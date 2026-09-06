#!/bin/sh
# Runs all tests in tests/bin/

srcdir=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd) || exit 1
export srcdir

if [ ! -x "$srcdir/target/x86_64-unknown-linux-none/release/neko" ]
then
  printf 'neko executable not found\n' >&2
  exit 99
fi

# shellcheck disable=SC2123
PATH="$srcdir/target/x86_64-unknown-linux-none/release${PATH:+:$PATH}"
export PATH

set -- "$srcdir"/tests/bin/*.sh

if [ ! -f "$1" ]
then
  printf 'no tests found\n'
  exit 0
fi

printf 'running %d tests\n' "$#"

for test
do
  if sh "$test"
  then
    printf 'test %s ... ok\n' "${test#"$srcdir"/}"
  else
    printf 'test %s ... FAILED\n' "${test#"$srcdir"/}"
    fail=1
  fi
done

exit "${fail:-0}"
