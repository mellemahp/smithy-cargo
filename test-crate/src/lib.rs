extern crate smithy_cargo_macros;

#[cfg(test)]
mod tests {
    use smithy_cargo_macros::add_smithy_files;

    add_smithy_files!("source", "test-rust-codegen");

   // use crate::Rectangle;

    #[test]
    fn test_add() {
        println!("STUFF!!!!");
        let _x = Rectangle {
            width: 52,
            height: 50
        };
        // let result = SmithyBuild::new()
        //     .execute()
        //     .expect("Failed building Smithy");
        // println!("{:?}", result.projection_plugin_path("source", "test-rust-codegen").as_os_str());
        //
    }
}
