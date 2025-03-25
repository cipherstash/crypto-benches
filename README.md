# Crypto Benchmarks

## Running the Benchmarks

Ensure you have [set up the repo locally](#setup), then run:

```sh
./build.sh setup
./build.sh bench
```

The results will output to a file called `BENCHMARKS.md`.

## Results

Feel free to open a PR and add your own results if your system is different to the ones tested below.

Please include details of the system you test with in the PR.

_Updated to include AWS LC AES-GCM-SIV benches_

| System | OS | Results |
|--------|----|---------|
| Mac Studio M1 Max 64GB RAM | MacOS 14.5 | [BENCHMARKS](results/mac-studio-14.md) |

## Setup

Set up the benchmark suite locally by running:

```bash
# clone the repo
git clone https://github.com/cipherstash/crypto-benches
cd crypto-benches

# install dependencies
./build.sh setup

# check everything compiles
./build.sh test
```
