# RustHLL - Zack McKevitt and Jackson Sippe

Final project for CSCI 5454 Algorithms. This repository presents a Rust based implementation of the [HyperLogLog](https://hal.science/hal-00406166/) algorithm for estimating the cardinality of large datasets.

RustHLL is implemented as a library and allows users to estimate the cardinality of large text files where elements are tokenized by whitespace.

## Building

RustHLL can be built with ```cargo build```.

## Testing

Due to the large file size of each test used in the project presentation, they are not included in this repository. However, it is easy to generate your own test files and add them to the tests in the ```tests/``` directory. There is a sample text file for War and Peace with a small cardinality (roughly 8000 elements) that can be used, although the small cardinality of this file will induce larger error. All tests can be run by running the command: ```cargo test```.
