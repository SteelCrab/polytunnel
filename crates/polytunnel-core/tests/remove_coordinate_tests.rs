use polytunnel_core::parse_remove_coordinate;

#[test]
fn valid_coordinate() {
    let result = parse_remove_coordinate("com.google.guava:guava").unwrap();
    assert_eq!(result, "com.google.guava:guava");
}

#[test]
fn single_part_fails() {
    let result = parse_remove_coordinate("guava");
    assert!(result.is_err());
}

#[test]
fn three_parts_with_version_fails() {
    let result = parse_remove_coordinate("com.google.guava:guava:33.0.0");
    assert!(result.is_err());
}

#[test]
fn empty_parts_fails() {
    let result = parse_remove_coordinate(":guava");
    assert!(result.is_err());

    let result = parse_remove_coordinate("com.google.guava:");
    assert!(result.is_err());
}
