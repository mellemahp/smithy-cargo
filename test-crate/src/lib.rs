
// Import generated shapes.
pub mod shapes {
    use smithy_cargo_macros::add_smithy_files;

    add_smithy_files!("source", "test-rust-codegen");
}

#[cfg(test)]
mod tests {
    use crate::shapes;

    #[test]
    fn test_added_demo() {
        let shape = shapes::Rectangle {
            width: 52,
            height: 50,
        };
        assert_eq!(shape.width, 52);
        assert_eq!(shape.height, 50);
    }

    #[test]
    fn test_added_other() {
        let generated = shapes::Generated { a: 2 };
        assert_eq!(generated.a, 2);
    }
}
