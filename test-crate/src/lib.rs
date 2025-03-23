extern crate smithy_cargo_macros;

#[cfg(test)]
mod tests {
    use smithy_cargo_macros::add_smithy_files;

    add_smithy_files!("source", "test-rust-codegen");

    #[test]
    fn test_added_demo() {
        let shape = Rectangle {
            width: 52,
            height: 50
        };
        assert_eq!(shape.width, 52);
        assert_eq!(shape.height, 50);
    }

    #[test]
    fn test_added_other() {
        let generated = Generated {
            a: 2
        };
        assert_eq!(generated.a, 2);
    }
}
