<div align="center">
<img alt="logo" width="300" src="./logo.svg"/>
</div>
<h1 align="center">smithy-cargo</h1>
<p align="center">
   <a href="https://github.com/mellemahp/smithy4rs/actions/workflows/rust-checks.yml" title="CI Status">
      <img alt="CI Status" src="https://github.com/mellemahp/smithy-cargo/workflows/ci/badge.svg">
   </a>
   <a href="https://github.com/mellemahp/smithy-cargo/blob/main/LICENSE" title="License">
      <img alt="License" src="https://img.shields.io/badge/License-Apache_2.0-blue.svg">
   </a>
   <a href="https://crates.io/crates/smithy-cargo" title="crates.io">
      <img alt="crates.io" src="https://img.shields.io/crates/v/smithy-cargo.svg">
   </a>
   <a href="https://deps.rs/repo/github/mellemahp/smithy-cargo" title="dependencies">
      <img alt="dependencies" src ="https://deps.rs/repo/github/mellemahp/smithy-cargo/status.svg">
   </a>
</p>

Cargo build tool for using the [Smithy](https://smithy.io/) toolchain from cargo build scripts (`build.rs`).

This crate does not build models itself; it calls out to the [Smithy CLI](https://smithy.io/2.0/guides/smithy-cli/index.html), 
expecting it to be installed on the current machine.

## Getting Started
> [!IMPORTANT]  
> Before you proceed, make sure you have the [Smithy CLI installed](https://smithy.io/2.0/guides/smithy-cli/cli_installation.html#cli-installation).

### Create a project

First, create a new cargo project and add `smithy-cargo` as a build dependency 

```console
cargo new smithy-cargo-example \
&& cd smithy-cargo-example \
&& cargo add --build smithy-cargo
```

### Add a `smithy-build` configuration

Next, add a [`smithy-build.json`](https://smithy.io/2.0/guides/smithy-build-json.html#smithy-build-json) 
config file to the root of the project.

This config file determines how Smithy will build your models.

### Add Smithy models

Now, add any smithy models we want to build to a `model/` directory within our cargo project.
`smithy-cargo` will automatically discover any smithy files within the `model/` directory
and include them as sources for the Smithy build.

### Configure Cargo build script
Finally, configure `smithy-cargo` to run as part of your cargo build script (`build.rs`): 

```rust
use smithy_cargo::SmithyBuild;

fn main() {
    SmithyBuild::new().execute().expect("Failed to build Smithy models");
}
```

### Project layout

Your fully configured cargo project should look something like:
```console 
.
├── Cargo.toml
├── build.rs
├── model/
│   └── a.smithy
├── smithy-build.json
└── src/
    └── main.rs
```

### Executing the Smithy toolchain

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
    "my-rust-codegen": {
       "config-value": "..."
    }
  }
}
```

Might generate a number of `.rs` files as build artifacts.

We can use the built-in `include` macro and the `$SMITHY_OUTPUT_DIR` 
environment variable added by the smithy-cargo build tool to quickly add 
generated files to our project:

```rust

// Module containing all of our generated Smithy shapes
mod shapes {
    // Adds generated file from the "example-rust-codegen" plugin in the "source" projection. 
    // Note: the "source" projection is the default projection for Smithy.
    include!(concat!(env!("SMITHY_OUTPUT_DIR"),
        "/", "source", // <- Projection name
        "/", "example-rust-codegen", // <- Plugin name
        "/", "example.rs") // <- Generated file to include
    );
}

fn my_function(string: String) {
    // Example usage
    let shapes::GeneratedShapeA { fieldA: 2 };
    
    // ...
}
```

## Configuration 

To configure the Smithy-build process, use one of the following configuration methods on `SmithyBuild` struct:  

| config method            | description                                                                | default                                                                                   |
|--------------------------|----------------------------------------------------------------------------|-------------------------------------------------------------------------------------------|
| `path("/path/")`         | Set the relative path to use as the root for the Smithy build process      | crate root dir                                                                            |
| `out_dir("/path/")`      | Sets the output directory for the Smithy Build.                            | `$OUT_DIR/smithy` or the setting in the `smithy-build.json`                               |
| `projection("name")`     | Sets the projection to use for the Smithy build                            | `source`                                                                                  |
| `plugin("name")`         | Sets a single plugin to build. If left unset, all plugins will be built    | `None`                                                                                    |
| `model("/path/")`        | Adds a model file to build discovery path. Repeatable                      | By default, any models in the `model/` directory are added.                               |
| `config("/path/")`       | Add a smithy-build config                                                  | `smithy-build.json` config at root of crate.                                              |
| `no_config()`            | Disable use of config file                                                 | `false`                                                                                   |
| `debug()`                | Enables debug printing in Smithy build output                              | the `$CARGO_LOG` environment variable is scraped to determine whether to set this or not. |
| `format(true/false)`     | Whether to enable/disable formatting                                       | `true`                                                                                    |
| `quiet()`                | Disables all output except errors                                          | N/A                                                                                       |
| `allow_unknown_traits()` | Ignore unknown traits when validating models                               | N/A                                                                                       |
| `env("key", "val")`      | Configure an environment variable for the Smithy build process. Repeatable | N/A                                                                                       |


## License
This library is licensed under the Apache 2.0 License.