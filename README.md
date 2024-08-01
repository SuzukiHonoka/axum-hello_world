# axum-hello_world

**Version:** 0.1.0  
**Authors:** starx  
**Description:** Hello World with Axum: A practice project using the Tokio ecosystem, including signal handling.

## Overview

`axum-hello_world` is a practice project demonstrating the use of the Axum framework within the Tokio ecosystem. This project focuses on creating a simple "Hello World" web application with signal handling capabilities, showcasing the integration of several popular Rust libraries.

## Dependencies

This project relies on the following Rust crates:

- **axum**: A web framework built on top of Tokio, designed to be simple and ergonomic. Version `0.7.5` is used.
- **tokio**: An asynchronous runtime for Rust, providing concurrency and I/O capabilities. Version `1.39.2` is used with features `rt-multi-thread` and `signal` enabled.
- **tracing**: A framework for instrumenting Rust programs to collect structured, contextual, and async-aware diagnostics. Version `0.1.40` is used.
- **tracing-subscriber**: A library for collecting and recording tracing data. Version `0.3.18` is used.
- **serde**: A framework for serializing and deserializing Rust data structures efficiently and generically. Version `1.0.204` is used.
- **tokio-stream**: An extension of the Tokio runtime providing utilities for working with asynchronous streams. Version `0.1.15` is used.