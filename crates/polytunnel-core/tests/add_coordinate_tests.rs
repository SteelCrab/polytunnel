//! Tests for parse_add_coordinate and add_dependency_to_file

use polytunnel_core::parse_add_coordinate;

#[test]
fn test_parse_valid_coordinate() {
    let (ga, ver) = parse_add_coordinate("com.google.guava:guava:33.0.0-jre").unwrap();
    assert_eq!(ga, "com.google.guava:guava");
    assert_eq!(ver, "33.0.0-jre");
}

#[test]
fn test_parse_coordinate_missing_version() {
    assert!(parse_add_coordinate("com.google.guava:guava").is_err());
}

#[test]
fn test_parse_coordinate_empty_parts() {
    assert!(parse_add_coordinate("::").is_err());
    assert!(parse_add_coordinate(":artifact:1.0").is_err());
    assert!(parse_add_coordinate("group::1.0").is_err());
    assert!(parse_add_coordinate("group:artifact:").is_err());
}

#[test]
fn test_parse_coordinate_single_part() {
    assert!(parse_add_coordinate("just-a-name").is_err());
}

#[test]
fn test_parse_coordinate_too_many_parts() {
    // 4+ parts should fail (we only accept groupId:artifactId:version)
    assert!(parse_add_coordinate("a:b:c:d").is_err());
}
