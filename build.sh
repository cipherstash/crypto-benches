#!/usr/bin/env bash

# The script builds & tests all of the pnpm-managed @cipherstash projects.

# This script is written using the bash script best practices (see:
# https://kvz.io/bash-best-practices.html)

set -e # exit when a command fails
set -u # exit when script tries to use undeclared variables
set -o pipefail # exit if any command in a pipeline fails
if [[ -n "${DEBUG_BUILD_SH:-}" ]]; then
  set -x # trace what gets executed (useful for debugging)
fi

trap "echo SOMETHING WENT WRONG - please read the logs above and see if it helps you figure out what is wrong - and also ask an engineer help" ERR

subproject_setup() {
  cargo binstall --no-confirm cargo-criterion
  cargo binstall --no-confirm criterion-table
}

subproject_bench() {
  if ! cargo criterion -V >/dev/null 2>&1; then
    echo "error: unable to find: cargo criterion"
    exit 1
  fi
  if ! which criterion-table >/dev/null 2>&1; then
    echo "error: unable to find criterion-table"
    exit 1
  fi
  RUSTFLAGS="--cfg aes_armv8" cargo criterion --message-format=json | criterion-table > BENCHMARKS.md
}

subproject_test() {
  cargo check
}

subcommand="${1:-bench}"
case $subcommand in
  setup)
    subproject_setup
    ;;

  bench)
    subproject_bench
    ;;

  test)
    subproject_test
    ;;

  *)
    echo "Unknown build subcommand '$subcommand'"
    exit 1
    ;;
esac
