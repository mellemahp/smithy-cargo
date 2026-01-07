// Import generated shapes.
mod shapes {
    include!(concat!(env!("SMITHY_OUTPUT_DIR"),
        "/", "source",
        "/", "test-rust-codegen",
        "/", "demo.rs")
    );
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
}
