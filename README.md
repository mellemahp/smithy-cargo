# smithy-cargo

[![CI Status](https://github.com/mellemahp/smithy-cargo/workflows/ci/badge.svg)](https://github.com/mellemahp/smithy-cargo/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/smithy-cargo.svg)](https://crates.io/crates/smithy-cargo)
[![dependency status](https://deps.rs/repo/github/mellemahp/smithy-cargo/status.svg)](https://deps.rs/repo/github/mellemahp/smithy-cargo)

Tooling for executing Smithy builds from a cargo build scripts (build.rs)

A library for Cargo build scripts to build [Smithy](smithy.io) models. 

This crate does not build models itself; it calls out to the [smithy CLI](), 
expecting it to be installed on the current machine.

## License
This library is licensed under the Apache 2.0 License.