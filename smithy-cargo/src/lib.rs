use std::{env, io};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::ffi::{OsStr, OsString};
use std::io::ErrorKind;

// Workaround for cargo not printing out log messages from build
// See: https://github.com/rust-lang/cargo/issues/985#issuecomment-1071667472
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub struct SmithyBuild {
    // Path to use as root for smithy build process
    path: PathBuf,
    // the output dir for the build command (default `$OUT_DIR/smithy`)
    out_dir: PathBuf,
    // projection to use for build (default: `source`)
    projection: Option<OsString>,
    // Plugin to use for build (default: none)
    plugin: Option<OsString>,
    // Additional Model files and directories to load.
    models: Vec<PathBuf>,
    // Path to smithy-build.json config
    configs: Vec<PathBuf>,
    // Disables config discover. Cannot be set if `configs` are provided.
    no_config: bool,
    // Force the use of ANSI colors in output
    force_color: bool,
    // Determines if debug logging should be printed by smithy CLI. Uses the
    // `CARGO_LOG` log level by default.
    debug: bool,
    //Force the use of ANSI colors.
    quiet: bool,
    // Ignore unknown traits when validating models.
    allow_unknown_traits: bool,
    // Environment variables to pass to Smithy build process.
    env: Vec<(OsString, OsString)>,
}

impl SmithyBuild {
    pub fn new() -> SmithyBuild {
        let path = env::current_dir().unwrap();
        let out_dir = path.join(env::var("OUT_DIR").unwrap_or("target".into()))
            .join(String::from("smithy"));
        SmithyBuild {
            path,
            out_dir,
            projection: None,
            plugin: None,
            models: vec![],
            configs: vec![],
            no_config: false,
            debug: match env::var("CARGO_LOG") {
                Ok(s) => s == "debug",
                Err(_) => false,
            },
            force_color: false,
            quiet: false,
            allow_unknown_traits: false,
            env: vec![],
        }
    }

    /// Set the relative path to use as the root for the Smithy build process
    ///
    /// The default path for executing the build process is the crate root dir.
    pub fn path<>(mut self, path: impl AsRef<Path>) -> SmithyBuild {
        self.path = env::current_dir().unwrap().join(path);
        self
    }

    /// Sets the output directory for the Smithy Build.
    ///
    /// This is automatically set to `$OUT_DIR/smithy` or the setting in the `smithy-build.json` by default.
    /// Most users will not need to change this.
    pub fn out_dir<P: AsRef<Path>>(&mut self, out: P) -> &mut SmithyBuild {
        self.out_dir = out.as_ref().to_path_buf();
        self
    }

    /// Sets the projection to use for the Smithy build.
    ///
    /// If unset, the `source` projection will be used as the default.
    pub fn projection<T: AsRef<OsStr>>(&mut self, projection: T) -> &mut SmithyBuild {
        self.projection = Some(projection.as_ref().to_owned());
        self
    }

    /// Sets a single plugin to build.
    ///
    /// If unset, the Smithy build process will build all plugins.
    pub fn plugin<T: AsRef<OsStr>>(&mut self, plugin: T) -> &mut SmithyBuild {
        self.projection = Some(plugin.as_ref().to_owned());
        self
    }

    /// Adds a model file to build discovery path.
    ///
    /// By default, any models in the `model/` directory are discovered
    pub fn model<T: AsRef<PathBuf>>(&mut self, model: T) -> &mut SmithyBuild {
        self.models.push(model.as_ref().to_owned());
        self
    }

    /// Add a smithy-build config
    ///
    /// If no configs are specified, the build will default to `smithy-build.json`
    /// config at root of crate.
    pub fn config<T: AsRef<PathBuf>>(&mut self, config: T) -> &mut SmithyBuild {
        self.configs.push(config.as_ref().to_owned());
        self
    }

    /// Disable config file detection and use.
    pub fn no_config(&mut self) -> &mut Self {
        self.no_config = true;
        self
    }

    /// Enables debug printing in Smithy build output.
    ///
    /// By default, the `$CARGO_LOG` environment variable is scraped to determine
    /// whether to set this or not.
    pub fn debug(&mut self) -> &mut SmithyBuild {
        self.debug = true;
        self
    }

    /// Silence output except errors.
    pub fn quiet(&mut self) -> &mut SmithyBuild {
        self.quiet = true;
        self
    }

    /// Ignore unknown traits when validating models.
    pub fn allow_unknown_traits(&mut self) -> &mut SmithyBuild {
        self.allow_unknown_traits = true;
        self
    }

    /// Configure an environment variable for the Smithy build process.
    pub fn env<K, V>(&mut self, key: K, value: V) -> &mut SmithyBuild
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.env.push((key.as_ref().to_owned(), value.as_ref().to_owned()));
        self
    }

    fn args(&self) -> Vec<OsString> {
        let mut args = vec![OsString::from("build")];

        // Set output directory
        // TODO: Should this respect settings in smithy-build.json?
        args.push("--output".into());
        args.push(self.out_dir.as_os_str().into());

        if let Some(p) = &self.projection {
            args.push("--projection".into());
            args.push(p.into());
        }

        if let Some(p) = &self.plugin {
            args.push("--plugin".into());
            args.push(p.into());
        }

        // Add configs
        for config in &self.configs {
            args.push("--config".into());
            args.push(config.into());
        }

        // Flags
        if self.no_config { args.push("--no-config".into()) };
        if self.debug { args.push("--debug".into()); }
        if self.force_color { args.push("--force-color".into()); }
        if self.quiet { args.push("--quiet".into()) };
        if self.allow_unknown_traits { args.push("--aut".into()) };

        // Add models, starting with model/ default dir if it exists
        if self.path.join("model").exists() {
            println!("cargo:rerun-if-changed=model/");
            args.push("model/".into());
        }
        for model in &self.models {
            println!("cargo:rerun-if-changed={:?}", model.as_os_str());
            args.push(model.into());
        }
        args
    }

    pub fn execute(&self) -> io::Result<Output> {
        let output = Command::new("smithy")
            .current_dir(&self.path)
            .args(self.args())
            .envs(self.env.clone())
            .output()
            .expect("Failed to execute Smithy build");
        if self.quiet || !output.status.success() {
            p!("\n{}", String::from_utf8(output.stderr.clone()).unwrap());
        }
        if !output.status.success() {
            return Err(io::Error::new(ErrorKind::Other, "Smithy build failed"));
        }

        // Set env var so it can be used to help include output in your code
        println!("cargo:rustc-env=SMITHY_OUTPUT_DIR={}", self.out_dir.display());

        Ok(output)
    }
}
