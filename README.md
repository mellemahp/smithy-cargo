# smithy-cargo

[![CI Status](https://github.com/mellemahp/smithy-cargo/workflows/ci/badge.svg)](https://github.com/mellemahp/smithy-cargo/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/smithy-cargo.svg)](https://crates.io/crates/smithy-cargo)
[![dependency status](https://deps.rs/repo/github/mellemahp/smithy-cargo/status.svg)](https://deps.rs/repo/github/mellemahp/smithy-cargo)

Tooling for building [Smithy](https://smithy.io/) models from cargo build scripts (`build.rs`).

This crate does not build models itself; it calls out to the [Smithy CLI](https://smithy.io/2.0/guides/smithy-cli/index.html), 
expecting it to be installed on the current machine.

## Getting Started
> [!IMPORTANT]  
> Before you proceed, make sure you have the [Smithy CLI installed](https://smithy.io/2.0/guides/smithy-cli/cli_installation.html#cli-installation).

First, create a new cargo project and add `smithy-cargo` as a build dependency 

```console
cargo new smithy-cargo-example \
&& cd smithy-cargo-example \
&& cargo add --build smithy-cargo
```

Next, add a [`smithy-build`](https://smithy.io/2.0/guides/smithy-build-json.html#smithy-build-json) config file to the 
root of the project. This config file determines how Smithy will build your models.

Now, add any smithy models we want to build to a `model/` directory within our cargo project.
`smithy-cargo` will automatically discover any smithy files within the `model/` directory
and include them as sources for the Smithy build.

Finally, configure `smithy-cargo` to run as part of your cargo build script (`build.rs`): 

```rust
use smithy_cargo::SmithyBuild;

fn main() {
    SmithyBuild::new().execute().expect("Failed to build Smithy models");
}
```

Your fully configured cargo project should now look something like:
```console 
.
├── Cargo.toml
├── build.rs
├── model
│   └── a.smithy
├── smithy-build.json
└── src
    └── main.rs
```

To run the Smithy build, just run `cargo build` as you would normally and the smithy build 
will be executed by the build script.

## Including generated Rust code
> [!WARNING]
> This package does not provide any Smithy code generation plugins for rust on its own. You 
> will still need to add a rust codegen plugin (such as [smithy4rs](https://github.com/mellemahp/smithy4rs)) 
> to actually generate rust code

Your Smithy build may use a [build plugin](https://smithy.io/2.0/guides/smithy-build-json.html#plugins) 
to generate Rust code that you want to include as part of your crate.

For example the following `smithy-build` config: 
```json 
{
  "version": "1.0",
  "maven": {
    "dependencies": ["com.example:my-rust-code-generator:1.0.0"]
  },
  "plugins": {
    "example-rust-codegen": { }
  }
}
```
Might generate a number of `.rs` files as build artifacts.

The `smithy-cargo-macros` package provides a `add_smithy_files` macro to 
make it easy to include generated rust code in your crate. 

To use the macro, add the following dependencies to your `Cargo.toml`:

```toml 
[dependencies]
smithy-cargo-macros = "<VERSION>"
crabtime = "<VERSION>"
```

Then apply the `add_smithy_files` macro within your rust code to include the generated 
artifacts.

```rust
use smithy_cargo_macros::add_smithy_files;

// Module containing all of our generated Smithy shapes
mod shapes {
    // Adds generated files from the "example-rust-codegen" plugin in the "source" projection. 
    // Note: the "source" projection is the default projection for Smithy.
    add_smithy_files!("source", "example-rust-codegen");
}

fn my_function(string: String) {
    // Example usage
    let shapes::GeneratedShapeA { fieldA: 2 };
    
    // ...
}
```

## License
This library is licensed under the Apache 2.0 License.