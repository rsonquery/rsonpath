# Overview

This document provides a concise guide on how to use the current state of the Lookup-Table (LUT) code in this crate. While the code is not yet complete, it already offers some useful functionality.

## Environment Parameters

LUT does not support SIMD (Single Instruction, Multiple Data) at this time. To ensure proper functionality, you need to set the following environment parameters:

This forces `nosimd` as the classifier:

```bash
export RSONPATH_UNSAFE_FORCE_SIMD="nosimd;slow_quotes;slow_popcnt"
```

## Set up Python

The LUT code generates graphs to visualize the results of its performance tests. These graphs are created using Python.

Check out the environment.yml file for the necessary Python dependencies:
    
```bash
rsonpath/crates/rsonpath-lib/src/lookup_table/python_statistic/environment.yml
```

## How to run

Currently, the LUT supports the following operations: distances, sichash, performance, and query.

- distances: Generates distance plots for every JSON file in a specified folder.
- sichash: Saves the key and value lists for each JSON file in a given folder to a destination folder.
- performance: Creates graphs measuring build time, query time, and LUT size on the heap.
- query: Executes an rsonpath query on a specified JSON file.

### Examples

Here are some example commands to run the various operations:

```bash
cargo run --bin lut --release -- distances .a_lut_tests/test_data/MB_100 .a_lut_tests
cargo run --bin lut --release -- sichash .a_lut_tests/test_data/MB_100 .a_lut_tests
cargo run --bin lut --release -- performance .a_lut_tests/test_data/MB_1 .a_lut_tests
cargo run --bin lut --release -- query $.person.spouse.person.phoneNumber[*] .a_lut_tests/test_data/kB_1/john_big.json
```

## Run the tests with

The tests are in `crates/rsonpath-lib/tests/lut_query_tests.rs`. You can run them with the command:

```bash
cargo test --test lut_build_tests
cargo test --test lut_query_tests
```

If you want to run the tests with debug messages and filter which files will add to the debug messages run the commands as in the examples below. Here they are for example only allowing the files "tail_skipping" and "lut_query_tests" to write debug messages.

```bash
cargo test --test lut_query_tests -- query_john_big_log --nocapture | rg "(tail_skipping|lut_query_tests)"
cargo test --test lut_query_tests -- query_john_big_log --nocapture | rg "(tail_skipping|lut_query_tests)" --passthru
cargo test --test lut_query_tests -- query_error_1 --nocapture | rg "(tail_skipping|lut_query_tests)"
```