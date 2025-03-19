
include!(concat!(env!("SMITHY_OUTPUT_DIR"), "/source/test-rust-codegen/demo.rs"));

#[cfg(test)]
mod tests {
   // use crate::Rectangle;

    use crate::Rectangle;

    #[test]
    fn test_add() {
        println!("STUFF!!!!");
        // let result = SmithyBuild::new()
        //     .execute()
        //     .expect("Failed building Smithy");
        // println!("{:?}", result.projection_plugin_path("source", "test-rust-codegen").as_os_str());
        //
    }
}
