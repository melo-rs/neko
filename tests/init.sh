#!/bin/sh
# shellcheck disable=SC2034
# Utilities for neko's executable tests

# This is used to simplify checking of the return value which is useful when
# ensuring a command fails as desired. Just doing `command ... &&fail=1` will
# not catch a segfault in command for example. With this helper you instead 
# check an explicit exit code like
#   returns_ 1 command ... || fail
returns_ () {
  # Disable tracing so it doesn't interfere with stderr of the wrapped command
  { set +x; } 2>/dev/null

  exp_exit="$1"
  shift
  "$@"
  ret=$?
  [ "$ret" -eq "$exp_exit" ] && ret_=0 || ret_=1

  [ "$VERBOSE" = true ] && set -x

  { return "$ret_"; } 2>/dev/null
}

# Used as
#   compare EXPECTED ACTUAL
compare () {
  LC_ALL=C diff -u "$1" "$2"
}

ME_=${0##*/}

warn_ () {
  printf '%s\n' "$*" >&2
}

framework_failure_ () {
  warn_ "$ME_: set-up failure: $*"
  exit 99
}

setup_ () {
  [ "$VERBOSE" = true ] && set -x

  initial_cwd_=$PWD

  test_dir_=$(mktemp -d "$initial_cwd_/.test-${ME_}.XXXXXX") \
    || framework_failure_ "failed to create temporary directory"

  cd "$test_dir_" \
    || framework_failure_ "failed to enter temporary directory"
}

remove_tmp_ () {
  status=$?

  cd "$initial_cwd_" || exit 99

  chmod -R u+rwx "$test_dir_" 2>/dev/null

  rm -rf "$test_dir_" || {
    [ "$status" -ne 0 ] || status=1
  }

  exit "$status"
}

setup_
trap remove_tmp_ EXIT

# Error messages
EBADF='不正なファイル記述子です'
ENOENT='そのようなファイルやディレクトリはありません'
ENOSPC='デバイスに空き領域がありません'
EISDIR='ディレクトリです'
