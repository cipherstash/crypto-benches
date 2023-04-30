#!/usr/bin/env bash

# The script builds & tests all of the pnpm-managed @cipherstash projects.

# This script is written using the bash script best practices (see:
# https://kvz.io/bash-best-practices.html)

set -e # exit when a command fails
set -u # exit when script tries to use undeclared variables
if [[ -n "${DEBUG_BUILD_SH:-}" ]]; then
  set -x # trace what gets executed (useful for debugging)
fi

trap "echo SOMETHING WENT WRONG - please read the logs above and see if it helps you figure out what is wrong - and also ask an engineer help" ERR

subproject_setup() {
  cargo install cargo-criterion
  cargo install criterion-table
}

subproject_bench() {
  RUSTFLAGS="--cfg aes_armv8" cargo criterion --message-format=json | criterion-table > BENCHMARKS.md
}

subcommand="${1:-build}"
case $subcommand in
  setup)
    subproject_setup
    ;;

  bench)
    subproject_bench
    ;;

  *)
    echo "Unknown build subcommand '$subcommand'"
    exit 1
    ;;
esac
