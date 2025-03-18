use std::env;
use std::process::Command;
use smithy_cargo::SmithyBuild;

fn main() {
    // Publish the test code generator to maven local so the
    // Smithy CLI can detect it
    Command::new("./gradlew")
        .current_dir("test-code-generator")
        .arg("publishToMavenLocal")
        .output().unwrap();
    println!("cargo::rerun-if-changed=test-code-generator");

    SmithyBuild::new()
        .execute()
        .expect("Smithy Build failed");
}